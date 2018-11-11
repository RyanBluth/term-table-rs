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
    pub fn format(&self, column_widths: &Vec<usize>, style: &TableStyle) -> String {
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
                width += column_widths[j + spanned_columns];
            }
            // Wrap to the total width - col_span to account for separators
            let wrapped_cell = cell.wrapped_content(width + cell.col_span - 1);
            row_height = max(row_height, wrapped_cell.len());
            wrapped_cells.push(wrapped_cell);
            spanned_columns += cell.col_span;
        }

        // reset spanned_columns so we can reuse it in the next loop
        spanned_columns = 0;

        // Row lines to combine into the final string at the end
        let mut lines = vec![String::new(); row_height];

        // We need to iterate over all of the column widths
        // We may not have as many cells as column widths, or the cells may not even span
        // as many columns as are in column widths. In that case weill will create empty cells
        for col_idx in 0..column_widths.len() {
            // Check to see if we actually have a cell for the column index
            // Otherwise we will just need to print out empty space as filler
            if self.cells.len() > col_idx {
                // Number of characters spanned by column
                let mut cell_span = 0;
            
                // Get the cell using the column index
                // 
                // This is a little bit confusing because cells and columns aren't always one to one
                // We may have fewer cells than columns or some cells may span multiple columns
                // If there are fewer cells than columns we just end drawing empty cells in the else block
                // If there are fewer cells than columns but they span the total number of columns we just break out
                // of the outer for loop at the end. We know how many cells we've spanned by adding the cell's col_span to spanned_columns
                let cell = &self.cells[col_idx];
                // Calculate the cell span by adding up the widths of the columns spanned by the cell
                for c in 0..cell.col_span {
                    cell_span += column_widths[spanned_columns + c];
                }
                // Since cells can wrap we need to loop over all of the lines
                for line_idx in 0..row_height {
                    // Check to see if the wrapped cell has a line for the line index
                    if wrapped_cells[col_idx].len() > line_idx {
                        // We may need to pad the cell if it's contents are not as wide as some other cell in the column
                        let mut padding = 0;
                        // We need to calculate the string_width because some characters take up extra space and we need to 
                        // ignore ANSI characters
                        let str_width = string_width(&wrapped_cells[col_idx][line_idx]);
                        if cell_span >= str_width {
                            padding += cell_span - str_width;
                            // If the cols_span is greater than one we need to add extra padding for the missing vertical characters
                            if cell.col_span > 1 {
                                padding += char_width(style.vertical).unwrap_or_default() as usize
                                    * (cell.col_span - 1); // Subtract one since we add a vertical character to the beginning
                            }
                        }
                        // Finally we can push the string into the lines vec
                        lines[line_idx].push_str(
                            format!(
                                "{}{}",
                                style.vertical,
                                self.pad_string(padding, cell.alignment, &wrapped_cells[col_idx][line_idx])
                            ).as_str(),
                        );
                    } else {
                        // If the cell doesn't have any content for this line just fill it with empty space
                        lines[line_idx].push_str(
                            format!(
                                "{}{}",
                                style.vertical,
                                str::repeat(
                                    " ",
                                    column_widths[spanned_columns] * cell.col_span + cell.col_span - 1
                                )
                            ).as_str(),
                        );
                    }
                }
                // Keep track of how many columns we have actually spanned since
                // cells can be wider than a single column
                spanned_columns += cell.col_span;
            } else {
                // If we don't have a cell for the coulumn then we just create an empty one
                for line in 0..row_height {
                    lines[line].push_str(
                        format!(
                            "{}{}",
                            style.vertical,
                            str::repeat(" ", column_widths[spanned_columns])
                        ).as_str(),
                    );
                }
                // Add one to the spanned column since the empty space is basically a cell
                spanned_columns += 1;
            }
            // If we have spanned as many columns as there are then just break out of the loop
            if spanned_columns == column_widths.len() {
                break;
            }
        }
        // Finally add all the lines together to create the row content
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
        column_widths: &Vec<usize>,
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

        for i in 0..column_widths.len() {
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
                str::repeat(style.horizontal.to_string().as_str(), column_widths[i]).as_str(),
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
