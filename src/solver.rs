use crate::field::Field;
use crate::field::Cell;
use crate::field::MAX_NUM;
use crate::field::SIZE;

pub fn solve_sudoku(field: Field) -> Field {
    unimplemented!()
}

/// For each number, test if it can be in
/// the specified empty cell.
/// Ignore given and solved cells.
/// Borrowing the field immutable and returning an owned result enables multi-threading.
fn find_possibilities(field: &Field, x: usize, y: usize) -> Result<Cell, String> {
    let cell = &field.cells[y][x];
    
    // Nothing must be computed for given and solved cells.
    match cell {
        Cell::Known(_) => return Ok(cell.clone()),
        _ => {}
    }
    // Possible optimisation: for Cell::Possible iterate over the
    // vector instead of iterating over all numbers.
    
    for num in 1..MAX_NUM {
        
    }
    
    
    Err("unimplemented!()".to_owned())
}

fn check(num: usize, field: &Field, x: usize, y: usize) -> bool {
    // check line
    for i in 0..SIZE-1 {
        if i != x  // ignore current cell
            && field.cells[y][i] == Cell::Known(num as i32)
        {
            return false
        }
    }
    // check column
    // check block
    true
}


#[cfg(test)]
mod test {
    use crate::csv_parser::parse_csv;
    use crate::csv_parser::EXAMPLE;
    use crate::solver::find_possibilities;
    use crate::field::Cell;
    
    #[test]
    fn find_possibilities_test() {
        let mut field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        field.cells[3][1] = Cell::Known(4);
        
        assert_eq!(find_possibilities(&field, 1,3), Ok(Cell::Known(4)));
        assert_eq!(find_possibilities(&field, 1,0), Ok(Cell::Known(5)));
        //assert_eq!(find_possibilities(&field, 8,8), TODO Test last entry.);
    }
}

