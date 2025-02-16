//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyNamedImport;
impl Format for JsAnyNamedImport {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsNamedImportSpecifiers(node) => node.format(formatter),
            Self::JsNamespaceImportSpecifier(node) => node.format(formatter),
        }
    }
}
