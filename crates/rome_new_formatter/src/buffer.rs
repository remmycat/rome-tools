use crate::{write, Arguments, FormatElement, FormatOptions, Result};
use std::ops::{Deref, DerefMut};

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

    /// Writes the elements from this buffer into the passed buffer
    pub fn write_into(&mut self, buffer: &mut dyn Buffer) -> crate::Result<()> {
        for element in self.drain(..) {
            buffer.write_element(element)?;
        }

        Ok(())
    }

    pub fn into_vec(self) -> Vec<FormatElement> {
        self.elements
    }
}

impl Deref for VecBuffer {
    type Target = Vec<FormatElement>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl DerefMut for VecBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
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
