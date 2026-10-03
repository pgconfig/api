//! The GUC tables in C or guc_parameters.dat: every parameter's context, category, descriptions,
//! unit, default, limits, and values, as a standard build has them.

use std::collections::{BTreeMap, HashMap};

use crate::c::{self, Init, Macros, Num, Token};

/// One parameter as the GUC tables define it, in the words `pg_settings`
/// uses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Setting {
    /// The type in the manual's words: `boolean`, `integer`, `floating point`,
    /// `string`, or `enum`.
    pub vartype: String,
    pub category: String,
    pub short_desc: String,
    pub extra_desc: Option<String>,
    pub context: String,
    pub unit: Option<String>,
    pub default: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    /// The values an enum accepts, without the hidden aliases.
    pub values: Vec<String>,
}

/// The C sources the settings come from, read at one release tag.
pub struct Sources<'a> {
    /// `guc_tables.c`, or `guc.c` before PostgreSQL 16.
    pub tables: &'a str,
    /// `src/include/utils/guc_tables.h`, which orders `config_group` for
    /// the releases that list the group names by position.
    pub header: &'a str,
    /// Other C files that define enum option arrays.
    pub options: &'a [&'a str],
    /// The headers in `src/include`, for the macros the tables use.
    pub headers: &'a [&'a str],
    /// The release the files come from, such as `18.6` or `9.6.24`.
    pub release: &'a str,
}

/// The arrays of parameters, with the type of each in the manual's words.
const ARRAYS: [(&str, &str); 5] = [
    ("ConfigureNamesBool", "boolean"),
    ("ConfigureNamesInt", "integer"),
    ("ConfigureNamesReal", "floating point"),
    ("ConfigureNamesString", "string"),
    ("ConfigureNamesEnum", "enum"),
];

/// The settings of every parameter the tables define, as a standard x86-64
/// Linux build has them.
pub fn settings(sources: &Sources) -> Result<BTreeMap<String, Setting>, String> {
    let macros = Macros::new(
        sources.headers.iter().copied().chain([sources.tables]),
        sources.release,
    )?;
    let groups = groups(sources, &macros)?;

    let mut settings = BTreeMap::new();
    let mut arrays = 0;
    for (array, vartype) in ARRAYS {
        let Some(text) = c::initializer(sources.tables, array, &macros)? else {
            continue;
        };
        arrays += 1;
        for item in c::parse_initializer(&c::tokenize(&text)?)? {
            let reader = Reader {
                sources,
                macros: &macros,
                groups: &groups,
            };
            if let Some((name, setting)) = reader.setting(&item, vartype)? {
                settings.insert(name, setting);
            }
        }
    }
    if arrays == 0 {
        return Err("the tables define none of the ConfigureNames arrays".into());
    }
    Ok(settings)
}

/// PostgreSQL 19 keeps GUC declarations in quoted records instead of C
/// arrays. Defaults and enum options still use the same C expressions.
pub fn settings_from_dat(
    sources: &Sources,
    data: &str,
) -> Result<BTreeMap<String, Setting>, String> {
    let macros = Macros::new(
        sources.headers.iter().copied().chain([sources.tables]),
        sources.release,
    )?;
    let groups = groups(sources, &macros)?;
    let reader = Reader {
        sources,
        macros: &macros,
        groups: &groups,
    };
    let mut settings = BTreeMap::new();
    for fields in crate::dat::records(data)? {
        let required = |field: &str| {
            fields
                .get(field)
                .map(String::as_str)
                .ok_or_else(|| format!("GUC record {:?} lacks {field}", fields.get("name")))
        };
        for key in fields.keys() {
            if ![
                "name",
                "type",
                "context",
                "group",
                "short_desc",
                "long_desc",
                "flags",
                "variable",
                "boot_val",
                "min",
                "max",
                "options",
                "ifdef",
                "check_hook",
                "assign_hook",
                "show_hook",
            ]
            .contains(&key.as_str())
            {
                return Err(format!("unknown GUC field {key}"));
            }
        }
        let name = required("name")?;
        let vartype = match required("type")? {
            "bool" => "boolean",
            "int" => "integer",
            "real" => "floating point",
            "string" => "string",
            "enum" => "enum",
            other => return Err(format!("{name}: unknown GUC type {other}")),
        };
        if let Some(condition) = fields.get("ifdef")
            && !macros.defined(condition)?
        {
            continue;
        }
        let expr = |text: &str| c::tokenize(text).map(Init::Expr);
        // Match gen_guc_tables.pl: descriptions retain their C escapes.
        let quoted = |text: &str| expr(&format!("\"{}\"", text.replace('"', "\\\"")));
        let generic = vec![
            quoted(name)?,
            expr(required("context")?)?,
            expr(required("group")?)?,
            quoted(required("short_desc")?)?,
            fields
                .get("long_desc")
                .map_or_else(|| expr("NULL"), |text| quoted(text))?,
            expr(fields.get("flags").map_or("0", String::as_str))?,
        ];
        let mut values = vec![expr(required("variable")?)?, expr(required("boot_val")?)?];
        if matches!(vartype, "integer" | "floating point") {
            values.extend([expr(required("min")?)?, expr(required("max")?)?]);
        } else if vartype == "enum" {
            values.push(expr(required("options")?)?);
        }
        let setting = reader
            .read(name, &generic, &values, vartype)
            .map_err(|err| format!("{name}: {err}"))?;
        if settings.insert(name.to_string(), setting).is_some() {
            return Err(format!("duplicate GUC {name}"));
        }
    }
    if settings.is_empty() {
        return Err("guc_parameters.dat defines no settings".into());
    }
    Ok(settings)
}

/// The category of each `config_group`.
fn groups(sources: &Sources, macros: &Macros) -> Result<HashMap<String, String>, String> {
    let text = c::initializer(sources.tables, "config_group_names", macros)?
        .ok_or("the tables define no config_group_names")?;
    let mut groups = HashMap::new();
    let mut by_position = Vec::new();
    for item in c::parse_initializer(&c::tokenize(&text)?)? {
        let Init::Expr(tokens) = item else {
            return Err("a group name is not an expression".into());
        };
        // `[RESOURCES_MEM] = gettext_noop("...")`, from PostgreSQL 16 on.
        if let [open, Token::Ident(group), close, equals, rest @ ..] = tokens.as_slice()
            && open.is("[")
            && close.is("]")
            && equals.is("=")
        {
            if let Some(name) = string(rest, macros)? {
                groups.insert(group.clone(), name);
            }
        } else if let Some(name) = string(&tokens, macros)? {
            by_position.push(name);
        }
    }
    if !by_position.is_empty() {
        let order = group_order(sources.header)?;
        if order.len() < by_position.len() {
            return Err("guc_tables.h lists fewer groups than config_group_names".into());
        }
        groups.extend(order.into_iter().zip(by_position));
    }
    Ok(groups)
}

/// The members of `enum config_group`, in order.
fn group_order(header: &str) -> Result<Vec<String>, String> {
    let start = header
        .find("enum config_group")
        .ok_or("guc_tables.h has no enum config_group")?;
    let body = &header[start..];
    let open = body.find('{').ok_or("enum config_group has no body")?;
    let close = body.find('}').ok_or("enum config_group never closes")?;
    let tokens = c::tokenize(&body[open + 1..close])?;
    Ok(tokens
        .split(|token| token.is(","))
        .filter_map(|member| member.first().and_then(Token::ident).map(str::to_string))
        .collect())
}

struct Reader<'a> {
    sources: &'a Sources<'a>,
    macros: &'a Macros,
    groups: &'a HashMap<String, String>,
}

impl Reader<'_> {
    /// One parameter of an array, or `None` for the end-of-list marker.
    fn setting(&self, item: &Init, vartype: &str) -> Result<Option<(String, Setting)>, String> {
        let Init::List(fields) = item else {
            return Err("a parameter is not a braced list".into());
        };
        let Some(Init::List(generic)) = fields.first() else {
            return Err("a parameter has no generic part".into());
        };
        let name = match expression(generic.first())? {
            [Token::Str(name)] => name.clone(),
            [Token::Ident(null)] if null == "NULL" => return Ok(None),
            other => return Err(format!("a parameter is named {}", c::show(other))),
        };
        let context = |err: String| format!("{name}: {err}");
        self.read(&name, generic, &fields[1..], vartype)
            .map(|setting| Some((name.clone(), setting)))
            .map_err(context)
    }

    fn read(
        &self,
        name: &str,
        generic: &[Init],
        values: &[Init],
        vartype: &str,
    ) -> Result<Setting, String> {
        let context = expression(generic.get(1))?
            .first()
            .and_then(Token::ident)
            .and_then(context_name)
            .ok_or("its context is not a PGC_ value")?;
        let group = expression(generic.get(2))?
            .first()
            .and_then(Token::ident)
            .ok_or("its group is not a name")?;
        let category = self
            .groups
            .get(group)
            .ok_or_else(|| format!("the group {group} has no name"))?
            .clone();
        let short_desc = string(expression(generic.get(3))?, self.macros)?.unwrap_or_default();
        let extra_desc = match generic.get(4) {
            Some(init) => string(expression(Some(init))?, self.macros)?,
            None => None,
        };
        let unit = match generic.get(5) {
            Some(init) => unit(expression(Some(init))?)?,
            None => None,
        };

        // After the variable come the default, then the limits or options.
        let value = |index: usize| expression(values.get(index + 1));
        let mut setting = Setting {
            vartype: vartype.to_string(),
            category,
            short_desc,
            extra_desc,
            context: context.to_string(),
            unit,
            default: None,
            min: None,
            max: None,
            values: Vec::new(),
        };
        match vartype {
            "boolean" => {
                let on = c::evaluate(value(0)?, self.macros)? != Num::Int(0);
                setting.default = Some(if on { "on" } else { "off" }.to_string());
            }
            "integer" => {
                setting.default = Some(self.integer(value(0)?)?);
                setting.min = Some(self.integer(value(1)?)?);
                setting.max = Some(self.integer(value(2)?)?);
            }
            "floating point" => {
                setting.default = Some(self.real(value(0)?)?);
                setting.min = Some(self.real(value(1)?)?);
                setting.max = Some(self.real(value(2)?)?);
            }
            "string" => setting.default = string(value(0)?, self.macros)?,
            "enum" => {
                // `options + 1` leaves out the first option.
                let (array, skip) = match value(1)? {
                    [Token::Ident(array)] => (array, 0),
                    [Token::Ident(array), plus, Token::Number(skip)] if plus.is("+") => {
                        (array, skip.parse().map_err(|_| "a bad options offset")?)
                    }
                    other => return Err(format!("its options are {}", c::show(other))),
                };
                let mut options = self.options(array)?;
                options.drain(..skip.min(options.len()));
                setting.values = options
                    .iter()
                    .filter(|option| !option.hidden)
                    .map(|option| option.name.clone())
                    .collect();
                setting.default = Some(self.enum_default(value(0)?, &options)?);
            }
            _ => return Err(format!("{name} has the unknown type {vartype}")),
        }
        Ok(setting)
    }

    /// The entries of the enum option array `array`, from the tables or from
    /// the other files.
    fn options(&self, array: &str) -> Result<Vec<EnumOption>, String> {
        let mut text = None;
        for source in std::iter::once(&self.sources.tables).chain(self.sources.options) {
            text = c::initializer(source, array, self.macros)?;
            if text.is_some() {
                break;
            }
        }
        let text = text.ok_or_else(|| format!("no file defines {array}"))?;
        let mut options = Vec::new();
        for item in c::parse_initializer(&c::tokenize(&text)?)? {
            let Init::List(fields) = item else {
                return Err(format!("an entry of {array} is not a braced list"));
            };
            let name = match expression(fields.first())? {
                [Token::Str(name)] => name.clone(),
                [Token::Ident(null)] if null == "NULL" => break,
                other => return Err(format!("an entry of {array} is named {}", c::show(other))),
            };
            let value = expression(fields.get(1))?.to_vec();
            let hidden = c::evaluate(expression(fields.get(2))?, self.macros)? != Num::Int(0);
            options.push(EnumOption {
                name,
                value,
                hidden,
            });
        }
        Ok(options)
    }

    /// The option whose value is `boot`, a visible one first. A default
    /// written as a macro, such as `DEFAULT_SYNC_METHOD`, is expanded until
    /// it names an option.
    fn enum_default(&self, boot: &[Token], options: &[EnumOption]) -> Result<String, String> {
        let mut boot = boot.to_vec();
        for _ in 0..8 {
            if let Some(option) = options
                .iter()
                .filter(|option| option.value == boot)
                .min_by_key(|option| option.hidden)
            {
                return Ok(option.name.clone());
            }
            let [Token::Ident(name)] = boot.as_slice() else {
                break;
            };
            let Some(definition) = self.macros.definition(name)? else {
                break;
            };
            boot = c::tokenize(definition)?;
        }
        Err(format!(
            "its default {} is none of its options",
            c::show(&boot)
        ))
    }

    fn real(&self, tokens: &[Token]) -> Result<String, String> {
        let value = match c::evaluate(tokens, self.macros)? {
            Num::Int(value) => value as f64,
            Num::Float(value) => value,
        };
        Ok(c::format_g(value))
    }

    fn integer(&self, tokens: &[Token]) -> Result<String, String> {
        match c::evaluate(tokens, self.macros)? {
            Num::Int(value) => Ok(value.to_string()),
            Num::Float(value) if value.fract() == 0.0 => Ok(format!("{value:.0}")),
            Num::Float(value) => Err(format!("{value} is not an integer")),
        }
    }
}

struct EnumOption {
    name: String,
    value: Vec<Token>,
    hidden: bool,
}

fn expression(init: Option<&Init>) -> Result<&[Token], String> {
    match init {
        Some(Init::Expr(tokens)) => Ok(tokens),
        Some(Init::List(_)) => Err("a braced list where a value belongs".into()),
        None => Err("a value is missing".into()),
    }
}

/// A string written as literals, `gettext_noop(...)`, a macro, or `NULL`.
fn string(tokens: &[Token], macros: &Macros) -> Result<Option<String>, String> {
    let tokens = match tokens {
        [Token::Ident(function), open, inner @ .., close]
            if function == "gettext_noop" && open.is("(") && close.is(")") =>
        {
            inner
        }
        other => other,
    };
    match tokens {
        [Token::Ident(null)] if null == "NULL" => Ok(None),
        [Token::Ident(name)] => {
            let definition = macros
                .definition(name)?
                .ok_or_else(|| format!("{name} is not defined. Add it to the platform table"))?;
            string(&c::tokenize(definition)?, macros)
        }
        literals if !literals.is_empty() => {
            let mut text = String::new();
            for token in literals {
                match token {
                    Token::Str(part) => text.push_str(part),
                    _ => return Err(format!("cannot read the string {}", c::show(tokens))),
                }
            }
            Ok(Some(text))
        }
        _ => Err("an empty string expression".into()),
    }
}

/// The unit `pg_settings` shows for the `GUC_UNIT_` flag among `flags`.
fn unit(flags: &[Token]) -> Result<Option<String>, String> {
    let Some(flag) = flags
        .iter()
        .filter_map(Token::ident)
        .find(|flag| flag.starts_with("GUC_UNIT_"))
    else {
        return Ok(None);
    };
    let unit = match flag {
        "GUC_UNIT_BYTE" => "B",
        "GUC_UNIT_KB" => "kB",
        "GUC_UNIT_MB" => "MB",
        "GUC_UNIT_BLOCKS" | "GUC_UNIT_XBLOCKS" => "8kB",
        "GUC_UNIT_XSEGS" => "16MB",
        "GUC_UNIT_MS" => "ms",
        "GUC_UNIT_S" => "s",
        "GUC_UNIT_MIN" => "min",
        _ => {
            return Err(format!(
                "{flag} is a unit the extractor does not know. Add the unit pg_settings shows for it to unit() in src/guc.rs"
            ));
        }
    };
    Ok(Some(unit.to_string()))
}

/// The name `pg_settings` gives a context.
fn context_name(context: &str) -> Option<&'static str> {
    Some(match context {
        "PGC_INTERNAL" => "internal",
        "PGC_POSTMASTER" => "postmaster",
        "PGC_SIGHUP" => "sighup",
        "PGC_SU_BACKEND" => "superuser-backend",
        "PGC_BACKEND" => "backend",
        "PGC_SUSET" => "superuser",
        "PGC_USERSET" => "user",
        _ => return None,
    })
}
