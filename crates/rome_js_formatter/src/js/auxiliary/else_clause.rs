use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsElseClause;
use rome_js_syntax::JsElseClauseFields;

impl FormatNode for JsElseClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsElseClauseFields {
            else_token,
            alternate,
        } = self.as_fields();

        Ok(format_elements![
            else_token.format(formatter)?,
            space_token(),
            alternate.format(formatter)?,
        ])
    }
}
