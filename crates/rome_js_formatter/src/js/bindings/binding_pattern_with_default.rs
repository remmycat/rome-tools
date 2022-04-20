use crate::formatter_traits::FormatTokenAndNode;
use rome_formatter::format_elements;

use crate::{space_token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsBindingPatternWithDefault;
use rome_js_syntax::JsBindingPatternWithDefaultFields;

impl ToFormatElement for JsBindingPatternWithDefault {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBindingPatternWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = self.as_fields();

        Ok(format_elements![
            pattern.format(formatter)?,
            space_token(),
            eq_token.format(formatter)?,
            space_token(),
            default.format(formatter)?
        ])
    }
}
