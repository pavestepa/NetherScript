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
            out.push_str(&format!("{sev}[{code}]: {}\n", self.message));
        } else {
            out.push_str(&format!("{sev}: {}\n", self.message));
        }
        if let Some(span) = &self.span {
            let file = span.file.as_deref().unwrap_or("<unknown>");
            let line = span.line.unwrap_or(0);
            let col = span.column.unwrap_or(0);
            out.push_str(&format!("  --> {file}:{line}:{col}\n"));
        } else if self.snippet.is_some() {
            out.push_str("  --> <semantic>:0:0\n");
        }
        if let Some(snippet) = &self.snippet {
            out.push_str("   |\n");
            out.push_str(&format!("   | {snippet}\n"));
            out.push_str("   |\n");
        }
        for note in &self.notes {
            out.push_str(&format!("   = note: {note}\n"));
        }
        out
    }
}
