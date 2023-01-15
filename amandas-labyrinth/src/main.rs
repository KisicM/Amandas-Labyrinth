use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
#[allow(dead_code)]
struct AvailablePath {
    direction: Direction,
    has_door: bool
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
    let mut _steps: i32 = 0;
    let mut _keys: i32 = 0;
    let mut _exit: bool = false;
    let mut labyrinth : Vec<Field> = vec![];
    let mut exit_fields: Vec<Field> = vec![];
    let mut current_field: Option<Field> = None;

    //File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./labyrinth.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut i: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
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
                //field.print();
                if field.is_exit {
                    exit_fields.push(field.clone());
                };
                labyrinth.push(field);
                i += 1;
            }
        }
    }
    //print!("{:#?}", labyrinth);
    current_field = Some(labyrinth[0].clone());
    //print!("{:#?}, vector len: {}\n", current_field, labyrinth.len());
    let surrounding_fields = get_surrounding_fields(&labyrinth, current_field.unwrap());
    println!("{:#?}", surrounding_fields);
    

    
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

fn get_surrounding_fields(labyrinth : &Vec<Field>, current_field: Field) -> Vec<&Field> {
    let mut surrounding_fields: Vec<&Field> = vec![];
    let available_paths = current_field.available_paths;
    for path in available_paths.into_iter() {
        let field_index: i32;
        match  path.direction {
            Direction::W => {
                field_index = current_field.index - 1;
            },
            Direction::E => {
                field_index = current_field.index + 1;
            },
            Direction::N => {
                field_index = current_field.index - 9;
            },
            Direction::S => {
                field_index = current_field.index + 9;
            }
        }
        if field_index >= 0 && field_index <= 53 {
            for field in labyrinth {
                if field.index == field_index {
                    surrounding_fields.push(field);
                }
            }
        }
    }
    return surrounding_fields;
}