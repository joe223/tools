use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsObjectAssignmentPatternRest;
use rome_js_syntax::JsObjectAssignmentPatternRestFields;

impl FormatNode for JsObjectAssignmentPatternRest {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternRestFields {
            dotdotdot_token,
            target,
        } = self.as_fields();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            target.format(formatter)?,
        ])
    }
}
