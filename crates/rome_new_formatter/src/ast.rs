use crate::format_element::Token;
use crate::{normalize_newlines, write, Format, Formatter, LINE_TERMINATORS};
use rome_rowan::{Language, SyntaxToken, SyntaxTriviaPieceComments};

trait FormatNode<L: Language>: Format {
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        self.format_fields(formatter)
    }

    fn format_fields(&self, formatter: &mut Formatter) -> crate::Result<()>;
}

impl<L: Language> Format for SyntaxToken<L> {
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        write!(formatter, Token::from(self))
    }
}

impl<L: Language> Format for SyntaxTriviaPieceComments<L> {
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        let range = self.text_range();
        let token = Token::from_syntax_token_cow_slice(
            normalize_newlines(self.text().trim(), LINE_TERMINATORS),
            &self.as_piece().token(),
            range.start(),
        );

        write!(formatter, token)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        block_indent, format, format_args, format_with, group, hard_line_break, space_token, token,
        write, Buffer, Format, FormatElement, Formatter, VecBuffer,
    };
    use rome_js_syntax::{JsBlockStatement, JsIfStatement, JsIfStatementFields};
    use rome_rowan::{SyntaxResult, SyntaxToken};

    impl Format for JsBlockStatement {
        fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
            let mut buf = VecBuffer::new(*formatter.options());

            // TODO use join_elements_with which handles the spacing
            let list = formatter.join_with(&hard_line_break());

            for statement in self.statements() {
                match write!(buf, statement) {
                    Ok(_) => {
                        buf.write_into(formatter)?;
                    }
                    Err(_) => {
                        buf.clear();
                        // Format unknown
                    }
                }
            }

            Ok(())
        }
    }

    impl Format for JsIfStatement {
        fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
            let JsIfStatementFields {
                if_token,
                l_paren_token,
                test,
                r_paren_token,
                consequent,
                else_clause,
            } = self.as_fields();

            // Wouldn't work because group and `block_indent` would want to take ownership of formatter
            // formatter
            //  .group()
            //      .element(token("{"))
            //      .element(
            //          formatter
            //              .block_indent()
            //              .entry(stmt))
            //      .element(token("}")

            // list.entry(token("next"));

            write!(
                formatter,
                group(format_args!(if_token?, block_indent(&token("test"))))
            )?;

            // formatter.group().entry().finish()?;

            // formatter.fill().entry(item).finish()?;
            //
            // formatter.fill().entries(entries).finish();

            // write!(
            //     formatter,
            //     if_token?,
            //     space_token(),
            //     l_paren_token?,
            //     test?,
            //     r_paren_token,
            //     space_token()
            // )?;

            Ok(())
        }
    }
}
