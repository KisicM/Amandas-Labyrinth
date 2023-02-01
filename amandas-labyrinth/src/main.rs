//Mihajlo Kisic E259/2022
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;
//use rand::Rng;

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
        println!("Field index: {},\nis_exit: {}\n", self.index, self.is_exit);
    }       
}

fn main() {
    //let mut steps: i32 = 0;
    //let mut keys: i32 = 0;
    let mut labyrinth : Vec<Field> = vec![];
    let mut exit_fields: Vec<Field> = vec![];
    // let mut current_field: Field = Field {
    //     index: 0,
    //     has_key: false,
    //     is_exit: false,
    //     available_paths: Default::default()
    // };

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

    //current_field = labyrinth[0].clone();

    //let mut adj_list: Vec<Vec<usize>> = vec![];
    // for field in labyrinth.clone().into_iter() {
    //     let surrounding_fields = get_surrounding_fields(labyrinth.clone(), field);
    //     let mut array_of_indexes = vec![];
    //     for neighbour in surrounding_fields.into_iter() {
    //         array_of_indexes.push(neighbour.index as usize)
    //     }
    //     adj_list.push(array_of_indexes);
    // }

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for field in labyrinth.clone().into_iter() {
        let surrounding_fields = get_surrounding_fields(labyrinth.clone(), field.clone());
        let mut array_of_indexes = vec![];
        for neighbour in surrounding_fields.into_iter() {
            array_of_indexes.push(neighbour.index as usize)
        }
        graph.insert(field.index as usize, array_of_indexes);
    }

    let mut shortest_paths = vec![];
    for exit in exit_fields.into_iter() {
        //shortest_paths.push(bfs(labyrinth[0].index as usize, exit.index as usize, &adj_list, &mut labyrinth))
        shortest_paths.push(bfs_dynamic(&mut graph, labyrinth[0].index as usize, exit.index as usize))
    }
    print!("{:#?}", shortest_paths);
    // while !current_field.is_exit {
    //     current_field.print();
    //     keys = pick_key_up(&mut labyrinth, &mut current_field, keys);
    //     keys = unlock_door(&mut labyrinth, &mut current_field, keys);
    //     let mut surrounding_fields = get_surrounding_fields(labyrinth.clone(), current_field.clone());
    //     for (i, path) in current_field.available_paths.into_iter().enumerate() {
    //         if path.has_door {
    //             surrounding_fields.swap_remove(i);
    //         }
    //     }
    //     current_field = surrounding_fields[rand::thread_rng().gen_range(0..surrounding_fields.len())].clone();
    //     steps += 1;
    //     println!("Step number: {}", steps);
    // }
    // current_field.print();

    
}

fn bfs(start: usize, end: usize, adj_list: &Vec<Vec<usize>>, labyrinth : &mut Vec<Field>) -> Option<(Vec<usize>, usize)> {
    let mut visited = vec![false; adj_list.len()];
    let mut queue = VecDeque::new();
    let mut distances = vec![std::usize::MAX; adj_list.len()];
    let mut paths = vec![vec![]; adj_list.len()];

    visited[start] = true;
    queue.push_back(start);
    distances[start] = 0;
    paths[start] = vec![start];
    let mut keys = 0;

    while let Some(vertex) = queue.pop_front() {
        println!("Vertex {}", vertex);
        let mut current_field = labyrinth[vertex].clone();
        let surrounding_fields = get_surrounding_fields(labyrinth.clone(), current_field.clone());
        let mut available_neighbors: Vec<usize> = vec![];
        let mut fields_with_doors: Vec<i32> = vec![];

        keys = pick_key_up(labyrinth, &mut current_field, keys);
        keys = unlock_door(labyrinth, &mut current_field, keys);

        for (i, path) in current_field.available_paths.into_iter().enumerate() {
            if path.has_door {
                fields_with_doors.push(surrounding_fields[i].index)
            }
        }
        if fields_with_doors.len() != 0 {
            for index in adj_list[vertex].clone().into_iter() {
                for field_index in fields_with_doors.clone() {
                    if field_index != index as i32 {
                        available_neighbors.push(index);
                    }
                }
            }
        } else {
            available_neighbors = adj_list[vertex].clone()
        }
        // println!("{:#?}", current_field);
        // println!("{:?}", available_neighbors);
        for &neighbor in &available_neighbors {
            if !visited[neighbor] {
                println!("VISITED {}", neighbor);
                visited[neighbor] = true;
                distances[neighbor] = distances[vertex] + 1;
                paths[neighbor] = paths[vertex].clone();
                paths[neighbor].push(neighbor);
                queue.push_back(neighbor);

                if neighbor == end {
                    return Some((paths[end].clone(), distances[end]));
                }
            }
        }
    }

    None
}


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

fn get_surrounding_fields(labyrinth : Vec<Field>, current_field: Field) -> Vec<Field> {
    let mut surrounding_fields: Vec<Field> = vec![];
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
            for field in labyrinth.clone() {
                if field.index == field_index {
                    surrounding_fields.push(field);
                }
            }
        }
    }
    return surrounding_fields;
}

fn unlock_door(labyrinth : &mut Vec<Field>, current_field: &mut Field, mut keys: i32) -> i32 {
    if keys == 0 {
        return keys
    }

    for (i, path) in current_field.available_paths.into_iter().enumerate() {
        if keys != 0 && path.has_door{
            labyrinth[current_field.index as usize].available_paths[i].has_door = false;
            keys -= 1;
        }
    }
    //update current_field
    for field in labyrinth {
        if field.index == current_field.index {
            current_field.available_paths = field.available_paths.clone();
        }
    }

    keys
}

fn pick_key_up(labyrinth : &mut Vec<Field>, current_field: &mut Field, mut keys: i32) -> i32 {
    if current_field.has_key {
        labyrinth[current_field.index as usize].has_key = false;
        keys += 1;
    }
    //update current_field
    for field in labyrinth {
        if field.index == current_field.index {
            current_field.has_key = field.has_key;
        }
    }
    keys
}


use std::collections::{HashMap};

enum State {
    Visited,
    Unvisited,
}

fn bfs_dynamic(graph: &mut HashMap<usize, Vec<usize>>, start: usize, end: usize) -> Option<(usize, Vec<usize>)> {
    let mut distances = HashMap::new();
    let mut parents = HashMap::new();
    let mut queue = VecDeque::new();
    let mut state = HashMap::new();

    distances.insert(start, 0);
    queue.push_back(start);
    state.insert(start, State::Visited);

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        if let Some(neighbors) = graph.get_mut(&vertex) {
            for neighbor in neighbors {
                if let Some(dist) = distances.get(&vertex) {
                    if !distances.contains_key(&neighbor) {
                        distances.insert(neighbor.clone(), dist + 1);
                        parents.insert(neighbor.clone(), vertex);
                        state.insert(neighbor.clone(), State::Visited);
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }

    let mut current = end;
    let mut path = vec![end];
    while current != start {
        current = match parents.get(&current) {
            Some(p) => *p,
            None => return None,
        };
        path.push(current);
    }
    path.reverse();

    Some((distances[&end], path))
}

