use crate::prelude::*;
use crate::utils::format_call_expression;

use crate::FormatNodeFields;
use rome_js_syntax::{JsAnyExpression, JsCallExpression};

impl FormatNodeFields<JsCallExpression> for FormatNodeRule<JsCallExpression> {
    fn fmt_fields(node: &JsCallExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_call_expression(JsAnyExpression::JsCallExpression(node.clone()), formatter)
    }
}
