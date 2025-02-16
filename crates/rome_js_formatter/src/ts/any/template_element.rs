//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::TsAnyTemplateElement;
impl Format for TsAnyTemplateElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsTemplateChunkElement(node) => node.format(formatter),
            Self::TsTemplateElement(node) => node.format(formatter),
        }
    }
}
