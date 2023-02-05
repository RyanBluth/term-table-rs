extern crate term_table;
use term_table::{
    row,
    row::Row,
    rows,
    table_cell::{Alignment, TableCell},
};
use term_table::{Table, TableStyle};
fn main() {
    let table = Table::builder()
    .separate_rows(false)
    .style(TableStyle::elegant())
    .max_column_width(80)
    .rows(rows![
        row![
            TableCell::builder("This is some centered text")
            .col_span(2)
            .alignment(Alignment::Center)
        ],
        row![
            TableCell::builder("This is left aligned text"),
            TableCell::builder("This is right aligned text")
                .alignment(Alignment::Right)
        ],
        row![
            TableCell::builder("This is left aligned text"),
            TableCell::builder("This is right aligned text")
                .alignment(Alignment::Right)
        ],
        row![
            TableCell::builder("This is some really really really really really really really really really that is going to wrap to the next line")
            .col_span(2)
        ],
    ])
    .build();

    println!("{}", table.render());
}
