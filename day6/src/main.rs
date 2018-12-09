
use std::fs;
use std::collections::{HashMap, HashSet};
use std::fmt;

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("day6/input.txt").expect("failed to read input.txt");

    let coordinates = Coordinates::from_str("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9");

    let mut map = Map::new(&coordinates);
    map.populate();

    println!("{}", map);

    let largest_inner_area = map.find_largest_inner_area();
    println!("largest inner area: {}", largest_inner_area);
}

#[derive(Clone)]
pub struct Map {
    pub size: usize,
    cells: Vec<Vec<CellType>>
}

impl Map {
    pub fn new(coordinates: &Coordinates) -> Map {
        let size = Map::find_size(&coordinates) + 1;
        let mut cells: Vec<Vec<CellType>> = Vec::new();

        for _ in 0..size {
            let mut row = Vec::new();
            for _ in 0..size {
                row.push(CellType::Unprocessed);
            }
            cells.push(row);
        }

        coordinates.coords.iter().enumerate().for_each(|(id, coordinate)| {
            cells[coordinate.x][coordinate.y] = CellType::Center(id as CoordID);
        });

        Map{
            size: size,
            cells: cells,
        }
    }

    fn find_size(coordinates: &Coordinates) -> usize {
        coordinates.coords.iter().fold(0, |max, coord| {
            if coord.x > max {
                coord.x + 1
            } else if coord.y > max {
                coord.y + 1
            } else {
                max
            }
        })
    }

    pub fn populate(&mut self) {
        for x in 0..self.size {
            for y in 0..self.size {
                let coord = Coordinate::new(x, y);
                let new_cell = self.closest_center(coord);
                self.cells[x][y] = new_cell;
            }
        }
    }

    pub fn closest_center(&self, coordinate: Coordinate) -> CellType {
        if let CellType::Center(id) = self.cells[coordinate.x][coordinate.y] {
            return CellType::Center(id);
        }

        let mut centers = Vec::new();
        for i in 0..self.size {
            self.closest_cells(i, coordinate).iter().for_each(|coord| {
                if let CellType::Center(id) = self.cells[coord.x][coord.y] {
                    centers.push(id);
                }
            });
            if centers.len() > 0 {
                break;
            }
        }
        if centers.len() == 1 {
            CellType::Closest(centers[0])
        } else {
            CellType::Equal
        }
    }

    // assumes x, y are valid coordinates
    fn closest_cells(&self, idx: usize, coordinate: Coordinate) -> Vec<Coordinate> {
        let x = coordinate.x;
        let y = coordinate.y;

        let mut cells = Vec::new();
        let x_sub_idx = x.checked_sub(idx);
        let y_sub_idx = y.checked_sub(idx);
        let x_add_idx = x + idx;
        let y_add_idx = y + idx;
        if let Some(new_x) = x_sub_idx {
            cells.push(Coordinate::new(new_x, y));
            if y_add_idx < self.size {
                cells.push(Coordinate::new(new_x, y_add_idx));
            }
        }

        if let Some(new_y) = y_sub_idx {
            cells.push(Coordinate::new(x, new_y));
            if x_add_idx < self.size {
                cells.push(Coordinate::new(x_add_idx, new_y));
            }
        }
        if y_sub_idx.is_some() && x_sub_idx.is_some() {
            cells.push(Coordinate::new(x - idx, y - idx));
        }
        if x_add_idx < self.size {
            cells.push(Coordinate::new(x_add_idx, y));
        }
        if y_add_idx < self.size {
            cells.push(Coordinate::new(x, y_add_idx));
        }
        if x_add_idx < self.size && y_add_idx < self.size {
            cells.push(Coordinate::new(x_add_idx, y_add_idx));
        }
        cells
    }

    pub fn is_border_cell(&self, coord: Coordinate) -> bool {
        coord.x == 0 || coord.x == self.size - 1 || coord.y == 0 || coord.y == self.size - 1
    }

    pub fn find_largest_inner_area(&self) -> usize {
        let mut outer_areas = HashSet::new();
        let mut inner_areas = HashMap::new();
        for x in 0..self.size {
            for y in 0..self.size {
                let cell = self.cells[x][y];
                let coord = Coordinate::new(x, y);
                if self.is_border_cell(coord) {
                    println!("is_border_Cell {:?}", cell);
                    if let CellType::Closest(id) = cell {
                        outer_areas.insert(id);
                    } else if let CellType::Center(id) = cell {
                        outer_areas.insert(id);
                    }
                } else {
                    match cell {
                        CellType::Center(id) => {
                            println!("addin a new one {}", id);
                            let area = inner_areas.entry(id).or_insert(0);
                            *area += 1;
                        }
                        CellType::Closest(id) => {
                            println!("addin a new one {}", id);
                            let area = inner_areas.entry(id).or_insert(0);
                            *area += 1;
                        }
                        _ => {
                        }
                    }
                }
            }
        }
        inner_areas.into_iter().map(|(_, v)| v).max().expect("failed to get largest inner area")
    }
}


impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for x in 0..self.size {
            for y in 0..self.size {
                let cell = self.cells[x][y];
                match cell {
                    CellType::Center(id) => {
                        write!(f, "{}", char::from((id + 65)as u8));
                    }
                    CellType::Closest(id) => {
                        write!(f, "{}", char::from((id + 97)as u8));
                    }
                    CellType::Equal => {
                        write!(f, ".");
                    }
                    CellType::Unprocessed => {
                        write!(f, "*");
                    }
                }
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = Cell;
    type IntoIter = MapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapIterator{map: self, curr_coord: Coordinate::new(0,0), start: true}
    }
}

pub struct MapIterator<'a> {
    map: &'a Map,
    curr_coord: Coordinate,
    start: bool,
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        if self.start {
            let current = self.map.cells[self.curr_coord.x][self.curr_coord.y];
            self.start = false;
            Some(Cell{
                cell_type: current,
                coord: self.curr_coord,
            })
        } else {
            if self.curr_coord.y < self.map.size - 1 {
                self.curr_coord.y += 1;
            } else if self.curr_coord.x < self.map.size - 1 {
                self.curr_coord.x += 1;
            } else {
                return None;
            }
            let cell_type = self.map.cells[self.curr_coord.x][self.curr_coord.y];
            println!("iterating {:?} cell: {:?}", self.curr_coord, cell_type);
            Some(Cell{
                cell_type: cell_type,
                coord: self.curr_coord,
            })
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Cell {
    coord: Coordinate,
    cell_type: CellType,
}

#[derive(Debug, Copy, Clone)]
pub enum CellType {
    Unprocessed,
    Equal,
    Closest(CoordID),
    Center(CoordID),
}

type CoordID = i64;

#[derive(Debug, Clone)]
pub struct Coordinates {
    pub coords: Vec<Coordinate>,
}

impl Coordinates {
    pub fn from_str(input: &str) -> Coordinates {
        let coordinates = input.split("\n")
            .filter(|x| !x.is_empty())
            .map(|line| Coordinate::from_str(line)).collect();
        Coordinates {
            coords: coordinates,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate{x: x, y: y}
    }

    pub fn from_str(input: &str) -> Coordinate {
        let mut iter = input.split(",");
        let x = iter.next().expect("failed to get x").parse().expect("failed to parse x");
        let y = iter.next().expect("failed to get y").trim().parse().expect("failed to parse y");
        Coordinate::new(x, y)
    }
}
