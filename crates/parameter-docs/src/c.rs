//! Just enough C to read the GUC tables: tokens, `#if` blocks, brace
//! initializers, and constant expressions.

use std::collections::HashMap;

use crate::platform;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    Ident(String),
    Number(String),
    Str(String),
    Char(String),
    Punct(&'static str),
}

impl Token {
    pub fn is(&self, punct: &str) -> bool {
        matches!(self, Token::Punct(p) if *p == punct)
    }

    pub fn ident(&self) -> Option<&str> {
        match self {
            Token::Ident(name) => Some(name),
            _ => None,
        }
    }
}

/// Longer punctuators first, so `<<` is not read as two `<`.
const PUNCTUATORS: &[&str] = &[
    "...", "<<", ">>", "<=", ">=", "==", "!=", "&&", "||", "->", "++", "--", "##", "{", "}", "(",
    ")", "[", "]", ",", ";", "&", "|", "*", "/", "+", "-", "<", ">", "=", "!", "?", ":", ".", "~",
    "^", "%", "#",
];

pub(crate) fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &source[i..];
        let c = bytes[i];
        if c.is_ascii_whitespace() || c == b'\\' {
            i += 1;
        } else if let Some(comment) = rest.strip_prefix("/*") {
            let end = comment.find("*/").ok_or("an unclosed comment")?;
            i += 2 + end + 2;
        } else if rest.starts_with("//") {
            i += rest.find('\n').unwrap_or(rest.len());
        } else if c == b'"' || c == b'\'' {
            let (text, length) = literal(rest)?;
            tokens.push(if c == b'"' {
                Token::Str(text)
            } else {
                Token::Char(text)
            });
            i += length;
        } else if c.is_ascii_digit()
            || (c == b'.' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit))
        {
            let hex = rest.starts_with("0x") || rest.starts_with("0X");
            let start = i;
            i += 1;
            while i < bytes.len() {
                let b = bytes[i];
                let exponent_sign =
                    !hex && matches!(b, b'+' | b'-') && matches!(bytes[i - 1], b'e' | b'E');
                if b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || exponent_sign {
                    i += 1;
                } else {
                    break;
                }
            }
            tokens.push(Token::Number(source[start..i].to_string()));
        } else if c.is_ascii_alphabetic() || c == b'_' {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            tokens.push(Token::Ident(source[start..i].to_string()));
        } else {
            let punctuator = PUNCTUATORS
                .iter()
                .find(|punctuator| rest.starts_with(**punctuator))
                .ok_or_else(|| format!("cannot read C at {:?}", &rest[..rest.len().min(20)]))?;
            tokens.push(Token::Punct(punctuator));
            i += punctuator.len();
        }
    }
    Ok(tokens)
}

/// A string or character literal at the start of `source`, decoded, and its
/// length in the source.
fn literal(source: &str) -> Result<(String, usize), String> {
    let quote = source.chars().next().expect("a quote");
    let mut text = String::new();
    let mut chars = source.char_indices().skip(1);
    while let Some((at, c)) = chars.next() {
        match c {
            '\\' => {
                let (_, escaped) = chars.next().ok_or("an unclosed literal")?;
                match escaped {
                    'n' => text.push('\n'),
                    't' => text.push('\t'),
                    'r' => text.push('\r'),
                    '0' => text.push('\0'),
                    other => text.push(other),
                }
            }
            '\n' => return Err("a line break inside a literal".into()),
            c if c == quote => return Ok((text, at + 1)),
            c => text.push(c),
        }
    }
    Err("an unclosed literal".into())
}

/// The macros the tables use: the platform's, then the headers'.
pub(crate) struct Macros {
    /// Every definition of each object-like macro, with the `#if`
    /// conditions around it.
    defines: HashMap<String, Vec<Define>>,
    /// What configure derives from the release number.
    release: Vec<(&'static str, String)>,
}

struct Define {
    value: String,
    /// Conditions that all hold where the definition is read.
    conditions: Vec<String>,
}

impl Macros {
    /// Reads the `#define` lines of C files, for the release `release`, such
    /// as `18.6`. A macro with parameters is left out.
    pub fn new<'a>(
        files: impl IntoIterator<Item = &'a str>,
        release: &str,
    ) -> Result<Self, String> {
        let numbers: Vec<u32> = release
            .split('.')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(|_| format!("{release} is not a release number"))?;
        let (major, number) = match numbers[..] {
            [9, minor, patch] => (format!("9.{minor}"), 90_000 + minor * 100 + patch),
            [major, patch] if major >= 10 => (major.to_string(), major * 10_000 + patch),
            _ => return Err(format!("{release} is not a release number")),
        };
        let release = vec![
            ("PG_VERSION", format!("{release:?}")),
            ("PG_VERSION_NUM", number.to_string()),
            ("PG_MAJORVERSION", format!("{major:?}")),
            ("PG_MAJORVERSION_NUM", numbers[0].to_string()),
        ];

        let mut defines: HashMap<String, Vec<Define>> = HashMap::new();
        for file in files {
            // Per open `#if`: the conditions of its earlier branches, and the
            // conditions that hold in the current one.
            let mut levels: Vec<(Vec<String>, Vec<String>)> = Vec::new();
            let mut lines = file.lines();
            while let Some(line) = lines.next() {
                let Some(directive) = line.trim_start().strip_prefix('#') else {
                    continue;
                };
                let mut directive = directive.trim_end().to_string();
                while directive.ends_with('\\') {
                    directive.pop();
                    directive.push(' ');
                    directive.push_str(lines.next().unwrap_or_default().trim());
                }
                let directive = strip_comments(&directive);
                let directive = directive.trim();
                let (keyword, argument) = directive
                    .split_once(|c: char| c.is_whitespace())
                    .map_or((directive, ""), |(keyword, argument)| {
                        (keyword, argument.trim())
                    });
                match keyword {
                    "if" => levels.push((Vec::new(), vec![format!("({argument})")])),
                    "ifdef" => levels.push((Vec::new(), vec![format!("defined({argument})")])),
                    "ifndef" => levels.push((Vec::new(), vec![format!("!defined({argument})")])),
                    "elif" | "else" => {
                        let Some((earlier, current)) = levels.last_mut() else {
                            continue;
                        };
                        if let Some(condition) = current.last() {
                            earlier.push(format!("!{condition}"));
                        }
                        *current = earlier.clone();
                        if keyword == "elif" {
                            current.push(format!("({argument})"));
                        }
                    }
                    "endif" => {
                        levels.pop();
                    }
                    "define" => {
                        let name_end = argument
                            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                            .unwrap_or(argument.len());
                        let (name, value) = argument.split_at(name_end);
                        // `#ifndef X_H` then `#define X_H` guards the
                        // header against a second include: no condition.
                        if let Some((earlier, current)) = levels.last_mut()
                            && earlier.is_empty()
                            && *current == [format!("!defined({name})")]
                        {
                            current.clear();
                        }
                        if name.is_empty() || value.starts_with('(') {
                            continue;
                        }
                        let conditions = levels
                            .iter()
                            .flat_map(|(_, current)| current.iter().cloned())
                            .collect();
                        defines.entry(name.to_string()).or_default().push(Define {
                            value: value.trim().to_string(),
                            conditions,
                        });
                    }
                    _ => {}
                }
            }
        }
        Ok(Macros { defines, release })
    }

    /// Whether `#ifdef name` holds. Only the platform table answers, so every
    /// condition the extraction depends on is a recorded decision.
    pub fn defined(&self, name: &str) -> Result<bool, String> {
        platform::macro_value(name)
            .map(|value| value.is_some())
            .ok_or_else(|| unknown_condition(name))
    }

    /// The definition of a macro used as a value: the platform table's, or
    /// the headers'. Definitions that differ are told apart by the
    /// conditions around them.
    pub fn definition(&self, name: &str) -> Result<Option<&str>, String> {
        if let Some(value) = platform::macro_value(name) {
            return Ok(value);
        }
        if let Some((_, value)) = self
            .release
            .iter()
            .find(|(macro_name, _)| *macro_name == name)
        {
            return Ok(Some(value));
        }
        let Some(defines) = self.defines.get(name) else {
            return Ok(None);
        };
        // Even a single definition may sit in a block that Linux skips, such
        // as a fallback for a system that lacks the macro.
        let mut chosen: Option<&str> = None;
        for define in defines {
            let mut holds = true;
            for expression in &define.conditions {
                holds &= condition(expression, self)
                    .map_err(|err| format!("{name}, defined under {expression}: {err}"))?;
            }
            match chosen {
                _ if !holds => {}
                Some(value) if value != define.value => {
                    return Err(format!(
                        "{name} has several definitions on Linux. Add the one a 64-bit Linux build uses to the platform table"
                    ));
                }
                _ => chosen = Some(&define.value),
            }
        }
        Ok(chosen)
    }
}

fn unknown_condition(name: &str) -> String {
    format!(
        "the preprocessor condition {name} is not in the platform table. Add whether a standard 64-bit Linux build defines it"
    )
}

fn strip_comments(value: &str) -> String {
    let mut out = String::new();
    let mut rest = value;
    loop {
        let block = rest.find("/*");
        let line = rest.find("//");
        match (block, line) {
            (Some(b), l) if l.is_none_or(|l| b < l) => {
                out.push_str(&rest[..b]);
                match rest[b..].find("*/") {
                    Some(end) => rest = &rest[b + end + 2..],
                    None => return out,
                }
            }
            (_, Some(l)) => {
                out.push_str(&rest[..l]);
                return out;
            }
            _ => {
                out.push_str(rest);
                return out;
            }
        }
    }
}

/// The initializer of the array `name`, from its opening brace to the
/// closing one, with the lines that `#if` blocks leave out removed. `None`
/// when `source` does not define the array.
pub(crate) fn initializer(
    source: &str,
    name: &str,
    macros: &Macros,
) -> Result<Option<String>, String> {
    let needle = format!("{name}[]");
    let mut from = 0;
    let equals = loop {
        let Some(at) = source[from..].find(&needle).map(|at| from + at) else {
            return Ok(None);
        };
        let starts_word = source[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '_'));
        let after = &source[at + needle.len()..];
        let gap = after.len() - after.trim_start().len();
        if starts_word && after.trim_start().starts_with('=') {
            break at + needle.len() + gap;
        }
        from = at + needle.len();
    };

    let mut branches: Vec<Branch> = Vec::new();
    let mut out = String::new();
    let mut scan = Scan::default();
    let mut lines = source[equals + 1..].split_inclusive('\n');
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') && !scan.in_comment {
            let mut directive = trimmed.trim_end().to_string();
            while directive.ends_with('\\') {
                directive.pop();
                directive.push(' ');
                directive.push_str(lines.next().unwrap_or_default().trim());
            }
            directive_applies(&directive, &mut branches, macros)
                .map_err(|err| format!("{name}: {err}"))?;
            continue;
        }
        if !branches.iter().all(|branch| branch.active) {
            continue;
        }
        if let Some(end) = scan.closing_brace(line) {
            out.push_str(&line[..=end]);
            return Ok(Some(out));
        }
        out.push_str(line);
    }
    Err(format!("the initializer of {name} never closes"))
}

struct Branch {
    /// Whether the enclosing block is read.
    parent: bool,
    /// Whether a branch of this block was read already.
    taken: bool,
    active: bool,
}

fn directive_applies(
    directive: &str,
    branches: &mut Vec<Branch>,
    macros: &Macros,
) -> Result<(), String> {
    let directive = strip_comments(directive);
    let body = directive.trim_start_matches('#').trim_start();
    let (keyword, argument) = body
        .split_once(|c: char| c.is_whitespace())
        .map_or((body, ""), |(keyword, argument)| (keyword, argument.trim()));
    let parent = branches.iter().all(|branch| branch.active);
    match keyword {
        "if" | "ifdef" | "ifndef" => {
            let holds = parent
                && match keyword {
                    "ifdef" => macros.defined(argument)?,
                    "ifndef" => !macros.defined(argument)?,
                    _ => condition(argument, macros)?,
                };
            branches.push(Branch {
                parent,
                taken: holds,
                active: holds,
            });
        }
        "elif" => {
            let branch = branches.last_mut().ok_or("#elif without #if")?;
            let holds = branch.parent && !branch.taken && condition(argument, macros)?;
            branch.active = holds;
            branch.taken |= holds;
        }
        "else" => {
            let branch = branches.last_mut().ok_or("#else without #if")?;
            branch.active = branch.parent && !branch.taken;
            branch.taken = true;
        }
        "endif" => {
            branches.pop().ok_or("#endif without #if")?;
        }
        _ => {}
    }
    Ok(())
}

/// Evaluates an `#if` condition. Every macro in it comes from the platform
/// table.
fn condition(expression: &str, macros: &Macros) -> Result<bool, String> {
    let tokens = tokenize(expression)?;
    let mut resolved = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i].ident() == Some("defined") {
            let (name, skip) = match (tokens.get(i + 1), tokens.get(i + 2)) {
                (Some(open), Some(Token::Ident(name))) if open.is("(") => (name, 4),
                (Some(Token::Ident(name)), _) => (name, 2),
                _ => return Err(format!("cannot read #if {expression}")),
            };
            let value = if macros.defined(name)? { "1" } else { "0" };
            resolved.push(Token::Number(value.to_string()));
            i += skip;
        } else if let Some(name) = tokens[i].ident() {
            let value = platform::macro_value(name)
                .ok_or_else(|| unknown_condition(name))?
                .unwrap_or("0");
            resolved.extend(tokenize(value)?);
            i += 1;
        } else {
            resolved.push(tokens[i].clone());
            i += 1;
        }
    }
    Ok(match evaluate(&resolved, macros)? {
        Num::Int(value) => value != 0,
        Num::Float(value) => value != 0.0,
    })
}

/// Tracks comments and literals across lines, to find the brace that closes
/// an initializer.
#[derive(Default)]
struct Scan {
    in_comment: bool,
    depth: i32,
    opened: bool,
}

impl Scan {
    /// The offset in `line` of the brace that closes the initializer.
    fn closing_brace(&mut self, line: &str) -> Option<usize> {
        let bytes = line.as_bytes();
        let mut i = 0;
        let mut quote = None;
        while i < bytes.len() {
            let b = bytes[i];
            if self.in_comment {
                if line[i..].starts_with("*/") {
                    self.in_comment = false;
                    i += 1;
                }
            } else if let Some(q) = quote {
                if b == b'\\' {
                    i += 1;
                } else if b == q {
                    quote = None;
                }
            } else if line[i..].starts_with("/*") {
                self.in_comment = true;
                i += 1;
            } else if line[i..].starts_with("//") {
                return None;
            } else if b == b'"' || b == b'\'' {
                quote = Some(b);
            } else if b == b'{' {
                self.depth += 1;
                self.opened = true;
            } else if b == b'}' {
                self.depth -= 1;
                if self.opened && self.depth == 0 {
                    return Some(i);
                }
            }
            i += 1;
        }
        None
    }
}

/// A brace initializer: a list, or one expression.
#[derive(Debug)]
pub(crate) enum Init {
    List(Vec<Init>),
    Expr(Vec<Token>),
}

/// Parses `{ ... }` into its items.
pub(crate) fn parse_initializer(tokens: &[Token]) -> Result<Vec<Init>, String> {
    let mut at = 0;
    let items = list(tokens, &mut at)?;
    if at != tokens.len() {
        return Err("text after the initializer".into());
    }
    Ok(items)
}

fn list(tokens: &[Token], at: &mut usize) -> Result<Vec<Init>, String> {
    let truncated = || "a truncated initializer".to_string();
    if !tokens.get(*at).ok_or_else(truncated)?.is("{") {
        return Err("an initializer does not start with {".into());
    }
    *at += 1;
    let mut items = Vec::new();
    loop {
        let token = tokens.get(*at).ok_or_else(truncated)?;
        if token.is("}") {
            *at += 1;
            return Ok(items);
        }
        if token.is("{") {
            items.push(Init::List(list(tokens, at)?));
        } else {
            let mut expression = Vec::new();
            let mut parens = 0;
            loop {
                let token = tokens.get(*at).ok_or_else(truncated)?;
                if parens == 0 && (token.is(",") || token.is("}")) {
                    break;
                }
                if token.is("(") {
                    parens += 1;
                } else if token.is(")") {
                    parens -= 1;
                }
                expression.push(token.clone());
                *at += 1;
            }
            items.push(Init::Expr(expression));
        }
        if tokens.get(*at).is_some_and(|token| token.is(",")) {
            *at += 1;
        }
    }
}

/// A constant: C promotes to floating point as soon as one operand is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Num {
    Int(i128),
    Float(f64),
}

impl Num {
    fn float(self) -> f64 {
        match self {
            Num::Int(value) => value as f64,
            Num::Float(value) => value,
        }
    }

    fn int(self) -> Result<i128, String> {
        match self {
            Num::Int(value) => Ok(value),
            Num::Float(value) => Err(format!("{value} is not an integer")),
        }
    }

    fn truth(self) -> bool {
        match self {
            Num::Int(value) => value != 0,
            Num::Float(value) => value != 0.0,
        }
    }
}

/// Evaluates a constant expression, expanding macros.
pub(crate) fn evaluate(tokens: &[Token], macros: &Macros) -> Result<Num, String> {
    let mut parser = Eval {
        tokens,
        at: 0,
        macros,
        depth: 0,
    };
    let value = parser.ternary()?;
    if parser.at != tokens.len() {
        return Err(format!("cannot evaluate {}", show(tokens)));
    }
    Ok(value)
}

/// The type names a cast can name.
const TYPES: &[&str] = &[
    "Size", "bool", "char", "double", "float", "int", "int16", "int32", "int64", "long", "short",
    "signed", "size_t", "uint16", "uint32", "uint64", "unsigned",
];

struct Eval<'a> {
    tokens: &'a [Token],
    at: usize,
    macros: &'a Macros,
    depth: usize,
}

impl Eval<'_> {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.at)
    }

    fn eat(&mut self, punct: &str) -> bool {
        if self.peek().is_some_and(|token| token.is(punct)) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    fn ternary(&mut self) -> Result<Num, String> {
        let condition = self.binary(0)?;
        if self.eat("?") {
            let yes = self.ternary()?;
            if !self.eat(":") {
                return Err("a ? without :".into());
            }
            let no = self.ternary()?;
            return Ok(if condition.truth() { yes } else { no });
        }
        Ok(condition)
    }

    /// Binary operators by precedence, loosest first.
    fn binary(&mut self, level: usize) -> Result<Num, String> {
        const LEVELS: &[&[&str]] = &[
            &["||"],
            &["&&"],
            &["|"],
            &["^"],
            &["&"],
            &["==", "!="],
            &["<", ">", "<=", ">="],
            &["<<", ">>"],
            &["+", "-"],
            &["*", "/", "%"],
        ];
        if level == LEVELS.len() {
            return self.unary();
        }
        let mut left = self.binary(level + 1)?;
        while let Some(op) = self
            .peek()
            .and_then(|token| LEVELS[level].iter().find(|op| token.is(op)))
        {
            self.at += 1;
            let right = self.binary(level + 1)?;
            left = apply(op, left, right)?;
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Num, String> {
        if self.eat("-") {
            return Ok(match self.unary()? {
                Num::Int(value) => Num::Int(-value),
                Num::Float(value) => Num::Float(-value),
            });
        }
        if self.eat("+") {
            return self.unary();
        }
        if self.eat("!") {
            return Ok(Num::Int(i128::from(!self.unary()?.truth())));
        }
        if self.eat("~") {
            return Ok(Num::Int(!self.unary()?.int()?));
        }
        // A cast: `(type) operand`.
        if self.peek().is_some_and(|token| token.is("(")) {
            let mut end = self.at + 1;
            while self
                .tokens
                .get(end)
                .and_then(Token::ident)
                .is_some_and(|name| TYPES.contains(&name))
            {
                end += 1;
            }
            if end > self.at + 1 && self.tokens.get(end).is_some_and(|token| token.is(")")) {
                let floating = self.tokens[self.at + 1..end]
                    .iter()
                    .any(|token| matches!(token.ident(), Some("double" | "float")));
                self.at = end + 1;
                let value = self.unary()?;
                return Ok(if floating {
                    Num::Float(value.float())
                } else {
                    match value {
                        Num::Float(value) => Num::Int(value.trunc() as i128),
                        integer => integer,
                    }
                });
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Num, String> {
        let token = self.peek().cloned().ok_or("an expression ends early")?;
        self.at += 1;
        match token {
            Token::Number(text) => number(&text),
            Token::Punct("(") => {
                let value = self.ternary()?;
                if !self.eat(")") {
                    return Err("an unclosed (".into());
                }
                Ok(value)
            }
            Token::Ident(name) if name == "true" => Ok(Num::Int(1)),
            Token::Ident(name) if name == "false" => Ok(Num::Int(0)),
            Token::Ident(name) if (name == "Min" || name == "Max") && self.eat("(") => {
                let first = self.ternary()?;
                if !self.eat(",") {
                    return Err(format!("{name} takes two arguments"));
                }
                let second = self.ternary()?;
                if !self.eat(")") {
                    return Err(format!("an unclosed {name}("));
                }
                let first_smaller = first.float() < second.float();
                Ok(if (name == "Min") == first_smaller {
                    first
                } else {
                    second
                })
            }
            Token::Ident(name) => {
                if self.depth > 32 {
                    return Err(format!("{name} expands without end"));
                }
                let definition = self.macros.definition(&name)?.ok_or_else(|| {
                    format!("{name} is not defined. Add it to the platform table")
                })?;
                let tokens = tokenize(definition)?;
                let mut nested = Eval {
                    tokens: &tokens,
                    at: 0,
                    macros: self.macros,
                    depth: self.depth + 1,
                };
                let value = nested.ternary()?;
                if nested.at != tokens.len() {
                    return Err(format!("cannot evaluate {name}, defined as {definition}"));
                }
                Ok(value)
            }
            other => Err(format!("cannot evaluate {other:?}")),
        }
    }
}

fn apply(op: &str, left: Num, right: Num) -> Result<Num, String> {
    if let (Num::Int(a), Num::Int(b)) = (left, right) {
        let value = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" | "%" if b == 0 => return Err("a division by zero".into()),
            "/" => a / b,
            "%" => a % b,
            "<<" => a << b,
            ">>" => a >> b,
            "&" => a & b,
            "|" => a | b,
            "^" => a ^ b,
            _ => {
                return Ok(Num::Int(i128::from(compare(
                    op, a as f64, b as f64, left, right,
                ))));
            }
        };
        return Ok(Num::Int(value));
    }
    let (a, b) = (left.float(), right.float());
    Ok(match op {
        "+" => Num::Float(a + b),
        "-" => Num::Float(a - b),
        "*" => Num::Float(a * b),
        "/" => Num::Float(a / b),
        _ => Num::Int(i128::from(compare(op, a, b, left, right))),
    })
}

fn compare(op: &str, a: f64, b: f64, left: Num, right: Num) -> bool {
    match op {
        "<" => a < b,
        ">" => a > b,
        "<=" => a <= b,
        ">=" => a >= b,
        "==" => a == b,
        "!=" => a != b,
        "&&" => left.truth() && right.truth(),
        "||" => left.truth() || right.truth(),
        _ => false,
    }
}

fn number(text: &str) -> Result<Num, String> {
    let lower = text.to_ascii_lowercase();
    let bad = || format!("cannot read the number {text}");
    if let Some(hex) = lower.strip_prefix("0x") {
        let digits = hex.trim_end_matches(['u', 'l']);
        return i128::from_str_radix(digits, 16)
            .map(Num::Int)
            .map_err(|_| bad());
    }
    if lower.contains(['.', 'e']) {
        let digits = lower.trim_end_matches(['f', 'l']);
        return digits.parse().map(Num::Float).map_err(|_| bad());
    }
    let digits = lower.trim_end_matches(['u', 'l']);
    if digits.len() > 1 && digits.starts_with('0') {
        return i128::from_str_radix(&digits[1..], 8)
            .map(Num::Int)
            .map_err(|_| bad());
    }
    digits.parse().map(Num::Int).map_err(|_| bad())
}

/// Tokens as C text, for error messages.
pub(crate) fn show(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|token| match token {
            Token::Ident(text) | Token::Number(text) => text.clone(),
            Token::Str(text) => format!("{text:?}"),
            Token::Char(text) => format!("'{text}'"),
            Token::Punct(punct) => punct.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A floating-point value as C's `printf("%g")` writes it, which is how
/// `pg_settings` shows one.
pub(crate) fn format_g(value: f64) -> String {
    if value == 0.0 {
        return if value.is_sign_negative() { "-0" } else { "0" }.to_string();
    }
    let scientific = format!("{value:.5e}");
    let (mantissa, exponent) = scientific.split_once('e').expect("an exponent");
    let exponent: i32 = exponent.parse().expect("a decimal exponent");
    if !(-4..6).contains(&exponent) {
        let mantissa = trim_zeros(mantissa);
        let sign = if exponent < 0 { '-' } else { '+' };
        format!("{mantissa}e{sign}{:02}", exponent.abs())
    } else {
        let decimals = usize::try_from(5 - exponent).expect("at most nine decimals");
        trim_zeros(&format!("{value:.decimals$}")).to_string()
    }
}

fn trim_zeros(number: &str) -> &str {
    if number.contains('.') {
        number.trim_end_matches('0').trim_end_matches('.')
    } else {
        number
    }
}
