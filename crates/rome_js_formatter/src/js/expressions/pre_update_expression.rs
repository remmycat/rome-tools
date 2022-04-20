use crate::formatter_traits::FormatTokenAndNode;
use rome_formatter::format_elements;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsPreUpdateExpression;
use rome_js_syntax::JsPreUpdateExpressionFields;

impl ToFormatElement for JsPreUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPreUpdateExpressionFields {
            operator_token,
            operand,
        } = self.as_fields();

        Ok(format_elements![
            operator_token.format(formatter)?,
            operand.format(formatter)?,
        ])
    }
}
