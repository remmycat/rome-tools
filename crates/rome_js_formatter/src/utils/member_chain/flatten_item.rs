use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsComputedMemberExpression, JsStaticMemberExpression,
    JsSyntaxNode,
};
use rome_rowan::{AstNode, SyntaxResult};
use std::fmt::Debug;

#[derive(Clone, Debug)]
/// Data structure that holds the node with its formatted version
pub(crate) enum FlattenItem {
    /// Holds onto a [rome_js_syntax::JsStaticMemberExpression]
    StaticMember(JsStaticMemberExpression),
    /// Holds onto a [rome_js_syntax::JsCallExpression]
    CallExpression(JsCallExpression),
    /// Holds onto a [rome_js_syntax::JsComputedMemberExpression]
    ComputedMember(JsComputedMemberExpression),
    /// Any other node that are not  [rome_js_syntax::JsCallExpression] or [rome_js_syntax::JsStaticMemberExpression]
    /// Are tracked using this variant
    Expression(JsAnyExpression),
}

impl FlattenItem {
    /// checks if the current node is a [rome_js_syntax::JsCallExpression],  [rome_js_syntax::JsImportExpression] or a [rome_js_syntax::JsNewExpression]
    pub fn is_loose_call_expression(&self) -> bool {
        matches!(
            self,
            FlattenItem::CallExpression(_)
                | FlattenItem::Expression(
                    JsAnyExpression::JsImportCallExpression(_)
                        | JsAnyExpression::JsNewExpression(_)
                )
        )
    }

    pub(crate) fn as_syntax(&self) -> &JsSyntaxNode {
        match self {
            FlattenItem::StaticMember(node) => node.syntax(),
            FlattenItem::CallExpression(node) => node.syntax(),
            FlattenItem::ComputedMember(node) => node.syntax(),
            FlattenItem::Expression(node) => node.syntax(),
        }
    }

    pub(crate) fn has_trailing_comments(&self) -> bool {
        self.as_syntax().has_trailing_comments()
    }

    pub fn is_computed_expression(&self) -> bool {
        matches!(self, FlattenItem::ComputedMember(..))
    }

    pub(crate) fn is_this_expression(&self) -> bool {
        matches!(
            self,
            FlattenItem::Expression(JsAnyExpression::JsThisExpression(_))
        )
    }

    pub(crate) fn is_identifier_expression(&self) -> bool {
        matches!(
            self,
            FlattenItem::Expression(JsAnyExpression::JsIdentifierExpression(_))
        )
    }

    /// There are cases like Object.keys(), Observable.of(), _.values() where
    /// they are the subject of all the chained calls and therefore should
    /// be kept on the same line:
    ///
    /// ```js
    ///   Object.keys(items)
    ///     .filter(x => x)
    ///     .map(x => x)
    /// ```
    /// In order to detect those cases, we use an heuristic: if the first
    /// node is an identifier with the name starting with a capital
    /// letter or just a sequence of _$. The rationale is that they are
    /// likely to be factories.
    ///
    /// Comment from [Prettier]
    ///
    /// [Prettier]: https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js#L252-L266
    pub(crate) fn is_factory(&self, check_left_hand_side: bool) -> SyntaxResult<bool> {
        fn check_str(text: &str) -> bool {
            text.chars().next().map_or(false, |c| c.is_uppercase())
                || text.starts_with('_')
                || text.starts_with('$')
        }

        if let FlattenItem::StaticMember(static_member, ..) = self {
            if check_left_hand_side {
                if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                    static_member.object()?
                {
                    let value_token = identifier_expression.name()?.value_token()?;
                    let text = value_token.text_trimmed();
                    Ok(check_str(text))
                } else {
                    Ok(false)
                }
            } else {
                Ok(check_str(static_member.member()?.text().as_str()))
            }
        } else if let FlattenItem::Expression(node, ..) = self {
            if let JsAnyExpression::JsIdentifierExpression(identifier_expression) = node {
                let value_token = identifier_expression.name()?.value_token()?;
                let text = value_token.text_trimmed();
                Ok(check_str(text))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub(crate) fn has_short_name(&self, tab_width: u8) -> SyntaxResult<bool> {
        if let FlattenItem::StaticMember(static_member, ..) = self {
            if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                static_member.object()?
            {
                let value_token = identifier_expression.name()?.value_token()?;
                let text = value_token.text_trimmed();
                Ok(text.len() <= tab_width as usize)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl Format<JsFormatContext> for FlattenItem {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            FlattenItem::StaticMember(static_member) => {
                write![
                    f,
                    [
                        static_member.operator_token().format(),
                        static_member.member().format(),
                    ]
                ]
            }
            FlattenItem::CallExpression(call_expression) => {
                write!(
                    f,
                    [
                        call_expression.optional_chain_token().format(),
                        call_expression.type_arguments().format(),
                        call_expression.arguments().format()
                    ]
                )
            }
            FlattenItem::ComputedMember(computed_member) => {
                write!(
                    f,
                    [
                        computed_member.optional_chain_token().format(),
                        computed_member.l_brack_token().format(),
                        computed_member.member().format(),
                        computed_member.r_brack_token().format(),
                    ]
                )
            }
            FlattenItem::Expression(node) => {
                write!(f, [node.format()])
            }
        }
    }
}
