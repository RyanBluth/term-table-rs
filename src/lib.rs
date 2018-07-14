//! The purpose of term-table is to make it easy for CLI apps to display data in a table format
//!# Example
//! Here is an example of how to create a simple table
//!```
//!let mut table = term_table::Table::new();
//!table.max_column_width = 40;
//!
//!table.style = term_table::TableStyle::extended();
//!table.add_row(term_table::row::Row::new(vec![
//!    term_table::cell::Cell::new_with_alignment("This is some centered text", 2, term_table::cell::Alignment::Center)
//!]));
//!table.add_row(term_table::row::Row::new(vec![
//!    term_table::cell::Cell::new("This is left aligned text", 1),
//!    term_table::cell::Cell::new_with_alignment("This is right aligned text", 1, term_table::cell::Alignment::Right)
//!]));
//! table.add_row(term_table::row::Row::new(vec![
//!    term_table::cell::Cell::new("This is left aligned text", 1),
//!    term_table::cell::Cell::new_with_alignment("This is right aligned text", 1, term_table::cell::Alignment::Right)
//!]));
//!table.add_row(term_table::row::Row::new(vec![
//!    term_table::cell::Cell::new("This is some really really really really really really really really really that is going to wrap to the next line", 2),
//!]));   
//!println!("{}", table.as_string());
//!```
//!
//!### This is the result
//!
//!<pre>
//! ╔═════════════════════════════════════════════════════════════════════════════════╗
//! ║                            This is some centered text                           ║
//! ╠════════════════════════════════════════╦════════════════════════════════════════╣
//! ║ This is left aligned text              ║             This is right aligned text ║
//! ╠════════════════════════════════════════╬════════════════════════════════════════╣
//! ║ This is left aligned text              ║             This is right aligned text ║
//! ╠════════════════════════════════════════╩════════════════════════════════════════╣
//! ║ This is some really really really really really really really really really tha ║
//! ║ t is going to wrap to the next line                                             ║
//! ╚═════════════════════════════════════════════════════════════════════════════════╝
//!</pre>

#[macro_use]
extern crate lazy_static;

extern crate wcwidth;
extern crate regex;

pub mod cell;
pub mod row;

use row::Row;

use std::cmp::{max, min};
use std::collections::HashMap;

/// Represents the vertical postion of a row
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum RowPosition {
    First,
    Mid,
    Last,
}

/// A set of characters which make up a table style
///
///# Example
///
///```
///   term_table::TableStyle {
///            top_left_corner: '╔',
///            top_right_corner: '╗',
///            bottom_left_corner: '╚',
///            bottom_right_corner: '╝',
///            outer_left_vertical: '╠',
///            outer_right_vertical: '╣',
///            outer_bottom_horizontal: '╩',
///            outer_top_horizontal: '╦',
///            intersection: '╬',
///            vertical: '║',
///            horizontal: '═',
///        };
///```
pub struct TableStyle {
    pub top_left_corner: char,
    pub top_right_corner: char,
    pub bottom_left_corner: char,
    pub bottom_right_corner: char,
    pub outer_left_vertical: char,
    pub outer_right_vertical: char,
    pub outer_bottom_horizontal: char,
    pub outer_top_horizontal: char,
    pub intersection: char,
    pub vertical: char,
    pub horizontal: char,
}

impl TableStyle {
    /// Basic terminal table style
    ///
    ///# Example
    ///
    ///<pre>
    ///   +---------------------------------------------------------------------------------+
    ///   |                            This is some centered text                           |
    ///   +----------------------------------------+----------------------------------------+
    ///   | This is left aligned text              |             This is right aligned text |
    ///   +----------------------------------------+----------------------------------------+
    ///   | This is left aligned text              |             This is right aligned text |
    ///   +----------------------------------------+----------------------------------------+
    ///   | This is some really really really really really really really really really tha |
    ///   | t is going to wrap to the next line                                             |
    ///   +---------------------------------------------------------------------------------+
    ///</pre>
    pub fn simple() -> TableStyle {
        return TableStyle {
            top_left_corner: '+',
            top_right_corner: '+',
            bottom_left_corner: '+',
            bottom_right_corner: '+',
            outer_left_vertical: '+',
            outer_right_vertical: '+',
            outer_bottom_horizontal: '+',
            outer_top_horizontal: '+',
            intersection: '+',
            vertical: '|',
            horizontal: '-',
        };
    }

    /// Table style using extended character set
    ///
    ///# Example
    ///
    ///<pre>
    /// ╔═════════════════════════════════════════════════════════════════════════════════╗
    /// ║                            This is some centered text                           ║
    /// ╠════════════════════════════════════════╦════════════════════════════════════════╣
    /// ║ This is left aligned text              ║             This is right aligned text ║
    /// ╠════════════════════════════════════════╬════════════════════════════════════════╣
    /// ║ This is left aligned text              ║             This is right aligned text ║
    /// ╠════════════════════════════════════════╩════════════════════════════════════════╣
    /// ║ This is some really really really really really really really really really tha ║
    /// ║ t is going to wrap to the next line                                             ║
    /// ╚═════════════════════════════════════════════════════════════════════════════════╝
    ///</pre>
    pub fn extended() -> TableStyle {
        return TableStyle {
            top_left_corner: '╔',
            top_right_corner: '╗',
            bottom_left_corner: '╚',
            bottom_right_corner: '╝',
            outer_left_vertical: '╠',
            outer_right_vertical: '╣',
            outer_bottom_horizontal: '╩',
            outer_top_horizontal: '╦',
            intersection: '╬',
            vertical: '║',
            horizontal: '═',
        };
    }

    /// Table style comprised of null characters
    ///
    ///# Example
    ///
    ///<pre>
    ///                           This is some centered text
    ///
    /// This is left aligned text                           This is right aligned text
    ///
    /// This is left aligned text                           This is right aligned text
    ///
    /// This is some really really really really really really really really really tha
    /// t is going to wrap to the next line
    ///</pre>
    pub fn blank() -> TableStyle {
        return TableStyle {
            top_left_corner: '\0',
            top_right_corner: '\0',
            bottom_left_corner: '\0',
            bottom_right_corner: '\0',
            outer_left_vertical: '\0',
            outer_right_vertical: '\0',
            outer_bottom_horizontal: '\0',
            outer_top_horizontal: '\0',
            intersection: '\0',
            vertical: '\0',
            horizontal: '\0',
        };
    }

    /// Returns the start character of a table style based on the
    /// vertical position of the row
    fn start_for_position(&self, pos: RowPosition) -> char {
        match pos {
            RowPosition::First => self.top_left_corner,
            RowPosition::Mid => self.outer_left_vertical,
            RowPosition::Last => self.bottom_left_corner,
        }
    }

    /// Returns the end character of a table style based on the
    /// vertical position of the row
    fn end_for_position(&self, pos: RowPosition) -> char {
        match pos {
            RowPosition::First => self.top_right_corner,
            RowPosition::Mid => self.outer_right_vertical,
            RowPosition::Last => self.bottom_right_corner,
        }
    }

    /// Returns the intersect character of a table style based on the
    /// vertical position of the row
    fn intersect_for_position(&self, pos: RowPosition) -> char {
        match pos {
            RowPosition::First => self.outer_top_horizontal,
            RowPosition::Mid => self.intersection,
            RowPosition::Last => self.outer_bottom_horizontal,
        }
    }

    /// Merges two intersecting characters based on the vertical position of a row.
    /// This is used to hanlde cases where one cell has a larger `col_span` value than the other
    fn merge_intersection_for_position(&self, top: char, bottom: char, pos: RowPosition) -> char {
        if (top == self.horizontal || top == self.outer_bottom_horizontal)
            && bottom == self.intersection
        {
            return self.outer_top_horizontal;
        } else if (top == self.intersection || top == self.outer_top_horizontal)
            && bottom == self.horizontal
        {
            return self.outer_bottom_horizontal;
        } else if top == self.outer_bottom_horizontal && bottom == self.horizontal {
            return self.horizontal;
        } else {
            return self.intersect_for_position(pos);
        }
    }
}

/// A set of rows containing data
pub struct Table<'data> {
    pub rows: Vec<Row<'data>>,
    pub style: TableStyle,
    /// The maximum width of all columns. Overriden by values in column_widths. Defults to `std::usize::max`
    pub max_column_width: usize,
    /// The maxium widths of specific columns. Override max_column
    pub max_column_widths: HashMap<usize, usize>,
}

impl<'data> Table<'data> {
    pub fn new() -> Table<'data> {
        return Table {
            rows: Vec::new(),
            style: TableStyle::extended(),
            max_column_width: std::usize::MAX,
            max_column_widths: HashMap::new(),
        };
    }

    /// Set the max width of a paticular column
    pub fn set_max_column_width(&mut self, column_index: usize, width: usize) {
        self.max_column_widths.insert(column_index, width);
    }

    /// Set the max widths of specific columns
    pub fn set_max_column_widths(&mut self, index_width_pairs: Vec<(usize, usize)>) {
        for pair in index_width_pairs {
            self.max_column_widths.insert(pair.0, pair.1);
        }
    }

    /// Simply adds a row to the rows Vec
    pub fn add_row(&mut self, row: Row<'data>) {
        self.rows.push(row);
    }

    /// Does all of the calculations to reformat the row based on it's current
    /// state and returns the result as a `String`
    pub fn as_string(&mut self) -> String {
        let mut print_buffer = String::new();
        let max_widths = self.calculate_max_column_widths();
        let mut previous_separator = None;
        if self.rows.len() > 0 {
            for i in 0..self.rows.len() {
                let mut row_pos = RowPosition::Mid;
                if i == 0 {
                    row_pos = RowPosition::First;
                }
                let separator = self.rows[i].gen_separator(
                    &max_widths,
                    &self.style,
                    row_pos,
                    previous_separator.clone(),
                );
                Table::buffer_line(&mut print_buffer, &separator);
                Table::buffer_line(
                    &mut print_buffer,
                    &self.rows[i].format(&max_widths, &self.style),
                );
                previous_separator = Some(separator.clone());
            }
            let separator = self.rows.last().unwrap().gen_separator(
                &max_widths,
                &self.style,
                RowPosition::Last,
                None,
            );
            Table::buffer_line(&mut print_buffer, &separator);
        }
        return print_buffer;
    }

    /// Calculates the maximum width for each column.
    /// If a cell has a column span greater than 1, then the width
    /// of it's contents are divided by the column span, otherwise the cell
    /// would use more space than it needed.
    fn calculate_max_column_widths(&self) -> Vec<usize> {
        let mut num_columns = 0;

        for row in &self.rows {
            num_columns = max(row.num_columns(), num_columns);
        }
        let mut max_widths: Vec<usize> = vec![0; num_columns];
        let mut min_widths: Vec<usize> = vec![0; num_columns];
        for row in &self.rows {
            let column_widths = row.split_column_widths();
            for i in 0..column_widths.len() {
                min_widths[i] = max(min_widths[i], column_widths[i].1);
                let mut max_width = *self.max_column_widths
                    .get(&i)
                    .unwrap_or(&self.max_column_width);
                max_width = max(min_widths[i] as usize, max_width);
                max_widths[i] = min(max_width, max(max_widths[i], column_widths[i].0 as usize));
            }
        }
        return max_widths;
    }

    /// Helper method for adding a line to a string buffer
    fn buffer_line(buffer: &mut String, line: &String) {
        buffer.push_str(format!("{}\n", line).as_str());
    }
}

#[cfg(test)]
mod test {

    use Table;
    use TableStyle;
    use cell::{Alignment, Cell};
    use row::Row;

    #[test]
    fn simple_table_style() {
        let mut table = Table::new();
        table.max_column_width = 40;

        table.style = TableStyle::simple();

        table.add_row(Row::new(vec![
            Cell::new_with_alignment("This is some centered text", 2, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is left aligned text", 1),
            Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is left aligned text", 1),
            Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is some really really really really really really really really really that is going to wrap to the next line", 2),
        ]));

        println!("{}", table.as_string());
    }

    #[test]
    fn extended_table_style() {
        let mut table = Table::new();
        table.max_column_width = 40;

        table.set_max_column_widths(vec![(0, 1), (1, 1)]);

        table.style = TableStyle::extended();

        table.add_row(Row::new(vec![
            Cell::new_with_alignment("This is some centered text", 2, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is left aligned text", 1),
            Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is left aligned text", 1),
            Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is some really really really really really really really really really that is going to wrap to the next line\n1\n2", 2),
        ]));

        println!("{}", table.as_string());
    }

    #[test]
    fn blank_table_style() {
        let mut table = Table::new();
        table.max_column_width = 40;

        table.style = TableStyle::blank();

        table.add_row(Row::new(vec![
            Cell::new_with_alignment("This is some centered text", 2, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is left aligned text", 1),
            Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is left aligned text", 1),
            Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("This is some really really really really really really really really really that is going to wrap to the next line", 2),
        ]));

        println!("{}", table.as_string());
    }

    #[test]
    fn complex_table() {
        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new("Col*1*Span*2", 2),
            Cell::new("Col 2 Span 1", 1),
            Cell::new("Col 3 Span 2", 2),
            Cell::new("Col 4 Span 1", 1),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Col 1 Span 1", 1),
            Cell::new("Col 2 Span 1", 1),
            Cell::new("Col 3 Span 1", 1),
            Cell::new("Col 4 Span 1", 2),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("fasdaff", 1),
            Cell::new("fff", 1),
            Cell::new("fff", 1),
        ]));
        table.add_row(Row::new(vec![
            Cell::new_with_alignment("fasdff", 3, Alignment::Right),
            Cell::new("fffdff", 4),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("fasdsaff", 1),
            Cell::new("fff", 1),
            Cell::new("f\nf\nf\nfff\nrrr\n\n\n", 1),
        ]));
        table.add_row(Row::new(vec![Cell::new("fasdsaff", 1)]));

        let s = table.as_string().clone();

        table.add_row(Row::new(vec![
            Cell::new_with_alignment(s, 3, Alignment::Left),
        ]));

        println!("{}", table.as_string());
    }
}
