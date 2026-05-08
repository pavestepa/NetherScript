use std::fmt::Write as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<String>,
    pub message: String,
    pub span: Option<SourceSpan>,
    pub snippet: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SourceSpan {
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub label: Option<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: None,
            message: message.into(),
            span: None,
            snippet: None,
            notes: Vec::new(),
        }
    }

    pub fn error_with_code(code: impl Into<String>, message: impl Into<String>) -> Self {
        let mut d = Self::error(message);
        d.code = Some(code.into());
        d
    }

    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }

    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn render_pretty(&self) -> String {
        let sev = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        let mut out = String::new();
        if let Some(code) = &self.code {
            let _ = writeln!(out, "{sev}[{code}]: {}", self.message);
        } else {
            let _ = writeln!(out, "{sev}: {}", self.message);
        }

        if let Some(span) = &self.span {
            let file = span.file.as_deref().unwrap_or("<unknown>");
            let (line, col) = if let (Some(file_path), Some(start)) = (span.file.as_deref(), span.start) {
                line_col_from_offset(file_path, start).unwrap_or((span.line.unwrap_or(0), span.column.unwrap_or(0)))
            } else {
                (span.line.unwrap_or(0), span.column.unwrap_or(0))
            };
            let _ = writeln!(out, "  --> {file}:{line}:{col}");

            if let Some(file_path) = span.file.as_deref() {
                if let Some(line_src) = source_line_at(file_path, line) {
                    out.push_str("   |\n");
                    let _ = writeln!(out, "{line:>3}| {line_src}");
                    if col > 0 {
                        let caret_pad = col.saturating_sub(1) as usize;
                        let _ = writeln!(out, "   | {}^", " ".repeat(caret_pad));
                    }
                    out.push_str("   |\n");
                }
            }
        } else if self.snippet.is_some() {
            out.push_str("  --> <semantic>:0:0\n");
        }

        if let Some(snippet) = &self.snippet {
            out.push_str("   |\n");
            let _ = writeln!(out, "   | {snippet}");
            out.push_str("   |\n");
        }
        for note in &self.notes {
            let _ = writeln!(out, "   = note: {note}");
        }
        out
    }
}

fn line_col_from_offset(path: &str, offset: u32) -> Option<(u32, u32)> {
    let src = std::fs::read_to_string(path).ok()?;
    let mut line = 1u32;
    let mut col = 1u32;
    let mut idx = 0u32;
    for ch in src.chars() {
        if idx >= offset {
            return Some((line, col));
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        idx += ch.len_utf8() as u32;
    }
    Some((line, col))
}

fn source_line_at(path: &str, line: u32) -> Option<String> {
    let src = std::fs::read_to_string(path).ok()?;
    src.lines()
        .nth(line.saturating_sub(1) as usize)
        .map(|s| s.to_string())
}
