use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_formatter::empty_element;

use rome_js_syntax::JsObjectExpression;
use rome_js_syntax::JsObjectExpressionFields;
use rome_rowan::AstSeparatedList;

impl ToFormatElement for JsObjectExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectExpressionFields {
            l_curly_token,
            members,
            r_curly_token,
        } = self.as_fields();

        if members.is_empty() {
            formatter.format_delimited_soft_block_indent(
                &l_curly_token?,
                empty_element(),
                &r_curly_token?,
            )
        } else {
            formatter.format_delimited_soft_block_spaces(
                &l_curly_token?,
                members.format(formatter)?,
                &r_curly_token?,
            )
        }
    }
}
