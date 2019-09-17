use std::fmt::{Display};
use std::fmt::Formatter;
use std::fmt::Error;

/// The greatest number possible in the sudoku field.
/// The smallest number is hardcoded as 1.
pub const MAX_NUM: usize = 9;
/// Height and width of the sudoku field.
pub const SIZE: usize = MAX_NUM;
/// height and width of one sudoku block. The sudoku field
/// consists of 3*3 blocks.
pub const BLOCK_SIZE: usize = 3; //sqrt(SIZE)

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Cell {
    Known(i32),
    Empty,
    Possible(Vec<i32>),
}

/// If the # flag was specified, Cell::Possible is displayed.
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // Amount of ' ' to print
        let width: usize = *f.width().get_or_insert(1);
        
        match self {
            Cell::Known(num) => i32::fmt(num, f),
            Cell::Empty => " ".repeat(width).fmt(f),
            Cell::Possible(vec) => {
                // If the # flag was specified.
                if f.alternate() {
                    // print leading spaces
                    write!(f, "{}", " ".repeat(usize::saturating_sub(width, vec.len())))?;
                    for num in vec {
                        write!(f, "{}", num)?;
                    }
                    Ok(())
                } else {
                    " ".repeat(width).fmt(f)
                }
            },
        }
    }
}

/// The field consists of an array
/// of lines that consist of an array
/// of cells.
/// Axises: `field[y][x]`
pub struct Field {
    pub cells: Vec<Vec<Cell>>
}

impl Field {
    pub fn new_with(cells: Vec<Vec<Cell>>) -> Field {
        Field {
            cells
        }
    }
    pub fn empty() -> Field {
        let mut cells = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            // Initialise with `Cell::Empty`
            let mut line = Vec::with_capacity(SIZE);
            line.extend((0..SIZE).map(|_| Cell::Empty));
            cells.push(line);
        }
        
        Field { cells }
    }
}

/// If the # flag was specified, Cell::Possible is displayed.
impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "\n+-----+-----+-----+")?;
        //writeln!(f, "|1 2 3|4 5 6|7 8 9|");
        
        for (i, line) in (&self.cells).iter().enumerate() {
            write!(f, "|")?;
            for (j, cell) in line.iter().enumerate() {
                // This passes the Formatter arguments. write!() doesn't.
                cell.fmt(f)?;
                
                if j % 3 == 2 {
                    write!(f, "|")?;
                } else { write!(f, " ")?; }
            }
            writeln!(f, "")?;
            if i % 3 == 2 { writeln!(f, "+-----+-----+-----+")?; }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::csv_parser::parse_csv;
    use crate::csv_parser::EXAMPLE;
    use crate::field::{SIZE, Cell};
    use crate::field::MAX_NUM;
    use crate::field::BLOCK_SIZE;
    use crate::field::Field;
    use std::mem;
    
    #[test]
    fn display_test() {
        let field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        println!("{}", field);
        assert_eq!(format!("{}", field), "
+-----+-----+-----+
|  5  |9    |3 7  |
|1 8 9|  4  |  6 5|
|3    |     |  4  |
+-----+-----+-----+
|     |  3  |    6|
|  9  |6 8 5|  2  |
|5    |  9  |     |
+-----+-----+-----+
|  4  |     |    1|
|9 3  |  6  |7 8 4|
|  1 8|    3|  5  |
+-----+-----+-----+
")
    }
    
    #[test]
    /// tests the # flag and width in the format string
    fn alternate_display_test() {
        let mut field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        // Overwrite some cells
        field.cells[0][0] = Cell::Possible(vec![1,2,3,4,5]);
        field.cells[2][5] = Cell::Possible(vec![3,1,4]);
        field.cells[1][1] = Cell::Possible(vec![]);
        // this one doesn't fit. That doesn't look so pretty.
        field.cells[8][8] = Cell::Possible(vec![1,2,3,4,5,6,7,8,9]);
        
        assert_eq!(dbg!(format!("{:#5}", field)), "
+-----+-----+-----+
|12345     5      |    9            |    3     7      |
|    1           9|          4      |          6     5|
|    3            |              314|          4      |
+-----+-----+-----+
|                 |          3      |                6|
|          9      |    6     8     5|          2      |
|    5            |          9      |                 |
+-----+-----+-----+
|          4      |                 |                1|
|    9     3      |          6      |    7     8     4|
|          1     8|                3|          5 123456789|
+-----+-----+-----+
");
    }
    
    #[test]
    fn constant_assertions() {
        assert_eq!(MAX_NUM, SIZE);
        assert_eq!(BLOCK_SIZE as f64, (SIZE as f64).sqrt());
        
        // mem::swap on fields should be very cheep,
        // because it should be heap-allocated.
        //println!("Size of `Field` is {}", mem::size_of::<Field>())
        assert_eq!(mem::size_of::<Field>(), 24) //three times usize
    }
}


