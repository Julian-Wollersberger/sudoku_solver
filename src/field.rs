use std::fmt::Display;
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

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Cell::Known(num) => write!(f, "{}", num),
            Cell::Possible(_) => write!(f, " "),
            Cell::Empty => write!(f, " "),
        }
    }
}

/// The field consists of an array
/// of lines that consist of an array
/// of cells.
/// `field[y][x]`
pub struct Field {
    pub cells: Vec<Vec<Cell>>
}

impl Field {
    pub fn new_with(cells: Vec<Vec<Cell>>) -> Field {
        Field {
            cells
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "\n+-----+-----+-----+");
        //writeln!(f, "|1 2 3|4 5 6|7 8 9|");
        
        for (i, line) in (&self.cells).iter().enumerate() {
            write!(f, "|");
            for (j, cell) in line.iter().enumerate() {
                write!(f, "{}", cell);
                
                if j % 3 == 2 {
                    write!(f, "|");
                } else { write!(f, " "); }
            }
            write!(f, "\n");
            if i % 3 == 2 { writeln!(f, "+-----+-----+-----+"); }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::csv_parser::parse_csv;
    use crate::csv_parser::EXAMPLE;
    use crate::field::SIZE;
    use crate::field::MAX_NUM;
    use crate::field::BLOCK_SIZE;
    
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
    fn constant_assertions() {
        assert_eq!(MAX_NUM, SIZE);
        assert_eq!(BLOCK_SIZE as f64, (SIZE as f64).sqrt());
    }
}


