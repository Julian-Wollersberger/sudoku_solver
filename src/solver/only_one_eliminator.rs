use crate::field::Cell;
use crate::field::BLOCK_SIZE;
use crate::field::SIZE;
use crate::field::Field;

/// Is this the only cell in a row/column/block
/// where the number is allowed?
pub fn is_only_one(num: i32, field: &Field, x: usize, y: usize) -> bool {
    // check line
    for i in 0..SIZE-1 {
        if other_cell_contains_num(&field.cells[y][i], num) {
            return false;
        }
    }
    // check column
    for j in 0..SIZE-1 {
        if other_cell_contains_num(&field.cells[j][x], num) {
            return false;
        }
    }
    // check the 3*3 block
    let block_x = x - (x % BLOCK_SIZE);
    let block_y = y - (y % BLOCK_SIZE);
    
    for i in block_x ..= block_x+2 {
        for j in block_y ..= block_y+2 {
            if other_cell_contains_num(&field.cells[j][i], num) {
                return false;
            }
        }
    }
    
    true
}

/// De-duplicates this `match cell {}`.
fn other_cell_contains_num(cell: &Cell, num: i32) -> bool {
    match cell {
        Cell::Possible(vec) => {
            if vec.contains(&num) {
                return true;
            }
        },
        Cell::Empty => {
            // Happens in the first iteration, because is_only_one()
            // doesn't ignore itself.
            // eprintln!("Empty Cells shouldn't exist in other_cell_contains_num()!");
            return true; // Be conservative
        },
        Cell::Known(known) => {
            // Only the found possibilities are tested,
            // so known cells should never conflict with this number.
            debug_assert_ne!(num, *known)
        }
    }
    false
}

#[cfg(test)]
mod test {

}