use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsQualifiedName;
use rome_js_syntax::TsQualifiedNameFields;

impl FormatNode for TsQualifiedName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsQualifiedNameFields {
            left,
            dot_token,
            right,
        } = self.as_fields();

        Ok(format_elements![
            left.format(formatter)?,
            dot_token.format(formatter)?,
            right.format(formatter)?,
        ])
    }
}
