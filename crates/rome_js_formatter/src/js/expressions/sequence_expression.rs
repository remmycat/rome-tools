use crate::formatter_traits::FormatTokenAndNode;
use rome_formatter::format_elements;

use crate::{space_token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsSequenceExpression;
use rome_js_syntax::JsSequenceExpressionFields;

impl ToFormatElement for JsSequenceExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSequenceExpressionFields {
            left,
            comma_token,
            right,
        } = self.as_fields();

        Ok(format_elements![
            left.format(formatter)?,
            comma_token.format(formatter)?,
            space_token(),
            right.format(formatter)?,
        ])
    }
}
