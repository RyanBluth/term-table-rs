use cell::Cell;
use {RowPosition, TableStyle};
use std::str::FromStr;
use std::cmp::{max, min};

pub struct Row<'data> {
    pub cells: Vec<Cell<'data>>,
}

impl<'data> Row<'data> {
    pub fn new<T>(cells: Vec<T>) -> Row<'data>
    where
        T: Into<Cell<'data>>,
    {
        let mut row = Row { cells: vec![] };

        for entry in cells {
            row.cells.push(entry.into());
        }

        return row;
    }

    pub fn format(&self, max_widths: &Vec<usize>, style: &TableStyle) -> String {
        let mut buf = String::new();

        let mut spanned_columns = 0;

        let mut max_row_span = 0;
        let mut wrapped_cells = Vec::new();

        for i in 0..self.cells.len() {
            let cell = &self.cells[i];
            let mut width = 0;
            for j in 0..cell.col_span {
                width += max_widths[j + spanned_columns];
            }
            let wrapped_cell = cell.wrap_to_width(width + cell.col_span - 1);
            max_row_span = max(max_row_span, wrapped_cell.len());
            wrapped_cells.push(wrapped_cell);
            spanned_columns += cell.col_span;
        }

        spanned_columns = 0;

        let mut lines = vec![String::new(); max_row_span];

        for i in 0..max_widths.len() {
            if self.cells.len() > i {
                let mut cell_span = 0;
                let cell = &self.cells[i];
                for c in 0..cell.col_span {
                    cell_span += max_widths[spanned_columns + c];
                }
                for h in 0..max_row_span {
                    if wrapped_cells[i].len() > h {
                        let mut padding = 0;
                        if cell_span > wrapped_cells[i][h].chars().count() {
                            padding += cell_span - wrapped_cells[i][h].chars().count();
                            if cell.col_span > 1 {
                                padding += cell.col_span - 1;
                            }
                        }
                        lines[h].push_str(
                            format!(
                                "{}{}{}",
                                style.vertical,
                                wrapped_cells[i][h],
                                str::repeat(" ", padding)
                            ).as_str(),
                        );
                    } else {
                        lines[h].push_str(
                            format!(
                                "{}{}",
                                style.vertical,
                                str::repeat(
                                    " ",
                                    max_widths[spanned_columns] * cell.col_span + cell.col_span - 1
                                )
                            ).as_str(),
                        );
                    }
                }
                spanned_columns += cell.col_span;
            } else {
                for h in 0..max_row_span {
                    lines[h].push_str(
                        format!(
                            "{}{}",
                            style.vertical,
                            str::repeat(" ", max_widths[spanned_columns])
                        ).as_str(),
                    );
                }
                spanned_columns += 1;
            }
            if spanned_columns == max_widths.len() {
                break;
            }
        }
        for line in &lines {
            buf.push_str(line.clone().as_str());
            buf.push(style.vertical);
            buf.push('\n');
        }
        buf.pop();
        return buf;
    }

    pub fn gen_separator(
        &self,
        max_widths: &Vec<usize>,
        style: &TableStyle,
        row_position: RowPosition,
        previous_separator: Option<String>,
    ) -> String {
        let mut buf = String::new();

        // If the first cell has a col_span > 1 we need to set the next
        // intersection point to that value
        let mut next_intersection = match self.cells.first() {
            Some(cell) => cell.col_span,
            None => 1,
        };

        // Push the initial char for the row
        buf.push(style.start_for_position(row_position));

        let mut current_column = 0;

        for i in 0..max_widths.len() {
            if i == next_intersection {
                // Draw the intersection character for the start of the column
                buf.push(style.intersect_for_position(row_position));

                current_column += 1;

                // If we still have remaining cells then we use the col_span to determine
                // when the next intersection character should be drawn
                if self.cells.len() > current_column {
                    next_intersection += self.cells[current_column].col_span;
                } else {
                    // Otherwise we just draw an intersection for every column
                    next_intersection += 1;
                }
            } else if i > 0 {
                // This means the current cell has a col_span > 1
                buf.push(style.horizontal);
            }
            // Fill in all of the horizontal space
            buf.push_str(
                str::repeat(style.horizontal.to_string().as_str(), max_widths[i]).as_str(),
            );
        }

        buf.push(style.end_for_position(row_position));

        let mut out = String::new();

        // Merge the previous seperator string with the current buffer
        // This will handle cases where a cell above/below has a different col_span value
        return match previous_separator {
            Some(prev) => {
                for pair in buf.chars().zip(prev.chars()) {
                    if pair.0 == style.outer_left_vertical || pair.0 == style.outer_right_vertical {
                        // Always take the start and end characters of the current buffer
                        out.push(pair.0);
                    } else if pair.0 != style.horizontal || pair.1 != style.horizontal {
                        out.push(style.merge_intersection_for_position(
                            pair.1,
                            pair.0,
                            row_position,
                        ));
                    } else {
                        out.push(style.horizontal);
                    }
                }
                out
            }
            None => buf,
        };
    }

    pub fn adjusted_column_widths(&self) -> Vec<f32> {
        let mut res = Vec::new();
        for cell in &self.cells {
            let val = cell.split_width();
            for _ in 0..cell.col_span {
                res.push(val);
            }
        }
        return res;
    }

    pub fn num_columns(&self) -> usize {
        return self.cells.iter().map(|x| x.col_span).sum();
    }
}
