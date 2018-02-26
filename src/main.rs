//No me rompas las pelotas
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fmt;

#[derive(PartialEq, Debug,Clone)]
enum TerrainGround {
    Soil,
    Stone,
}

#[derive(PartialEq, Debug,Clone)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone,
}

#[derive(PartialEq, Debug,Clone)]
enum Being {
    Orc,
    Human,
}
#[derive(Debug, PartialEq,Clone)]
struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    being: Option<Being>,
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square>,
}

#[derive(Debug, PartialEq)]
enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(Debug, PartialEq)]
enum MovementError {
    NoBeingInSquare,
    AnotherBeingInSquare,
    FellOffTheGrid,
    MovedToBadTerrain
}

impl fmt::Display for MovementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MovementError::AnotherBeingInSquare => {
                write!(f, "There's a being in the direction you're trying to move")
            }
            MovementError::MovedToBadTerrain => {
                write!(f, "This being cannot move to stone ground")
            }
            MovementError::NoBeingInSquare => {
                write!(f, "There's no being you can move at those coordinates")
            }
            MovementError::FellOffTheGrid => {
                write!(f, "New coordinates are off the grid!")
            }
        }
    }
}

impl Grid {
    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 0..number_of_squares {
            squares.push(Square {
                ground: TerrainGround::Soil,
                block: None,
                being: None,
            });
        }

        return Grid {
            size: (size_x, size_y),
            squares: squares,
        };
    }
    fn get_vec_index(&self, coord: (usize, usize)) -> usize {
        return coord.0 * self.size.0 + coord.1;
    }

    fn get_new_coords(&self, coord: (usize, usize), dir: &Direction) -> (usize, usize) {
        let x = coord.0;
        let y = coord.1;
        return match dir {
            &Direction::West => (x, y - 1),
            &Direction::East => (x, y + 1),
            &Direction::North => (x - 1, y),
            &Direction::South => (x + 1, y),
        };
    }

    fn move_being_in_coord(
        &mut self,
        coord: (usize, usize),
        dir: Direction,
    ) -> Result<(usize, usize), MovementError> {
        let copy_of_squares = self.squares.clone();
        
        let index_coords = self.get_vec_index((coord.0, coord.1));
        let new_coords = self.get_new_coords(coord, &dir);
        let index_new_coords = self.get_vec_index(new_coords);

        let square = copy_of_squares
                    // posicion en el vector (matriz representada como vector)
                    .get(index_coords) 
                    .expect("Index out of bounds trying to get being.");

        if square.being == None {
            return Err(MovementError::NoBeingInSquare);
        }

        if new_coords.0 >= self.size.0 || new_coords.1 >= self.size.1 {
            return Err(MovementError::FellOffTheGrid);
        }

        let new_square = copy_of_squares.get(index_new_coords).unwrap();

        if new_square.being != None {
            return Err(MovementError::AnotherBeingInSquare);
        }

        if new_square.ground == TerrainGround::Stone {
            return Err(MovementError::MovedToBadTerrain);
        }

        // Move the being
        // new square!
        self.squares[index_new_coords] =
            Square{
                ground: new_square.ground.clone(),
                block: new_square.block.clone(),
                being: square.being.clone()
            };
        
        // Old square!
        self.squares[index_coords] =
            Square {
                ground: square.ground.clone(),
                block: square.block.clone(),
                being: None
            };

        Ok(new_coords)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_grid() {
        let grid = ::Grid::generate_empty(5, 13);
        assert_eq!(grid.size, (5, 13));

        let mut number_of_squares = 0;
        for square in &grid.squares {
            assert_eq!(square.ground, ::TerrainGround::Soil);
            assert_eq!(square.block, None);
            assert_eq!(square.being, None);
            number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), 5 * 13);
        assert_eq!(number_of_squares, 5 * 13);
    }

    #[test]
    fn test_get_new_coords_north() {
        let grid = ::Grid::generate_empty(3, 3);
        let coord: (usize, usize) = (1, 1);

        let new_coord = grid.get_new_coords(coord, &::Direction::North);
        assert_eq!((0, 1), new_coord);
    }

    #[test]
    fn test_get_new_coords_east() {
        let grid = ::Grid::generate_empty(3, 3);
        let coord: (usize, usize) = (1, 1);

        let new_coord = grid.get_new_coords(coord, &::Direction::East);
        assert_eq!((1, 2), new_coord);
    }

    #[test]
    fn test_get_new_coords_south() {
        let grid = ::Grid::generate_empty(3, 3);
        let coord: (usize, usize) = (1, 1);

        let new_coord = grid.get_new_coords(coord, &::Direction::South);
        assert_eq!((2, 1), new_coord);
    }

    #[test]
    fn test_get_new_coords_west() {
        let grid = ::Grid::generate_empty(3, 3);
        let coord: (usize, usize) = (1, 1);

        let new_coord = grid.get_new_coords(coord, &::Direction::West);
        assert_eq!((1, 0), new_coord);
    }

    #[test]
    fn test_move_being_in_coord_no_being() {
        let mut grid = ::Grid::generate_empty(3, 3);
        assert_eq!(
            grid.move_being_in_coord((1, 1), ::Direction::West),
            Err(::MovementError::NoBeingInSquare)
        );
    }

    #[test]
    fn test_move_being_in_coord_being_in_square() {
        let mut grid = ::Grid::generate_empty(3, 3);

        grid.squares[4] = ::Square {
            ground: ::TerrainGround::Soil,
            block: None,
            being: Some(::Being::Orc)
        };
        grid.squares[7] = ::Square {
            ground: ::TerrainGround::Soil,
            block: None,
            being: Some(::Being::Human)
        };

        // 0  1  2
        // 3 Orc 5
        // 6  7  8
        assert_eq!(
            grid.move_being_in_coord((2, 1), ::Direction::North),
            Err(::MovementError::AnotherBeingInSquare)
        );
    }
}
