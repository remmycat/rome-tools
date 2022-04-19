use crate::{write, Arguments, FormatOptions};
use crate::{Buffer, Format, Formatter};
use rome_rowan::{
    Language, SyntaxToken, SyntaxTokenText, SyntaxTriviaPieceComments, TextLen, TextRange, TextSize,
};
use std::borrow::Cow;
use std::cell::Cell;
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;

/// Language agnostic IR for formatting source code.
///
/// Use the helper functions like [crate::space_token], [crate::soft_line_break] etc. defined in this file to create elements.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FormatElement {
    /// A space token, see [crate::space_token] for documentation.
    Space,

    /// A new line, see [crate::soft_line_break], [crate::hard_line_break], and [crate::soft_line_break_or_space] for documentation.
    Line(LineMode),

    /// A token that should be printed as is, see [token] for documentation and examples.
    Token(Token),

    /// A token that tracks tokens/nodes that are printed using [`format_verbatim`](crate::Formatter::format_verbatim) API
    VerbatimStart(VerbatimKind),
    VerbatimEnd,

    /// Indents the content one level deeper, see [crate::indent] for documentation and examples.
    IndentStart,
    IndentEnd,

    /// Creates a logical group where its content is either consistently printed:
    /// * on a single line: Omitting `LineMode::Soft` line breaks and printing spaces for `LineMode::SoftOrSpace`
    /// * on multiple lines: Printing all line breaks
    ///
    /// See [crate::group_elements] for documentation and examples.
    GroupStart,
    GroupEnd,

    /// See [crate::hard_group_elements] for documentation and examples.
    HardGroupStart,
    HardGroupEnd,

    /// Allows to specify content that gets printed depending on whatever the enclosing group
    /// is printed on a single line or multiple lines. See [crate::if_group_breaks] for examples.
    ConditionalGroupStart(GroupPrintMode),
    ConditionalGroupEnd,

    /// Concatenates multiple elements together with spaces or line breaks to fill the print width. See [fill_elements].
    FillStart,
    FillEnd,

    /// Delay the printing of its content until the next line break
    LineSuffixStart,
    LineSuffixEnd,

    /// Special semantic element letting the printer and formatter know this is
    /// a trivia content, and it should only have a limited influence on the
    /// formatting (for instance line breaks contained within will not cause
    /// the parent group to break if this element is at the start of it)
    CommentStart,
    CommentEnd,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineMode {
    /// See [soft_line_break_or_space] for documentation.
    SoftOrSpace,
    /// See [soft_line_break] for documentation.
    Soft,
    /// See [hard_line_break] for documentation.
    Hard,
    /// See [empty_line] for documentation.
    Empty,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GroupPrintMode {
    Flat,
    Multiline,
}

/// See [token] for documentation
#[derive(Eq, Clone)]
pub enum Token {
    /// Token constructed by the formatter from a static string
    Static { text: &'static str },
    /// Token constructed from the input source as a dynamics
    /// string and a range of the input source
    Dynamic {
        // There's no need for the text to be mutable, using `Box<str>` safes 8 bytes over `String`.
        text: Box<str>,
        // The position of the dynamic token in the unformatted source code
        source_position: TextSize,
    },
    // A token that is taken 1:1 from the source code
    SyntaxTokenSlice {
        /// The start position of the token in the unformatted source code
        source_position: TextSize,
        /// The token text
        slice: SyntaxTokenText,
    },
}

impl Debug for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // This does not use debug_tuple so the tokens are
        // written on a single line even when pretty-printing
        match self {
            Token::Static { text } => std::write!(fmt, "StaticToken({:?})", text),
            Token::Dynamic { text, .. } => std::write!(fmt, "DynamicToken({:?})", text),
            Token::SyntaxTokenSlice {
                slice: token_text, ..
            } => {
                std::write!(fmt, "SyntaxTokenSlice({:?})", token_text)
            }
        }
    }
}

impl Token {
    /// Create a token from a static string
    const fn new_static(text: &'static str) -> Self {
        Self::Static { text }
    }

    /// Create a token from a dynamic string and a range of the input source
    pub fn new_dynamic(text: String, position: TextSize) -> Self {
        Self::assert_no_newlines(&text);
        Self::Dynamic {
            text: text.into_boxed_str(),
            source_position: position,
        }
    }

    /// Creates a token from a [Cow] that is a sub-slice over the text of a token.
    ///
    /// The `start` is the absolute start of the token in the source text.
    ///
    /// ## Returns
    /// * [Token::Dynamic] if `text` is a [Cow::Owned] (text doesn't match syntax token text)
    /// * [Token::SyntaxTokenSlice] if `text` is borrowed. Avoids allocating a new string.
    pub fn from_syntax_token_cow_slice<L: Language>(
        text: Cow<str>,
        token: &SyntaxToken<L>,
        start: TextSize,
    ) -> Self {
        Self::assert_no_newlines(&text);

        match text {
            Cow::Owned(text) => Self::new_dynamic(text, start),
            Cow::Borrowed(text) => {
                let range = TextRange::at(start, text.text_len());
                debug_assert_eq!(
                    text,
                    &token.text()[range - token.text_range().start()],
                    "The borrowed string doesn't match the specified token substring"
                );
                Token::new_syntax_token_slice(token, range)
            }
        }
    }

    /// Creates a new [Token] with a text backed by the string of [SyntaxToken]
    pub fn new_syntax_token_slice<L: Language>(token: &SyntaxToken<L>, range: TextRange) -> Self {
        let relative_range = range - token.text_range().start();
        let slice = token.token_text().slice(relative_range);

        Self::assert_no_newlines(&slice);

        Self::SyntaxTokenSlice {
            slice,
            source_position: range.start(),
        }
    }

    fn assert_no_newlines(text: &str) {
        debug_assert!(!text.contains('\r'), "The content '{}' contains an unsupported '\\r' line terminator character but string tokens must only use line feeds '\\n' as line separator. Use '\\n' instead of '\\r' and '\\r\\n' to insert a line break in strings.", text);
    }

    /// Get the range of the input source covered by this token,
    /// or None if the token was synthesized by the formatter
    pub fn source_position(&self) -> Option<&TextSize> {
        match self {
            Token::Static { .. } => None,
            Token::Dynamic {
                source_position, ..
            } => Some(source_position),
            Token::SyntaxTokenSlice {
                source_position, ..
            } => Some(source_position),
        }
    }
}

// Token equality only compares the text content
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<L: Language> From<SyntaxToken<L>> for Token {
    fn from(token: SyntaxToken<L>) -> Self {
        Self::from(&token)
    }
}

impl<'a, L: Language> From<&'a SyntaxToken<L>> for Token {
    fn from(token: &'a SyntaxToken<L>) -> Self {
        let trimmed_range = token.text_trimmed_range();

        Self::new_syntax_token_slice(token, trimmed_range)
    }
}

impl<L: Language> From<SyntaxTriviaPieceComments<L>> for Token {
    fn from(trivia: SyntaxTriviaPieceComments<L>) -> Self {
        let range = trivia.text_range();
        Token::from_syntax_token_cow_slice(
            normalize_newlines(trivia.text().trim(), LINE_TERMINATORS),
            &trivia.as_piece().token(),
            range.start(),
        )
    }
}

impl Deref for Token {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Token::Static { text } => text,
            Token::Dynamic { text, .. } => text,
            Token::SyntaxTokenSlice {
                slice: token_text, ..
            } => token_text.deref(),
        }
    }
}

impl Format for Token {
    fn format(&self, formatter: &mut crate::Formatter) -> crate::Result<()> {
        formatter.write_element(FormatElement::Token(self.clone()))
    }
}

const LINE_SEPARATOR: char = '\u{2028}';
const PARAGRAPH_SEPARATOR: char = '\u{2029}';
pub const LINE_TERMINATORS: [char; 3] = ['\r', LINE_SEPARATOR, PARAGRAPH_SEPARATOR];

/// Replace the line terminators matching the provided list with "\n"
/// since its the only line break type supported by the printer
pub fn normalize_newlines<const N: usize>(text: &str, terminators: [char; N]) -> Cow<str> {
    let mut result = String::new();
    let mut last_end = 0;

    for (start, part) in text.match_indices(terminators) {
        result.push_str(&text[last_end..start]);
        result.push('\n');

        last_end = start + part.len();
        // If the current character is \r and the
        // next is \n, skip over the entire sequence
        if part == "\r" && text[last_end..].starts_with('\n') {
            last_end += 1;
        }
    }

    // If the result is empty no line terminators were matched,
    // return the entire input text without allocating a new String
    if result.is_empty() {
        Cow::Borrowed(text)
    } else {
        result.push_str(&text[last_end..text.len()]);
        Cow::Owned(result)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum VerbatimKind {
    Unknown,
    Suppressed,
    Verbatim {
        /// the length of the formatted node
        length: TextSize,
    },
}

pub const fn soft_line_break() -> Line {
    Line {
        mode: LineMode::Soft,
    }
}

pub const fn hard_line_break() -> Line {
    Line {
        mode: LineMode::Hard,
    }
}

pub const fn empty_line() -> Line {
    Line {
        mode: LineMode::Empty,
    }
}

pub const fn soft_line_break_or_space() -> Line {
    Line {
        mode: LineMode::SoftOrSpace,
    }
}

pub struct Line {
    mode: LineMode,
}

impl Format for Line {
    fn format(&self, formatter: &mut crate::Formatter) -> crate::Result<()> {
        formatter.write_element(FormatElement::Line(self.mode))
    }
}

pub const fn token(text: &'static str) -> Token {
    Token::new_static(text)
}

pub struct LineSuffix<'args> {
    args: Arguments<'args>,
}

impl Format for LineSuffix<'_> {
    fn format(&self, formatter: &mut crate::Formatter) -> crate::Result<()> {
        formatter.write_element(FormatElement::LineSuffixStart)?;
        formatter.write_fmt(&self.args)?;
        formatter.write_element(FormatElement::LineSuffixEnd)?;

        Ok(())
    }
}

pub const fn line_suffix(args: Arguments) -> LineSuffix {
    LineSuffix { args }
}

pub struct Comment<'args> {
    args: Arguments<'args>,
}

impl Format for Comment<'_> {
    fn format(&self, formatter: &mut crate::Formatter) -> crate::Result<()> {
        formatter.write_element(FormatElement::CommentStart)?;
        formatter.write_fmt(&self.args)?;
        formatter.write_element(FormatElement::CommentEnd)?;

        Ok(())
    }
}

pub const fn comment(args: Arguments) -> Comment {
    Comment { args }
}

pub struct Space;

impl Format for Space {
    fn format(&self, formatter: &mut crate::Formatter) -> crate::Result<()> {
        formatter.write_element(FormatElement::Space)
    }
}

pub const fn space_token() -> Space {
    Space
}

#[derive(Copy, Clone)]
pub struct Indent<'content> {
    content: &'content dyn Format,
    line_mode: Option<LineMode>,
}

pub fn indent(content: &dyn Format) -> Indent {
    Indent {
        content,
        line_mode: None,
    }
}

pub fn block_indent(content: &dyn Format) -> Indent {
    Indent {
        content,
        line_mode: Some(LineMode::Hard),
    }
}

pub fn soft_block_indent(content: &dyn Format) -> Indent {
    Indent {
        content,
        line_mode: Some(LineMode::Soft),
    }
}

pub fn soft_line_indent_or_space(content: &dyn Format) -> Indent {
    Indent {
        content,
        line_mode: Some(LineMode::SoftOrSpace),
    }
}

impl Format for Indent<'_> {
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        formatter.write_element(FormatElement::IndentStart)?;

        if let Some(line_mode) = self.line_mode {
            formatter.write_element(FormatElement::Line(line_mode))?;
        }

        self.content.format(formatter)?;

        if let Some(line_mode) = self.line_mode {
            formatter.write_element(FormatElement::Line(line_mode))?;
        }

        Ok(())
    }
}

pub fn group<F: Format>(content: F) -> Group<F> {
    Group { content }
}

pub struct Group<F: Format> {
    content: F,
}

impl<F: Format> Format for Group<F> {
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        let mut buffer = TriviaAdapter {
            inner: formatter,
            trailing: vec![],
            in_comment: false,
            passed_trivia: false,
        };

        write!(buffer, self.content)?;

        buffer.finish()
    }
}

struct TriviaAdapter<'fmt, 'buf> {
    passed_trivia: bool,
    in_comment: bool,
    // TODO: Use a small vec, don't push the comment start and end
    trailing: Vec<FormatElement>,
    inner: &'fmt mut Formatter<'buf>,
}

impl TriviaAdapter<'_, '_> {
    fn finish(&mut self) -> crate::Result<()> {
        self.inner.write_element(FormatElement::GroupEnd)?;

        for element in self.trailing.drain(..) {
            self.inner.write_element(element)?;
        }

        Ok(())
    }
}

impl Buffer for TriviaAdapter<'_, '_> {
    fn options(&self) -> &FormatOptions {
        self.inner.options()
    }

    fn write_element(&mut self, element: FormatElement) -> crate::Result<()> {
        if !self.passed_trivia && matches!(element, FormatElement::CommentStart) {
            self.in_comment = true;
        } else if self.in_comment && matches!(element, FormatElement::CommentEnd) {
            self.in_comment = false;
        } else if !self.passed_trivia && !self.in_comment {
            self.passed_trivia = true;
            self.inner.write_element(FormatElement::GroupStart)?;
        }

        if self.passed_trivia {
            if matches!(element, FormatElement::CommentStart) {
                assert!(!self.in_comment, "Nested comments not supported.");
                self.in_comment = true;
                self.trailing.push(element);
                return Ok(());
            }

            // Comments can not be nested
            if self.in_comment && matches!(element, FormatElement::CommentEnd) {
                self.in_comment = false;
                self.trailing.push(element);
                return Ok(());
            }

            if self.in_comment {
                self.trailing.push(element);
                return Ok(());
            }

            // Wasn't the last comment after all
            if !self.in_comment && !self.trailing.is_empty() {
                for element in self.trailing.drain(..) {
                    self.inner.write_element(element)?;
                }
            }
        }

        self.inner.write_element(element)?;

        Ok(())
    }
}

pub struct Join<'with, F, I>
where
    F: Format,
    I: Iterator<Item = F>,
{
    items: Cell<Option<I>>,
    with: Option<&'with dyn Format>,
}

pub fn join<F, I>(items: I) -> Join<'static, F, I::IntoIter>
where
    F: Format,
    I: IntoIterator<Item = F>,
{
    Join {
        items: Cell::new(Some(items.into_iter())),
        with: None,
    }
}

pub fn join_with<'w, F, I>(items: I, with: &'w dyn Format) -> Join<'w, F, I::IntoIter>
where
    F: Format,
    I: IntoIterator<Item = F>,
{
    Join {
        items: Cell::new(Some(items.into_iter())),
        with: Some(with),
    }
}

impl<F, I> Format for Join<'_, F, I>
where
    F: Format,
    I: Iterator<Item = F> + Clone,
{
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        if let Some(items) = self.items.take() {
            if let Some(with) = self.with {
                formatter
                    .join_with(with)
                    .entries(items.into_iter())
                    .finish()
            } else {
                formatter.join().entries(items.into_iter()).finish()
            }
        } else {
            panic!("Iterator has already been consumed");
        }
    }
}

pub struct Fill<'with, F, I>
where
    F: Format,
    I: Iterator<Item = F>,
{
    items: Cell<Option<I>>,
    with: Option<&'with dyn Format>,
}

pub fn fill<F, I>(items: I) -> Fill<'static, F, I::IntoIter>
where
    F: Format,
    I: IntoIterator<Item = F>,
{
    Fill {
        items: Cell::new(Some(items.into_iter())),
        with: None,
    }
}

pub fn fill_with<'w, F, I>(items: I, with: &'w dyn Format) -> Fill<'w, F, I::IntoIter>
where
    F: Format,
    I: IntoIterator<Item = F>,
{
    Fill {
        items: Cell::new(Some(items.into_iter())),
        with: Some(with),
    }
}

impl<F, I> Format for Fill<'_, F, I>
where
    F: Format,
    I: Iterator<Item = F> + Clone,
{
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        if let Some(items) = self.items.take() {
            if let Some(with) = self.with {
                formatter
                    .fill_with(with)
                    .entries(items.into_iter())
                    .finish()
            } else {
                formatter.fill().entries(items.into_iter()).finish()
            }
        } else {
            panic!("Iterator has already been consumed");
        }
    }
}
