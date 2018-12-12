
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
    cells: Vec<Vec<CellType>>,
    centers: Vec<Coordinate>,
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
            centers: coordinates.coords.clone(),
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
                println!("populating {:?}", coord);
                let new_cell = self.closest_center(coord);
                self.cells[x][y] = new_cell;
            }
        }
    }

    pub fn closest_center(&self, coordinate: Coordinate) -> CellType {
        println!("closest_center {:?}", coordinate);
        if let CellType::Center(id) = self.cells[coordinate.x][coordinate.y] {
            return CellType::Center(id);
        }

        let mut closest_center = None;
        let mut closest_distance = None;
        for (id, center) in self.centers.iter().enumerate() {
            if let Some(closest_dist) = closest_distance {
                let dist = coordinate.distance(&center);
                if closest_dist == dist {
                    println!("equal {:?} and {:?}", closest_center, id);
                    return CellType::Equal;
                } else if dist < closest_dist {
                    closest_distance = Some(dist);
                    closest_center = Some(id);
                }
            } else {
                closest_center = Some(id);
                closest_distance = Some(coordinate.distance(&center));
            }
        }

        let closest_id = closest_center.expect("failed to ge closest center");
        println!("coord: {:?} closest: {}, center: {:?}", coordinate, closest_id, self.centers[closest_id]);
        CellType::Closest(closest_id as i64)
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
                    if let CellType::Closest(id) = cell {
                        outer_areas.insert(id);
                    } else if let CellType::Center(id) = cell {
                        outer_areas.insert(id);
                    }
                } else {
                    match cell {
                        CellType::Center(id) => {
                            let area = inner_areas.entry(id).or_insert(0);
                            *area += 1;
                        }
                        CellType::Closest(id) => {
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

    pub fn distance(&self, other: &Coordinate) -> usize {
        let x_diff = self.x as i64 - other.x as i64;
        let y_diff = self.y as i64 - other.y as i64;
        (x_diff.abs() + y_diff.abs()) as usize
    }
}
