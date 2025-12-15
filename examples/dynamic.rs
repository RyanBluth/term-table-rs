use term_table::{row, row::Row, rows, table_cell::*, Table, TableStyle};

fn main() {
    let num_numbers = 6;

    let mut table = Table::builder()
        .rows(rows![row!(TableCell::builder("My Lucky Numbers")
            .alignment(Alignment::Center)
            .col_span(num_numbers))])
        .style(TableStyle::elegant())
        .build();

    let draws = [
        [1, 2, 3, 4, 5, 6],
        [7, 8, 9, 10, 11, 12],
        [13, 14, 15, 16, 17, 18],
        [19, 20, 21, 22, 23, 24],
        [25, 26, 27, 28, 29, 30],
    ];

    for draw in draws.iter() {
        let mut row = Row::empty();
        for num in draw.iter() {
            row.add_cell(TableCell::new(num.to_string()));
        }
        table.add_row(row);
    }

    println!("\n{}", table.render());
}
