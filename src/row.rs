use cell::{string_width, Alignment, Cell};
use std::cmp::max;
use wcwidth::char_width;
use {RowPosition, TableStyle};

/// A set of table cells
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

    /// Formats a row based on the provided table style
    pub fn format(&self, max_widths: &Vec<usize>, style: &TableStyle) -> String {
        let mut buf = String::new();

        // Since a cell can span multiple columns we need to track
        // how many columns we have actually spand. We cannot just depend
        // on the index of the current cell when iterating
        let mut spanned_columns = 0;

        // The height of the row determined by how many times a cell had to wrap
        let mut row_height = 0;

        // Wrapped cell content
        let mut wrapped_cells = Vec::new();

        // The first thing we do is wrap the cells if their
        // content is greater than the max width of the column they are in
        for cell in &self.cells {
            let mut width = 0;
            // Iterate from 0 to the cell's col_span and add up all the max width
            // values for each column so we can properly pad the cell content later
            for j in 0..cell.col_span {
                width += max_widths[j + spanned_columns];
            }
            // Wrap to the total width - col_span to account for separators
            let wrapped_cell = cell.wrapped_content(width + cell.col_span - 1);
            row_height = max(row_height, wrapped_cell.len());
            wrapped_cells.push(wrapped_cell);
            spanned_columns += cell.col_span;
        }

        spanned_columns = 0;

        let mut lines = vec![String::new(); row_height];

        for i in 0..max_widths.len() {
            if self.cells.len() > i {
                let mut cell_span = 0;
                let cell = &self.cells[i];
                for c in 0..cell.col_span {
                    cell_span += max_widths[spanned_columns + c];
                }
                for h in 0..row_height {
                    if wrapped_cells[i].len() > h {
                        let mut padding = 0;
                        let str_width = string_width(&wrapped_cells[i][h]);
                        if cell_span >= str_width {
                            padding += cell_span - str_width;
                            if cell.col_span > 1 {
                                padding += char_width(style.vertical).unwrap_or_default() as usize
                                    * (cell.col_span - 1);
                            }
                        }
                        lines[h].push_str(
                            format!(
                                "{}{}",
                                style.vertical,
                                self.pad_string(padding, cell.alignment, &wrapped_cells[i][h])
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
                for h in 0..row_height {
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

    /// Generates the top separator for a row.
    ///
    /// The previous seperator is used to determine junction characters
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

    /// Returns a vector of split cell widths.
    ///
    /// A split width is the cell's total width divided by it's col_span value.
    ///
    /// Each cell's split width value is pushed into the resulting vector col_span times.
    /// Returns a vec of tuples containing the cell width and the min cell width
    pub fn split_column_widths(&self) -> Vec<(f32, usize)> {
        let mut res = Vec::new();
        for cell in &self.cells {
            let val = cell.split_width();

            let min = (cell.min_width() as f32 / cell.col_span as f32) as usize;

            for _ in 0..cell.col_span {
                res.push((val, min));
            }
        }
        return res;
    }

    /// Number of columns in the row.
    ///
    /// This is the sum of all cell's col_span values
    pub fn num_columns(&self) -> usize {
        return self.cells.iter().map(|x| x.col_span).sum();
    }

    /// Pads a string accoding to the provided alignment
    fn pad_string(&self, padding: usize, alignment: Alignment, text: &String) -> String {
        match alignment {
            Alignment::Left => return format!("{}{}", text, str::repeat(" ", padding)),
            Alignment::Right => return format!("{}{}", str::repeat(" ", padding), text),
            Alignment::Center => {
                let half_padding = padding as f32 / 2.0;
                return format!(
                    "{}{}{}",
                    str::repeat(" ", half_padding.ceil() as usize),
                    text,
                    str::repeat(" ", half_padding.floor() as usize)
                );
            }
        }
    }
}
