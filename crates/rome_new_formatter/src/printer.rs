// use crate::FormatElement::ConditionalGroupEnd;
// use crate::{
//     hard_line_break, space_token, Format, FormatElement, FormatOptions, GroupPrintMode,
//     IndentStyle, LineMode, LineWidth, Printed, SourceMarker, VerbatimKind,
// };
// use rome_rowan::{TextRange, TextSize};
// use std::iter::once;
//
// /// Options that affect how the [Printer] prints the format tokens
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct PrinterOptions {
//     /// Width of a single tab character (does it equal 2, 4, ... spaces?)
//     pub tab_width: u8,
//
//     /// What's the max width of a line. Defaults to 80
//     pub print_width: LineWidth,
//
//     /// The type of line ending to apply to the printed input
//     pub line_ending: LineEnding,
//
//     /// The never ending question whatever to use spaces or tabs, and if spaces, how many spaces
//     /// to indent code.
//     ///
//     /// * Tab: Value is '\t'
//     /// * Spaces: String containing the number of spaces per indention level, e.g. "  " for using two spaces
//     pub indent_string: String,
// }
//
// impl From<FormatOptions> for PrinterOptions {
//     fn from(options: FormatOptions) -> Self {
//         let tab_width = 2;
//
//         let indent_string = match options.indent_style {
//             IndentStyle::Tab => String::from("\t"),
//             IndentStyle::Space(width) => " ".repeat(width as usize),
//         };
//
//         PrinterOptions {
//             indent_string,
//             tab_width,
//             print_width: options.line_width,
//             ..PrinterOptions::default()
//         }
//     }
// }
//
// #[allow(dead_code)]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum LineEnding {
//     ///  Line Feed only (\n), common on Linux and macOS as well as inside git repos
//     LineFeed,
//
//     /// Carriage Return + Line Feed characters (\r\n), common on Windows
//     CarriageReturnLineFeed,
//
//     /// Carriage Return character only (\r), used very rarely
//     CarriageReturn,
// }
//
// impl LineEnding {
//     #[inline]
//     pub const fn as_str(&self) -> &'static str {
//         match self {
//             LineEnding::LineFeed => "\n",
//             LineEnding::CarriageReturnLineFeed => "\r\n",
//             LineEnding::CarriageReturn => "\r",
//         }
//     }
// }
//
// impl Default for PrinterOptions {
//     fn default() -> Self {
//         PrinterOptions {
//             tab_width: 2,
//             print_width: LineWidth::default(),
//             indent_string: String::from("\t"),
//             line_ending: LineEnding::LineFeed,
//         }
//     }
// }
//
// /// Error returned if printing an item as a flat string fails because it either contains
// /// explicit line breaks or would otherwise exceed the specified line width.
// struct LineBreakRequiredError;
//
// /// Prints the format elements into a string
// #[derive(Debug, Default)]
// pub struct Printer {
//     options: PrinterOptions,
//     state: PrinterState,
// }
//
// impl Printer {
//     pub fn new<T: Into<PrinterOptions>>(options: T) -> Self {
//         Self {
//             options: options.into(),
//             state: PrinterState::default(),
//         }
//     }
//
//     /// Prints the passed in element as well as all its content
//     pub fn print(self, elements: Vec<FormatElement>) -> Printed {
//         self.print_with_indent(elements, 0)
//     }
//
//     /// Prints the passed in element as well as all its content,
//     /// starting at the specified indentation level
//     pub fn print_with_indent(mut self, elements: Vec<FormatElement>, indent: u16) -> Printed {
//         // TODO set indent
//
//         let mut queue = ElementQueue::new(elements);
//
//         while let Some(element) = queue.next() {
//             self.print_element(element, &mut queue);
//
//             if queue.is_empty() && !self.state.line_suffixes.is_empty() {
//                 queue.extend(self.state.line_suffixes.drain(..).map(|element| element));
//             }
//         }
//
//         Printed::new(
//             self.state.buffer,
//             self.state.source_markers,
//             self.state.verbatim_markers,
//         )
//     }
//
//     /// Prints a single element and push the following elements to queue
//     fn print_element(&mut self, element: &FormatElement, queue: &mut ElementQueue) {
//         match element {
//             FormatElement::Space => {
//                 if self.state.line_width > 0 {
//                     self.state.pending_space = true;
//                 }
//             }
//             FormatElement::Token(token) => {
//                 // Print pending indention
//                 if self.state.pending_indent > 0 {
//                     self.print_str(
//                         self.options
//                             .indent_string
//                             .repeat(self.state.pending_indent as usize)
//                             .as_str(),
//                     );
//                     self.state.pending_indent = 0;
//                 }
//
//                 // Print pending spaces
//                 if self.state.pending_space {
//                     self.print_str(" ");
//                     self.state.pending_space = false;
//                 }
//
//                 if let Some(source) = token.source_position() {
//                     self.state.source_markers.push(SourceMarker {
//                         source: *source,
//                         dest: TextSize::from(self.state.buffer.len() as u32),
//                     });
//                 }
//
//                 self.print_str(token);
//             }
//
//             FormatElement::HardGroupStart => {
//                 self.state.stack.push(
//                     Context::HardGroup,
//                     self.state.stack.top_args().with_hard_group(true),
//                 );
//             }
//             FormatElement::HardGroupEnd => {
//                 self.state.stack.pop(Context::HardGroup);
//             }
//
//             FormatElement::GroupStart => {
//                 self.state.stack.push(
//                     Context::Group,
//                     self.state.stack.top_args().with_hard_group(false),
//                 );
//
//                 let snapshot = self.state.snapshot();
//
//                 if self.try_print_flat(queue).is_err() {
//                     // Flat printing didn't work, print with line breaks
//                     self.state.restore(snapshot);
//                 }
//             }
//             FormatElement::GroupEnd => {
//                 self.state.stack.pop(Context::Group);
//             }
//             //
//             //     FormatElement::Fill(list) => {
//             //         self.print_fill(queue, list, args);
//             //     }
//             //
//             //
//             FormatElement::IndentStart => {
//                 self.state.stack.push(
//                     Context::Indent,
//                     self.state.stack.top_args().with_incremented_indent(),
//                 );
//             }
//             FormatElement::IndentEnd => {
//                 self.state.stack.pop(Context::Indent);
//             }
//             //     FormatElement::ConditionalGroupContent(ConditionalGroupContent { mode, content }) => {
//             //         if args.hard_group == matches!(mode, GroupPrintMode::Flat) {
//             //             queue.enqueue(PrintElementCall::new(content, args));
//             //         }
//             //     }
//             //
//             //     FormatElement::Line(line) => {
//             //         if args.hard_group && matches!(line.mode, LineMode::Soft | LineMode::SoftOrSpace) {
//             //             self.state.pending_space |= line.mode == LineMode::SoftOrSpace;
//             //         } else if !self.state.line_suffixes.is_empty() {
//             //             // If the indentation level has changed since these line suffixes were queued,
//             //             // insert a line break before to push the comments into the new indent block
//             //             // SAFETY: Indexing into line_suffixes is guarded by the above call to is_empty
//             //             let has_line_break = self.state.line_suffixes[0].args.indent < args.indent;
//             //
//             //             // Print this line break element again once all the line suffixes have been flushed
//             //             let call_self = PrintElementCall::new(element, args.clone());
//             //
//             //             let line_break = if has_line_break {
//             //                 // Duplicate this line break element before the line
//             //                 // suffixes if a line break is required
//             //                 Some(call_self.clone())
//             //             } else {
//             //                 None
//             //             };
//             //
//             //             queue.extend(
//             //                 line_break
//             //                     .into_iter()
//             //                     .chain(self.state.line_suffixes.drain(..).map(move |mut call| {
//             //                         // Overwrite the arguments for the PrintElementCalls in the queue with the current arguments
//             //                         call.args = args.clone();
//             //                         call
//             //                     }))
//             //                     .chain(once(call_self)),
//             //             );
//             //         } else {
//             //             // Only print a newline if the current line isn't already empty
//             //             if self.state.line_width > 0 {
//             //                 self.print_str("\n");
//             //             }
//             //
//             //             // Print a second line break if this is an empty line
//             //             if line.mode == LineMode::Empty && !self.state.has_empty_line {
//             //                 self.print_str("\n");
//             //                 self.state.has_empty_line = true;
//             //             }
//             //
//             //             self.state.pending_space = false;
//             //             self.state.pending_indent = args.indent;
//             //         }
//             //     }
//             //
//             //     FormatElement::LineSuffix(suffix) => {
//             //         self.state
//             //             .line_suffixes
//             //             .push(PrintElementCall::new(&**suffix, args));
//             //     }
//             //     FormatElement::Comment(content) => {
//             //         queue.enqueue(PrintElementCall::new(content.as_ref(), args));
//             //     }
//             //
//             FormatElement::VerbatimStart(kind) => {
//                 if let VerbatimKind::Verbatim { length } = &kind {
//                     self.state.verbatim_markers.push(TextRange::at(
//                         TextSize::from(self.state.buffer.len() as u32),
//                         *length,
//                     ));
//                 }
//
//                 self.state
//                     .stack
//                     .push(Context::Verbatim, self.state.stack.top_args());
//             }
//
//             FormatElement::VerbatimEnd => {
//                 self.state.stack.pop(Context::Verbatim);
//             }
//             // TODO
//             FormatElement::Line(_) => {}
//             FormatElement::ConditionalGroupStart(_) => {}
//             ConditionalGroupEnd => {}
//             FormatElement::FillStart => {}
//             FormatElement::FillEnd => {}
//             FormatElement::LineSuffixStart => {}
//             FormatElement::LineSuffixEnd => {}
//             FormatElement::CommentStart => {}
//             FormatElement::CommentEnd => {}
//         }
//     }
//
//     /// Tries to print an element without any line breaks. Reverts any made `state` changes (by this function)
//     /// and returns with a [LineBreakRequiredError] if the `element` contains any hard line breaks
//     /// or printing the group exceeds the configured maximal print width.
//     fn try_print_flat(&mut self, queue: &mut ElementQueue) -> Result<(), LineBreakRequiredError> {
//         debug_assert_eq!(self.state.stack.top_context(), Context::Group);
//
//         let stack_end = self.state.stack.len() - 1;
//
//         while let Some(element) = queue.next() {
//             if let Err(err) = self.try_print_flat_element(element, queue) {
//                 return Err(err);
//             }
//
//             if self.state.stack.len() == stack_end {
//                 break;
//             }
//
//             debug_assert!(self.state.stack.len() > stack_end);
//         }
//
//         Ok(())
//     }
//
//     fn try_print_flat_element(
//         &mut self,
//         element: &FormatElement,
//         queue: &mut ElementQueue,
//     ) -> Result<(), LineBreakRequiredError> {
//         match element {
//             FormatElement::Token(_) => {
//                 let current_line = self.state.generated_line;
//
//                 // Delegate to generic string printing
//                 self.print_element(element, queue);
//
//                 // If the line is too long, break the group
//                 if self.state.line_width > self.options.print_width.value().into() {
//                     return Err(LineBreakRequiredError);
//                 }
//
//                 // If a new line was printed, break the group
//                 if current_line != self.state.generated_line {
//                     return Err(LineBreakRequiredError);
//                 }
//             }
//             //     FormatElement::Line(line) => {
//             //         match line.mode {
//             //             LineMode::SoftOrSpace => {
//             //                 if self.state.line_width > 0 {
//             //                     self.state.pending_space = true;
//             //                 }
//             //             }
//             //             // We want a flat structure, so omit soft line wraps
//             //             LineMode::Soft => {}
//             //             LineMode::Hard | LineMode::Empty => return Err(LineBreakRequiredError),
//             //         }
//             //     }
//             //
//             FormatElement::GroupStart => {
//                 self.state.stack.push(
//                     Context::Group,
//                     self.state.stack.top_args().with_hard_group(false),
//                 );
//             }
//             FormatElement::GroupEnd => {
//                 self.state.stack.pop(Context::Group);
//             }
//             //
//             //     // Fill elements are printed as space-separated lists in flat mode
//             //     FormatElement::Fill(list) => {
//             //         // Intersperse the list of elements with spaces before pushing
//             //         // them to the queue, however elements in the queue are stored
//             //         // as references so the space element must be allocated in a
//             //         // static so its reference is bound to the static lifetime
//             //         static SPACE: FormatElement = space_token();
//             //          // Instead use while loop that gets element for as long as it's inside of the fill
//             //         // tracks if a space is required
//             //         // calls print on the space
//             //         // then calls print on the next element
//             //         queue.0.extend(
//             //             Intersperse::new(list.iter().rev(), &SPACE)
//             //                 .map(|t| PrintElementCall::new(t, args.clone())),
//             //         );
//             //     }
//             //
//             //     FormatElement::ConditionalGroupContent(ConditionalGroupContent {
//             //         mode: GroupPrintMode::Flat,
//             //         content,
//             //     }) => queue.enqueue(PrintElementCall::new(content, args)),
//             //
//             //     // Omit if there's no flat_contents
//             //     FormatElement::ConditionalGroupContent(ConditionalGroupContent {
//             //         mode: GroupPrintMode::Multiline,
//             //         ..
//             //     }) => {}
//             //
//             //     FormatElement::Comment(content) => {
//             //         queue.enqueue(PrintElementCall::new(content.as_ref(), args));
//             //     }
//             //
//             //     FormatElement::LineSuffix { .. } => return Err(LineBreakRequiredError),
//             //
//             //     FormatElement::Empty
//             //     | FormatElement::Space
//             //     | FormatElement::Indent { .. }
//             //     | FormatElement::Verbatim { .. }
//             //     | FormatElement::List { .. } => self.print_element(queue, element, args),
//             FormatElement::HardGroupStart { .. } | FormatElement::HardGroupEnd => {
//                 self.print_element(element, queue)
//             } // }
//
//             // TODO
//             FormatElement::Space => {}
//             FormatElement::Line(_) => {}
//             FormatElement::VerbatimStart(_) => {}
//             FormatElement::VerbatimEnd => {}
//             FormatElement::IndentStart => {}
//             FormatElement::IndentEnd => {}
//             FormatElement::ConditionalGroupStart(_) => {}
//             ConditionalGroupEnd => {}
//             FormatElement::FillStart => {}
//             FormatElement::FillEnd => {}
//             FormatElement::LineSuffixStart => {}
//             FormatElement::LineSuffixEnd => {}
//             FormatElement::CommentStart => {}
//             FormatElement::CommentEnd => {}
//         }
//         Ok(())
//     }
//
//     // /// Print a list in fill mode.
//     // ///
//     // /// Prints the elements of the list separated by spaces, but backtrack if
//     // /// they go over the print width and insert a line break before resuming
//     // /// printing
//     // fn print_fill(
//     //     &mut self,
//     //     queue: &mut ElementCallQueue<'a>,
//     //     content: &'a List,
//     //     args: PrintElementArgs,
//     // ) {
//     //     let mut snapshot = None;
//     //
//     //     for item in content.iter() {
//     //         if snapshot.is_some() {
//     //             self.state.pending_space = true;
//     //         }
//     //
//     //         self.print_all(queue, item, args.clone());
//     //
//     //         if self.state.line_width > self.options.print_width.value().into() {
//     //             if let Some(snapshot) = snapshot.take() {
//     //                 self.state.restore(snapshot);
//     //
//     //                 static LINE: FormatElement = hard_line_break();
//     //                 self.print_all(queue, &LINE, args.clone());
//     //
//     //                 self.print_all(queue, item, args.clone());
//     //             }
//     //         }
//     //
//     //         snapshot = Some(self.state.snapshot());
//     //     }
//     // }
//
//     /// Fully print an element (print the element itself and all its descendants)
//     ///
//     /// Unlike [print_element], this function ensures the entire element has
//     /// been printed when it returns and the queue is back to its original state
//     fn print_all(&mut self, element: &FormatElement, queue: &mut ElementQueue) {
//         let extra_length = queue.extra_elements.len();
//         self.print_element(element, queue);
//
//         if queue.extra_elements.len() == extra_length {
//             return;
//         }
//
//         while let Some(element) = queue.next() {
//             self.print_element(element, queue);
//
//             if queue.extra_elements.len() == extra_length {
//                 return;
//             }
//
//             debug_assert!(queue.extra_elements.len() > extra_length);
//         }
//     }
//
//     fn print_str(&mut self, content: &str) {
//         self.state.buffer.reserve(content.len());
//
//         for char in content.chars() {
//             if char == '\n' {
//                 for char in self.options.line_ending.as_str().chars() {
//                     self.state.buffer.push(char);
//                 }
//
//                 self.state.generated_line += 1;
//                 self.state.generated_column = 0;
//                 self.state.line_width = 0;
//             } else {
//                 self.state.buffer.push(char);
//                 self.state.generated_column += 1;
//
//                 let char_width = if char == '\t' {
//                     self.options.tab_width as usize
//                 } else {
//                     1
//                 };
//
//                 self.state.line_width += char_width;
//             }
//
//             self.state.has_empty_line = false;
//         }
//     }
// }
//
// /// Printer state that is global to all elements.
// /// Stores the result of the print operation (buffer and mappings) and at what
// /// position the printer currently is.
// #[derive(Default, Debug)]
// struct PrinterState {
//     stack: ArgsStack,
//     buffer: String,
//     source_markers: Vec<SourceMarker>,
//     pending_indent: u16,
//     pending_space: bool,
//     generated_line: usize,
//     generated_column: usize,
//     line_width: usize,
//     has_empty_line: bool,
//     line_suffixes: Vec<FormatElement>,
//     verbatim_markers: Vec<TextRange>,
// }
//
// impl PrinterState {
//     /// Allows creating a snapshot of the state that can be restored using [restore]
//     pub fn snapshot(&self) -> PrinterStateSnapshot {
//         PrinterStateSnapshot {
//             pending_space: self.pending_space,
//             pending_indents: self.pending_indent,
//             generated_line: self.generated_line,
//             generated_column: self.generated_column,
//             line_width: self.line_width,
//             has_empty_line: self.has_empty_line,
//             buffer_position: self.buffer.len(),
//             tokens_position: self.source_markers.len(),
//             verbatim_markers: self.verbatim_markers.len(),
//             stack_length: self.stack.len(),
//         }
//     }
//
//     /// Restores the printer state to the state stored in the snapshot.
//     pub fn restore(&mut self, snapshot: PrinterStateSnapshot) {
//         let PrinterStateSnapshot {
//             pending_indents,
//             pending_space,
//             generated_column,
//             generated_line,
//             line_width,
//             has_empty_line,
//             buffer_position,
//             tokens_position,
//             verbatim_markers,
//             stack_length,
//         } = snapshot;
//         self.pending_space = pending_space;
//         self.pending_indent = pending_indents;
//         self.generated_column = generated_column;
//         self.generated_line = generated_line;
//         self.line_width = line_width;
//         self.has_empty_line = has_empty_line;
//         self.buffer.truncate(buffer_position);
//         self.source_markers.truncate(tokens_position);
//         self.verbatim_markers.truncate(verbatim_markers);
//         self.stack.truncate(stack_length);
//     }
// }
//
// /// Snapshot of a printer state.
// struct PrinterStateSnapshot {
//     pending_indents: u16,
//     pending_space: bool,
//     generated_column: usize,
//     generated_line: usize,
//     line_width: usize,
//     has_empty_line: bool,
//     buffer_position: usize,
//     tokens_position: usize,
//     verbatim_markers: usize,
//     stack_length: usize,
// }
//
// /// Stores arguments passed to `print_element` call, holding the state specific to printing an element.
// /// E.g. the `indent` depends on the token the Printer's currently processing. That's why
// /// it must be stored outside of the [PrinterState] that stores the state common to all elements.
// ///
// /// The state is passed by value, which is why it's important that it isn't storing any heavy
// /// data structures. Such structures should be stored on the [PrinterState] instead.
// #[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
// struct PrintElementArgs {
//     indent: u16,
//     hard_group: bool,
// }
//
// impl PrintElementArgs {
//     pub fn new(indent: u16) -> Self {
//         Self {
//             indent,
//             hard_group: false,
//         }
//     }
//
//     pub fn with_incremented_indent(self) -> Self {
//         Self::new(self.indent + 1)
//     }
//
//     pub fn with_hard_group(self, hard_group: bool) -> Self {
//         Self { hard_group, ..self }
//     }
// }
//
// /// The Printer uses a stack that emulates recursion. E.g. recursively processing the elements:
// /// `indent(concat(string, string))` would result in the following call stack:
// ///
// /// ```plain
// /// print_element(indent, indent = 0);
// ///   print_element(concat, indent = 1);
// ///     print_element(string, indent = 1);
// ///     print_element(string, indent = 1);
// /// ```
// /// The `PrintElementCall` stores the data for a single `print_element` call consisting of the element
// /// and the `args` that's passed to `print_element`.
// ///
// #[derive(Debug, Eq, PartialEq, Clone)]
// struct PrintElementCall<'element> {
//     element: &'element FormatElement,
//     args: PrintElementArgs,
// }
//
// impl<'element> PrintElementCall<'element> {
//     pub fn new(element: &'element FormatElement, args: PrintElementArgs) -> Self {
//         Self { element, args }
//     }
// }
//
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// enum Context {
//     Root,
//     Fill,
//     Group,
//     HardGroup,
//     Indent,
//     Verbatim,
//     Comment,
// }
//
// #[derive(Debug)]
// struct StackElement {
//     context: Context,
//     args: PrintElementArgs,
// }
//
// #[derive(Debug)]
// struct ArgsStack {
//     stack: Vec<StackElement>,
// }
//
// impl Default for ArgsStack {
//     fn default() -> Self {
//         Self::new(PrintElementArgs::default())
//     }
// }
//
// impl ArgsStack {
//     fn new(current: PrintElementArgs) -> Self {
//         Self {
//             stack: vec![StackElement {
//                 context: Context::Root,
//                 args: current,
//             }],
//         }
//     }
//
//     fn truncate(&mut self, len: usize) {
//         assert!(len > 1, "Stack must contain at least one element.");
//         self.stack.truncate(len);
//     }
//
//     fn len(&self) -> usize {
//         self.stack.len()
//     }
//
//     fn top_args(&self) -> PrintElementArgs {
//         // SAFETY: Stack is never empty because
//         // * `new` inserts the first args
//         // * `pop` never removes the last element
//         self.stack.last().unwrap().args
//     }
//
//     fn top_context(&self) -> Context {
//         // SAFETY: Stack is never empty because
//         // * `new` inserts the first args
//         // * `pop` never removes the last element
//         self.stack.last().unwrap().context
//     }
//
//     fn pop(&mut self, context: Context) -> PrintElementArgs {
//         assert_ne!(
//             context,
//             Context::Root,
//             "Popping the root context isn't allowed"
//         );
//         let top = self.stack.pop().unwrap();
//
//         assert_eq!(
//             context, top.context,
//             "Stack context mismatch. Poinsed format elements?"
//         );
//         top.args
//     }
//
//     fn push(&mut self, context: Context, args: PrintElementArgs) {
//         self.stack.push(StackElement { context, args });
//     }
//
//     fn finish(self) {
//         if self.stack.len() > 1 {
//             panic!(
//                 "Stack contains more than one remaining arg {:?}.",
//                 self.stack
//             );
//         }
//     }
// }
//
// /// Small helper that manages the order in which the elements should be visited.
// #[derive(Debug, Default)]
// struct ElementQueue {
//     elements: Vec<FormatElement>,
//     current: usize,
//     // Extra elements that have been inserted by the printer and must be formatted
//     // before the next element in `elements`
//     extra_elements: Vec<FormatElement>,
//     // Index into the extra collection
//     extra_index: usize,
// }
//
// impl ElementQueue {
//     #[inline]
//     fn new(elements: Vec<FormatElement>) -> Self {
//         Self {
//             elements,
//             current: 0,
//             extra_elements: vec![],
//             extra_index: 0,
//         }
//     }
//
//     fn next(&mut self) -> Option<&FormatElement> {
//         if self.extra_index < self.extra_elements.len() {
//             let current = self.extra_index;
//             self.extra_index += 1;
//             Some(&self.extra_elements[current])
//         } else if self.current < self.elements.len() {
//             let current = self.current;
//             self.current += 1;
//             Some(&self.elements[current])
//         } else {
//             None
//         }
//     }
//
//     fn extend<I: IntoIterator<Item = FormatElement>>(&mut self, elements: I) {
//         self.extra_elements.extend(elements)
//     }
//
//     fn is_empty(&self) -> bool {
//         self.current >= self.elements.len() && self.extra_index >= self.extra_elements.len()
//     }
// }
