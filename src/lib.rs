pub mod row;
pub mod cell;

use row::Row;

use std::cmp::max;

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
    pub column_titles: Vec<String>,
    pub rows: Vec<Row<'data>>,
    pub style: TableStyle,
}

impl<'data> Table<'data> {
    pub fn new() -> Table<'data> {
        return Table {
            column_titles: Vec::new(),
            rows: Vec::new(),
            style: TableStyle::extended(),
        };
    }

    pub fn add_row(&mut self, row: Row<'data>) {
        self.rows.push(row);
    }

    pub fn as_string(&mut self) -> String{
        let mut print_buffer = String::new();
        let max_widths = self.calculate_max_column_widths();
        let mut previous_separator = None;
        if self.rows.len() > 0 {
            for i in 0..self.rows.len() {
                let mut row_pos = RowPosition::Mid;
                if i == 0 {
                    row_pos = RowPosition::First;
                }
                let separator = self.rows[i].get_separator(
                    &max_widths,
                    &self.style,
                    row_pos,
                    previous_separator.clone(),
                );
                Table::buffer_line(&mut print_buffer, &separator);
                Table::buffer_line(
                    &mut print_buffer,
                    &self.format_row(&self.rows[i], &max_widths),
                );
                previous_separator = Some(separator.clone());
            }
            let separator = self.rows.last().unwrap().get_separator(
                &max_widths,
                &self.style,
                RowPosition::Last,
                None,
            );
            Table::buffer_line(&mut print_buffer, &separator);
        }
        return print_buffer;
    }

    fn format_row(&self, row: &Row<'data>, max_widths: &Vec<usize>) -> String {
        let mut buf = String::new();

        let mut spanned_columns = 0;

        for i in 0..max_widths.len() {
            if row.cells.len() > i {
                let mut cell_span = 0;
                let cell = &row.cells[i];

                for c in 0..cell.col_span {
                    cell_span += max_widths[spanned_columns + c];
                }
                let mut padding = 0;
                if cell_span > cell.width() {
                    padding += cell_span - cell.width();
                    if cell.col_span > 1 {
                        padding += cell.col_span - 1;
                    }
                }
                buf.push_str(
                    format!(
                        "{}{}{}",
                        self.style.vertical,
                        cell,
                        str::repeat(" ", padding)
                    ).as_str(),
                );
                spanned_columns += cell.col_span;
            } else {
                buf.push_str(
                    format!(
                        "{}{}",
                        self.style.vertical,
                        str::repeat(" ", max_widths[spanned_columns])
                    ).as_str(),
                );
                spanned_columns += 1;
            }
            if spanned_columns == max_widths.len() {
                break;
            }
        }

        buf.push(self.style.vertical);

        return buf;
    }

    fn calculate_max_column_widths(&self) -> Vec<usize> {
        let mut num_columns = 0;

        for row in &self.rows {
            num_columns = max(row.num_columns(), num_columns);
        }

        let mut max_widths: Vec<usize> = vec![0; num_columns];
        for row in &self.rows {
            let column_widths = row.adjusted_column_widths();
            for i in 0..column_widths.len() {
                max_widths[i] = max(max_widths[i], column_widths[i] as usize);
            }
        }
        return max_widths;
    }

    fn buffer_line(buffer: &mut String, line: &String) {
        buffer.push_str(format!("{}\n", line).as_str());
    }
}

#[cfg(test)]
mod test{

    use cell::Cell;
    use row::Row;
    use Table;

    #[test]
    fn complex_table(){
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("asdasff", 2),
            Cell::new("ffdasdasdasff", 1),
            Cell::new("ffqqqqdasdasffr", 2),
            Cell::new("ffdasdasdasff", 1),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("fasdsadff", 1),
            Cell::new("fffedddde", 1),
            Cell::new("fff", 1),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("fasdaff", 1),
            Cell::new("fff", 1),
            Cell::new("fff", 1),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("fasdff", 3),
            Cell::new("fffdff", 4),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("fasdsaff", 1),
            Cell::new("fff", 1),
            Cell::new("fff", 1),
        ]));
        table.add_row(Row::new(vec![Cell::new("fasdsaff", 1)]));
        table.add_row(Row::new(vec![Cell::new("fasdsaff", 15)]));
        println!("{}", table.as_string());
    }
}