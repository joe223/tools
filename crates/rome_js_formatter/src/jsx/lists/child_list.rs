use crate::utils::{format_jsx_text_replaced_with, is_meaningful_jsx_text};
use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_formatter::{
    concat_elements, empty_line, format_elements, group_elements, hard_line_break, indent,
    join_elements, join_elements_soft_line, soft_block_indent, soft_line_break,
    soft_line_break_or_space, token, IndentStyle, Token,
};
use rome_js_syntax::{JsxAnyChild, JsxChildList};
use rome_rowan::AstNode;

impl Format for JsxChildList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_jsx_child_list(self.clone(), formatter, 1))
    }
}

pub(crate) fn format_jsx_child_list(
    list: JsxChildList,
    formatter: &Formatter,
    indentation_level: u8,
) -> FormatElement {
    let list = list.into_iter().enumerate().map(|(index, child)| {
        let formatted_child = if let JsxAnyChild::JsxText(text) = child.clone() {
            if is_meaningful_jsx_text(&text) {
                let content = text.text();
                let resu: Vec<_> = content.split_ascii_whitespace().collect();
                dbg!(resu);
                if content.matches("\n").count() > 1 {
                    format_jsx_text_replaced_with(text, formatter, empty_line())
                } else if content.starts_with('\n') || content.ends_with('\n') {
                    format_jsx_text_replaced_with(text, formatter, hard_line_break())
                } else {
                    Ok(format_elements![text.format(formatter).unwrap()])
                }
                // let mut spaces = String::from("\n");
                // let indentation =
                //     if let IndentStyle::Space(space_quantity) = formatter.options().indent_style {
                //         " ".repeat(space_quantity as usize)
                //     } else {
                //         String::from("\t")
                //     };
                // spaces.push_str(indentation.repeat(indentation_level as usize).as_str());
                // let new_string = FormatElement::from(Token::new_dynamic(
                //     spaces,
                //     text.syntax().text_trimmed_range().start(),
                // ));
                // format_jsx_text_replaced_with(text, formatter, new_string)
            } else {
                text.format(formatter)
                // format_jsx_text_replaced_with(text, formatter, hard_line_break())
            }
        } else {
            child.format(formatter)
        };
        let snapshot = formatter.snapshot();

        let formatted_child = match formatted_child {
            Ok(result) => result,
            Err(_) => {
                formatter.restore(snapshot);
                // Lists that yield errors are formatted as they were unknown nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                formatter.format_unknown(child.syntax())
            }
        };

        formatted_child
    });

    concat_elements(list)
}
