use crate::field::Field;
use crate::field::Cell;
use crate::field::MAX_NUM;
use crate::field::SIZE;
use crate::field::BLOCK_SIZE;

/// Find all numbers that fit at the given position,
/// according to the sudoku rules.
/// Returns 1 if Possible was converted to Known, 0 otherwise.
/// Borrowing the field immutable and returning an owned result enables multi-threading.
pub fn find_possibilities(field: &Field, x: usize, y: usize) -> Result<(Cell, i32), String> {
    let cell = &field.cells[y][x];
    
    // Nothing must be computed for given and solved cells.
    match cell {
        Cell::Known(_) => return Ok((cell.clone(), 0)),
        _ => {}
    }
    // Possible optimisation: for Cell::Possible iterate over the
    // vector instead of iterating over all numbers.
    
    // Optimisation: first use stack-allocated array with max size
    // and later heap-allocate vec with necessary size. TODO benchmark it.
    let mut len = 0;
    let mut possible = [0i32; MAX_NUM];
    
    for num in 1..=MAX_NUM as i32 {
        if check(num, field, x,y) {
            possible[len] = num;
            len += 1;
        }
    }
    
    if len <= 0 {
        Err(format!("No possibilities found at ({}|{}). This sudoku is unsolvable.", x, y))
        
    } else if len == 1 {
        // Hurray, we found it!
        eprintln!("Found cell at ({}|{})", x, y);
        Ok((Cell::Known(possible[0]), 1))
        
    } else {
        // Allocate vec on heap with right size.
        let vec = Vec::from(&possible[..len]);
        debug_assert_eq!(vec.len(), vec.capacity());
        
        Ok((Cell::Possible(vec),0))
    }
}

/// For a single number check if that same number is
/// already somewhere in the row, column or block of
/// the given cell position.
/// Returns true if num is possible.
fn check(num: i32, field: &Field, x: usize, y: usize) -> bool {
    // check line
    for i in 0..SIZE {
        if field.cells[y][i] == Cell::Known(num) {
            return false;
        }
    }
    // check column
    for j in 0..SIZE {
        if field.cells[j][x] == Cell::Known(num) {
            return false;
        }
    }
    // check the 3*3 block
    let block_x = x - (x % BLOCK_SIZE);
    let block_y = y - (y % BLOCK_SIZE);
    
    for i in block_x ..= block_x+2 {
        for j in block_y ..= block_y+2 {
            if field.cells[j][i] == Cell::Known(num) {
                return false;
            }
        }
    }
    
    // no check fails
    true
}


#[cfg(test)]
mod test {
    use crate::csv_parser::parse_csv;
    use crate::csv_parser::EXAMPLE;
    use crate::solver::possibilities_finder::find_possibilities;
    use crate::field::Cell;
    use crate::solver::possibilities_finder::check;
    
    #[test]
    fn find_possibilities_test() {
        let mut field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        
        assert_eq!(find_possibilities(&field, 0,0), Ok((Cell::Possible(vec![2,4,6]), 0)));
        assert_eq!(find_possibilities(&field, 8,8), Ok((Cell::Possible(vec![2,9]), 0)));
        assert_eq!(find_possibilities(&field, 5,3), Ok((Cell::Possible(vec![1,2,4,7]), 0)));
        // Solved in find_possibilities
        assert_eq!(find_possibilities(&field, 6,1), Ok((Cell::Known(2), 1)));
        // Known from beginning
        assert_eq!(find_possibilities(&field, 1,0), Ok((Cell::Known(5), 0)));
        field.cells[3][1] = Cell::Known(3);
        assert_eq!(find_possibilities(&field, 1,3), Ok((Cell::Known(3), 0)));
    }
    
    #[test]
    fn check_test() {
        let field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        println!("{}", field);
        
        assert_eq!(check(8, &field, 3,8), false); //row
        assert_eq!(check(1, &field, 8,0), false); //column
        assert_eq!(check(6, &field, 6,5), false); //block
        
        assert_eq!(check(7, &field, 7,7), false);
        assert_eq!(check(3, &field, 1,4), false);
        assert_eq!(check(4, &field, 3,3), true);
        assert_eq!(check(2, &field, 4,1), true);
    
        // There was a off-by-one error in check line & column
        assert_eq!(check(6, &field, 8,0), false);
        assert_eq!(check(5, &field, 0,8), false);
        assert_eq!(check(1, &field, 5,6), false);
    }
}