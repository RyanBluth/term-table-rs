use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result};

pub enum Alignment {
    Left,
    Right,
    Center,
}

pub struct Cell<'data> {
    pub data: Cow<'data, str>,
    pub col_span: usize,
    pub alignment: Alignment,
}

impl<'data> Cell<'data> {
    pub fn new<C>(data: C, col_span: usize) -> Cell<'data>
    where
        C: Into<Cow<'data, str>>,
    {
        return Cell {
            data: data.into(),
            col_span: col_span,
            alignment: Alignment::Left,
        };
    }

    pub fn new_with_alignment<C>(data: C, col_span: usize, alignment: Alignment) -> Cell<'data>
    where
        C: Into<Cow<'data, str>>,
    {
        return Cell {
            data: data.into(),
            col_span: col_span,
            alignment: alignment,
        };
    }

    pub fn width(&self) -> usize {
        return self.data.chars().count() + 2;
    }

    pub fn split_width(&self) -> f32 {
        let res = self.width() as f32 / self.col_span as f32;
        return res;
    }

    pub fn format_with_padding(&self, padding: usize) -> String {
        match self.alignment {
            Alignment::Left => return format!("{}{}", self, str::repeat(" ", padding)),
            Alignment::Right => return format!("{}{}", str::repeat(" ", padding), self),
            Alignment::Center => {
                let half_padding = padding as f32 / 2.0;
                return format!(
                    "{}{}{}",
                    str::repeat(" ", half_padding.ceil() as usize),
                    self,
                    str::repeat(" ", half_padding.floor() as usize)
                );
            }
        }
    }
}

impl<'data, T> From<&'data T> for Cell<'data>
where
    T: Display,
{
    fn from(x: &'data T) -> Cell<'data> {
        return Cell::new(format!("{}", x), 1);
    }
}

impl<'data> Display for Cell<'data> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, " {} ", self.data)
    }
}
