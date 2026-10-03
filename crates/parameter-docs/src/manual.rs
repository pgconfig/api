//! The parameters `config.sgml` documents, with their text in Markdown.

use std::collections::HashMap;

use crate::markup::{self, Element, Node, elements};

/// The elements the manual's HTML puts on a page of their own. A link to any
/// other element points into the page of its nearest chunk.
const CHUNKS: &[&str] = &[
    "appendix",
    "article",
    "bibliography",
    "book",
    "chapter",
    "colophon",
    "glossary",
    "index",
    "part",
    "preface",
    "refentry",
    "reference",
    "sect1",
    "set",
];

/// Inline elements the manual's HTML sets in a monospace font.
const CODE: &[&str] = &[
    "classname",
    "code",
    "command",
    "computeroutput",
    "constant",
    "envar",
    "filename",
    "function",
    "keycap",
    "literal",
    "option",
    "parameter",
    "prompt",
    "property",
    "returnvalue",
    "sgmltag",
    "structfield",
    "structname",
    "symbol",
    "systemitem",
    "token",
    "type",
    "userinput",
    "varname",
];

/// Inline elements the manual's HTML sets in italics.
const EMPHASIS: &[&str] = &[
    "citetitle",
    "emphasis",
    "firstterm",
    "foreignphrase",
    "glossterm",
    "replaceable",
    "wordasword",
];

/// Inline elements that only mark up their text.
const PLAIN: &[&str] = &[
    "abbrev",
    "acronym",
    "application",
    "database",
    "interfacename",
    "orgname",
    "phrase",
    "productname",
    "trademark",
];

/// The elements of the manual that a cross-reference can point to.
pub struct Book {
    targets: HashMap<String, Target>,
}

struct Target {
    /// The id of the page the element is on.
    page: Option<String>,
    /// What a cross-reference to the element reads.
    title: Option<String>,
}

impl Book {
    /// Indexes every element with an `id` in the manual's source files, given
    /// as name and content.
    pub fn index<'a>(files: impl IntoIterator<Item = (&'a str, &'a str)>) -> Result<Book, String> {
        let mut targets = HashMap::new();
        for (name, source) in files {
            let nodes = markup::parse(source, false).map_err(|err| format!("{name}: {err}"))?;
            let mut pages = Vec::new();
            for element in elements(&nodes) {
                index(element, &mut pages, &mut targets);
            }
        }
        Ok(Book { targets })
    }

    /// The address of the element `id` in the manual of the major version.
    fn url(&self, id: &str, major: &str) -> Result<String, String> {
        let page = self
            .targets
            .get(id)
            .ok_or_else(|| format!("nothing in the manual has the id {id}"))?
            .page
            .as_deref()
            .ok_or_else(|| format!("{id} is on no page of the manual"))?;
        let mut url = format!("https://www.postgresql.org/docs/{major}/{page}.html");
        if page != id {
            url.push('#');
            url.push_str(&id.to_ascii_uppercase());
        }
        Ok(url)
    }

    /// What a cross-reference to the element `id` reads.
    fn title(&self, id: &str) -> Result<&str, String> {
        self.targets
            .get(id)
            .ok_or_else(|| format!("nothing in the manual has the id {id}"))?
            .title
            .as_deref()
            .ok_or_else(|| format!("a reference to {id} has no text to show"))
    }
}

fn index(element: &Element, pages: &mut Vec<String>, targets: &mut HashMap<String, Target>) {
    let id = element.id();
    let chunk = id.is_some() && CHUNKS.contains(&element.name.as_str());
    if let (true, Some(id)) = (chunk, &id) {
        pages.push(id.clone());
    }
    if let Some(id) = id {
        let target = Target {
            page: pages.last().cloned(),
            title: title(element),
        };
        targets.insert(id, target);
    }
    for child in element.elements() {
        index(child, pages, targets);
    }
    if chunk {
        pages.pop();
    }
}

/// What a cross-reference to `element` reads, as the manual's HTML writes it,
/// except that a section reads as its title instead of its number.
fn title(element: &Element) -> Option<String> {
    if let Some(label) = element.attr("xreflabel") {
        return Some(collapse(label));
    }
    let text = match element.name.as_str() {
        "varlistentry" => element
            .child("term")
            .map(|term| term.find("varname").unwrap_or(term).text()),
        "refentry" => element.find("refentrytitle").map(Element::text),
        "glossentry" => element.child("glossterm").map(Element::text),
        _ => element
            .child("title")
            .or_else(|| element.child("info").and_then(|info| info.child("title")))
            .map(Element::text),
    }?;
    Some(collapse(&text))
}

/// One parameter as the manual documents it.
#[derive(Debug, PartialEq, Eq)]
pub struct ManualEntry {
    pub name: String,
    /// The type the manual states, such as `integer` or `floating point`.
    pub param_type: Option<String>,
    pub url: String,
    /// The description, in Markdown.
    pub text: String,
}

/// Reads the parameters `config` documents for the major version `major`.
/// Every `varlistentry` whose id starts with `guc-` documents the parameters
/// its terms name.
pub fn entries(config: &str, book: &Book, major: &str) -> Result<Vec<ManualEntry>, String> {
    let nodes = markup::parse(config, true)?;
    let mut found = Vec::new();
    for element in elements(&nodes) {
        collect(element, &mut found);
    }

    let renderer = Renderer { book, major };
    let mut entries = Vec::new();
    for entry in found {
        let id = entry.id().expect("a parameter entry has an id");
        let context = |err: String| format!("{id}: {err}");
        let url = book.url(&id, major).map_err(context)?;
        let item = entry
            .child("listitem")
            .ok_or_else(|| context("it has no description".into()))?;
        let text = renderer
            .blocks(&item.children)
            .map_err(context)?
            .join("\n\n");

        let mut named = false;
        for term in entry.elements().filter(|element| element.name == "term") {
            let Some(name) = term.find("varname") else {
                continue;
            };
            named = true;
            let name = collapse(&name.text());
            // A parameter documented in two places, such as for a sending
            // server and for a subscriber, reads as both.
            if let Some(earlier) = entries
                .iter_mut()
                .find(|earlier: &&mut ManualEntry| earlier.name == name)
            {
                earlier.text = format!("{}\n\n{text}", earlier.text);
                continue;
            }
            entries.push(ManualEntry {
                name,
                param_type: term.find("type").map(|kind| collapse(&kind.text())),
                url: url.clone(),
                text: text.clone(),
            });
        }
        if !named {
            return Err(context("no term names a parameter".into()));
        }
    }
    Ok(entries)
}

/// Gathers the parameter entries. An entry nested in another one's
/// description is part of that description, not a parameter.
fn collect<'a>(element: &'a Element, found: &mut Vec<&'a Element>) {
    if element.name == "varlistentry" && element.id().is_some_and(|id| id.starts_with("guc-")) {
        found.push(element);
        return;
    }
    for child in element.elements() {
        collect(child, found);
    }
}

/// Turns DocBook into Markdown.
struct Renderer<'a> {
    book: &'a Book,
    major: &'a str,
}

impl Renderer<'_> {
    /// Renders block content. Inline content between blocks becomes a
    /// paragraph, as DocBook lets a `para` hold a list.
    fn blocks(&self, nodes: &[Node]) -> Result<Vec<String>, String> {
        self.blocks_of(&nodes.iter().collect::<Vec<_>>())
    }

    fn blocks_of(&self, nodes: &[&Node]) -> Result<Vec<String>, String> {
        let mut blocks = Vec::new();
        let mut run: Vec<&Node> = Vec::new();
        for &node in nodes {
            match node {
                Node::Element(element) if is_block(element) => {
                    self.paragraph(&mut run, &mut blocks)?;
                    blocks.extend(self.block(element)?);
                }
                _ => run.push(node),
            }
        }
        self.paragraph(&mut run, &mut blocks)?;
        Ok(blocks)
    }

    fn paragraph(&self, run: &mut Vec<&Node>, blocks: &mut Vec<String>) -> Result<(), String> {
        let mut inline = Inline::default();
        for node in run.drain(..) {
            self.inline(node, &mut inline)?;
        }
        let text = inline.finish();
        if !text.is_empty() {
            blocks.push(guard_line_start(&text));
        }
        Ok(())
    }

    fn block(&self, element: &Element) -> Result<Vec<String>, String> {
        match element.name.as_str() {
            "para" | "simpara" => self.blocks(&element.children),
            "itemizedlist" => {
                let items = self.items(element, "listitem")?;
                Ok(vec![list(&items, |_| "- ".to_string())])
            }
            "orderedlist" => {
                let items = self.items(element, "listitem")?;
                Ok(vec![list(&items, |n| format!("{}. ", n + 1))])
            }
            "simplelist" => {
                let items = self.items(element, "member")?;
                Ok(vec![list(&items, |_| "- ".to_string())])
            }
            "variablelist" => {
                let mut items = Vec::new();
                for entry in element
                    .elements()
                    .filter(|child| child.name == "varlistentry")
                {
                    items.push(self.definition(entry)?);
                }
                Ok(vec![list(&items, |_| "- ".to_string())])
            }
            "programlisting" | "screen" | "synopsis" | "literallayout" => {
                let text = element.text();
                let text = text.strip_prefix('\n').unwrap_or(&text).trim_end();
                let fence = "`".repeat(longest_backtick_run(text).max(2) + 1);
                Ok(vec![format!("{fence}\n{text}\n{fence}")])
            }
            "table" | "informaltable" => {
                let mut blocks = Vec::new();
                if let Some(title) = element.child("title") {
                    blocks.push(format!("**{}**", escape(&collapse(&title.text()))));
                }
                for group in element.elements().filter(|child| child.name == "tgroup") {
                    blocks.push(self.table(group)?);
                }
                Ok(blocks)
            }
            "note" | "tip" | "warning" | "caution" | "important" => {
                let kind = element.name.to_ascii_uppercase();
                let mut blocks = Vec::new();
                if let Some(title) = element.child("title") {
                    blocks.push(format!("**{}**", escape(&collapse(&title.text()))));
                }
                let body: Vec<&Node> = element
                    .children
                    .iter()
                    .filter(|node| !matches!(node, Node::Element(child) if child.name == "title"))
                    .collect();
                blocks.extend(self.blocks_of(&body)?);
                let mut alert = format!("> [!{kind}]");
                for line in blocks.join("\n\n").lines() {
                    alert.push('\n');
                    alert.push('>');
                    if !line.is_empty() {
                        alert.push(' ');
                        alert.push_str(line);
                    }
                }
                Ok(vec![alert])
            }
            name => Err(format!("<{name}> is not rendered")),
        }
    }

    /// A Markdown table. Without a `thead`, the first row is the header.
    fn table(&self, group: &Element) -> Result<String, String> {
        let mut rows = Vec::new();
        for part in ["thead", "tbody"] {
            for section in group.elements().filter(|child| child.name == part) {
                for row in section.elements().filter(|child| child.name == "row") {
                    let mut cells = Vec::new();
                    for entry in row.elements().filter(|child| child.name == "entry") {
                        let text = self.blocks(&entry.children)?.join(" ");
                        cells.push(text.replace('\n', " ").replace('|', "\\|"));
                    }
                    rows.push(cells);
                }
            }
        }
        let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
        let line = |cells: &[String]| {
            let mut cells = cells.to_vec();
            cells.resize(columns, String::new());
            format!("| {} |", cells.join(" | "))
        };
        let mut lines = Vec::new();
        for (n, row) in rows.iter().enumerate() {
            lines.push(line(row));
            if n == 0 {
                lines.push(line(&vec!["---".to_string(); columns]));
            }
        }
        Ok(lines.join("\n"))
    }

    /// The blocks of each `child` element of a list.
    fn items(&self, element: &Element, child: &str) -> Result<Vec<Vec<String>>, String> {
        element
            .elements()
            .filter(|item| item.name == child)
            .map(|item| self.blocks(&item.children))
            .collect()
    }

    /// One entry of a variable list: its terms, then its description, which
    /// starts on the terms' line when it starts with a paragraph.
    fn definition(&self, entry: &Element) -> Result<Vec<String>, String> {
        let mut terms = Vec::new();
        for term in entry.elements().filter(|child| child.name == "term") {
            let mut inline = Inline::default();
            for node in &term.children {
                self.inline(node, &mut inline)?;
            }
            terms.push(inline.finish());
        }
        let terms = terms.join(", ");
        let Some(item) = entry.child("listitem") else {
            return Ok(vec![terms]);
        };
        let mut blocks = self.blocks(&item.children)?;
        let starts_with_paragraph = item
            .elements()
            .next()
            .is_some_and(|first| matches!(first.name.as_str(), "para" | "simpara"));
        match blocks.first_mut() {
            Some(first) if starts_with_paragraph => *first = format!("{terms}: {first}"),
            _ => blocks.insert(0, terms),
        }
        Ok(blocks)
    }

    fn inline(&self, node: &Node, out: &mut Inline) -> Result<(), String> {
        let element = match node {
            Node::Text(text) => {
                out.text(text);
                return Ok(());
            }
            Node::Element(element) => element,
        };
        match element.name.as_str() {
            "indexterm" | "remark" => {}
            name if CODE.contains(&name) => out.code(&element.text()),
            "glossterm" if element.attr("linkend").is_some() => {
                let url = self.book.url(&linkend(element)?, self.major)?;
                self.link(element, &url, out)?;
            }
            name if EMPHASIS.contains(&name) => {
                let strong = matches!(element.attr("role"), Some("bold" | "strong"));
                let mark = if strong { "**" } else { "*" };
                self.wrap(element, mark, mark, out)?;
            }
            name if PLAIN.contains(&name) => {
                for node in &element.children {
                    self.inline(node, out)?;
                }
            }
            "quote" => self.wrap(element, "\"", "\"", out)?,
            "simplelist" => {
                for (n, member) in element
                    .elements()
                    .filter(|child| child.name == "member")
                    .enumerate()
                {
                    if n > 0 {
                        out.text(", ");
                    }
                    for node in &member.children {
                        self.inline(node, out)?;
                    }
                }
            }
            "superscript" => self.wrap(element, "^", "", out)?,
            "citerefentry" => {
                let title = element
                    .find("refentrytitle")
                    .map(Element::text)
                    .unwrap_or_default();
                let volume = element
                    .find("manvolnum")
                    .map(Element::text)
                    .unwrap_or_default();
                out.text(&format!("{}({})", collapse(&title), collapse(&volume)));
            }
            "xref" => {
                let id = linkend(element)?;
                let url = self.book.url(&id, self.major)?;
                let label = element
                    .attr("endterm")
                    .map_or(id.clone(), str::to_ascii_lowercase);
                let title = self.book.title(&label)?;
                // A parameter reads as its name, which is code.
                let text = if id.starts_with("guc-") {
                    code_span(title)
                } else {
                    escape(title)
                };
                out.markdown(&format!("[{text}]({url})"));
            }
            "link" => {
                let url = match (element.attr("linkend"), element.attr("xlink:href")) {
                    (Some(_), _) => self.book.url(&linkend(element)?, self.major)?,
                    (None, Some(href)) => href.to_string(),
                    (None, None) => return Err("a <link> points nowhere".into()),
                };
                self.link(element, &url, out)?;
            }
            "ulink" => {
                let url = element.attr("url").ok_or("a <ulink> has no url")?;
                self.link(element, url, out)?;
            }
            name => return Err(format!("<{name}> is not rendered")),
        }
        Ok(())
    }

    /// The element's content between `open` and `close`. Spaces at its
    /// edges stay outside, where Markdown expects them.
    fn wrap(
        &self,
        element: &Element,
        open: &str,
        close: &str,
        out: &mut Inline,
    ) -> Result<(), String> {
        let mut inner = Inline::default();
        for node in &element.children {
            self.inline(node, &mut inner)?;
        }
        let text = inner.finish();
        if !text.is_empty() {
            out.markdown(&format!("{open}{text}{close}"));
        }
        Ok(())
    }

    /// A link whose text is the element's content, or the address itself.
    fn link(&self, element: &Element, url: &str, out: &mut Inline) -> Result<(), String> {
        let mut inner = Inline::default();
        for node in &element.children {
            self.inline(node, &mut inner)?;
        }
        let text = inner.finish();
        let text = if text.is_empty() { escape(url) } else { text };
        let destination = if url.contains(|c: char| c.is_whitespace() || c == '(' || c == ')') {
            format!("<{url}>")
        } else {
            url.to_string()
        };
        out.markdown(&format!("[{text}]({destination})"));
        Ok(())
    }
}

fn linkend(element: &Element) -> Result<String, String> {
    element
        .attr("linkend")
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| format!("an <{}> has no linkend", element.name))
}

fn is_block(element: &Element) -> bool {
    if element.name == "simplelist" {
        return element.attr("type") != Some("inline");
    }
    matches!(
        element.name.as_str(),
        "para"
            | "simpara"
            | "itemizedlist"
            | "orderedlist"
            | "variablelist"
            | "programlisting"
            | "screen"
            | "synopsis"
            | "literallayout"
            | "table"
            | "informaltable"
            | "note"
            | "tip"
            | "warning"
            | "caution"
            | "important"
    )
}

/// A Markdown list. Its items sit on consecutive lines when each is one
/// block, and are separated by blank lines otherwise.
fn list(items: &[Vec<String>], marker: impl Fn(usize) -> String) -> String {
    let tight = items.iter().all(|blocks| blocks.len() <= 1);
    let rendered: Vec<String> = items
        .iter()
        .enumerate()
        .map(|(n, blocks)| {
            let marker = marker(n);
            let indent = " ".repeat(marker.len());
            let mut text = marker;
            for (i, line) in blocks.join("\n\n").lines().enumerate() {
                if i > 0 {
                    text.push('\n');
                    if !line.is_empty() {
                        text.push_str(&indent);
                    }
                }
                text.push_str(line);
            }
            text
        })
        .collect();
    rendered.join(if tight { "\n" } else { "\n\n" })
}

/// Builds one paragraph of Markdown from text, code, and finished Markdown.
#[derive(Default)]
struct Inline {
    out: String,
    /// Code waiting to be written, so that adjacent code elements become one
    /// span: Markdown would read two touching spans as one with backticks.
    code: Option<String>,
}

impl Inline {
    fn text(&mut self, raw: &str) {
        let text = collapse_spaces(raw);
        if text.is_empty() {
            return;
        }
        self.flush();
        let text = match (self.out.ends_with(' '), text.strip_prefix(' ')) {
            (true, Some(rest)) => rest,
            _ => &text,
        };
        self.out.push_str(&escape(text));
    }

    /// Appends Markdown that is already escaped.
    fn markdown(&mut self, markdown: &str) {
        self.flush();
        self.out.push_str(markdown);
    }

    fn code(&mut self, raw: &str) {
        let code = collapse_spaces(raw);
        if code.is_empty() {
            return;
        }
        self.code.get_or_insert_with(String::new).push_str(&code);
    }

    fn flush(&mut self) {
        if let Some(code) = self.code.take() {
            self.out.push_str(&code_span(&code));
        }
    }

    fn finish(mut self) -> String {
        self.flush();
        self.out.trim().to_string()
    }
}

/// The length of the longest run of backticks in `text`, which a fence
/// around it has to exceed.
fn longest_backtick_run(text: &str) -> usize {
    text.split(|c| c != '`').map(str::len).max().unwrap_or(0)
}

/// A code span that holds `code` whatever backticks it contains.
fn code_span(code: &str) -> String {
    let fence = "`".repeat(longest_backtick_run(code) + 1);
    let pad = if code.starts_with('`') || code.ends_with('`') {
        " "
    } else {
        ""
    };
    format!("{fence}{pad}{code}{pad}{fence}")
}

/// Every run of whitespace as one space.
fn collapse_spaces(raw: &str) -> String {
    let mut text = String::with_capacity(raw.len());
    let mut space = false;
    for c in raw.chars() {
        if c.is_whitespace() && c != '\u{a0}' {
            space = true;
        } else {
            if space {
                text.push(' ');
                space = false;
            }
            text.push(c);
        }
    }
    if space {
        text.push(' ');
    }
    text
}

fn collapse(raw: &str) -> String {
    collapse_spaces(raw).trim().to_string()
}

/// Escapes the characters Markdown would read as markup.
fn escape(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    for (i, &c) in chars.iter().enumerate() {
        let before = i.checked_sub(1).and_then(|j| chars.get(j)).copied();
        let after = chars.get(i + 1).copied();
        let escaped = match c {
            '\\' | '`' | '*' | '[' | ']' | '~' => true,
            '_' => {
                !(before.is_some_and(char::is_alphanumeric)
                    && after.is_some_and(char::is_alphanumeric))
            }
            '<' => after
                .is_some_and(|next| next.is_ascii_alphabetic() || matches!(next, '/' | '!' | '?')),
            '&' => {
                let rest: String = chars[i + 1..]
                    .iter()
                    .take_while(|c| c.is_ascii_alphanumeric() || **c == '#')
                    .collect();
                !rest.is_empty() && chars.get(i + 1 + rest.chars().count()) == Some(&';')
            }
            _ => false,
        };
        if escaped {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Escapes what would make a paragraph's first characters a heading, a
/// quote, or a list.
fn guard_line_start(text: &str) -> String {
    let digits = text.chars().take_while(char::is_ascii_digit).count();
    let after_digits = &text[digits..];
    let list_number =
        digits > 0 && (after_digits.starts_with(". ") || after_digits.starts_with(") "));
    if text.starts_with(['#', '>', '=']) || text.starts_with("- ") || text.starts_with("+ ") {
        format!("\\{text}")
    } else if list_number {
        format!("{}\\{}", &text[..digits], after_digits)
    } else {
        text.to_string()
    }
}
