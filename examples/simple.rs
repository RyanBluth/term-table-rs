extern crate term_table;
use term_table::{Table,TableStyle};
use term_table::{row::Row,
    table_cell::{Alignment,TableCell},
};
fn main(){
    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::elegant();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("This is some centered text", 2, Alignment::Center)
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("This is left aligned text"),
        TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("This is left aligned text"),
        TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
    ]));

    table.add_row(Row::new(vec![
        TableCell::new_with_col_span("This is some really really really really really really really really really that is going to wrap to the next line", 2),
    ]));

    println!("{}", table.render());
}
