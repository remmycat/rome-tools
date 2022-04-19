/// TODO consider adding JSX kind of support: `<group>{}</group>
#[macro_export]
macro_rules! format_args {
    ($($value:expr),+ $(,)?) => {
        &$crate::Arguments::new(&[
            $(
                $crate::Argument::new(&$value)
            ),+
        ])
    }
}

#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:expr),+ $(,)?) => {{
        use $crate::Buffer;
        $dst.write_fmt($crate::format_args!($($arg),+))
    }}
}

/// Creates the Format IR for a value.
///
/// The first argument `format!` receives is the [FormatOptions] that specify how elements must be formatted.
/// Additional parameters passed get formatted by using their [Format] implementation.
///
///
/// ## Examples
///
/// ```
/// use rome_new_formatter::{space_token, token, FormatElement, Token, format, FormatOptions};
/// let formatted = format!(FormatOptions::default(), token("("), token("a"), token(")")).unwrap();
///
/// assert_eq!(formatted.into_vec(), vec![
///     FormatElement::Token(Token::Static { text: "(" }),
///     FormatElement::Token(Token::Static { text: "a" }),
///     FormatElement::Token(Token::Static { text: ")" }),
/// ]);
/// ```
#[macro_export]
macro_rules! format {
    ($options:expr, $($arg:expr),+ $(,)?) => {{
        ($crate::format($options, $crate::format_args!($($arg),+)))
    }}
}

#[cfg(test)]
mod tests {
    use crate::{space_token, token, FormatElement, FormatOptions, Token, VecBuffer};

    #[test]
    fn test_format_args() {
        let formatted = std::format!(
            "{:?}",
            format!(
                FormatOptions::default(),
                token("test"),
                space_token(),
                token("sentence")
            )
            .unwrap()
            .into_vec()
        );

        assert_eq!(
            formatted,
            "[Token(StaticToken(\"test\")), Space, Token(StaticToken(\"sentence\"))]"
        );
    }

    #[test]
    fn test_write() {
        let mut buffer = VecBuffer::new(FormatOptions::default());

        // References work too
        let space = &space_token();

        write!(buffer, token("test"), space, token("sentence")).unwrap();

        assert_eq!(
            buffer.into_vec(),
            vec![
                FormatElement::Token(Token::Static { text: "test" }),
                FormatElement::Space,
                FormatElement::Token(Token::Static { text: "sentence" })
            ]
        )
    }
}
