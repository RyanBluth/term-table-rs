use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result};
use wcwidth::str_width;
use std::cmp;
use std;

#[derive(Clone, Copy)]
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

    pub fn split_width(&self) -> f32 {
        let res = self.width() as f32 / self.col_span as f32;
        return res;
    }

    pub fn wrap_to_width(&self, width: usize) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut buf = String::new();
        let mut current_width = 1;
        buf.push(' ');
        for c in self.data.chars().enumerate() {
            if current_width + 1 >= width || c.1 == '\n' {
                buf.push(' ');
                res.push(buf);
                buf = String::new();
                buf.push(' ');
                current_width = 1;
                if c.1 == '\n' {
                    continue;
                }
            }
            buf.push(c.1);
            current_width += 1;
        }
        buf.push(' ');
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
        write!(f, " {} ", self.data)
    }
}
