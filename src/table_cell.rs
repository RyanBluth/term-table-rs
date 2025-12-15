use lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;

use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

/// Represents the horizontal alignment of content within a cell.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub struct TableCell {
    pub data: String,
    pub col_span: usize,
    pub alignment: Alignment,
    pub pad_content: bool,
}

impl TableCell {
    pub fn new<T>(data: T) -> TableCell
    where
        T: ToString,
    {
        Self {
            data: data.to_string(),
            col_span: 1,
            alignment: Alignment::Left,
            pad_content: true,
        }
    }

    pub fn builder<T>(data: T) -> TableCellBuilder
    where
        T: ToString,
    {
        TableCellBuilder::new(data.to_string())
    }

    #[deprecated(since = "1.4.0", note = "Use builder instead")]
    pub fn new_with_col_span<T>(data: T, col_span: usize) -> TableCell
    where
        T: ToString,
    {
        Self {
            data: data.to_string(),
            alignment: Alignment::Left,
            pad_content: true,
            col_span,
        }
    }

    #[deprecated(since = "1.4.0", note = "Use builder instead")]
    pub fn new_with_alignment<T>(data: T, col_span: usize, alignment: Alignment) -> TableCell
    where
        T: ToString,
    {
        Self {
            data: data.to_string(),
            pad_content: true,
            col_span,
            alignment,
        }
    }

    #[deprecated(since = "1.4.0", note = "Use builder instead")]
    pub fn new_with_alignment_and_padding<T>(
        data: T,
        col_span: usize,
        alignment: Alignment,
        pad_content: bool,
    ) -> TableCell
    where
        T: ToString,
    {
        Self {
            data: data.to_string(),
            col_span,
            alignment,
            pad_content,
        }
    }

    /// Calculates the width of the cell.
    ///
    /// New line characters are taken into account during the calculation.
    pub fn width(&self) -> usize {
        let wrapped = self.wrapped_content(usize::MAX);
        let mut max = 0;
        for s in wrapped {
            let str_width = string_width(&s);
            max = cmp::max(max, str_width);
        }
        max
    }

    /// The width of the cell's content divided by its `col_span` value.
    pub fn split_width(&self) -> f32 {
        self.width() as f32 / self.col_span as f32
    }

    /// The minium width required to display the cell properly
    pub fn min_width(&self) -> usize {
        let mut max_char_width: usize = 0;
        for c in self.data.chars() {
            max_char_width = cmp::max(max_char_width, c.width().unwrap_or(1));
        }

        if self.pad_content {
            max_char_width + ' '.width().unwrap_or(1) * 2
        } else {
            max_char_width
        }
    }

    /// Wraps the cell's content to the provided width.
    ///
    /// New line characters are taken into account.
    pub fn wrapped_content(&self, width: usize) -> Vec<String> {
        let pad_char = ' ';
        let hidden: HashSet<usize> = STRIP_ANSI_RE
            .find_iter(&self.data)
            .flat_map(|m| m.start()..m.end())
            .collect();
        let mut res: Vec<String> = Vec::new();
        let mut buf = String::new();

        if self.pad_content {
            buf.push(pad_char);
        }

        let mut byte_index = 0;
        for c in self.data.chars() {
            let pad_width = if self.pad_content { 1 } else { 0 };
            if !hidden.contains(&byte_index)
                && (string_width(&buf) >= width - pad_width || c == '\n')
            {
                if self.pad_content {
                    buf.push(pad_char);
                }
                res.push(buf);
                buf = String::new();
                if self.pad_content {
                    buf.push(pad_char);
                }
                if c == '\n' {
                    byte_index += 1;
                    continue;
                }
            }
            byte_index += c.len_utf8();
            buf.push(c);
        }
        if self.pad_content {
            buf.push(pad_char);
        }
        res.push(buf);

        res
    }
}

impl<T> From<T> for TableCell
where
    T: ToString,
{
    fn from(other: T) -> Self {
        TableCell::new(other)
    }
}

pub struct TableCellBuilder {
    data: String,
    col_span: usize,
    alignment: Alignment,
    pad_content: bool,
}

impl From<TableCellBuilder> for TableCell {
    fn from(val: TableCellBuilder) -> Self {
        val.build()
    }
}

impl From<&mut TableCellBuilder> for TableCell {
    fn from(val: &mut TableCellBuilder) -> Self {
        val.build()
    }
}

impl TableCellBuilder {
    fn new(data: String) -> TableCellBuilder {
        TableCellBuilder {
            data,
            col_span: 1,
            alignment: Alignment::Left,
            pad_content: true,
        }
    }

    pub fn col_span(&mut self, col_span: usize) -> &mut Self {
        self.col_span = col_span;
        self
    }

    pub fn alignment(&mut self, alignment: Alignment) -> &mut Self {
        self.alignment = alignment;
        self
    }

    pub fn pad_content(&mut self, pad_content: bool) -> &mut Self {
        self.pad_content = pad_content;
        self
    }

    pub fn build(&self) -> TableCell {
        TableCell {
            data: self.data.clone(),
            col_span: self.col_span,
            alignment: self.alignment,
            pad_content: self.pad_content,
        }
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
