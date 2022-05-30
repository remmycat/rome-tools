use crate::generated::FormatTsPropertyParameterModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsPropertyParameterModifierList;

impl FormatRule<TsPropertyParameterModifierList> for FormatTsPropertyParameterModifierList {
    type Context = JsFormatContext;

    fn format(
        node: &TsPropertyParameterModifierList,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(sort_modifiers_by_precedence(node).into_iter().formatted())?,
        ))
    }
}
