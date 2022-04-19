use crate::arguments::Argument;
use crate::format_element::Token;
use crate::{Arguments, Buffer, Format, Formatter, Sequence};
use rome_rowan::{
    AstNode, AstNodeList, AstNodeListIterator, AstSeparatedList, AstSeparatedListNodesIterator,
    Language, SyntaxElement, SyntaxList, SyntaxSlots, SyntaxToken,
};

trait FormatNode<L: Language>: Format {
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        self.format_fields(formatter)
    }

    fn format_fields(&self, formatter: &mut Formatter) -> crate::Result<()>;
}

impl<L: Language> Format for SyntaxToken<L> {
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        // print leading trivia
        // TODO use macro
        let token = Token::from(self);
        let token_arg = Argument::new(&token);
        let args = [token_arg];

        formatter.write_fmt(&Arguments::new(&args))
    }
}

// impl<L: Language> Sequence for SyntaxList<L> {
//     type Item = SyntaxElement<L>;
//     type Iter = SyntaxSlots<L>;
//
//     fn iter(&self) -> Self::Iter {
//         SyntaxList::iter(self)
//     }
// }
//
// impl<T> Sequence for T
// where
//     T: AstNodeList,
// {
//     type Item = T::Node;
//     type Iter = AstNodeListIterator<T::Language, T::Node>;
//
//     fn iter(&self) -> Self::Iter {
//         AstNodeList::iter(self)
//     }
// }

// impl<L: Language, N: AstNode<L>, T: AstSeparatedList<L, N>> Sequence for T {
//     type Item = N;
//     type Iter = AstSeparatedListNodesIterator<L, N>;
//
//     fn iter(&self) -> Self::Iter {
//         AstSeparatedList::iter(self)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{
        block_indent, format_args, format_with, group, space_token, token, write, Format,
        FormatElement, Formatter,
    };
    use rome_js_syntax::{JsIfStatement, JsIfStatementFields};
    use rome_rowan::{SyntaxResult, SyntaxToken};

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
