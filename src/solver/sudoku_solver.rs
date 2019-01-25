use crate::field::Field;
use crate::field::Cell;
use crate::field::SIZE;
use std::mem;
use crate::solver::possibilities_finder::find_possibilities;
use crate::solver::only_one_eliminator::is_only_one;

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
            
            // 1. What numbers are allowed here?
            let mut possible = find_possibilities(field, x, y)?;
            // 2. Convert single possibility to Known.
            match possible {
                Cell::Known(_) => {},
                Cell::Empty =>
                    return Err("find_possibilities() shouldn't return Cell::Empty".to_owned()),
                Cell::Possible(vec) => {
                    // TODO move this mess into a function.
                    if vec.len() == 1 {
                        new_known_cells += 1;
                        possible = Cell::Known(vec[0]);
                        println!("Solved cell at ({}|{}) to be {}", x,y, vec[0])
                    } else {
                        //TODO 3. Is this the only cell in a row/column/block
                        // where a number is possible?
                        for num in &vec {
                            if is_only_one(num.clone(), field, x,y) {
                                //TODO
                            }
                        }
                        
                        // Needed to move the vec back.
                        possible = Cell::Possible(vec)
                    }
                }
            }
            new_field.cells[y][x] = possible;
        }
    }
    
    println!("Solved {} cells.", new_known_cells);
    Ok(new_known_cells)
}




#[cfg(test)]
mod test {
    use crate::csv_parser::parse_csv;
    use crate::csv_parser::EXAMPLE;
    use crate::field::Cell;
    use crate::field::Field;
    use crate::solver::sudoku_solver::solve_step;
    
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
}

