use crate::diagnostic::{Diagnostic, SourceSpan};

use super::{checker::TypeChecker, types::CheckedType};

impl TypeChecker<'_> {
    pub(super) fn report(&mut self, code: &str, message: String, snippet: String, notes: Vec<String>) {
        let mut diagnostic = Diagnostic::error_with_code(code, message)
            .with_span(SourceSpan {
                file: None,
                line: None,
                column: None,
                start: None,
                end: None,
                label: Some("synthetic span from sema node".to_string()),
            })
            .with_snippet(snippet);
        for note in notes {
            diagnostic = diagnostic.with_note(note);
        }
        self.ctx.diagnostics.push(diagnostic);
    }

    pub(super) fn expect_assignable(
        &mut self,
        expected: &CheckedType,
        got: &CheckedType,
        what: &str,
        snippet: String,
    ) {
        if self.is_assignable(expected, got)
            || matches!(expected, CheckedType::Error)
            || matches!(got, CheckedType::Error)
        {
            return;
        }
        // E0613: central expected/found assignability failure.
        self.report(
            "E0613",
            what.to_string(),
            snippet,
            vec![
                format!("expected `{}`", self.type_name(expected)),
                format!("found `{}`", self.type_name(got)),
            ],
        );
    }

    pub(super) fn type_name(&self, ty: &CheckedType) -> String {
        match ty {
            CheckedType::Resolved(id) => self
                .type_leaf_name(*id)
                .unwrap_or_else(|| format!("type#{}", id.0)),
            CheckedType::Callable { params, ret } => {
                let params_s = params
                    .iter()
                    .map(|id| self.type_leaf_name(*id).unwrap_or_else(|| format!("type#{}", id.0)))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret_s = self.type_leaf_name(*ret).unwrap_or_else(|| format!("type#{}", ret.0));
                format!("fn({params_s}) -> {ret_s}")
            }
            CheckedType::Error => "Error".to_string(),
        }
    }
}
