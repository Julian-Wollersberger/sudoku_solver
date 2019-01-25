use crate::field::Field;
use crate::field::Cell;
use crate::field::MAX_NUM;
use crate::field::SIZE;
use crate::field::BLOCK_SIZE;
use std::mem;

pub fn solve_sudoku(field: Field) -> Result<Field, String> {
    let mut field = field;
    let mut new_field = Field::empty();
    let mut total_progress = 0;
    
    loop {
        let progress= solve_step(&field, &mut new_field)?;
        total_progress += progress;
        if progress <= 0 {
            println!("Solved {} cells in total.", total_progress);
            return Ok(new_field)
        }
        
        mem::swap(&mut field, &mut new_field);
    }
}

/// Do one iteration to try to solve the sudoku:
/// First find all possibilities,
/// then if only one possibility exists, replace with Cell:Known.
/// TODO additional elimination algorithm.
/// Returns number of solved cells.
fn solve_step(field: &Field, new_field: &mut Field) -> Result<i32, String> {
    let mut new_known_cells = 0;
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            
            let mut possible = find_possibilities(field, x, y)?;
            // Convert single possibility to Known.
            match possible {
                Cell::Known(_) => {},
                Cell::Empty =>
                    return Err("find_possibilities() shouldn't return Cell::Empty".to_owned()),
                Cell::Possible(vec) => if vec.len() == 1 {
                    new_known_cells += 1;
                    possible = Cell::Known(vec[0]);
                    println!("Solved cell at ({}|{}) to be {}", x,y, vec[0])
                } else {
                    // Needed to move the vec back.
                    possible = Cell::Possible(vec)
                }
            }
            new_field.cells[y][x] = possible;
        }
    }
    
    println!("Solved {} cells.", new_known_cells);
    Ok(new_known_cells)
}

/// Find all numbers that fit at the given position,
/// according to the sudoku rules.
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
    
    // Optimisation: first use stack-allocated array with max size
    // and later heap-allocate vec with necessary size. TODO benchmark it.
    let mut len = 0;
    let mut possible = [0i32; MAX_NUM];
    
    for num in 1..=MAX_NUM {
        if check(num, field, x,y) {
            possible[len] = num as i32;
            len += 1;
        }
    }
    
    if len <= 0 {
        Err(format!("No possibilities found at ({}|{})", x, y))
    
    // If we check it here, the calling function can't know
    // if we made progress.
    /*} else if len == 1 {
        // Hurray, we found it!
        Ok(Cell::Known(possible[0]))
    */
    } else {
        // Allocate vec on heap with right size.
        let vec = Vec::from(&possible[..len]);
        debug_assert_eq!(vec.len(), vec.capacity());
    
        Ok(Cell::Possible(vec))
    }
}

/// For a single number check if that same number is
/// already somewhere in the row, column or block of
/// the given cell position.
fn check(num: usize, field: &Field, x: usize, y: usize) -> bool {
    // check line
    for i in 0..SIZE-1 {
        if field.cells[y][i] == Cell::Known(num as i32) {
            return false;
        }
    }
    // check column
    for j in 0..SIZE-1 {
        if field.cells[j][x] == Cell::Known(num as i32) {
            return false;
        }
    }
    // check the 3*3 block
    let block_x = x - (x % BLOCK_SIZE);
    let block_y = y - (y % BLOCK_SIZE);
    
    for i in block_x ..= block_x+2 {
        for j in block_y ..= block_y+2 {
            if field.cells[j][i] == Cell::Known(num as i32) {
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
    use crate::solver::find_possibilities;
    use crate::field::Cell;
    use crate::solver::check;
    use crate::field::Field;
    use crate::solver::solve_step;
    
    #[ignore]
    #[test]
    fn solve_example_completely() {
        let field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        unimplemented!();
    }
        
    #[test]
    fn solve_step_test() {
        let field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        let mut new_field = Field::empty();
        
        let progress = solve_step(&field, &mut new_field).unwrap();
        assert!(progress >= 1);
        //  I haven't tested if the solver misses some cells.
        assert_eq!(new_field.cells[1][6], Cell::Known(2))
    }
    
    #[test]
    fn find_possibilities_test() {
        let mut field = parse_csv(EXAMPLE.into()).expect("Parsing failed");

        assert_eq!(find_possibilities(&field, 0,0), Ok(Cell::Possible(vec![2,4,6])));
        assert_eq!(find_possibilities(&field, 8,8), Ok(Cell::Possible(vec![2,9])));
        assert_eq!(find_possibilities(&field, 5,3), Ok(Cell::Possible(vec![1,2,4,7])));
        
        assert_eq!(find_possibilities(&field, 1,0), Ok(Cell::Known(5)));
        field.cells[3][1] = Cell::Known(3);
        assert_eq!(find_possibilities(&field, 1,3), Ok(Cell::Known(3)));
    }
    
    #[test]
    fn check_test() {
        let field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        
        assert_eq!(check(8, &field, 3,8), false); //row
        assert_eq!(check(1, &field, 8,0), false); //column
        assert_eq!(check(6, &field, 6,5), false); //block
        
        assert_eq!(check(7, &field, 7,7), false);
        assert_eq!(check(3, &field, 1,4), false);
        assert_eq!(check(4, &field, 3,3), true);
        assert_eq!(check(2, &field, 4,1), true);
    }
}

