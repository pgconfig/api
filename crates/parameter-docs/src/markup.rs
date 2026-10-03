//! A tolerant reader for the manual's markup. The sources are DocBook XML from
//! PostgreSQL 11 on and DocBook SGML before it, where `</>` closes the open
//! element and an empty element such as `<xref linkend="x">` has no end tag.

use std::fmt::Write as _;

#[derive(Debug)]
pub(crate) enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug)]
pub(crate) struct Element {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<Node>,
}

/// The elements DocBook declares empty. SGML writes them without an end tag.
const EMPTY: &[&str] = &[
    "anchor",
    "area",
    "beginpage",
    "co",
    "colspec",
    "coref",
    "footnoteref",
    "imagedata",
    "sbr",
    "spanspec",
    "varargs",
    "void",
    "xref",
];

impl Element {
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    pub fn id(&self) -> Option<String> {
        self.attr("id").map(str::to_ascii_lowercase)
    }

    /// The child elements, in order.
    pub fn elements(&self) -> impl Iterator<Item = &Element> {
        elements(&self.children)
    }

    pub fn child(&self, name: &str) -> Option<&Element> {
        self.elements().find(|element| element.name == name)
    }

    /// The first descendant named `name`, depth first.
    pub fn find(&self, name: &str) -> Option<&Element> {
        self.elements().find_map(|element| {
            (element.name == name)
                .then_some(element)
                .or_else(|| element.find(name))
        })
    }

    /// Every text below the element, in document order, without index terms.
    pub fn text(&self) -> String {
        let mut text = String::new();
        self.collect_text(&mut text);
        text
    }

    fn collect_text(&self, text: &mut String) {
        for node in &self.children {
            match node {
                Node::Text(chunk) => text.push_str(chunk),
                Node::Element(element) if element.name == "indexterm" => {}
                Node::Element(element) => element.collect_text(text),
            }
        }
    }
}

/// Parses one source file. An entity the manual does not define is an error
/// when `strict`, and dropped otherwise: the other files reference their
/// includes as entities, and only their ids and titles matter.
pub(crate) fn parse(source: &str, strict: bool) -> Result<Vec<Node>, String> {
    Parser {
        source,
        at: 0,
        strict,
    }
    .run()
}

struct Parser<'a> {
    source: &'a str,
    at: usize,
    strict: bool,
}

impl Parser<'_> {
    fn run(mut self) -> Result<Vec<Node>, String> {
        let mut root = Vec::new();
        let mut open: Vec<Element> = Vec::new();

        while self.at < self.source.len() {
            let rest = &self.source[self.at..];
            if let Some(after) = rest.strip_prefix("<!--") {
                let end = after
                    .find("-->")
                    .ok_or_else(|| self.error("an unclosed comment"))?;
                self.at += 4 + end + 3;
            } else if let Some(after) = rest.strip_prefix("<![CDATA[") {
                let end = after
                    .find("]]>")
                    .ok_or_else(|| self.error("an unclosed CDATA"))?;
                push(&mut open, &mut root, Node::Text(after[..end].to_string()));
                self.at += 9 + end + 3;
            } else if let Some(section) = rest.strip_prefix("<![") {
                // A marked section: its content is read as if it were not
                // marked, and `]]>` ends it.
                let start = section
                    .find('[')
                    .ok_or_else(|| self.error("a bad marked section"))?;
                self.at += 3 + start + 1;
            } else if rest.starts_with("]]>") {
                self.at += 3;
            } else if rest.starts_with("<!") {
                self.at += declaration_length(rest)
                    .ok_or_else(|| self.error("an unclosed declaration"))?;
            } else if rest.starts_with("<?") {
                let end = rest
                    .find('>')
                    .ok_or_else(|| self.error("an unclosed instruction"))?;
                self.at += end + 1;
            } else if let Some(after) = rest.strip_prefix("</") {
                let end = after
                    .find('>')
                    .ok_or_else(|| self.error("an unclosed end tag"))?;
                let name = after[..end].trim().to_ascii_lowercase();
                self.close(&mut open, &mut root, &name)?;
                self.at += 2 + end + 1;
            } else if rest.starts_with('<')
                && rest[1..].starts_with(|c: char| c.is_ascii_alphabetic())
            {
                let (element, empty) = self.start_tag()?;
                if empty || EMPTY.contains(&element.name.as_str()) {
                    push(&mut open, &mut root, Node::Element(element));
                } else {
                    open.push(element);
                }
            } else {
                // At least the first character is text, whatever it is.
                let first = rest.chars().next().map_or(1, char::len_utf8);
                let end = rest[first..]
                    .find(['<', ']'])
                    .map_or(rest.len(), |end| end + first);
                let text = self.decode(&rest[..end])?;
                push(&mut open, &mut root, Node::Text(text));
                self.at += end;
            }
        }

        if let Some(element) = open.last() {
            return Err(format!("<{}> is never closed", element.name));
        }
        Ok(root)
    }

    /// Closes the open element `name`, and any element opened inside it that
    /// was left open. An empty name, the SGML `</>`, closes the innermost.
    fn close(
        &self,
        open: &mut Vec<Element>,
        root: &mut Vec<Node>,
        name: &str,
    ) -> Result<(), String> {
        let depth = if name.is_empty() {
            open.len().checked_sub(1)
        } else {
            open.iter().rposition(|element| element.name == name)
        };
        let depth = depth.ok_or_else(|| self.error(&format!("</{name}> closes nothing")))?;
        while open.len() > depth {
            let element = open.pop().expect("an open element");
            push(open, root, Node::Element(element));
        }
        Ok(())
    }

    /// Reads `<name attr="value" ...>` or `<name .../>` at the cursor.
    fn start_tag(&mut self) -> Result<(Element, bool), String> {
        let tag_start = self.at;
        let bytes = self.source.as_bytes();
        let mut at = self.at + 1;
        let name_end = self.source[at..]
            .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ':' | '.')))
            .map_or(self.source.len(), |end| at + end);
        let name = self.source[at..name_end].to_ascii_lowercase();
        at = name_end;

        let mut attrs = Vec::new();
        loop {
            while at < bytes.len() && bytes[at].is_ascii_whitespace() {
                at += 1;
            }
            match bytes.get(at) {
                None => {
                    self.at = tag_start;
                    return Err(self.error("an unclosed start tag"));
                }
                Some(b'>') => {
                    self.at = at + 1;
                    return Ok((element(name, attrs), false));
                }
                Some(b'/') if bytes.get(at + 1) == Some(&b'>') => {
                    self.at = at + 2;
                    return Ok((element(name, attrs), true));
                }
                _ => {}
            }
            let key_end = self.source[at..]
                .find(|c: char| c.is_ascii_whitespace() || matches!(c, '=' | '>' | '/'))
                .map_or(self.source.len(), |end| at + end);
            let key = self.source[at..key_end].to_ascii_lowercase();
            if key.is_empty() {
                self.at = at;
                return Err(self.error("a bad attribute"));
            }
            at = key_end;
            while at < bytes.len() && bytes[at].is_ascii_whitespace() {
                at += 1;
            }
            if bytes.get(at) != Some(&b'=') {
                // An SGML attribute given by its value alone.
                attrs.push((key.clone(), key));
                continue;
            }
            at += 1;
            while at < bytes.len() && bytes[at].is_ascii_whitespace() {
                at += 1;
            }
            let value = match bytes.get(at) {
                Some(&quote @ (b'"' | b'\'')) => {
                    let end = self.source[at + 1..]
                        .find(quote as char)
                        .map(|end| at + 1 + end)
                        .ok_or_else(|| self.error("an unclosed attribute value"))?;
                    let raw = &self.source[at + 1..end];
                    at = end + 1;
                    raw
                }
                _ => {
                    let end = self.source[at..]
                        .find(|c: char| c.is_ascii_whitespace() || c == '>')
                        .map_or(self.source.len(), |end| at + end);
                    let raw = &self.source[at..end];
                    at = end;
                    raw
                }
            };
            let value = self.decode(value)?;
            attrs.push((key, value));
        }
    }

    /// Replaces the entity references in `raw`.
    fn decode(&self, raw: &str) -> Result<String, String> {
        let mut text = String::with_capacity(raw.len());
        let mut rest = raw;
        while let Some(start) = rest.find('&') {
            text.push_str(&rest[..start]);
            rest = &rest[start..];
            let name_end = rest[1..]
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '#' || c == '.'))
                .map(|end| end + 1);
            match name_end {
                Some(end) if end > 1 && rest[end..].starts_with(';') => {
                    let name = &rest[1..end];
                    match entity(name) {
                        Some(value) => text.push_str(&value),
                        None if self.strict => {
                            return Err(self.error(&format!("the entity &{name}; is not known")));
                        }
                        None => {}
                    }
                    rest = &rest[end + 1..];
                }
                _ => {
                    text.push('&');
                    rest = &rest[1..];
                }
            }
        }
        text.push_str(rest);
        Ok(text)
    }

    fn error(&self, what: &str) -> String {
        let line = self.source[..self.at.min(self.source.len())]
            .bytes()
            .filter(|&b| b == b'\n')
            .count()
            + 1;
        let mut message = String::new();
        let _ = write!(message, "line {line}: {what}");
        message
    }
}

/// The elements among `nodes`, in order.
pub(crate) fn elements(nodes: &[Node]) -> impl Iterator<Item = &Element> {
    nodes.iter().filter_map(|node| match node {
        Node::Element(element) => Some(element),
        Node::Text(_) => None,
    })
}

fn element(name: String, attrs: Vec<(String, String)>) -> Element {
    Element {
        name,
        attrs,
        children: Vec::new(),
    }
}

fn push(open: &mut [Element], root: &mut Vec<Node>, node: Node) {
    let children = match open.last_mut() {
        Some(parent) => &mut parent.children,
        None => root,
    };
    // Adjacent text, split by a skipped comment or a `]`, reads as one.
    if let (Node::Text(text), Some(Node::Text(previous))) = (&node, children.last_mut()) {
        previous.push_str(text);
        return;
    }
    children.push(node);
}

/// The length of a `<!...>` declaration, an internal subset included.
fn declaration_length(rest: &str) -> Option<usize> {
    let close = rest.find('>')?;
    match rest.find('[') {
        Some(open) if open < close => rest.find("]>").map(|end| end + 2),
        _ => Some(close + 1),
    }
}

/// The value of a character or named entity the manual uses.
fn entity(name: &str) -> Option<String> {
    if let Some(number) = name.strip_prefix('#') {
        let code = match number.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok()?,
            None => number.parse().ok()?,
        };
        return char::from_u32(code).map(String::from);
    }
    let value = match name {
        "amp" => "&",
        "lt" => "<",
        "gt" => ">",
        "quot" => "\"",
        "apos" => "'",
        "nbsp" => "\u{a0}",
        "mdash" => "\u{2014}",
        "ndash" => "\u{2013}",
        "hellip" => "\u{2026}",
        "bull" => "\u{2022}",
        "copy" => "\u{a9}",
        "reg" => "\u{ae}",
        "trade" => "\u{2122}",
        "times" => "\u{d7}",
        "minus" => "\u{2212}",
        "deg" => "\u{b0}",
        "plusmn" => "\u{b1}",
        "le" => "\u{2264}",
        "ge" => "\u{2265}",
        "ne" => "\u{2260}",
        "larr" => "\u{2190}",
        "rarr" => "\u{2192}",
        "percnt" => "%",
        "num" => "#",
        "dollar" => "$",
        "lowbar" => "_",
        "ast" => "*",
        "commat" => "@",
        "lsqb" => "[",
        "rsqb" => "]",
        "lcub" => "{",
        "rcub" => "}",
        "verbar" => "|",
        "sect" => "\u{a7}",
        "middot" => "\u{b7}",
        _ => return None,
    };
    Some(value.to_string())
}
