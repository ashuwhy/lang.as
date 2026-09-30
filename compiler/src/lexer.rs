//! Lexer. Newlines end statements (a `;` token is inserted, as in Go) unless the line clearly
//! continues: the next line starts with a binary operator, `.`, a closing bracket, `else` or a
//! contract keyword. Explicit `;` is also accepted.

use crate::diag::{Diagnostic, Span};

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Ident(String),
    Int(i128),
    Str(String),
    Kw(&'static str),
    P(&'static str),
    Semi,
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tok: Tok,
    pub span: Span,
    pub newline_before: bool,
}

pub const KEYWORDS: &[&str] = &[
    "fn", "pub", "type", "enum", "let", "var", "if", "else", "match", "while", "return", "requires",
    "ensures", "invariant", "decreases", "uses", "where", "true", "false", "is", "trusted", "for", "in",
    "forall", "exists",
];

const PUNCT: &[&str] = &[
    "==>", "...", "..", "->", "=>", "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "(", ")", "{",
    "}", "[", "]", ",", ":", ".", "=", "<", ">", "+", "-", "*", "/", "%", "!", "?",
];

fn ends_statement(t: &Tok) -> bool {
    matches!(t, Tok::Ident(_) | Tok::Int(_) | Tok::Str(_))
        || matches!(t, Tok::Kw("true") | Tok::Kw("false") | Tok::Kw("return"))
        || matches!(t, Tok::P(")") | Tok::P("]") | Tok::P("}") | Tok::P("?") | Tok::P(">"))
}

fn continues_line(t: &Tok) -> bool {
    match t {
        Tok::P(p) => matches!(
            *p,
            "." | ".." | "&&" | "||" | "==>" | "+" | "*" | "/" | "%" | "==" | "!=" | "<" | "<=" | ">" | ">="
                | "=>" | "->" | ")" | "]" | "," | "=" | "+=" | "-=" | "*=" | "{"
        ),
        Tok::Kw(k) => matches!(*k, "else" | "requires" | "ensures" | "invariant" | "decreases" | "uses" | "where" | "is"),
        _ => false,
    }
}

pub fn lex(src: &str) -> Result<Vec<Token>, Diagnostic> {
    let b = src.as_bytes();
    let mut i = 0;
    let mut raw: Vec<Token> = Vec::new();
    let mut newline = false;
    while i < b.len() {
        let c = b[i];
        if c == b'\n' {
            newline = true;
            i += 1;
            continue;
        }
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c == b'/' && b.get(i + 1) == Some(&b'/') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if c == b'/' && b.get(i + 1) == Some(&b'*') {
            let start = i;
            i += 2;
            while i + 1 < b.len() && !(b[i] == b'*' && b[i + 1] == b'/') {
                if b[i] == b'\n' {
                    newline = true;
                }
                i += 1;
            }
            if i + 1 >= b.len() {
                return Err(Diagnostic::error("E0001", Span::new(start, start + 2), "unterminated block comment").with_fix("close it with `*/`"));
            }
            i += 2;
            continue;
        }
        let start = i;
        let tok = if c.is_ascii_alphabetic() || c == b'_' {
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            let word = &src[start..i];
            match KEYWORDS.iter().find(|k| **k == word) {
                Some(k) => Tok::Kw(k),
                None => Tok::Ident(word.to_string()),
            }
        } else if c.is_ascii_digit() {
            let (radix, digits_start) = if c == b'0' && matches!(b.get(i + 1), Some(b'x') | Some(b'X')) { (16, i + 2) } else { (10, i) };
            i = digits_start;
            while i < b.len() && (b[i].is_ascii_hexdigit() && (radix == 16 || b[i].is_ascii_digit()) || b[i] == b'_') {
                i += 1;
            }
            let digits: String = src[digits_start..i].chars().filter(|c| *c != '_').collect();
            match i128::from_str_radix(&digits, radix) {
                Ok(v) if v <= u64::MAX as i128 => Tok::Int(v),
                _ => return Err(Diagnostic::error("E0001", Span::new(start, i), "integer literal is too large").with_note("`int` is a 64-bit signed integer")),
            }
        } else if c == b'"' {
            i += 1;
            let mut s = String::new();
            loop {
                if i >= b.len() || b[i] == b'\n' {
                    return Err(Diagnostic::error("E0001", Span::new(start, i), "unterminated string literal").with_fix("add the closing `\"` on the same line"));
                }
                let ch = src[i..].chars().next().unwrap();
                if ch == '"' {
                    i += 1;
                    break;
                }
                if ch == '\\' {
                    let esc = b.get(i + 1).copied().unwrap_or(b' ');
                    s.push(match esc {
                        b'n' => '\n',
                        b't' => '\t',
                        b'\\' => '\\',
                        b'"' => '"',
                        b'0' => '\0',
                        b'{' => '{',
                        _ => return Err(Diagnostic::error("E0001", Span::new(i, i + 2), "unknown escape sequence").with_fix("use one of \\n \\t \\\\ \\\" \\0 \\{")),
                    });
                    i += 2;
                    continue;
                }
                s.push(ch);
                i += ch.len_utf8();
            }
            Tok::Str(s)
        } else if c == b';' {
            i += 1;
            Tok::Semi
        } else {
            match PUNCT.iter().find(|p| src[i..].starts_with(**p)) {
                Some(p) => {
                    i += p.len();
                    Tok::P(p)
                }
                None => {
                    let ch = src[i..].chars().next().unwrap();
                    return Err(Diagnostic::error("E0001", Span::new(i, i + ch.len_utf8()), format!("unexpected character `{ch}`")));
                }
            }
        };
        raw.push(Token { tok, span: Span::new(start, i), newline_before: newline });
        newline = false;
    }
    raw.push(Token { tok: Tok::Eof, span: Span::new(b.len(), b.len()), newline_before: true });

    // Insert statement ends at newlines.
    let mut out = Vec::with_capacity(raw.len() + raw.len() / 4);
    for t in raw {
        if t.newline_before {
            if let Some(prev) = out.last() {
                let prev: &Token = prev;
                if ends_statement(&prev.tok) && !continues_line(&t.tok) {
                    let at = prev.span.hi as usize;
                    out.push(Token { tok: Tok::Semi, span: Span::new(at, at), newline_before: false });
                }
            }
        }
        out.push(t);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn toks(s: &str) -> Vec<Tok> {
        lex(s).unwrap().into_iter().map(|t| t.tok).collect()
    }
    #[test]
    fn inserts_semicolons_at_line_ends() {
        assert_eq!(
            toks("let x = 1\nx"),
            vec![Tok::Kw("let"), Tok::Ident("x".into()), Tok::P("="), Tok::Int(1), Tok::Semi, Tok::Ident("x".into()), Tok::Semi, Tok::Eof]
        );
    }
    #[test]
    fn continuation_lines_do_not_end_statements() {
        assert_eq!(toks("a\n  && b").iter().filter(|t| **t == Tok::Semi).count(), 1);
        assert_eq!(toks("fn f(x: int)\n  requires x > 0\n{").iter().filter(|t| **t == Tok::Semi).count(), 0);
    }
    #[test]
    fn numbers_strings_and_comments() {
        assert_eq!(toks("1_000 0xff // c\n\"a\\n\""), vec![Tok::Int(1000), Tok::Int(255), Tok::Semi, Tok::Str("a\n".into()), Tok::Semi, Tok::Eof]);
    }
}
