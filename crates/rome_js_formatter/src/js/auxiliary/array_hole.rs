use crate::{empty_element, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsArrayHole;

impl FormatNode for JsArrayHole {
    fn format_fields(&self, _: &Formatter) -> FormatResult<FormatElement> {
        Ok(empty_element())
    }
}
