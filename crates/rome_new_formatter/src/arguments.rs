use crate::{Buffer, Format, Formatter};

// Ideally, a extern "C" { type Opaque } but the opaque features isn't stabilized.
// Use an empty enum as an opaque type instead.
enum Opaque {}

/// Stack allocated element that is pending for formatting
///
/// This struct is similar to dynamic dispatch (using `dyn Format`) by it stores a pointer to the value.
/// However, it doesn't store the pointer to `dyn Format`'s vtable, instead it statically resolves the function
/// pointer of `Format::format` and stores it in `formatter.
pub struct Argument<'fmt> {
    /// The value to format.
    value: &'fmt Opaque,
    /// The function pointer to `value`'s `Format::format` method
    formatter: fn(&'fmt Opaque, &mut Formatter<'_>) -> crate::Result<()>,
}

impl<'fmt> Argument<'fmt> {
    #[doc(hidden)]
    #[inline]
    pub fn new<F: Format>(value: &'fmt F) -> Self {
        let formatter: fn(&F, &mut Formatter<'_>) -> crate::Result<()> = F::format;

        unsafe {
            Self {
                // SAFETY: `mem::transmute` is safe because
                // 1. `&'fmt F` keeps the lifetime it originated with `'fmt`
                // 2. `&'fmt F` and `&'fmt Opaque` have the same memory layout
                value: std::mem::transmute(value),
                // SAFETY: `mem::transmute` is safe because `fn(&F, &mut Formatter<'_>) -> Result`
                // and `fn(&Opaque, &mut Formatter<'_> -> Result` have the same ABI
                formatter: std::mem::transmute(formatter),
            }
        }
    }

    /// Formats the value stored by this argument using the given formatter.
    pub(crate) fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        (self.formatter)(self.value, formatter)
    }
}

/// Stack allocated collection of items that should be formatted.
#[derive(Copy, Clone)]
pub struct Arguments<'fmt>(pub &'fmt [Argument<'fmt>]);

impl<'fmt> Arguments<'fmt> {
    #[doc(hidden)]
    #[inline]
    pub fn new(arguments: &'fmt [Argument<'fmt>]) -> Self {
        Self(arguments)
    }

    /// Returns the arguments
    pub(crate) fn items(&self) -> &'fmt [Argument<'fmt>] {
        &self.0
    }
}

impl Format for Arguments<'_> {
    fn format(&self, formatter: &mut Formatter) -> crate::Result<()> {
        formatter.write_fmt(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{format, format_args, group, space_token, token};
    use crate::{FormatElement, FormatOptions, Token};

    #[test]
    fn test_nesting() {
        // Format_arguments not very useful, but I guess the same as normal format_args

        let formatted = format!(
            FormatOptions::default(),
            token("function"),
            space_token(),
            token("a"),
            space_token(),
            group(format_args!(token("("), token(")")))
        );

        assert_eq!(
            formatted.unwrap().into_vec(),
            vec![
                FormatElement::Token(Token::Static { text: "function" }),
                FormatElement::Space,
                FormatElement::Token(Token::Static { text: "a" }),
                FormatElement::Space,
                FormatElement::GroupStart,
                FormatElement::Token(Token::Static { text: "(" }),
                FormatElement::Token(Token::Static { text: ")" }),
                FormatElement::GroupEnd
            ]
        );
    }
}
