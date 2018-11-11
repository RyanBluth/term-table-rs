use std::borrow::Cow;
use std::cmp;
use wcwidth::{char_width, str_width};
use regex::Regex;
use std::borrow::Borrow;

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
pub struct TableCell<'data> {
    pub data: Cow<'data, str>,
    pub col_span: usize,
    pub alignment: Alignment,
    pub pad_content: bool,
}

impl<'data> TableCell<'data> {
    pub fn new<T>(data: T) -> TableCell<'data>
    where
        T: ToString
    {
        return TableCell {
            data: data.to_string().into(),
            col_span: 1,
            alignment: Alignment::Left,
            pad_content: true,
        };
    }

    pub fn new_with_col_span<T>(data: T, col_span: usize) -> TableCell<'data>
    where
        T: ToString
    {
        return TableCell {
            data: data.to_string().into(),
            col_span: col_span,
            alignment: Alignment::Left,
            pad_content: true,
        };
    }

    pub fn new_with_alignment<T>(data: T, col_span: usize, alignment: Alignment) -> TableCell<'data>
    where
        T: ToString,
    {
        return TableCell {
            data: data.to_string().into(),
            col_span: col_span,
            alignment: alignment,
            pad_content: true,
        };
    }

    pub fn new_with_alignment_and_padding<T>(
        data: T,
        col_span: usize,
        alignment: Alignment,
        pad_content: bool,
    ) -> TableCell<'data>
    where
        T: ToString,
    {
        return TableCell {
            data: data.to_string().into(),
            col_span: col_span,
            alignment: alignment,
            pad_content: pad_content,
        };
    }

    /// Calculates the width of the cell.
    ///
    /// New line characters are taken into account during the calculation.
    pub fn width(&self) -> usize {
        let wrapped = self.wrapped_content(std::usize::MAX);
        let mut max = 0;
        for s in wrapped {
            let str_width = string_width(&s);
            max = cmp::max(max, str_width);
        }
        return max + match self.pad_content{
            true => 2 * char_width(' ').unwrap_or(1) as usize,
            false => 0
        }
    }

    /// The width of the cell's content divided by its `col_span` value.
    pub fn split_width(&self) -> f32 {
        let res = self.width() as f32 / self.col_span as f32;
        return res;
    }

    /// The minium width required to display the cell properly
    pub fn min_width(&self) -> usize {
        let mut max_char_width:usize = 0;
        for c in self.data.chars(){
            max_char_width = cmp::max(max_char_width, char_width(c).unwrap_or(1) as usize);
        }
        return match self.pad_content{
            true => max_char_width + char_width(' ').unwrap_or(1) as usize * 2,
            false => max_char_width
        }
    }

    /// Wraps the cell's content to the provided width.
    ///
    /// New line characters are taken into account.
    pub fn wrapped_content(&self, width: usize) -> Vec<String> {
        let pad_char = match self.pad_content {
            true => ' ',
            false => '\0',
        };
        let mut res: Vec<String> = Vec::new();
        let mut buf = String::new();
        buf.push(pad_char);
        for c in self.data.chars().enumerate() {
            if string_width(&buf) as usize
                >= width - char_width(pad_char).unwrap_or(1) as usize
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

impl<'data, T> From<T> for TableCell<'data> where T: ToString{
    fn from(other:T)-> Self{
        return TableCell::new(other);
    }
}  

// Taken from https://github.com/mitsuhiko/console
lazy_static! {
    static ref STRIP_ANSI_RE: Regex = Regex::new(
        r"[\x1b\x9b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]").unwrap();
}

// The width of a string. Strips ansi characters
pub fn string_width(string:&String) -> usize{
    let stripped = STRIP_ANSI_RE.replace_all(string.as_str(), "");
    return str_width(stripped.borrow()).unwrap_or_default();
}

