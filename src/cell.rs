use std;
use std::borrow::Cow;
use std::cmp;
use std::fmt::{Display, Formatter, Result};
use wcwidth::{char_width, str_width};

/// Represents the horizontal alignment of content within a cell.
#[derive(Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

///A table cell containing some str data.
///
///A cell may span multiple columns by setting the value of `col_span`.
///
///`pad_content` will add a space to either side of the cell's content.AsRef
pub struct Cell<'data> {
    pub data: Cow<'data, str>,
    pub col_span: usize,
    pub alignment: Alignment,
    pub pad_content: bool,
}

impl<'data> Cell<'data> {
    pub fn new<T>(data: T, col_span: usize) -> Cell<'data> where T: Into<Cow<'data, str>>{
        return Cell {
            data: data.into(),
            col_span: col_span,
            alignment: Alignment::Left,
            pad_content: true,
        };
    }

    pub fn new_with_alignment<T>(
        data: impl Into<Cow<'data, str>>,
        col_span: usize,
        alignment: Alignment,
    ) -> Cell<'data> where T: Into<Cow<'data, str>>{
        return Cell {
            data: data.into(),
            col_span: col_span,
            alignment: alignment,
            pad_content: true,
        };
    }

    pub fn new_with_alignment_and_padding<T>(
        data: impl Into<Cow<'data, str>>,
        col_span: usize,
        alignment: Alignment,
        pad_content: bool,
    ) -> Cell<'data> where T: Into<Cow<'data, str>>{
        return Cell {
            data: data.into(),
            col_span: col_span,
            alignment: alignment,
            pad_content: pad_content,
        };
    }

    /// Calculates the width of the cell.
    ///
    /// New line characters are taken into account during the calculation.
    pub fn width(&self) -> usize {
        let wrapped = self.wrap_to_width(std::usize::MAX);
        let mut max = 0;
        for s in wrapped {
            let str_width = match str_width(s.as_str()) {
                Some(w) => w,
                None => 0,
            };
            max = cmp::max(max, str_width);
        }
        return max + 2;
    }

    /// The width of the cell's content divided by its `col_span` value.
    pub fn split_width(&self) -> f32 {
        let res = self.width() as f32 / self.col_span as f32;
        return res;
    }

    /// Wraps the cell's content to the provided width.
    ///
    /// New line characters are taken into account.
    pub fn wrap_to_width(&self, width: usize) -> Vec<String> {
        let pad_char = match self.pad_content {
            true => ' ',
            false => '\0',
        };
        let mut res: Vec<String> = Vec::new();
        let mut buf = String::new();
        buf.push(pad_char);
        for c in self.data.chars().enumerate() {
            if str_width(buf.as_str()).unwrap_or_default() as usize
                >= width - char_width(pad_char).unwrap_or_default() as usize
                || c.1 == '\n'
            {
                buf.push(pad_char);
                res.push(buf);
                buf = String::new();
                buf.push(pad_char);
                if c.1 == '\n' {
                    continue;
                }
            }
            buf.push(c.1);
        }
        buf.push(pad_char);
        res.push(buf);
        return res;
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
        write!(f, "{}", self.data)
    }
}
