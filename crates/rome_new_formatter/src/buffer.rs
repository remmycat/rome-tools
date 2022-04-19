use crate::{write, Arguments, FormatElement, FormatOptions, Result};

pub trait Buffer {
    fn write_element(&mut self, element: FormatElement) -> Result<()>;

    fn write_fmt(mut self: &mut Self, arguments: &Arguments) -> Result<()> {
        write(&mut self, arguments)
    }

    fn options(&self) -> &FormatOptions;
}

impl<W: Buffer + ?Sized> Buffer for &mut W {
    fn write_element(&mut self, element: FormatElement) -> Result<()> {
        (**self).write_element(element)
    }

    fn write_fmt(&mut self, args: &Arguments<'_>) -> Result<()> {
        (**self).write_fmt(args)
    }

    fn options(&self) -> &FormatOptions {
        (**self).options()
    }
}

#[derive(Clone, Debug)]
pub struct VecBuffer {
    options: FormatOptions,
    elements: Vec<FormatElement>,
}

impl VecBuffer {
    pub fn new(options: FormatOptions) -> Self {
        Self {
            options,
            elements: vec![],
        }
    }

    pub fn with_capacity(capacity: usize, options: FormatOptions) -> Self {
        Self {
            options,
            elements: Vec::with_capacity(capacity),
        }
    }

    pub fn into_vec(self) -> Vec<FormatElement> {
        self.elements
    }
}

impl Buffer for VecBuffer {
    fn write_element(&mut self, element: FormatElement) -> Result<()> {
        self.elements.push(element);
        Ok(())
    }

    fn options(&self) -> &FormatOptions {
        &self.options
    }
}
