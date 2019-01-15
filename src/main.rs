fn main() {
    println!("Hello, world!");
}

// Für jede Zelle eine Liste der noch möglichen Zahlen, die passen könnten.
// Wenn nur eine übrig bleibt, dann ist diese fix.
const MAX: usize = 9;

enum Cell {
    Given(i32),
    Solved(i32),
    Empty,
    Possible([i32; MAX]),
}

/* CSV-Format zum einlesen.
_,5,_,9,_,_,3,7,_
1,8,9,_,4,_,_,6,5
3,_,_,_,_,_,_,4,_
_,_,_,_,3,_,_,_,6
_,9,_,6,8,5,_,2,_
5,_,_,_,9,_,_,_,_
_,4,_,_,_,_,_,_,1
9,3,_,_,6,_,7,8,4
_,1,8,_,_,3,_,5,_
*/

// Not all sudokus can be solved with elimination.
// Others need trial and error. Rekursive or try  flat?
// If no number fits, -> impossible

