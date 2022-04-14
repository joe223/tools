use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_formatter::format_elements;
use rome_js_syntax::{JsxSpreadAttribute, JsxSpreadAttributeFields};

impl ToFormatElement for JsxSpreadAttribute {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxSpreadAttributeFields {
            l_curly_token,
            dotdotdot_token,
            argument,
            r_curly_token,
        } = self.as_fields();

        Ok(format_elements![
            l_curly_token.format(formatter)?,
            dotdotdot_token.format(formatter)?,
            argument.format(formatter)?,
            r_curly_token.format(formatter)?,
        ])
    }
}
