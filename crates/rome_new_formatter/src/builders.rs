use crate::{Buffer, Format, FormatElement, Formatter};

pub struct FillBuilder<'a, 'with, 'buf> {
    inner: JoinBuilder<'a, 'with, 'buf>,
}

// TODO optimize to not write start if empty
impl<'a, 'with, 'buf> FillBuilder<'a, 'with, 'buf> {
    pub(crate) fn new(fmt: &'a mut Formatter<'buf>) -> Self {
        let result = fmt.write_element(FormatElement::FillStart);
        Self {
            inner: JoinBuilder {
                result,
                fmt,
                with: None,
                has_elements: false,
            },
        }
    }

    pub(crate) fn with(fmt: &'a mut Formatter<'buf>, with: &'with dyn Format) -> Self {
        let result = fmt.write_element(FormatElement::FillStart);

        Self {
            inner: JoinBuilder {
                result,
                fmt,
                with: Some(with),
                has_elements: false,
            },
        }
    }

    pub fn entries<F, I>(&mut self, entries: I) -> &mut Self
    where
        F: Format,
        I: IntoIterator<Item = F>,
    {
        for entry in entries {
            self.inner.entry(entry);
        }

        self
    }

    pub fn entry<F>(&mut self, entry: F) -> &mut Self
    where
        F: Format,
    {
        self.inner.entry(entry);
        self
    }

    pub fn finish(&mut self) -> crate::Result<()> {
        self.inner
            .result
            .and_then(|_| self.inner.fmt.write_element(FormatElement::FillEnd))
    }
}

pub struct JoinBuilder<'fmt, 'joiner, 'buf> {
    result: crate::Result<()>,
    fmt: &'fmt mut Formatter<'buf>,
    with: Option<&'joiner dyn Format>,
    has_elements: bool,
}

impl<'fmt, 'joiner, 'buf> JoinBuilder<'fmt, 'joiner, 'buf> {
    pub(crate) fn new(fmt: &'fmt mut Formatter<'buf>) -> Self {
        Self {
            result: Ok(()),
            fmt,
            has_elements: false,
            with: None,
        }
    }

    pub(crate) fn with(fmt: &'fmt mut Formatter<'buf>, with: &'joiner dyn Format) -> Self {
        Self {
            result: Ok(()),
            fmt,
            has_elements: false,
            with: Some(with),
        }
    }

    pub fn entry<F>(&mut self, entry: F) -> &mut Self
    where
        F: Format,
    {
        self.result = self.result.and_then(|_| {
            if let Some(with) = &self.with {
                if self.has_elements {
                    with.format(self.fmt)?;
                }
            }

            entry.format(self.fmt)
        });

        self
    }

    pub fn entries<F, I>(&mut self, entries: I) -> &mut Self
    where
        F: Format,
        I: IntoIterator<Item = F>,
    {
        for entry in entries {
            self.entry(entry);
        }

        self
    }

    pub fn finish(&mut self) -> crate::Result<()> {
        self.result
    }
}
