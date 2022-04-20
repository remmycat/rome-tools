use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_with_semicolon;
use crate::{space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_formatter::format_elements;
use rome_js_syntax::TsExportAsNamespaceClause;
use rome_js_syntax::TsExportAsNamespaceClauseFields;

impl ToFormatElement for TsExportAsNamespaceClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExportAsNamespaceClauseFields {
            as_token,
            namespace_token,
            name,
            semicolon_token,
        } = self.as_fields();

        let result = format_with_semicolon(
            formatter,
            format_elements![
                as_token.format(formatter)?,
                space_token(),
                namespace_token.format(formatter)?,
                space_token(),
                name.format(formatter)?,
            ],
            semicolon_token,
        );
        result
    }
}
