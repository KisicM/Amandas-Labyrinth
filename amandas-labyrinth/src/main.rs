use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;

#[derive(Debug, Clone)]
enum Direction {
    W,
    E,
    N,
    S
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::W => write!(f, "WEST"),
            Direction::E => write!(f, "EAST"),
            Direction::N => write!(f, "NORTH"),
            Direction::S => write!(f, "SOUTH")
        }
    }
}

#[derive(Debug, Clone)]
struct AvailablePath {
    direction: Direction,
    has_door: bool
}

#[derive(Debug, Clone)]
struct Field {
    index: i32,
    has_key: bool,
    is_exit: bool,
    available_paths: Box<[AvailablePath]>
}

impl Field {
    fn print(&self) {
        println!("Field index: {},\nhas_key: {},\nis_exit: {},\n{:#?}", self.index, self.has_key, self.is_exit, self.available_paths);
    }       
}

fn main() {
    //File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./labyrinth.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut i: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                i += 1;
                let mut binary_field = vec![];
                for byte in ip.split_whitespace() {
                    binary_field.push(byte);
                }
                let mut has_key = vec![];
                for bit in binary_field[2].chars() {
                    has_key.push(bit);
                }
                let field = Field {
                    index: i,
                    has_key: has_key[0] == '1' && has_key[1] == '1',
                    is_exit: has_key[2] == '1' && has_key[3] == '1',
                    available_paths: get_available_paths(binary_field[0], binary_field[1])
                };
                field.print();
            }
        }
    }
    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_available_paths(paths: &str, doors: &str) -> Box<[AvailablePath]> {
    let mut available_paths = vec![];
    let mut has_doors = vec![];
    for door in doors.chars() {
        has_doors.push(door);
    }
    let mut i: usize = 0;
    for path in paths.chars() {
        if path == '1'  {
            available_paths.push(AvailablePath {
                direction: if i == 0 { Direction::W } else if i == 1 { Direction::E } else if i == 2 { Direction::N } else { Direction::S },
                has_door: has_doors[i] == '1',
            })
        }
        i += 1;
    }
    return available_paths.into_boxed_slice();
}