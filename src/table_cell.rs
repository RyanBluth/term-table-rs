use lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::cmp;

use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

/// Represents the horizontal alignment of content within a cell.
#[derive(Clone, Copy, Debug)]
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
#[derive(Debug, Clone)]
pub struct TableCell<'data> {
    pub data: Cow<'data, str>,
    pub col_span: usize,
    pub alignment: Alignment,
    pub pad_content: bool,
}

impl<'data> TableCell<'data> {
    pub fn new<T>(data: T) -> TableCell<'data>
    where
        T: ToString,
    {
        Self {
            data: data.to_string().into(),
            col_span: 1,
            alignment: Alignment::Left,
            pad_content: true,
        }
    }

    pub fn new_with_col_span<T>(data: T, col_span: usize) -> TableCell<'data>
    where
        T: ToString,
    {
        Self {
            data: data.to_string().into(),
            alignment: Alignment::Left,
            pad_content: true,
            col_span,
        }
    }

    pub fn new_with_alignment<T>(data: T, col_span: usize, alignment: Alignment) -> TableCell<'data>
    where
        T: ToString,
    {
        Self {
            data: data.to_string().into(),
            pad_content: true,
            col_span,
            alignment,
        }
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
        Self {
            data: data.to_string().into(),
            col_span,
            alignment,
            pad_content,
        }
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
        max + if self.pad_content {
            2 * ' '.width().unwrap_or(1) as usize
        } else {
            0
        }
    }

    /// The width of the cell's content divided by its `col_span` value.
    pub fn split_width(&self) -> f32 {
        let res = self.width() as f32 / self.col_span as f32;
        res
    }

    /// The minium width required to display the cell properly
    pub fn min_width(&self) -> usize {
        let mut max_char_width: usize = 0;
        for c in self.data.chars() {
            max_char_width = cmp::max(max_char_width, c.width().unwrap_or(1) as usize);
        }

        if self.pad_content {
            max_char_width + ' '.width().unwrap_or(1) as usize * 2
        } else {
            max_char_width
        }
    }

    /// Wraps the cell's content to the provided width.
    ///
    /// New line characters are taken into account.
    pub fn wrapped_content(&self, width: usize) -> Vec<String> {
        let pad_char = if self.pad_content { ' ' } else { '\0' };
        let mut res: Vec<String> = Vec::new();
        let mut buf = String::new();
        buf.push(pad_char);
        for c in self.data.chars().enumerate() {
            if string_width(&buf) as usize >= width - pad_char.width().unwrap_or(1) as usize
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

        res
    }
}

impl<'data, T> From<T> for TableCell<'data>
where
    T: ToString,
{
    fn from(other: T) -> Self {
        TableCell::new(other)
    }
}

// Taken from https://github.com/mitsuhiko/console
lazy_static! {
    static ref STRIP_ANSI_RE: Regex =
        Regex::new(r"[\x1b\x9b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]")
            .unwrap();
}

// The width of a string. Strips ansi characters
pub fn string_width(string: &str) -> usize {
    let stripped = STRIP_ANSI_RE.replace_all(string, "");
    stripped.width()
}
