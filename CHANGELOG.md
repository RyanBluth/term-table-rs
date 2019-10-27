# 1.2.0
Switched to unicode-width from wcwidth since it has been yanked

# 1.1.0
Added options for disabling row seperators

# 1.0.0

Cleaned up code and added comments

Renamed `cel::Cell` to `table_cell::TableCell`

Fixed clippy lints


# 0.1.6

New table styles have been added 

# 0.1.5

Ansi characters are now stripped off when determining string lenghts

# 0.1.4

Cell new methods now take ToString for data as opposed to Into<Cow<'data, &str>>

# 0.1.3

Actually reverted experimental usage of impl trait in `cell::Cell::new`, `cell::Cell::new_with_alignment` and `cell::Cell::new_with_alignment_and_padding` 

# 0.1.2

Reverted experimental usage of impl trait in `cell::Cell::new`, `cell::Cell::new_with_alignment` and `cell::Cell::new_with_alignment_and_padding` 

# 0.1.1

Added repository link to cargo.toml

# 0.1.0

Initial Release