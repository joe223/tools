//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyForInitializer;
impl Format for JsAnyForInitializer {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsVariableDeclaration(node) => node.format(formatter),
            Self::JsAnyExpression(node) => node.format(formatter),
        }
    }
}
