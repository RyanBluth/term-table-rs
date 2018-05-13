# term-table-rs

![example](https://i.imgur.com/XwIzWkU.png)

The purpose of term-table-rs is to make displaying table data in CLI apps easier

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

<pre>
 ╔═════════════════════════════════════════════════════════════════════════════════╗
 ║                            This is some centered text                           ║
 ╠════════════════════════════════════════╦════════════════════════════════════════╣
 ║ This is left aligned text              ║             This is right aligned text ║
 ╠════════════════════════════════════════╬════════════════════════════════════════╣
 ║ This is left aligned text              ║             This is right aligned text ║
 ╠════════════════════════════════════════╩════════════════════════════════════════╣
 ║ This is some really really really really really really really really really tha ║
 ║ t is going to wrap to the next line                                             ║
 ╚═════════════════════════════════════════════════════════════════════════════════╝
</pre>
