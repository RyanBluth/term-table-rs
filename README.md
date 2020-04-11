<h1 align="center" >term-table</h1>

<div align="center">
 <strong>
   CLI Tables Made Easy
 </strong>
</div>

## Example

```rust
let mut table = Table::new();
table.max_column_width = 40;

table.style = TableStyle::extended(); 

table.add_row(Row::new(vec![
    TableCell::new_with_alignment("This is some centered text", 2, Alignment::Center)
])); 

table.add_row(Row::new(vec![
    TableCell::new("This is left aligned text", 1),
    TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
]));

table.add_row(Row::new(vec![
    TableCell::new("This is left aligned text", 1),
    TableCell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
]));

table.add_row(Row::new(vec![
    TableCell::new("This is some really really really really really really really really really that is going to wrap to the next line", 2),
]));   

println!("{}", table.render());

```
### Here's the result

![extended style](https://i.imgur.com/NHEg0Sf.png)

### Using TableBuilder

```rust
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

```


## Table Styles

It is possible to define your own table styles by creating a new instance of `TableStyle`

This is what the extend table style implementation looks like. This is the default style in term-table-rs

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

It is possible to control the maximum width of table columns. The `max_column_width` property of `Table` can be set to restrict the width of all TableCells. The `set_max_column_width` function of `Table` can be used to set the max width of a specific column. The `set_max_column_widths` function provides the ability to set the width of multiple columns by passing in a `Vec` of tuples containing an index and width.

## Disabling Row Separators

There are a few different options for disabling row separation. 

`Table` has three flags for controlling row separation:
1.  `separate_rows` dictates whether rows are separated within the table 
    
    ![separate_rows](https://i.imgur.com/a8nAg5o.png)

2.  `has_top_boarder` dictates whether or not the table has a top border

    ![has_top_boarder](https://i.imgur.com/336tbDm.png)

3.  `has_bottom_boarder` dictates whether or not the table has a bottom border

    ![has_bottom_boarder](https://i.imgur.com/C0ETZFi.png)

Separators can also be controlled per row by setting the `has_separator` flag on `Row`

![has_separator](https://i.imgur.com/VAZJnC7.png)