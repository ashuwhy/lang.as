//! Source spans and diagnostics. Every diagnostic is written as an instruction: a stable code,
//! what is wrong, where, a concrete fix when one is known, and a counterexample when the
//! verifier found one. Rendered as text for people and as JSON for agents.

use serde_json::{json, Value};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Span {
        Span { lo: lo as u32, hi: hi as u32 }
    }
    pub fn to(self, other: Span) -> Span {
        Span { lo: self.lo.min(other.lo), hi: self.hi.max(other.hi) }
    }
}

pub struct Source {
    pub name: String,
    pub text: String,
    line_starts: Vec<usize>,
}

impl Source {
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Source {
        let text = text.into();
        let mut line_starts = vec![0];
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Source { name: name.into(), text, line_starts }
    }

    /// 1-based line and column of a byte offset.
    pub fn line_col(&self, off: u32) -> (usize, usize) {
        let off = off as usize;
        let line = match self.line_starts.binary_search(&off) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let col = self.text[self.line_starts[line]..off.min(self.text.len())].chars().count();
        (line + 1, col + 1)
    }

    pub fn line_text(&self, line: usize) -> &str {
        let start = self.line_starts[line - 1];
        let end = self.line_starts.get(line).map(|e| e - 1).unwrap_or(self.text.len());
        &self.text[start..end.max(start)]
    }

    pub fn slice(&self, s: Span) -> &str {
        &self.text[s.lo as usize..s.hi as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub code: &'static str,
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub fix: Option<String>,
    pub notes: Vec<String>,
    pub counterexample: Vec<(String, String)>,
}

impl Diagnostic {
    pub fn error(code: &'static str, span: Span, message: impl Into<String>) -> Diagnostic {
        Diagnostic {
            code,
            severity: Severity::Error,
            message: message.into(),
            span,
            fix: None,
            notes: vec![],
            counterexample: vec![],
        }
    }
    pub fn warning(code: &'static str, span: Span, message: impl Into<String>) -> Diagnostic {
        Diagnostic { severity: Severity::Warning, ..Diagnostic::error(code, span, message) }
    }
    pub fn with_fix(mut self, fix: impl Into<String>) -> Diagnostic {
        self.fix = Some(fix.into());
        self
    }
    pub fn with_note(mut self, note: impl Into<String>) -> Diagnostic {
        self.notes.push(note.into());
        self
    }
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }

    pub fn render(&self, src: &Source) -> String {
        let sev = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
        };
        let (line, col) = src.line_col(self.span.lo);
        let mut out = format!("{sev}[{}]: {}\n  --> {}:{line}:{col}\n", self.code, self.message, src.name);
        let text = src.line_text(line);
        let gutter = line.to_string().len();
        let (end_line, end_col) = src.line_col(self.span.hi.max(self.span.lo + 1));
        let width = if end_line == line { end_col.saturating_sub(col).max(1) } else { text.chars().count().saturating_sub(col - 1).max(1) };
        out += &format!("{:gutter$} |\n{line} | {text}\n{:gutter$} | {}{}\n", "", "", " ".repeat(col - 1), "^".repeat(width));
        if !self.counterexample.is_empty() {
            let ce: Vec<String> = self.counterexample.iter().map(|(k, v)| format!("{k} = {v}")).collect();
            out += &format!("{:gutter$} = counterexample: {}\n", "", ce.join(", "));
        }
        for n in &self.notes {
            out += &format!("{:gutter$} = note: {n}\n", "");
        }
        if let Some(f) = &self.fix {
            out += &format!("{:gutter$} = fix: {f}\n", "");
        }
        out
    }

    pub fn to_json(&self, src: &Source) -> Value {
        let (line, col) = src.line_col(self.span.lo);
        let (end_line, end_col) = src.line_col(self.span.hi);
        json!({
            "code": self.code,
            "severity": match self.severity { Severity::Error => "error", Severity::Warning => "warning", Severity::Note => "note" },
            "message": self.message,
            "file": src.name,
            "line": line, "col": col, "end_line": end_line, "end_col": end_col,
            "fix": self.fix,
            "notes": self.notes,
            "counterexample": self.counterexample.iter().map(|(k, v)| json!({"name": k, "value": v})).collect::<Vec<_>>(),
        })
    }
}
