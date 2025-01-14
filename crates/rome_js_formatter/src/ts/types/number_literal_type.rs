use crate::format_traits::FormatOptional;
use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsNumberLiteralType;

impl FormatNode for TsNumberLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let minus = self.minus_token().format_or_empty(formatter)?;
        let literal = self.literal_token().format(formatter)?;
        Ok(format_elements![minus, literal])
    }
}
