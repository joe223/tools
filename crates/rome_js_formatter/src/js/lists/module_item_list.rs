use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_formatter::format_element::get_lines_between_nodes;
use rome_formatter::{concat_elements, empty_line, format_elements, hard_line_break};
use rome_js_syntax::{JsAnyImportClause, JsAnyModuleItem, JsModuleItemList, JsSyntaxNode};
use rome_rowan::AstNode;
use std::cmp::Ordering;
use std::fmt::Debug;

impl ToFormatElement for JsModuleItemList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let unstable_features = formatter.unstable_features();
        if unstable_features.unstable_sort_imports {
            unstable_sort_imports(self.clone(), formatter)
        } else {
            Ok(formatter.format_list(self.clone()))
        }
    }
}

/// Small data structure to track all the [JsImport] found inside a [JsModuleItemList]
///
/// Because of the fact that the list items can appear in every order, and that potential statements
/// can be found between [JsImport], we track each [JsImport] until we find another item that is not
/// a [JsImport]. When this condition is met, we sort the [JsImport] found so far, empty the list, and then we start
/// to track them again.
///
/// Given the following example:
///
/// ```js
/// import * as fs from "node:fs";
/// import "some-polyfill";
///
/// window.loadPolyfill();
///
/// import { sort } from "lodash";
/// import "bootstrap"
///
/// ```
///
/// The first two imports will be sorted by themselves, because the third item found is not a [JsImport]
/// and we can't moving it around because it might depend on some side effect of the previous nodes.
/// The statement `import "some-polyfill"` will be put at the top.
///
///  The end result will be something like:
///
/// ```js
/// import "some-polyfill";
/// import * as fs from "node:fs";
///
/// window.loadPolyfill();
///
/// import "bootstrap"
/// import { sort } from "lodash";
/// ```
///
///
/// [JsImport]: rome_js_syntax::JsImport
/// [JsModuleItemList]: rome_js_syntax::JsModuleItemList
#[derive(Debug, Default)]
struct SortedImports {
    import_list: Vec<Import>,
    result: Vec<FormatElement>,
}

impl SortedImports {
    /// Given a reference to the type of the import, it stores it inside an intermediate list
    pub fn store_formatted_import_clause(
        &mut self,
        import_clause: JsAnyImportClause,
        formatted: FormatElement,
        trailing_lines: usize,
    ) {
        if matches!(import_clause, JsAnyImportClause::JsImportBareClause(_)) {
            self.import_list.push(Import::PossiblyWithSideEffects(
                import_clause.syntax().clone(),
                formatted,
                trailing_lines,
            ))
        } else {
            self.import_list.push(Import::Safe(
                import_clause.syntax().clone(),
                formatted,
                trailing_lines,
            ))
        }
    }

    /// It stores any module item that is not a [JsImport]
    pub fn store_formatted_module_item(
        &mut self,
        item: JsAnyModuleItem,
        formatted: FormatElement,
        trailing_lines: usize,
    ) {
        // we don't want to deliberately store JsImport nodes because they should be treated differently
        debug_assert!(!matches!(item, JsAnyModuleItem::JsImport(_)));
        if !self.import_list.is_empty() {
            self.sort_and_store_import_list(false);
        }
        self.result.push(if trailing_lines > 1 {
            format_elements![formatted, empty_line()]
        } else {
            format_elements![formatted, hard_line_break()]
        })
    }

    pub fn into_format_element(mut self) -> FormatElement {
        // we retrieve potential dangling items inside the import list
        if !self.import_list.is_empty() {
            self.sort_and_store_import_list(true);
        }

        concat_elements(self.result.into_iter())
    }

    /// It sorts the [JsImport] stored so far and then empty them
    fn sort_and_store_import_list(&mut self, is_last: bool) {
        self.import_list.sort_unstable_by_key(|key| match key {
            Import::PossiblyWithSideEffects(_, _, _) => Ordering::Less,
            Import::Safe(_, _, _) => Ordering::Greater,
            _ => Ordering::Equal,
        });
        let formatted_list = self.formatted_import_list(is_last);
        self.result.push(formatted_list);
    }

    fn formatted_import_list(&mut self, is_last: bool) -> FormatElement {
        let mut found_trailing_lines = false;
        let len = self.import_list.len();
        let formatted_list = self
            .import_list
            .drain(..)
            .enumerate()
            .map(|(index, import_item)| {
                if import_item.has_trailing_lines() {
                    found_trailing_lines = true
                }
                let formatted = import_item.into_format_element();
                if index + 1 == len {
                    if is_last {
                        format_elements![formatted, hard_line_break()]
                    } else if found_trailing_lines {
                        format_elements![formatted, empty_line()]
                    } else {
                        format_elements![formatted, hard_line_break()]
                    }
                } else {
                    format_elements![formatted, hard_line_break()]
                }
            });

        concat_elements(formatted_list)
    }
}

#[derive(Eq, PartialEq)]
enum Import {
    PossiblyWithSideEffects(JsSyntaxNode, FormatElement, usize),
    Safe(JsSyntaxNode, FormatElement, usize),
}

impl Import {
    /// Consumes self to to return a [FormatElement]
    pub fn into_format_element(self) -> FormatElement {
        match self {
            Import::PossiblyWithSideEffects(_, element, _) => element,
            Import::Safe(_, element, _) => element,
        }
    }

    pub fn has_trailing_lines(&self) -> bool {
        match self {
            Import::PossiblyWithSideEffects(_, _, trailing_lines) => *trailing_lines > 1,
            Import::Safe(_, _, trailing_lines) => *trailing_lines > 1,
        }
    }
}

impl Debug for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Import::PossiblyWithSideEffects(_, _, lines) => write!(f, "Side effects {}", lines),
            Import::Safe(_, _, lines) => write!(f, "Safe {}", lines),
        }
    }
}

/// Function that implements the sorting of imports
fn unstable_sort_imports(
    list: JsModuleItemList,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let mut sorted_imports = SortedImports::default();
    let mut peekable_list = list.into_iter().peekable();
    while let Some(item) = peekable_list.next() {
        // before applying sorting, we want to know how many lines there are between the current node
        // and the next one, so we maintain possible empty lines when we reformat the statements
        let next_item = peekable_list.peek();
        let trailing_lines = next_item.map_or(0, |next_item| {
            get_lines_between_nodes(item.syntax(), next_item.syntax())
        });

        if let JsAnyModuleItem::JsImport(import) = item {
            let formatted = import.format(formatter)?;
            sorted_imports.store_formatted_import_clause(
                import.import_clause()?,
                formatted,
                trailing_lines,
            );
        } else {
            let state = formatter.snapshot();

            // we might encounter unknown nodes, hence we catch the error case and we restore the snapshot
            let formatted = match item.format(formatter) {
                Err(_) => {
                    formatter.restore(state);
                    // Lists that yield errors are formatted as they were unknown nodes.
                    // Doing so, the formatter formats the nodes/tokens as is.
                    formatter.format_unknown(item.syntax())
                }
                Ok(element) => element,
            };
            sorted_imports.store_formatted_module_item(item, formatted, trailing_lines);
        }
    }

    Ok(sorted_imports.into_format_element())
}
