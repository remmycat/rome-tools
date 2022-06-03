use std::iter;

use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{JsAnyRoot, JsAnyStatement, JsCaseClause, JsSyntaxToken, TriviaPieceKind, T};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, TriviaPiece};

use crate::registry::{Rule, RuleAction, RuleDiagnostic};
use crate::{ActionCategory, RuleCategory};

pub(crate) enum UseSingleCaseStatement {}

impl Rule for UseSingleCaseStatement {
    const NAME: &'static str = "useSingleCaseStatement";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsCaseClause;
    type State = ();

    fn run(n: &Self::Query) -> Option<Self::State> {
        if n.consequent().len() > 1 {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(n: &Self::Query, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Error,
            message: markup! {
                "A switch case should only have a single statement."
            }
            .to_owned(),
            range: n.syntax().text_trimmed_range(),
        })
    }

    fn action(root: JsAnyRoot, n: &Self::Query, _: &Self::State) -> Option<RuleAction> {
        let mut token_text = String::new();
        let mut leading = Vec::new();

        if let Ok(token) = n.case_token() {
            for piece in token.leading_trivia().pieces() {
                token_text.push_str(piece.text());
                leading.push(TriviaPiece::new(piece.kind(), piece.text_len()));
            }
        }

        token_text.push('}');

        let root = root.replace_node_discard_trivia(
            n.consequent(),
            make::js_statement_list(iter::once(JsAnyStatement::JsBlockStatement(
                make::js_block_statement(
                    JsSyntaxToken::new_detached(
                        T!['{'],
                        " {",
                        [TriviaPiece::new(TriviaPieceKind::Whitespace, 1)],
                        [],
                    ),
                    n.consequent(),
                    JsSyntaxToken::new_detached(T!['}'], &token_text, leading, []),
                ),
            ))),
        )?;

        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the statements in a block" }.to_owned(),
            root,
        })
    }
}
