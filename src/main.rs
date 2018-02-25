//No me rompas las pelotas
#![allow(dead_code)]

#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
enum Being {
    Orc,
    Human,
}

struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    being: Option<Being>,
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square>,
}

enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(Debug, PartialEq)]
enum MovementError {
    NoBeingInSquare,
    BeingAlreadyInSquare,
    BeingCannotMoveToStoneGround,
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

    fn get_new_coords(&self, coord: (usize, usize), dir:Direction) -> (usize, usize) {
       return match dir {
            Direction::West => (coord.0 - 1, coord.1),
            Direction::East => (coord.0, coord.1 + 1),
            Direction::North => (coord.0, coord.1 - 1),
            Direction::South => (coord.0 - 1, coord.1 + 1),
        }

    }
    fn move_being_in_coord(
        &self,
        coord: (usize, usize),
        dir: Direction,
    ) -> Result<(usize, usize), MovementError> {
        let index_coords = self.get_vec_index((coord.0, coord.1));
        let new_coords = self.get_new_coords(coord,dir);
        let index_new_coords = self.get_vec_index(new_coords);
        
        let square = self.squares
                    // posicion en el vector (matriz representada como vector)
                    .get(index_coords) 
                    .expect("Index out of bounds trying to get being.");

        let destination_square = self.squares
                    // Posicion a la que se mueve el ser
                    .get(index_new_coords)
                    .expect("New coords (after movement) would be out of bounds.");

        let mut being_new_coords: Result<(usize, usize), MovementError> = 
            match square.being {
                Some(_) => Ok(coord),
                None => Err(MovementError::NoBeingInSquare),
            };

        println!("New Coords {:?}", being_new_coords);

        being_new_coords = match destination_square.being {
            Some(_) => Err(MovementError::BeingAlreadyInSquare),
            None => Ok(new_coords),
        };

        // Solo se puede mover a Soil
        being_new_coords = match destination_square.ground {
            TerrainGround::Stone => Err(MovementError::BeingCannotMoveToStoneGround),
            TerrainGround::Soil => Ok(new_coords),
        };

        return being_new_coords;
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
    fn test_move_being_in_coord_no_being() {
        let grid = ::Grid::generate_empty(3, 3);
        assert_eq!(
            grid.move_being_in_coord((1, 0), ::Direction::West),
            Err(::MovementError::NoBeingInSquare)
        );
    }

    #[test]
    fn test_move_being_in_coord_being_in_square() {
        let mut grid = ::Grid::generate_empty(3, 3);

        grid.squares[4] = ::Square {
            ground: ::TerrainGround::Soil,
            block: None,
            being: Some(::Being::Orc),
        };

        assert_eq!(
            grid.move_being_in_coord((2, 1), ::Direction::North),
            Err(::MovementError::BeingAlreadyInSquare)
        );
    }
}
