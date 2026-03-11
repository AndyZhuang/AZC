//! AZC Compiler - Ruby-like to C Transpiler v2

pub fn compile(source: &str) -> Result<String, String> {
    let mut out = String::new();

    // Header
    out.push_str(
        "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n#include <stdbool.h>\n\n",
    );
    out.push_str("/* AZC Runtime */\n");
    out.push_str("typedef const char* AZC;\n\n");
    out.push_str("/* AZC Helper Functions */\n");
    out.push_str("static inline AZC azc_num(long v) { static char buf[32]; sprintf(buf, \"%ld\", v); return buf; }\n");
    out.push_str("#define azc_bool(v) ((v) ? \"true\" : \"false\")\n");
    out.push_str("#define azc_strlit(s) (s)\n\n");
    out.push_str("void azc_puts(AZC s) { if (s) printf(\"%s\\n\", s); }\n");
    out.push_str("void azc_print(AZC s) { if (s) printf(\"%s\", s); }\n\n");

    // Parse source into statements
    let stmts = parse(source);

    // Generate code - collect statements first
    let mut code = String::new();
    gen(&mut code, &stmts, 0);

    // Main function wrapper
    out.push_str("int main() {\n");
    out.push_str(&code);
    out.push_str("    return 0;\n");
    out.push_str("}\n");

    Ok(out)
}

#[derive(Debug, Clone)]
enum Stmt {
    Put(String),
    Let(String, Option<String>),
    Assign(String, String),
    Def(String, Vec<String>, Vec<Stmt>),
    If(String, Vec<Stmt>, Option<Vec<Stmt>>),
    While(String, Vec<Stmt>),
    Return(Option<String>),
    Expr(String),
}

fn parse(source: &str) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    let mut lines: Vec<&str> = source.lines().map(|l| l.trim()).collect();

    // Remove empty lines
    lines.retain(|l| !l.is_empty() && !l.starts_with('#'));

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        if line.starts_with("puts ") {
            if let Some(s) = line.strip_prefix("puts ") {
                stmts.push(Stmt::Put(s.to_string()));
            }
        } else if line.starts_with("let ") {
            if let Some(rest) = line.strip_prefix("let ") {
                if let Some((name, val)) = rest.split_once('=') {
                    stmts.push(Stmt::Let(
                        name.trim().to_string(),
                        Some(val.trim().to_string()),
                    ));
                } else {
                    stmts.push(Stmt::Let(rest.trim().to_string(), None));
                }
            }
        } else if line.contains(" = ") && !line.starts_with("let ") {
            if let Some((l, r)) = line.split_once('=') {
                stmts.push(Stmt::Assign(l.trim().to_string(), r.trim().to_string()));
            }
        } else if line.starts_with("def ") {
            if let Some(rest) = line.strip_prefix("def ") {
                if let Some((name, _)) = rest.split_once('(') {
                    let fname = name.trim().to_string();
                    i += 1;
                    let mut body = Vec::new();
                    while i < lines.len() && lines[i] != "end" {
                        let l = lines[i].to_string();
                        // Parse body statement recursively
                        if l.starts_with("puts ") {
                            if let Some(s) = l.strip_prefix("puts ") {
                                body.push(Stmt::Put(s.to_string()));
                            }
                        } else if l.contains(" = ") {
                            if let Some((lv, rv)) = l.split_once('=') {
                                body.push(Stmt::Assign(
                                    lv.trim().to_string(),
                                    rv.trim().to_string(),
                                ));
                            }
                        }
                        i += 1;
                    }
                    stmts.push(Stmt::Def(fname, Vec::new(), body));
                }
            }
        } else if line.starts_with("if ") {
            if let Some(cond) = line.strip_prefix("if ") {
                i += 1;
                let mut then_br = Vec::new();
                let mut else_br = None;

                while i < lines.len() && lines[i] != "end" && lines[i] != "else" {
                    let l = lines[i].to_string();
                    if l.starts_with("puts ") {
                        if let Some(s) = l.strip_prefix("puts ") {
                            then_br.push(Stmt::Put(s.to_string()));
                        }
                    } else if l.contains(" = ") {
                        if let Some((lv, rv)) = l.split_once('=') {
                            then_br
                                .push(Stmt::Assign(lv.trim().to_string(), rv.trim().to_string()));
                        }
                    }
                    i += 1;
                }

                if i < lines.len() && lines[i] == "else" {
                    i += 1;
                    let mut else_body = Vec::new();
                    while i < lines.len() && lines[i] != "end" {
                        let l = lines[i].to_string();
                        if l.starts_with("puts ") {
                            if let Some(s) = l.strip_prefix("puts ") {
                                else_body.push(Stmt::Put(s.to_string()));
                            }
                        } else if l.contains(" = ") {
                            if let Some((lv, rv)) = l.split_once('=') {
                                else_body.push(Stmt::Assign(
                                    lv.trim().to_string(),
                                    rv.trim().to_string(),
                                ));
                            }
                        }
                        i += 1;
                    }
                    else_br = Some(else_body);
                }

                stmts.push(Stmt::If(cond.to_string(), then_br, else_br));
            }
        } else if line.starts_with("while ") {
            if let Some(cond) = line.strip_prefix("while ") {
                i += 1;
                let mut body = Vec::new();
                while i < lines.len() && lines[i] != "end" {
                    let l = lines[i].to_string();
                    if l.starts_with("puts ") {
                        if let Some(s) = l.strip_prefix("puts ") {
                            body.push(Stmt::Put(s.to_string()));
                        }
                    } else if l.contains(" = ") {
                        if let Some((lv, rv)) = l.split_once('=') {
                            body.push(Stmt::Assign(lv.trim().to_string(), rv.trim().to_string()));
                        }
                    }
                    i += 1;
                }
                stmts.push(Stmt::While(cond.to_string(), body));
            }
        }

        i += 1;
    }

    stmts
}

fn gen(out: &mut String, stmts: &[Stmt], indent: usize) {
    let ind = "    ".repeat(indent);

    for stmt in stmts {
        match stmt {
            Stmt::Put(s) => {
                // Check if string is a quoted literal
                let s = s.trim();
                if s.starts_with('"') && s.ends_with('"') {
                    out.push_str(&format!("{}azc_puts(azc_strlit({}));\n", ind, s));
                } else {
                    out.push_str(&format!("{}azc_puts({});\n", ind, s));
                }
            }
            Stmt::Let(n, v) => {
                if let Some(val) = v {
                    let val = val.trim();
                    // Wrap values in appropriate helpers
                    let wrapped = if val.parse::<i64>().is_ok() {
                        format!("azc_num({})", val)
                    } else if val == "true" || val == "false" {
                        format!("azc_bool({})", if val == "true" { 1 } else { 0 })
                    } else if val.starts_with('"') {
                        format!("azc_strlit({})", val)
                    } else {
                        val.to_string()
                    };
                    out.push_str(&format!("{}AZC {} = {};\n", ind, n, wrapped));
                } else {
                    out.push_str(&format!("{}AZC {};\n", ind, n));
                }
            }
            Stmt::Assign(l, r) => {
                out.push_str(&format!("{}{} = {};\n", ind, l, r));
            }
            Stmt::Def(name, _params, body) => {
                out.push_str(&format!("{}void azc_{}(void) {{\n", ind, name));
                gen(out, body, indent + 1);
                out.push_str(&format!("{}}}\n\n", ind));
            }
            Stmt::If(cond, then_br, else_br) => {
                out.push_str(&format!("{}if ({}) {{\n", ind, cond));
                gen(out, then_br, indent + 1);
                if let Some(eb) = else_br {
                    out.push_str(&format!("{}}} else {{\n", ind));
                    gen(out, eb, indent + 1);
                }
                out.push_str(&format!("{}}}\n", ind));
            }
            Stmt::While(cond, body) => {
                out.push_str(&format!("{}while ({}) {{\n", ind, cond));
                gen(out, body, indent + 1);
                out.push_str(&format!("{}}}\n", ind));
            }
            Stmt::Return(v) => {
                if let Some(val) = v {
                    out.push_str(&format!("{}return {};\n", ind, val));
                } else {
                    out.push_str(&format!("{}return;\n", ind));
                }
            }
            Stmt::Expr(e) => {
                out.push_str(&format!("{}/* {} */\n", ind, e));
            }
        }
    }
}
