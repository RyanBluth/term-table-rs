pub mod row;
pub mod cell;

use row::Row;

use std::cmp::{max, min};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum RowPosition {
    First,
    Mid,
    Last,
}

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

    fn start_for_position(&self, pos: RowPosition) -> char {
        match pos {
            RowPosition::First => self.top_left_corner,
            RowPosition::Mid => self.outer_left_vertical,
            RowPosition::Last => self.bottom_left_corner,
        }
    }

    fn end_for_position(&self, pos: RowPosition) -> char {
        match pos {
            RowPosition::First => self.top_right_corner,
            RowPosition::Mid => self.outer_right_vertical,
            RowPosition::Last => self.bottom_right_corner,
        }
    }

    fn intersect_for_position(&self, pos: RowPosition) -> char {
        match pos {
            RowPosition::First => self.outer_top_horizontal,
            RowPosition::Mid => self.intersection,
            RowPosition::Last => self.outer_bottom_horizontal,
        }
    }

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

pub struct Table<'data> {
    pub rows: Vec<Row<'data>>,
    pub style: TableStyle,
    pub max_column_width: usize,
}

impl<'data> Table<'data> {
    pub fn new() -> Table<'data> {
        return Table {
            rows: Vec::new(),
            style: TableStyle::extended(),
            max_column_width: std::usize::MAX,
        };
    }

    pub fn add_row(&mut self, row: Row<'data>) {
        self.rows.push(row);
    }

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

    /// Calculates the maximum width for each column
    /// If a cell has a column span greater than 1, then the width
    /// of it's contents are divided by the column span, otherwise the cell
    /// would use more space than it needed
    fn calculate_max_column_widths(&self) -> Vec<usize> {
        let mut num_columns = 0;

        for row in &self.rows {
            num_columns = max(row.num_columns(), num_columns);
        }

        let mut max_widths: Vec<usize> = vec![0; num_columns];
        for row in &self.rows {
            let column_widths = row.adjusted_column_widths();
            for i in 0..column_widths.len() {
                max_widths[i] = min(
                    self.max_column_width,
                    max(max_widths[i], column_widths[i] as usize),
                );
            }
        }
        return max_widths;
    }

    fn buffer_line(buffer: &mut String, line: &String) {
        buffer.push_str(format!("{}\n", line).as_str());
    }
}

#[cfg(test)]
mod test {

    use cell::{Alignment, Cell};
    use row::Row;
    use Table;

    #[test]
    fn complex_table() {
        let mut table = Table::new();
        table.max_column_width = 5;
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
            Cell::new("fff", 1),
        ]));
        table.add_row(Row::new(vec![Cell::new("fasdsaff", 1)]));
        table.add_row(Row::new(vec![
            Cell::new_with_alignment("fasdsaff", 15, Alignment::Center),
        ]));
        println!("{}", table.as_string());
    }
}
