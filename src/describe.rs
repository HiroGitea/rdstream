//! Human-readable tree of a parsed document, annotated with the paths that
//! [`crate::policy`] globs match against.

use std::io::{self, Write};

use crate::sexp::{Document, RObject, Value, sexptype, type_name};

const MAX_CHILDREN: usize = 60;
const PREVIEW: usize = 4;

pub fn describe(doc: &Document, max_depth: usize, out: &mut impl Write) -> io::Result<()> {
    match &doc.root.value {
        Value::Pairlist { items, .. } if doc.format == crate::Format::RData => {
            for (name, obj) in items {
                node(
                    out,
                    obj,
                    &format!("${}", name.as_deref().unwrap_or("")),
                    0,
                    max_depth,
                )?;
            }
            Ok(())
        }
        _ => node(out, &doc.root, "<root>", 0, max_depth),
    }
}

/// `1234567` -> `1,234,567`
pub fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

fn preview<T: std::fmt::Debug>(v: &[T]) -> String {
    let head: Vec<String> = v.iter().take(PREVIEW).map(|x| format!("{x:?}")).collect();
    format!(
        "[{}{}]",
        head.join(", "),
        if v.len() > PREVIEW { ", …" } else { "" }
    )
}

fn summary(o: &RObject) -> String {
    let n = |len: usize| thousands(len as u64);
    match &o.value {
        Value::Null => "NULL".into(),
        Value::Skipped { sexptype, len } => {
            format!("{}[{}] (skipped)", type_name(*sexptype), thousands(*len))
        }
        Value::Streamed {
            sexptype,
            len,
            target,
        } => {
            format!(
                "{}[{}] (streamed to {target})",
                type_name(*sexptype),
                thousands(*len)
            )
        }
        Value::Logical(v) => format!("lgl[{}] {}", n(v.len()), preview(v)),
        Value::Integer(v) => format!("int[{}] {}", n(v.len()), preview(v)),
        Value::Real(v) => format!("dbl[{}] {}", n(v.len()), preview(v)),
        Value::Complex(v) => format!("cplx[{}]", n(v.len())),
        Value::Raw(v) => format!("raw[{}]", n(v.len())),
        Value::Character(v) => {
            let head: Vec<String> = v
                .iter()
                .take(PREVIEW)
                .map(|s| s.as_deref().map_or("NA".into(), |s| format!("{s:?}")))
                .collect();
            format!(
                "chr[{}] [{}{}]",
                n(v.len()),
                head.join(", "),
                if v.len() > PREVIEW { ", …" } else { "" }
            )
        }
        Value::List(v) => format!("list[{}]", n(v.len())),
        Value::Expression(v) => format!("expr[{}]", n(v.len())),
        Value::Symbol(s) => format!("sym {s}"),
        Value::Pairlist { sexptype, items } => {
            format!("{}[{}]", type_name(*sexptype), n(items.len()))
        }
        Value::Environment(i) => format!("env #{i}"),
        Value::Builtin { name, .. } => format!("builtin {name}"),
        Value::AltRep { class, .. } => format!("altrep {class}"),
        Value::Bytecode { consts, .. } => format!("bytecode ({} constants)", n(consts.len())),
        Value::Reference { sexptype, info } => format!("{} {:?}", type_name(*sexptype), info),
        _ => type_name(o.sexptype()).to_string(),
    }
}

fn node(
    out: &mut impl Write,
    o: &RObject,
    label: &str,
    depth: usize,
    max_depth: usize,
) -> io::Result<()> {
    let pad = "  ".repeat(depth);
    let mut line = format!("{pad}{label}: {}", summary(o));
    if let Some(Value::Integer(d)) = o.attr("dim").map(|d| &d.value) {
        line += &format!(" dim={d:?}");
    }
    let classes = o.classes();
    if !classes.is_empty() {
        line += &format!(" <{}>", classes.join(","));
    }
    writeln!(out, "{line}")?;
    if depth >= max_depth {
        return Ok(());
    }
    for (k, a) in &o.attributes {
        let hidden =
            matches!(k.as_str(), "class" | "names" | "dim") || (k == "row.names" && depth > 0);
        if !hidden && !matches!(o.value, Value::Character(_)) {
            node(out, a, &format!("@{k}"), depth + 1, max_depth)?;
        }
    }
    match &o.value {
        Value::List(items) | Value::Expression(items) => {
            let names = o.names();
            for (k, el) in items.iter().enumerate().take(MAX_CHILDREN) {
                let name = names
                    .and_then(|ns| ns.get(k))
                    .and_then(|s| s.as_deref())
                    .unwrap_or("");
                node(out, el, &format!("[{k}]{name}"), depth + 1, max_depth)?;
            }
            if items.len() > MAX_CHILDREN {
                writeln!(out, "{pad}  … {} more", items.len() - MAX_CHILDREN)?;
            }
        }
        Value::Pairlist { sexptype, items } if *sexptype == sexptype::LIST => {
            for (k, (name, el)) in items.iter().enumerate().take(MAX_CHILDREN) {
                let label = name.as_ref().map_or(format!("[{k}]"), |n| format!("${n}"));
                node(out, el, &label, depth + 1, max_depth)?;
            }
        }
        _ => {}
    }
    Ok(())
}
