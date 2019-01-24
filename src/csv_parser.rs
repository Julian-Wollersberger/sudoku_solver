use std::str::FromStr;
use std::error::Error;
use std::string::String;
use crate::Cell;
use crate::MAX_NUM;

/// Parse a string that is in a Comma-Separated-Values format
/// with 9 rows and columns into a 2D array of Cells.
fn parse_csv(input: String) -> Result<Vec<Vec<Cell>>, String> {
    let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(MAX_NUM);
    
    for line in input.split('\n') {
        println!("line: {}", line);
        let mut line_parsed = Vec::with_capacity(MAX_NUM);
        for cell in line.split(',') {
            print!("\t{}",cell);
            line_parsed.push(
                parse_cell(cell)?
            );
        };
        if line_parsed.len() != MAX_NUM {
            return Err(format!("Error parsing the CSV input. \
                A line had length {}, but it must have {}",
                line_parsed.len(), MAX_NUM));
        }
        cells.push(line_parsed);
        println!();
    };
    
    if cells.len() != MAX_NUM {
        return Err(format!("Error parsing the CSV input. \
                There were {} lines, but there must be {}.",
            cells.len(), MAX_NUM));
    }
    
    Ok(cells)
}

/// Parse one CSV cell to a ´Cell´
/// or give an error message.
fn parse_cell(cell: &str) -> Result<Cell, String> {
    match cell {
        "_" => Ok(Cell::Empty),
        other => {
            match i32::from_str(other) {
                Ok(number) => if number <= MAX_NUM as i32 && number >= 1 {
                    Ok(Cell::Given(number))
                } else {
                    return Err(format!("Error while parsing the CSV input. \
                        The number must be between 1 and {} but was {}.",
                        MAX_NUM, number));
                },
                Err(err) =>
                    return Err(String::from(err.description()))
            }
        },
    }
}

/* CSV-Format zum einlesen. */
const EXAMPLE: &str =
    "_,5,_,9,_,_,3,7,_
1,8,9,_,4,_,_,6,5
3,_,_,_,_,_,_,4,_
_,_,_,_,3,_,_,_,6
_,9,_,6,8,5,_,2,_
5,_,_,_,9,_,_,_,_
_,4,_,_,_,_,_,_,1
9,3,_,_,6,_,7,8,4
_,1,8,_,_,3,_,5,_";

#[cfg(test)]
mod test {
    use crate::Cell;
    use crate::csv_parser::parse_cell;
    use crate::csv_parser::parse_csv;
    use crate::csv_parser::EXAMPLE;
    
    #[test]
    fn parse_cell_test() {
        assert_eq!(parse_cell("_").unwrap(), Cell::Empty);
        assert_eq!(parse_cell("1").unwrap(), Cell::Given(1));
        assert_eq!(parse_cell("9").unwrap(), Cell::Given(9));
        
        assert!(parse_cell("0").is_err());
        assert!(parse_cell("42").is_err());
        assert!(parse_cell(" ").is_err());
        assert!(parse_cell("O").is_err());
    }
    
    #[test]
    fn parse_csv_test() {
        parse_csv(EXAMPLE.into()).expect("Parsing failed");
        //TODO convert the result back into a string and compare with the input.
    }
}