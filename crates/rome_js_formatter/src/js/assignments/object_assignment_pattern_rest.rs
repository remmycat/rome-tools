use crate::formatter_traits::FormatTokenAndNode;
use rome_formatter::format_elements;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsObjectAssignmentPatternRest;
use rome_js_syntax::JsObjectAssignmentPatternRestFields;

impl ToFormatElement for JsObjectAssignmentPatternRest {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternRestFields {
            dotdotdot_token,
            target,
        } = self.as_fields();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            target.format(formatter)?,
        ])
    }
}
