use crate::{
    empty_element, hard_line_break, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_formatter::format_elements;
use rome_js_syntax::JsDirectiveList;
use rome_rowan::AstNodeList;

impl ToFormatElement for JsDirectiveList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if !self.is_empty() {
            Ok(format_elements![
                formatter.format_list(self),
                hard_line_break()
            ])
        } else {
            Ok(empty_element())
        }
    }
}
