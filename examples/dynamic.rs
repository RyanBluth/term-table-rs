use rand::Rng;
use term_table::{row, row::Row, rows, table_cell::*, Table, TableStyle};

fn main() {
    let mut rng = rand::thread_rng();
    let num_draws = 5;
    let num_numbers = 6;
    let range = 1..=99;

    let mut table = Table::builder()
        .rows(rows![row!(TableCell::builder("My Lucky Numbers")
            .alignment(Alignment::Center)
            .col_span(num_numbers))])
        .style(TableStyle::elegant())
        .build();

    for _ in 0..num_draws {
        let mut row = Row::empty();
        for _ in 0..num_numbers {
            let num: i32 = rng.gen_range(range.clone());
            row.add_cell(TableCell::new(num.to_string()));
        }
        table.add_row(row);
    }

    println!("\n{}", table.render());
}
