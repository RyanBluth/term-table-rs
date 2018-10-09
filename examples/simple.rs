extern crate term_table;
use term_table::{Table,TableStyle};
use term_table::{row::Row,
    cell::{Alignment,Cell},
};
fn main(){
    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::elegant();

    table.add_row(Row::new(vec![
        Cell::new_with_alignment("This is some centered text", 2, Alignment::Center)
    ]));

    table.add_row(Row::new(vec![
        Cell::new("This is left aligned text", 1),
        Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
    ]));

    table.add_row(Row::new(vec![
        Cell::new("This is left aligned text", 1),
        Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
    ]));

    table.add_row(Row::new(vec![
        Cell::new("This is some really really really really really really really really really that is going to wrap to the next line", 2),
    ]));

    println!("{}", table.as_string());
}
