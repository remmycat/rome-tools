use crate::prelude::*;

use rome_formatter::{normalize_newlines, ConcatBuilder, FormatResult, GroupId, LINE_TERMINATORS};
use rome_js_syntax::{JsLanguage, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};

use crate::{AsFormat, JsFormatOptions};
use rome_rowan::{AstNode, AstNodeList, AstSeparatedList, Language, SyntaxKind, SyntaxTriviaPiece};

use rome_rowan::syntax::SyntaxTrivia;
use std::iter::once;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrailingSeparator {
    Allowed,
    Disallowed,
    Mandatory,
}

impl TrailingSeparator {
    pub fn is_allowed(&self) -> bool {
        matches!(self, TrailingSeparator::Allowed)
    }
    pub fn is_mandatory(&self) -> bool {
        matches!(self, TrailingSeparator::Mandatory)
    }
}

impl Default for TrailingSeparator {
    fn default() -> Self {
        TrailingSeparator::Allowed
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct FormatSeparatedOptions {
    trailing_separator: TrailingSeparator,
    group_id: Option<GroupId>,
}

impl FormatSeparatedOptions {
    pub fn with_trailing_separator(mut self, separator: TrailingSeparator) -> Self {
        self.trailing_separator = separator;
        self
    }

    pub fn with_group_id(mut self, group_id: Option<GroupId>) -> Self {
        self.group_id = group_id;
        self
    }
}

/// Determines if the whitespace separating comment trivias
/// from their associated tokens should be printed or trimmed
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum TriviaPrintMode {
    Full,
    Trim,
}

/// "Formats" a node according to its original formatting in the source text. Being able to format
/// a node "as is" is useful if a node contains syntax errors. Formatting a node with syntax errors
/// has the risk that Rome misinterprets the structure of the code and formatting it could
/// "mess up" the developers, yet incomplete, work or accidentally introduce new syntax errors.
///
/// You may be inclined to call `node.text` directly. However, using `text` doesn't track the nodes
/// nor its children source mapping information, resulting in incorrect source maps for this subtree.
///
/// These nodes and tokens get tracked as [FormatElement::Verbatim], useful to understand
/// if these nodes still need to have their own implementation.
pub fn verbatim_node(node: &JsSyntaxNode) -> FormatVerbatimNode {
    FormatVerbatimNode { node }
}

#[derive(Debug, Clone)]
pub struct FormatVerbatimNode<'node> {
    node: &'node JsSyntaxNode,
}

impl Format for FormatVerbatimNode<'_> {
    type Options = JsFormatOptions;

    fn format(&self, formatter: &Formatter<JsFormatOptions>) -> FormatResult<FormatElement> {
        let verbatim = format_verbatim_node_or_token(self.node, formatter);
        Ok(FormatElement::Verbatim(Verbatim::new_verbatim(
            verbatim,
            self.node.text_range().len(),
        )))
    }
}

/// Formats unknown nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [FormatElement::Verbatim]. They are just printed as they are.
pub fn unknown_node(node: &JsSyntaxNode) -> FormatUnknownNode {
    FormatUnknownNode { node }
}

#[derive(Debug, Clone)]
pub struct FormatUnknownNode<'node> {
    node: &'node JsSyntaxNode,
}

impl Format for FormatUnknownNode<'_> {
    type Options = JsFormatOptions;

    fn format(&self, formatter: &Formatter<JsFormatOptions>) -> FormatResult<FormatElement> {
        Ok(FormatElement::Verbatim(Verbatim::new_unknown(
            format_verbatim_node_or_token(self.node, formatter),
        )))
    }
}

/// Format a node having formatter suppression comment applied to it
pub fn suppressed_node(node: &JsSyntaxNode) -> FormatSuppressedNode {
    FormatSuppressedNode { node }
}

#[derive(Debug, Clone)]
pub struct FormatSuppressedNode<'node> {
    node: &'node JsSyntaxNode,
}

impl Format for FormatSuppressedNode<'_> {
    type Options = JsFormatOptions;

    fn format(&self, formatter: &Formatter<JsFormatOptions>) -> FormatResult<FormatElement> {
        formatted![
            formatter,
            [
                // Insert a force a line break to ensure the suppression comment is on its own line
                // and correctly registers as a leading trivia on the opening token of this node
                hard_line_break(),
                FormatElement::Verbatim(Verbatim::new_suppressed(format_verbatim_node_or_token(
                    self.node, formatter
                ))),
            ]
        ]
    }
}

fn format_verbatim_node_or_token(
    node: &JsSyntaxNode,
    formatter: &Formatter<JsFormatOptions>,
) -> FormatElement {
    for token in node.descendants_tokens() {
        formatter.track_token(&token);
    }

    fn skip_whitespace<L: Language>(piece: &SyntaxTriviaPiece<L>) -> bool {
        piece.is_newline() || piece.is_whitespace()
    }

    fn trivia_token<L: Language>(piece: SyntaxTriviaPiece<L>) -> Token {
        Token::from_syntax_token_cow_slice(
            normalize_newlines(piece.text(), LINE_TERMINATORS),
            &piece.token(),
            piece.text_range().start(),
        )
    }

    let leading_trivia = node
        .first_leading_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces())
        .skip_while(skip_whitespace)
        .map(trivia_token);

    let content = Token::new_dynamic(
        normalize_newlines(&node.text_trimmed().to_string(), LINE_TERMINATORS).into_owned(),
        node.text_trimmed_range().start(),
    );

    // Clippy false positive: SkipWhile does not implement DoubleEndedIterator
    #[allow(clippy::needless_collect)]
    let trailing_trivia = node
        .last_trailing_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces().rev())
        .skip_while(skip_whitespace)
        .map(trivia_token)
        .collect::<Vec<_>>();

    concat_elements(
        leading_trivia
            .chain(once(content))
            .chain(trailing_trivia.into_iter().rev())
            .map(FormatElement::from),
    )
}

pub(crate) fn format_trailing_trivia(token: &JsSyntaxToken) -> FormatElement {
    let pieces = token.trailing_trivia().pieces();
    let mut elements = ConcatBuilder::new();
    let mut has_inline_comment = false;

    for comment in pieces.filter_map(|piece| piece.as_comments()) {
        let is_single_line = comment.text().trim_start().starts_with("//");

        let comment = FormatElement::from(Token::from(comment));

        let content = if !is_single_line {
            let first_inline_comment = !has_inline_comment;
            has_inline_comment = true;

            if first_inline_comment
                && matches!(
                    token.kind(),
                    JsSyntaxKind::L_PAREN
                        | JsSyntaxKind::L_CURLY
                        | JsSyntaxKind::L_BRACK
                        | JsSyntaxKind::DOLLAR_CURLY
                )
            {
                comment
            } else {
                format_elements![space_token(), comment]
            }
        } else {
            format_elements![
                line_suffix(format_elements![space_token(), comment]),
                expand_parent()
            ]
        };

        elements.entry(crate::comment(content));
    }

    if has_inline_comment
        && !matches!(
            token.next_token().map(|t| t.kind()),
            Some(
                JsSyntaxKind::R_BRACK
                    | JsSyntaxKind::R_PAREN
                    | JsSyntaxKind::R_CURLY
                    | JsSyntaxKind::SEMICOLON
                    | JsSyntaxKind::COMMA
            ) | None
        )
    {
        // TODO test if there's no leading trivia?
        elements.entry(space_token());
    }

    // TODO would it make more sense to format all trivia that appear between the current and the previous token
    // at once?

    elements.finish()
}

pub(super) fn format_leading_trivia(
    token: &JsSyntaxToken,
    mut trim_mode: TriviaPrintMode,
) -> FormatResult<FormatElement> {
    // Checks whether the previous token has any trailing newline
    let has_trailing_newline = token
        .prev_token()
        .and_then(|token| token.trailing_trivia().last())
        .map_or(false, |trivia| trivia.is_newline());

    let mut line_count = 0;
    let mut elements = Vec::new();

    // Get the index of the first comment in the trivia pieces list, and
    // checks whether this token has any leading newline the comment
    let mut has_leading_newline = false;
    let mut first_comment = 0;

    let mut pieces = token.leading_trivia().pieces().enumerate().peekable();

    // Peek at the next trivia piece, stopping if it is a comment and
    // advancing the iterator if it's not
    while let Some((index, piece)) = pieces.peek() {
        if piece.is_comments() {
            // Save the index and break the loop
            // without consuming the comment piece
            first_comment = *index;
            break;
        }

        if piece.is_skipped() {
            return Err(FormatError::SyntaxError);
        }

        if piece.is_newline() {
            has_leading_newline = true;
        }

        pieces.next();
    }

    // If any newline was found between the previous token and the first comment,
    // it will be prepended with a line break instead of a space
    let prepend_newline = has_trailing_newline || has_leading_newline;

    // This consumes the previously created iterator from the last trivia piece
    // towards the first (that was not consumed by the previous loop)
    for (index, piece) in pieces.rev() {
        if let Some(comment) = piece.as_comments() {
            let is_single_line = comment.text().starts_with("//");

            let comment = FormatElement::from(Token::from(comment));

            let element_before_comment = if prepend_newline && index == first_comment {
                hard_line_break()
            } else {
                space_token()
            };

            let element_after_comment = if is_single_line {
                match line_count {
                    0 | 1 => hard_line_break(),
                    _ => empty_line(),
                }
            } else {
                match line_count {
                    0 => space_token(),
                    1 => hard_line_break(),
                    _ => empty_line(),
                }
            };

            elements.push(crate::comment(format_elements![
                element_before_comment,
                comment,
                element_after_comment,
            ]));

            line_count = 0;
            trim_mode = TriviaPrintMode::Full;
        } else if piece.is_newline() && trim_mode == TriviaPrintMode::Full {
            line_count += 1;
        } else if piece.is_skipped() {
            return Err(FormatError::SyntaxError);
        }
    }

    Ok(concat_elements(elements.into_iter().rev()))
}

/// JS specific formatter extensions
pub(crate) trait JsFormatter {
    fn as_formatter(&self) -> &Formatter<JsFormatOptions>;

    #[must_use]
    fn delimited<'a, 'fmt>(
        &'fmt self,
        open_token: &'a JsSyntaxToken,
        content: FormatElement,
        close_token: &'a JsSyntaxToken,
    ) -> FormatDelimited<'a, 'fmt> {
        FormatDelimited::new(open_token, content, close_token, self.as_formatter())
    }

    /// Print out a `token` from the original source with a different `content`.
    ///
    /// This will print the trivias that belong to `token` to `content`;
    /// `token` is then marked as consumed by the formatter.
    fn format_replaced(
        &self,
        current_token: &JsSyntaxToken,
        content_to_replace_with: FormatElement,
    ) -> FormatResult<FormatElement> {
        // TODO move to Formatter
        let formatter = self.as_formatter();

        // TODO: Should this pass in the new token that it belongs to now after this content has been replaced
        // to make trim work correctly?
        // But issue remains: If inserting a new paren, format leading will still insert a space, right?
        // I guess currently not an issue because only formatting trailing?
        // Example let a = 3 +2  * 3  /* test */ -> let a = 3 + (2 * 3 /* test */ );
        // let a = 3 + (2 * 3 /* test */ ); -> let a = 3 + (2 * 3 /* test */);
        // Two challenges: Tokens that get removed, tokens that get inserted. It's then unclear what the prevuous token was.

        // The idea is that the formatter writes all comments up to the current token.
        // Problem, needs a way to remember the last token. Main issue here, `token` doesn't go through the formatter
        // AND replaced_token would require "rewritting? -> No, that's fine).
        // Main problem, how to know what the previous token was. Only solution that I see at the moment
        // is to somehow store the kind of the previous token. Together with `tracked_tokens`. The issue now becomes
        // that it's quiet common to format the tokens out of order. But I guess we could just call into that?
        // Except that it doesn't work for inserted tokens, except if we keep an internal map that stores the "respines".
        // formatter.inserted_token("(", inner.first_token()); This can force the formatting of the inner.first_token() preceeding
        // and then set an internal map that "(" preceeds inner.first_token.
        // Ideally this could be solved by simply getting formatter.elements.last() when we have a single mutable buffer.
        // This requires two things:
        // * Extend the formatter with mutual state that handles 'inserted_tokens'
        // * Rewrite format_leading / format_trailing to a single format_preceding_comments(token) that
        //   formats all comments that preceed token (token.prev_token().trailing + tokne.leading_trivia()).
        let result = format_elements![
            format_leading_trivia(current_token, TriviaPrintMode::Full)?,
            content_to_replace_with,
            format_trailing_trivia(current_token),
        ];

        formatter.track_token(current_token);

        Ok(result)
    }

    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated<L, F>(
        &self,
        list: &L,
        separator_factory: F,
    ) -> FormatResult<std::vec::IntoIter<FormatElement>>
    where
        L: AstSeparatedList<Language = JsLanguage>,
        for<'a> L::Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
        F: Fn() -> FormatElement,
    {
        self.format_separated_with_options(
            list,
            separator_factory,
            FormatSeparatedOptions::default(),
        )
    }

    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated_with_options<L, F>(
        &self,
        list: &L,
        separator_factory: F,
        options: FormatSeparatedOptions,
    ) -> FormatResult<std::vec::IntoIter<FormatElement>>
    where
        L: AstSeparatedList<Language = JsLanguage>,
        for<'a> L::Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
        F: Fn() -> FormatElement,
    {
        let mut result = Vec::with_capacity(list.len());
        let last_index = list.len().saturating_sub(1);
        let formatter = self.as_formatter();

        let trailing_separator_factory = || {
            if let Some(group_id) = options.group_id {
                if_group_with_id_breaks(separator_factory(), group_id)
            } else {
                if_group_breaks(separator_factory())
            }
        };

        let trailing_separator = options.trailing_separator;

        for (index, element) in list.elements().enumerate() {
            let node = formatted![formatter, [element.node()?.format()]]?;

            // Reuse the existing trailing separator or create it if it wasn't in the
            // input source. Only print the last trailing token if the outer group breaks
            let separator = if let Some(separator) = element.trailing_separator()? {
                if index == last_index {
                    if trailing_separator.is_allowed() {
                        // Use format_replaced instead of wrapping the result of format_token
                        // in order to remove only the token itself when the group doesn't break
                        // but still print its associated trivias unconditionally
                        self.format_replaced(separator, trailing_separator_factory())?
                    } else if trailing_separator.is_mandatory() {
                        formatted![formatter, [separator.format()]]?
                    } else {
                        empty_element()
                    }
                } else {
                    formatted![formatter, [separator.format()]]?
                }
            } else if index == last_index {
                if trailing_separator.is_allowed() {
                    trailing_separator_factory()
                } else if trailing_separator.is_mandatory() {
                    separator_factory()
                } else {
                    empty_element()
                }
            } else {
                separator_factory()
            };

            result.push(format_elements![group_elements(node), separator]);
        }

        Ok(result.into_iter())
    }

    /// It formats a list of nodes that are not separated. It's an ad-hoc function to
    /// format lists that implement [rome_js_syntax::AstNodeList].
    ///
    /// The elements of the list are joined together using [join_elements_hard_line], which will
    /// end up separated by hard lines or empty lines.
    ///
    /// If the formatter fails to format an element, said element gets printed verbatim.
    fn format_list<List, Node>(&self, list: &List) -> FormatElement
    where
        List: AstNodeList<Language = JsLanguage, Node = Node>,
        for<'a> Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
    {
        let formatter = self.as_formatter();
        let formatted_list = list.iter().map(|module_item| {
            let snapshot = formatter.snapshot();
            let format = module_item.format();

            let elem = match formatted![formatter, [format]] {
                Ok(result) => result,
                Err(_) => {
                    formatter.restore(snapshot);

                    // Lists that yield errors are formatted as they were unknown nodes.
                    // Doing so, the formatter formats the nodes/tokens as is.
                    // SAFETY: `FormatUnknownNode` always returns Ok
                    unknown_node(module_item.syntax())
                        .format(formatter)
                        .unwrap()
                }
            };

            (module_item.syntax().clone(), elem)
        });
        join_elements_hard_line(formatted_list)
    }
}

impl JsFormatter for Formatter<JsFormatOptions> {
    fn as_formatter(&self) -> &Formatter<JsFormatOptions> {
        self
    }
}

/// Formats a group delimited by an opening and closing token,
/// such as a function body delimited by '{' and '}' tokens
///
/// Calling this method is required to correctly handle the comments attached
/// to the opening and closing tokens and insert them inside the group block
pub struct FormatDelimited<'a, 'fmt> {
    open_token: &'a JsSyntaxToken,
    content: FormatElement,
    close_token: &'a JsSyntaxToken,
    mode: DelimitedMode,
    formatter: &'fmt Formatter<JsFormatOptions>,
}

impl<'a, 'fmt> FormatDelimited<'a, 'fmt> {
    fn new(
        open_token: &'a JsSyntaxToken,
        content: FormatElement,
        close_token: &'a JsSyntaxToken,
        formatter: &'fmt Formatter<JsFormatOptions>,
    ) -> Self {
        Self {
            open_token,
            content,
            close_token,
            mode: DelimitedMode::SoftBlockIndent(None),
            formatter,
        }
    }

    fn with_mode(mut self, mode: DelimitedMode) -> Self {
        self.mode = mode;
        self
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in a [block_indent] group
    pub fn block_indent(self) -> Self {
        self.with_mode(DelimitedMode::BlockIndent)
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in a [soft_block_indent] group
    pub fn soft_block_indent(self) -> Self {
        self.with_mode(DelimitedMode::SoftBlockIndent(None))
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in an [indent] group with [soft_line_break_or_space] tokens at the
    /// start and end
    pub fn soft_block_spaces(self) -> Self {
        self.with_mode(DelimitedMode::SoftBlockSpaces(None))
    }

    pub fn soft_block_indent_with_group_id(self, group_id: Option<GroupId>) -> Self {
        self.with_mode(DelimitedMode::SoftBlockIndent(group_id))
    }

    pub fn soft_block_spaces_with_group_id(self, group_id: Option<GroupId>) -> Self {
        self.with_mode(DelimitedMode::SoftBlockSpaces(group_id))
    }

    pub fn finish(self) -> FormatResult<FormatElement> {
        let FormatDelimited {
            formatter,
            open_token,
            close_token,
            content,
            mode,
        } = self;

        formatter.track_token(open_token);
        formatter.track_token(close_token);

        let open_token_trailing_trivia = format_trailing_trivia(open_token);
        let close_token_leading_trivia = format_leading_trivia(close_token, TriviaPrintMode::Trim)?;

        let close_token_leading_trivia = if !close_token_leading_trivia.is_empty() {
            formatted![
                formatter,
                [soft_line_break_or_space(), close_token_leading_trivia]
            ]?
        } else {
            empty_element()
        };

        let formatted_content = match mode {
            DelimitedMode::BlockIndent => block_indent(formatted![
                formatter,
                [
                    open_token_trailing_trivia,
                    content,
                    close_token_leading_trivia
                ]
            ]?),
            DelimitedMode::SoftBlockIndent(_) => soft_block_indent(formatted![
                formatter,
                [
                    open_token_trailing_trivia,
                    content,
                    close_token_leading_trivia
                ]
            ]?),
            DelimitedMode::SoftBlockSpaces(_) => {
                if open_token_trailing_trivia.is_empty()
                    && content.is_empty()
                    && close_token_leading_trivia.is_empty()
                {
                    empty_element()
                } else {
                    formatted![
                        formatter,
                        [
                            indent(formatted![
                                formatter,
                                [
                                    soft_line_break_or_space(),
                                    open_token_trailing_trivia,
                                    content,
                                    close_token_leading_trivia,
                                ]
                            ]?),
                            soft_line_break_or_space(),
                        ]
                    ]?
                }
            }
        };

        let delimited = format_elements![
            Token::from(open_token),
            formatted_content,
            Token::from(close_token),
        ];

        let grouped = match mode {
            // Group is useless, the block indent would expand it right anyway
            DelimitedMode::BlockIndent => delimited,
            DelimitedMode::SoftBlockIndent(group_id) | DelimitedMode::SoftBlockSpaces(group_id) => {
                match group_id {
                    None => group_elements(delimited),
                    Some(group_id) => group_elements_with_options(
                        delimited,
                        GroupElementsOptions {
                            group_id: Some(group_id),
                        },
                    ),
                }
            }
        };

        formatted![
            formatter,
            [
                format_leading_trivia(open_token, TriviaPrintMode::Full),
                grouped,
                format_trailing_trivia(close_token),
            ]
        ]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DelimitedMode {
    BlockIndent,
    SoftBlockIndent(Option<GroupId>),
    SoftBlockSpaces(Option<GroupId>),
}
