extern crate core;

use std::cell::{Ref, RefMut};
use std::fmt::Display;
use std::ops::Deref;

mod arguments;
mod ast;
mod buffer;
mod builders;
mod format_element;
mod formatter;
pub mod macros;
mod printer;

// use crate::printer::Printer;
use crate::{write, FormatOptions};
pub use arguments::{Argument, Arguments};
pub use buffer::{Buffer, VecBuffer};
pub use builders::*;
pub use format_element::*;
pub use formatter::*;
use rome_rowan::{SyntaxError, SyntaxResult, TextRange, TextSize};

/// Lightweight sourcemap marker between source and output tokens
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceMarker {
    /// Position of the marker in the original source
    pub source: TextSize,
    /// Position of the marker in the output code
    pub dest: TextSize,
}

pub fn write(output: &mut dyn Buffer, args: &Arguments<'_>) -> Result<()> {
    let mut formatter = Formatter::new(output);

    formatter.write_fmt(args)
}

/// Creates the formatter IR for the passed in item
/// Or use `format!` marco instead?
pub fn format(options: FormatOptions, arguments: &Arguments) -> Result<Formatted> {
    let mut buffer = VecBuffer::new(options);

    buffer.write_fmt(arguments)?;

    Ok(Formatted {
        elements: buffer.into_vec(),
    })
}

#[derive(Debug, Clone)]
pub struct Formatted {
    elements: Vec<FormatElement>,
}

impl Formatted {
    // /// Prints the formatted item into a string
    // pub fn print(self, options: PrintOptions) -> Printed {
    //     let printer = Printer::new(self.options);
    //
    //     printer.print(self.elements.last().unwrap())
    // }

    pub fn into_vec(self) -> Vec<FormatElement> {
        self.elements
    }
}

#[derive(Debug, Clone)]
pub struct Printed {
    code: String,
    range: Option<TextRange>,
    sourcemap: Vec<SourceMarker>,
    verbatim_ranges: Vec<TextRange>,
}

impl Printed {
    pub(crate) fn new(
        code: String,
        sourcemap: Vec<SourceMarker>,
        verbatim_ranges: Vec<TextRange>,
    ) -> Self {
        Printed {
            code,
            range: None,
            sourcemap,
            verbatim_ranges,
        }
    }

    pub(crate) fn with_range(mut self, range: Option<TextRange>) -> Self {
        self.range = range;
        self
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FormatError {
    MissingRequiredChild,
}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::MissingRequiredChild => std::write!(f, "Missing required child"),
        }
    }
}

impl From<&SyntaxError> for FormatError {
    fn from(syntax: &SyntaxError) -> Self {
        match syntax {
            SyntaxError::MissingRequiredChild => FormatError::MissingRequiredChild,
        }
    }
}

pub type Result<T> = std::result::Result<T, FormatError>;

pub trait Format {
    fn format(&self, formatter: &mut Formatter) -> Result<()>;
}

impl<'a, T> Format for &'a T
where
    T: Format + Sized,
{
    fn format(&self, formatter: &mut Formatter) -> Result<()> {
        Format::format(&**self, formatter)
    }
}

impl<T> Format for &'_ mut T
where
    T: Format + Sized,
{
    fn format(&self, formatter: &mut Formatter) -> Result<()> {
        Format::format(&**self, formatter)
    }
}

impl<T: ?Sized + Format> Format for Ref<'_, T> {
    fn format(&self, f: &mut Formatter<'_>) -> Result<()> {
        Format::format(&**self, f)
    }
}

impl<T: ?Sized + Format> Format for RefMut<'_, T> {
    fn format(&self, f: &mut Formatter<'_>) -> Result<()> {
        Format::format(&*(self.deref()), f)
    }
}

impl<T> Format for Option<T>
where
    T: Format,
{
    fn format(&self, formatter: &mut Formatter) -> Result<()> {
        match self {
            Some(value) => write!(formatter, value),
            None => Ok(()),
        }
    }
}

impl<T> Format for SyntaxResult<T>
where
    T: Format,
{
    fn format(&self, formatter: &mut Formatter) -> Result<()> {
        match self {
            Ok(value) => write!(formatter, value),
            Err(err) => Err(err.into()),
        }
    }
}

// pub trait FormatOption {
//     fn format_or(&self, alternative: &mut dyn Format) -> Result<()>;
//
//     // if let Some(element) { when write!(
//     fn format_with_or_empty<F>(&self, with) -> Result<()> where F: Fn(Self, )
// }

pub struct FormatWith<T>
where
    T: Fn(&mut Formatter) -> Result<()>,
{
    closure: T,
}

impl<T> Format for FormatWith<T>
where
    T: Fn(&mut Formatter) -> Result<()>,
{
    fn format(&self, formatter: &mut Formatter) -> Result<()> {
        (self.closure)(formatter)
    }
}

pub fn format_with<T>(closure: T) -> FormatWith<T>
where
    T: Fn(&mut Formatter) -> Result<()>,
{
    FormatWith { closure }
}
