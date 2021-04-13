//! The purpose of term-table is to make it easy for CLI apps to display data in a table format
//!# Example
//! Here is an example of how to create a simple table
//!```
//!use term_table::table_cell::TableCell;
//!use term_table::row::Row;
//!  
//!let mut table = term_table::Table::new();
//!table.max_column_width = 40;
//!
//!table.style = term_table::TableStyle::extended();
//!table.add_row(Row::new(vec![
//!    TableCell::new_with_alignment("This is some centered text", 2, term_table::table_cell::Alignment::Center)
//!]));
//!table.add_row(Row::new(vec![
//!    TableCell::new("This is left aligned text"),
//!    TableCell::new_with_alignment("This is right aligned text", 1, term_table::table_cell::Alignment::Right)
//!]));
//! table.add_row(Row::new(vec![
//!    TableCell::new("This is left aligned text"),
//!    TableCell::new_with_alignment("This is right aligned text", 1, term_table::table_cell::Alignment::Right)
//!]));
//!table.add_row(Row::new(vec![
//!    TableCell::new_with_col_span("This is some really really really really really really really really really that is going to wrap to the next line", 2),
//!]));
//!println!("{}", table.render());
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

pub mod row;
pub mod table_cell;

use crate::row::Row;
use crate::table_cell::Alignment;

use std::cmp::{max, min};
use std::collections::HashMap;

/// Represents the vertical position of a row
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
#[derive(Debug, Clone, Copy)]
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
        TableStyle {
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
        }
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
        TableStyle {
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
        }
    }

    /// <pre>
    /// ┌─────────────────────────────────────────────────────────────────────────────────┐
    /// │                            This is some centered text                           │
    /// ├────────────────────────────────────────┬────────────────────────────────────────┤
    /// │ This is left aligned text              │             This is right aligned text │
    /// ├────────────────────────────────────────┼────────────────────────────────────────┤
    /// │ This is left aligned text              │             This is right aligned text │
    /// ├────────────────────────────────────────┴────────────────────────────────────────┤
    /// │ This is some really really really really really really really really really tha │
    /// │ t is going to wrap to the next line                                             │
    /// └─────────────────────────────────────────────────────────────────────────────────┘
    /// </pre>
    pub fn thin() -> TableStyle {
        TableStyle {
            top_left_corner: '┌',
            top_right_corner: '┐',
            bottom_left_corner: '└',
            bottom_right_corner: '┘',
            outer_left_vertical: '├',
            outer_right_vertical: '┤',
            outer_bottom_horizontal: '┴',
            outer_top_horizontal: '┬',
            intersection: '┼',
            vertical: '│',
            horizontal: '─',
        }
    }

    ///  <pre>
    /// ╭─────────────────────────────────────────────────────────────────────────────────╮
    /// │                            This is some centered text                           │
    /// ├────────────────────────────────────────┬────────────────────────────────────────┤
    /// │ This is left aligned text              │             This is right aligned text │
    /// ├────────────────────────────────────────┼────────────────────────────────────────┤
    /// │ This is left aligned text              │             This is right aligned text │
    /// ├────────────────────────────────────────┴────────────────────────────────────────┤
    /// │ This is some really really really really really really really really really tha │
    /// │ t is going to wrap to the next line                                             │
    /// ╰─────────────────────────────────────────────────────────────────────────────────╯
    /// </pre>
    pub fn rounded() -> TableStyle {
        TableStyle {
            top_left_corner: '╭',
            top_right_corner: '╮',
            bottom_left_corner: '╰',
            bottom_right_corner: '╯',
            outer_left_vertical: '├',
            outer_right_vertical: '┤',
            outer_bottom_horizontal: '┴',
            outer_top_horizontal: '┬',
            intersection: '┼',
            vertical: '│',
            horizontal: '─',
        }
    }

    /// <pre>
    /// ╔─────────────────────────────────────────────────────────────────────────────────╗
    /// │                            This is some centered text                           │
    /// ╠────────────────────────────────────────╦────────────────────────────────────────╣
    /// │ This is left aligned text              │             This is right aligned text │
    /// ╠────────────────────────────────────────┼────────────────────────────────────────╣
    /// │ This is left aligned text              │             This is right aligned text │
    /// ╠────────────────────────────────────────╩────────────────────────────────────────╣
    /// │ This is some really really really really really really really really really tha │
    /// │ t is going to wrap to the next line                                             │
    /// ╚─────────────────────────────────────────────────────────────────────────────────╝
    /// </pre>

    pub fn elegant() -> TableStyle {
        TableStyle {
            top_left_corner: '╔',
            top_right_corner: '╗',
            bottom_left_corner: '╚',
            bottom_right_corner: '╝',
            outer_left_vertical: '╠',
            outer_right_vertical: '╣',
            outer_bottom_horizontal: '╩',
            outer_top_horizontal: '╦',
            intersection: '┼',
            vertical: '│',
            horizontal: '─',
        }
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
        TableStyle {
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
        }
    }

    /// Table style comprised of empty characters for compatibility with terminals
    /// that don't handle null characters appropriately
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
    pub fn empty() -> TableStyle {
        TableStyle {
            top_left_corner: ' ',
            top_right_corner: ' ',
            bottom_left_corner: ' ',
            bottom_right_corner: ' ',
            outer_left_vertical: ' ',
            outer_right_vertical: ' ',
            outer_bottom_horizontal: ' ',
            outer_top_horizontal: ' ',
            intersection: ' ',
            vertical: ' ',
            horizontal: ' ',
        }
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
    /// This is used to handle cases where one cell has a larger `col_span` value than the other
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
#[derive(Clone, Debug)]
pub struct Table<'data> {
    pub rows: Vec<Row<'data>>,
    pub style: TableStyle,
    /// The maximum width of all columns. Overridden by values in column_widths. Defaults to `std::usize::max`
    pub max_column_width: usize,
    /// The maximum widths of specific columns. Override max_column
    pub max_column_widths: HashMap<usize, usize>,
    /// Whether or not to vertically separate rows in the table
    pub separate_rows: bool,
    /// Whether the table should have a top boarder.
    /// Setting `has_separator` to false on the first row will have the same effect as setting this to false
    pub has_top_boarder: bool,
    /// Whether the table should have a bottom boarder
    pub has_bottom_boarder: bool,
}

impl<'data> Table<'data> {
    pub fn new() -> Table<'data> {
        Self {
            rows: Vec::new(),
            style: TableStyle::extended(),
            max_column_width: std::usize::MAX,
            max_column_widths: HashMap::new(),
            separate_rows: true,
            has_top_boarder: true,
            has_bottom_boarder: true,
        }
    }

    pub fn with_rows(rows: Vec<Row<'data>>) -> Table<'data> {
        Self {
            rows,
            style: TableStyle::extended(),
            max_column_width: std::usize::MAX,
            max_column_widths: HashMap::new(),
            separate_rows: true,
            has_top_boarder: true,
            has_bottom_boarder: true,
        }
    }

    pub fn max_column_width(&mut self, max_column_width: usize) -> &mut Self {
        self.max_column_width = max_column_width;
        self
    }

    /// Set the max width of a particular column
    pub fn set_max_width_for_column(&mut self, column_index: usize, width: usize) {
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
    pub fn render(&self) -> String {
        let mut print_buffer = String::new();
        let max_widths = self.calculate_max_column_widths();
        let mut previous_separator = None;
        if !self.rows.is_empty() {
            for i in 0..self.rows.len() {
                let row_pos = if i == 0 {
                    RowPosition::First
                } else {
                    RowPosition::Mid
                };

                let separator = self.rows[i].gen_separator(
                    &max_widths,
                    &self.style,
                    row_pos,
                    previous_separator.clone(),
                );

                previous_separator = Some(separator.clone());

                if self.rows[i].has_separator
                    && ((i == 0 && self.has_top_boarder) || i != 0 && self.separate_rows)
                {
                    Table::buffer_line(&mut print_buffer, &separator);
                }

                Table::buffer_line(
                    &mut print_buffer,
                    &self.rows[i].format(&max_widths, &self.style),
                );
            }
            if self.has_bottom_boarder {
                let separator = self.rows.last().unwrap().gen_separator(
                    &max_widths,
                    &self.style,
                    RowPosition::Last,
                    None,
                );
                Table::buffer_line(&mut print_buffer, &separator);
            }
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
                let mut max_width = *self
                    .max_column_widths
                    .get(&i)
                    .unwrap_or(&self.max_column_width);
                max_width = max(min_widths[i] as usize, max_width);
                max_widths[i] = min(max_width, max(max_widths[i], column_widths[i].0 as usize));
            }
        }

        // Here we are dealing with the case where we have a cell that is center
        // aligned but the max_width doesn't allow for even padding on either side
        for row in &self.rows {
            let mut col_index = 0;
            for cell in row.cells.iter() {
                let mut total_col_width = 0;
                for i in col_index..col_index + cell.col_span {
                    total_col_width += max_widths[i];
                }
                if cell.width() != total_col_width
                    && cell.alignment == Alignment::Center
                    && total_col_width as f32 % 2.0 <= 0.001
                {
                    let mut max_col_width = self.max_column_width;
                    if let Some(specific_width) = self.max_column_widths.get(&col_index) {
                        max_col_width = *specific_width;
                    }

                    if max_widths[col_index] < max_col_width {
                        max_widths[col_index] += 1;
                    }
                }
                if cell.col_span > 1 {
                    col_index += cell.col_span - 1;
                } else {
                    col_index += 1;
                }
            }
        }

        return max_widths;
    }

    /// Helper method for adding a line to a string buffer
    fn buffer_line(buffer: &mut String, line: &str) {
        buffer.push_str(format!("{}\n", line).as_str());
    }
}

impl<'data> Default for Table<'data> {
    fn default() -> Self {
        return Table::new();
    }
}

impl<'data> ToString for Table<'data> {
    fn to_string(&self) -> String {
        return self.render();
    }
}

/// Used to create non-mutable tables
#[derive(Clone, Debug)]
pub struct TableBuilder<'data> {
    rows: Vec<Row<'data>>,
    style: TableStyle,
    max_column_width: usize,
    max_column_widths: HashMap<usize, usize>,
    separate_rows: bool,
    has_top_boarder: bool,
    has_bottom_boarder: bool,
}

impl<'data> TableBuilder<'data> {
    pub fn new() -> TableBuilder<'data> {
        TableBuilder {
            rows: Vec::new(),
            style: TableStyle::extended(),
            max_column_width: std::usize::MAX,
            max_column_widths: HashMap::new(),
            separate_rows: true,
            has_top_boarder: true,
            has_bottom_boarder: true,
        }
    }

    pub fn rows(&mut self, rows: Vec<Row<'data>>) -> &mut Self {
        self.rows = rows;
        self
    }

    pub fn style(&mut self, style: TableStyle) -> &mut Self {
        self.style = style;
        self
    }

    /// The maximum width of all columns. Overridden by values in column_widths. Defaults to `std::usize::max`
    pub fn max_column_width(&mut self, max_column_width: usize) -> &mut Self {
        self.max_column_width = max_column_width;
        self
    }

    /// The maximum widths of specific columns. Override max_column
    pub fn max_column_widths(&mut self, max_column_widths: HashMap<usize, usize>) -> &mut Self {
        self.max_column_widths = max_column_widths;
        self
    }

    /// Whether or not to vertically separate rows in the table
    pub fn separate_rows(&mut self, separate_rows: bool) -> &mut Self {
        self.separate_rows = separate_rows;
        self
    }

    /// Whether the table should have a top boarder.
    /// Setting `has_separator` to false on the first row will have the same effect as setting this to false
    pub fn has_top_boarder(&mut self, has_top_boarder: bool) -> &mut Self {
        self.has_top_boarder = has_top_boarder;
        self
    }

    /// Whether the table should have a bottom boarder
    pub fn has_bottom_boarder(&mut self, has_bottom_boarder: bool) -> &mut Self {
        self.has_bottom_boarder = has_bottom_boarder;
        self
    }

    /// Build a Table using the current configuration
    pub fn build(&self) -> Table<'data> {
        Table {
            rows: self.rows.clone(),
            style: self.style,
            max_column_width: self.max_column_width,
            max_column_widths: self.max_column_widths.clone(),
            separate_rows: self.separate_rows,
            has_top_boarder: self.has_top_boarder,
            has_bottom_boarder: self.has_bottom_boarder,
        }
    }
}

impl<'data> Default for TableBuilder<'data> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {

    use crate::row::Row;
    use crate::table_cell::{Alignment, TableCell};
    use crate::Table;
    use crate::TableBuilder;
    use crate::TableStyle;
    use pretty_assertions::assert_eq;

    #[test]
    fn correct_default_padding() {
        let mut table = Table::new();
        table.separate_rows = false;
        table.style = TableStyle::simple();
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("A", 1, Alignment::Center),
            TableCell::new_with_alignment("B", 1, Alignment::Center),
        ]));
        table.add_row(Row::new(vec![TableCell::new(1), TableCell::new("1")]));
        table.add_row(Row::new(vec![TableCell::new(2), TableCell::new("10")]));
        table.add_row(Row::new(vec![TableCell::new(3), TableCell::new("100")]));
        let expected = r"+---+-----+
| A |  B  |
| 1 | 1   |
| 2 | 10  |
| 3 | 100 |
+---+-----+
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn uneven_center_alignment() {
        let mut table = Table::new();
        table.separate_rows = false;
        table.style = TableStyle::simple();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "A",
            1,
            Alignment::Center,
        )]));
        table.add_row(Row::new(vec![TableCell::new(11)]));
        table.add_row(Row::new(vec![TableCell::new(2)]));
        table.add_row(Row::new(vec![TableCell::new(3)]));
        let expected = r"+-----+
|  A  |
| 11  |
| 2   |
| 3   |
+-----+
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn uneven_center_alignment_2() {
        let mut table = Table::new();
        table.separate_rows = false;
        table.style = TableStyle::simple();
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("A1", 1, Alignment::Center),
            TableCell::new_with_alignment("B", 1, Alignment::Center),
        ]));
        println!("{}", table.render());
        let expected = r"+----+---+
| A1 | B |
+----+---+
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn simple_table_style() {
        let mut table = TableBuilder::new().style(TableStyle::simple()).build();

        add_data_to_test_table(&mut table);

        let expected = r"+---------------------------------------------------------------------------------+
|                            This is some centered text                           |
+----------------------------------------+----------------------------------------+
| This is left aligned text              |             This is right aligned text |
+----------------------------------------+----------------------------------------+
| This is left aligned text              |             This is right aligned text |
+----------------------------------------+----------------------------------------+
| This is some really really really really really really really really really tha |
| t is going to wrap to the next line                                             |
+---------------------------------------------------------------------------------+
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn uneven_with_varying_col_span() {
        let mut table = Table::new();
        table.separate_rows = true;
        table.style = TableStyle::simple();
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("A1111111", 1, Alignment::Center),
            TableCell::new_with_alignment("B", 1, Alignment::Center),
        ]));
        table.add_row(Row::new(vec![TableCell::new(1), TableCell::new("1")]));
        table.add_row(Row::new(vec![TableCell::new(2), TableCell::new("10")]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment_and_padding(3, 1, Alignment::Left, false),
            TableCell::new("100"),
        ]));
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "S",
            2,
            Alignment::Center,
        )]));
        let expected = "+----------+-----+
| A1111111 |  B  |
+----------+-----+
| 1        | 1   |
+----------+-----+
| 2        | 10  |
+----------+-----+
|\03\0         | 100 |
+----------+-----+
|        S       |
+----------------+
";
        println!("{}", table.render());
        assert_eq!(expected.trim(), table.render().trim());
    }

    // TODO - The output of this test isn't ideal. There is probably a better way to calculate the
    // the column/row layout that would improve this
    #[test]
    fn uneven_with_varying_col_span_2() {
        let mut table = Table::new();
        table.separate_rows = false;
        table.style = TableStyle::simple();
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("A", 1, Alignment::Center),
            TableCell::new_with_alignment("B", 1, Alignment::Center),
        ]));
        table.add_row(Row::new(vec![TableCell::new(1), TableCell::new("1")]));
        table.add_row(Row::new(vec![TableCell::new(2), TableCell::new("10")]));
        table.add_row(Row::new(vec![TableCell::new(3), TableCell::new("100")]));
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "Spanner",
            2,
            Alignment::Center,
        )]));
        let expected = "+------+-----+
|   A  |  B  |
| 1    | 1   |
| 2    | 10  |
| 3    | 100 |
|   Spanner  |
+------------+
";
        println!("{}", table.render());
        assert_eq!(expected.trim(), table.render().trim());
    }

    #[test]
    fn extended_table_style_wrapped() {
        let mut table = Table::new();
        table.max_column_width = 40;

        table.set_max_column_widths(vec![(0, 1), (1, 1)]);

        table.style = TableStyle::extended();

        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "This is some centered text",
            2,
            Alignment::Center,
        )]));

        table.add_row(Row::new(vec![
            TableCell::new("This is left aligned text"),
            TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new("This is left aligned text"),
            TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_col_span("This is some really really really really really really really really really that is going to wrap to the next line\n1\n2", 2),
        ]));

        let expected = r"╔═══════╗
║ This  ║
║ is so ║
║ me ce ║
║ ntere ║
║ d tex ║
║   t   ║
╠═══╦═══╣
║ T ║ T ║
║ h ║ h ║
║ i ║ i ║
║ s ║ s ║
║   ║   ║
║ i ║ i ║
║ s ║ s ║
║   ║   ║
║ l ║ r ║
║ e ║ i ║
║ f ║ g ║
║ t ║ h ║
║   ║ t ║
║ a ║   ║
║ l ║ a ║
║ i ║ l ║
║ g ║ i ║
║ n ║ g ║
║ e ║ n ║
║ d ║ e ║
║   ║ d ║
║ t ║   ║
║ e ║ t ║
║ x ║ e ║
║ t ║ x ║
║   ║ t ║
╠═══╬═══╣
║ T ║ T ║
║ h ║ h ║
║ i ║ i ║
║ s ║ s ║
║   ║   ║
║ i ║ i ║
║ s ║ s ║
║   ║   ║
║ l ║ r ║
║ e ║ i ║
║ f ║ g ║
║ t ║ h ║
║   ║ t ║
║ a ║   ║
║ l ║ a ║
║ i ║ l ║
║ g ║ i ║
║ n ║ g ║
║ e ║ n ║
║ d ║ e ║
║   ║ d ║
║ t ║   ║
║ e ║ t ║
║ x ║ e ║
║ t ║ x ║
║   ║ t ║
╠═══╩═══╣
║ This  ║
║ is so ║
║ me re ║
║ ally  ║
║ reall ║
║ y rea ║
║ lly r ║
║ eally ║
║  real ║
║ ly re ║
║ ally  ║
║ reall ║
║ y rea ║
║ lly r ║
║ eally ║
║  that ║
║  is g ║
║ oing  ║
║ to wr ║
║ ap to ║
║  the  ║
║ next  ║
║ line  ║
║ 1     ║
║ 2     ║
╚═══════╝
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn elegant_table_style() {
        let mut table = Table::new();
        table.style = TableStyle::elegant();

        add_data_to_test_table(&mut table);

        let expected = r"╔─────────────────────────────────────────────────────────────────────────────────╗
│                            This is some centered text                           │
╠────────────────────────────────────────╦────────────────────────────────────────╣
│ This is left aligned text              │             This is right aligned text │
╠────────────────────────────────────────┼────────────────────────────────────────╣
│ This is left aligned text              │             This is right aligned text │
╠────────────────────────────────────────╩────────────────────────────────────────╣
│ This is some really really really really really really really really really tha │
│ t is going to wrap to the next line                                             │
╚─────────────────────────────────────────────────────────────────────────────────╝
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn thin_table_style() {
        let mut table = Table::new();
        table.style = TableStyle::thin();

        add_data_to_test_table(&mut table);

        let expected = r"┌─────────────────────────────────────────────────────────────────────────────────┐
│                            This is some centered text                           │
├────────────────────────────────────────┬────────────────────────────────────────┤
│ This is left aligned text              │             This is right aligned text │
├────────────────────────────────────────┼────────────────────────────────────────┤
│ This is left aligned text              │             This is right aligned text │
├────────────────────────────────────────┴────────────────────────────────────────┤
│ This is some really really really really really really really really really tha │
│ t is going to wrap to the next line                                             │
└─────────────────────────────────────────────────────────────────────────────────┘
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn rounded_table_style() {
        let mut table = Table::new();

        table.style = TableStyle::rounded();

        add_data_to_test_table(&mut table);

        let expected = r"╭─────────────────────────────────────────────────────────────────────────────────╮
│                            This is some centered text                           │
├────────────────────────────────────────┬────────────────────────────────────────┤
│ This is left aligned text              │             This is right aligned text │
├────────────────────────────────────────┼────────────────────────────────────────┤
│ This is left aligned text              │             This is right aligned text │
├────────────────────────────────────────┴────────────────────────────────────────┤
│ This is some really really really really really really really really really tha │
│ t is going to wrap to the next line                                             │
╰─────────────────────────────────────────────────────────────────────────────────╯
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn complex_table() {
        let mut table = Table::new();

        table.add_row(Row::new(vec![
            TableCell::new_with_col_span("Col*1*Span*2", 2),
            TableCell::new("Col 2 Span 1"),
            TableCell::new_with_col_span("Col 3 Span 2", 2),
            TableCell::new("Col 4 Span 1"),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new("Col 1 Span 1"),
            TableCell::new("Col 2 Span 1"),
            TableCell::new("Col 3 Span 1"),
            TableCell::new_with_col_span("Col 4 Span 1", 2),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new("fasdaff"),
            TableCell::new("fff"),
            TableCell::new("fff"),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("fasdff", 3, Alignment::Right),
            TableCell::new_with_col_span("fffdff", 4),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new("fasdsaff"),
            TableCell::new("fff"),
            TableCell::new("f\nf\nf\nfff\nrrr\n\n\n"),
        ]));
        table.add_row(Row::new(vec![TableCell::new("fasdsaff")]));

        let s = table.render().clone();

        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            s,
            3,
            Alignment::Left,
        )]));

        let expected = r"╔═════════════════════════════════════════════════════════╦════════════════════════════╦════════════════╦══════════════╦═══╗
║ Col*1*Span*2                                            ║ Col 2 Span 1               ║ Col 3 Span 2   ║ Col 4 Span 1 ║   ║
╠════════════════════════════╦════════════════════════════╬════════════════════════════╬════════════════╬══════════════╬═══╣
║ Col 1 Span 1               ║ Col 2 Span 1               ║ Col 3 Span 1               ║ Col 4 Span 1   ║              ║   ║
╠════════════════════════════╬════════════════════════════╬════════════════════════════╬═══════╦════════╬══════════════╬═══╣
║ fasdaff                    ║ fff                        ║ fff                        ║       ║        ║              ║   ║
╠════════════════════════════╩════════════════════════════╩════════════════════════════╬═══════╩════════╩══════════════╩═══╣
║                                                                               fasdff ║ fffdff                            ║
╠════════════════════════════╦════════════════════════════╦════════════════════════════╬═══════╦════════╦══════════════╦═══╣
║ fasdsaff                   ║ fff                        ║ f                          ║       ║        ║              ║   ║
║                            ║                            ║ f                          ║       ║        ║              ║   ║
║                            ║                            ║ f                          ║       ║        ║              ║   ║
║                            ║                            ║ fff                        ║       ║        ║              ║   ║
║                            ║                            ║ rrr                        ║       ║        ║              ║   ║
║                            ║                            ║                            ║       ║        ║              ║   ║
║                            ║                            ║                            ║       ║        ║              ║   ║
║                            ║                            ║                            ║       ║        ║              ║   ║
╠════════════════════════════╬════════════════════════════╬════════════════════════════╬═══════╬════════╬══════════════╬═══╣
║ fasdsaff                   ║                            ║                            ║       ║        ║              ║   ║
╠════════════════════════════╩════════════════════════════╩════════════════════════════╬═══════╬════════╬══════════════╬═══╣
║ ╔═════════════════════════════╦══════════════╦════════════════╦══════════════╦═══╗   ║       ║        ║              ║   ║
║ ║ Col*1*Span*2                ║ Col 2 Span 1 ║ Col 3 Span 2   ║ Col 4 Span 1 ║   ║   ║       ║        ║              ║   ║
║ ╠══════════════╦══════════════╬══════════════╬════════════════╬══════════════╬═══╣   ║       ║        ║              ║   ║
║ ║ Col 1 Span 1 ║ Col 2 Span 1 ║ Col 3 Span 1 ║ Col 4 Span 1   ║              ║   ║   ║       ║        ║              ║   ║
║ ╠══════════════╬══════════════╬══════════════╬═══════╦════════╬══════════════╬═══╣   ║       ║        ║              ║   ║
║ ║ fasdaff      ║ fff          ║ fff          ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ╠══════════════╩══════════════╩══════════════╬═══════╩════════╩══════════════╩═══╣   ║       ║        ║              ║   ║
║ ║                                     fasdff ║ fffdff                            ║   ║       ║        ║              ║   ║
║ ╠══════════════╦══════════════╦══════════════╬═══════╦════════╦══════════════╦═══╣   ║       ║        ║              ║   ║
║ ║ fasdsaff     ║ fff          ║ f            ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ║              ║              ║ f            ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ║              ║              ║ f            ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ║              ║              ║ fff          ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ║              ║              ║ rrr          ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ║              ║              ║              ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ║              ║              ║              ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ║              ║              ║              ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ╠══════════════╬══════════════╬══════════════╬═══════╬════════╬══════════════╬═══╣   ║       ║        ║              ║   ║
║ ║ fasdsaff     ║              ║              ║       ║        ║              ║   ║   ║       ║        ║              ║   ║
║ ╚══════════════╩══════════════╩══════════════╩═══════╩════════╩══════════════╩═══╝   ║       ║        ║              ║   ║
║                                                                                      ║       ║        ║              ║   ║
╚══════════════════════════════════════════════════════════════════════════════════════╩═══════╩════════╩══════════════╩═══╝
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn no_top_boarder() {
        let mut table = Table::new();
        table.style = TableStyle::simple();
        table.has_top_boarder = false;

        add_data_to_test_table(&mut table);

        let expected = r"|                            This is some centered text                           |
+----------------------------------------+----------------------------------------+
| This is left aligned text              |             This is right aligned text |
+----------------------------------------+----------------------------------------+
| This is left aligned text              |             This is right aligned text |
+----------------------------------------+----------------------------------------+
| This is some really really really really really really really really really tha |
| t is going to wrap to the next line                                             |
+---------------------------------------------------------------------------------+
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn no_bottom_boarder() {
        let mut table = Table::new();
        table.style = TableStyle::simple();
        table.has_bottom_boarder = false;

        add_data_to_test_table(&mut table);

        let expected = r"+---------------------------------------------------------------------------------+
|                            This is some centered text                           |
+----------------------------------------+----------------------------------------+
| This is left aligned text              |             This is right aligned text |
+----------------------------------------+----------------------------------------+
| This is left aligned text              |             This is right aligned text |
+----------------------------------------+----------------------------------------+
| This is some really really really really really really really really really tha |
| t is going to wrap to the next line                                             |
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn no_separators() {
        let mut table = Table::new();
        table.style = TableStyle::simple();
        table.separate_rows = false;

        add_data_to_test_table(&mut table);

        let expected = r"+---------------------------------------------------------------------------------+
|                            This is some centered text                           |
| This is left aligned text              |             This is right aligned text |
| This is left aligned text              |             This is right aligned text |
| This is some really really really really really really really really really tha |
| t is going to wrap to the next line                                             |
+---------------------------------------------------------------------------------+
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn some_rows_no_separators() {
        let mut table = Table::new();
        table.style = TableStyle::simple();

        add_data_to_test_table(&mut table);

        table.rows[2].has_separator = false;

        let expected = r"+---------------------------------------------------------------------------------+
|                            This is some centered text                           |
+----------------------------------------+----------------------------------------+
| This is left aligned text              |             This is right aligned text |
| This is left aligned text              |             This is right aligned text |
+----------------------------------------+----------------------------------------+
| This is some really really really really really really really really really tha |
| t is going to wrap to the next line                                             |
+---------------------------------------------------------------------------------+
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    #[test]
    fn colored_data_works() {
        let mut table = Table::new();
        table.add_row(Row::new(vec![TableCell::new("\u{1b}[31ma\u{1b}[0m")]));
        let expected = "╔═══╗
║ \u{1b}[31ma\u{1b}[0m ║
╚═══╝
";
        println!("{}", table.render());
        assert_eq!(expected, table.render());
    }

    fn add_data_to_test_table(table: &mut Table) {
        table.max_column_width = 40;
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "This is some centered text",
            2,
            Alignment::Center,
        )]));

        table.add_row(Row::new(vec![
            TableCell::new(&"This is left aligned text"),
            TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new("This is left aligned text"),
            TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_col_span("This is some really really really really really really really really really that is going to wrap to the next line", 2),
        ]));
    }
}
