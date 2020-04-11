extern crate term_table;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
};
use term_table::{TableBuilder, TableStyle};

fn main() {
    let table = TableBuilder::new().style(TableStyle::extended()).rows(
        vec![
            Row::new(vec![
                TableCell::new_with_alignment("This is some centered text", 2, Alignment::Center)
            ]),
            Row::new(vec![
                TableCell::new("This is left aligned text"),
                TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
            ]),
            Row::new(vec![
                TableCell::new("This is left aligned text"),
                TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
            ]),
                Row::new(vec![
                TableCell::new_with_col_span("This is some really really really really really really really really really that is going to wrap to the next line", 2),
            ]),
        ]
    ).build();

    println!("{}", table.render());
}

