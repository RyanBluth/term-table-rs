# term-table-rs

### The purpose of term-table-rs is to make displaying table data in CLI apps easier

![example](https://i.imgur.com/XwIzWkU.png)


### Here is an example of how to create a table

```rust
let mut table = Table::new();
table.max_column_width = 40;

table.style = TableStyle::extended(); 

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

```
### Here's the result

![extended style](https://i.imgur.com/NHEg0Sf.png)

## Table Styles

It is possible to define your own table styles by creating a new instance of `TableStyle`

This is what the extend table style implementation looks like. This is the defualy style in term-table-rs

```rust
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
```

`TableStyle` also implements a `simple()` table style function and a `blank()` table style function

Those styles looks like this

### Blank

![blank style](https://i.imgur.com/HaKgXQj.png)


### Simple

![simple style](https://i.imgur.com/kGqlYD7.png)


## Column Widths

It is possible to control the maximum width of table columns. The `max_column_width` property of `Table` can be set to restrict the width of all cells. The `set_max_column_width` function of `Table` can be used to set the max width of a specific column. The `set_max_column_widths` function provides the ability to set the width of multiple columns by passing in a `Vec` of tuples containing an index and width.6