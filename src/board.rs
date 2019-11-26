use crate::board::Direction::{DECREASING, INCREASING, STABLE};
use std::collections::HashMap;

pub const WIDTH: u8 = 7;
pub const HEIGHT: u8 = 6;

#[derive(Clone, PartialEq)]
pub enum Token {
    YELLOW,
    RED,
}

#[derive(Clone)]
enum Direction {
    INCREASING,
    DECREASING,
    STABLE,
}

struct Direction2D {
    pub x: Direction,
    pub y: Direction,
}

impl Direction2D {
    fn left() -> Direction2D {
        Direction2D {
            x: DECREASING,
            y: STABLE,
        }
    }
    fn right() -> Direction2D {
        Direction2D {
            x: INCREASING,
            y: STABLE,
        }
    }
    fn up() -> Direction2D {
        Direction2D {
            x: STABLE,
            y: INCREASING,
        }
    }
    fn down() -> Direction2D {
        Direction2D {
            x: STABLE,
            y: DECREASING,
        }
    }
    fn upleft() -> Direction2D {
        Direction2D {
            x: DECREASING,
            y: INCREASING,
        }
    }
    fn upright() -> Direction2D {
        Direction2D {
            x: INCREASING,
            y: INCREASING,
        }
    }
    fn downleft() -> Direction2D {
        Direction2D {
            x: DECREASING,
            y: DECREASING,
        }
    }
    fn downright() -> Direction2D {
        Direction2D {
            x: INCREASING,
            y: INCREASING,
        }
    }
}

type Index = (u8, u8);

pub struct Board {
    tokens: HashMap<Index, Token>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tokens: HashMap::new(),
        }
    }

    fn cell_to_char(&self, x: u8, y: u8) -> char {
        match self.tokens.get(&(x, y)) {
            Some(color) => match color {
                Token::YELLOW => 'Y',
                Token::RED => 'R',
            },
            None => '_',
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut value = String::new();
        for x in 1..=WIDTH {
            for y in 1..=HEIGHT {
                value.push(self.cell_to_char(x, y));
            }
        }
        value
    }

    fn cell_to_number(&self, x: u8, y: u8) -> u128 {
        match self.tokens.get(&(x, y)) {
            Some(color) => match color {
                Token::YELLOW => 1,
                Token::RED => 2,
            },
            None => 0,
        }
    }

    pub fn to_number(&self) -> u128 {
        let mut value = 0;
        for x in 1..=WIDTH {
            for y in 1..=HEIGHT {
                value +=
                    (3 as u128).pow(((y - 1) * WIDTH + (x - 1)) as u32) * self.cell_to_number(x, y);
            }
        }
        value
    }

    pub fn from_number(value: u128) -> Board {
        let mut tokens = HashMap::new();
        let mut current_value = value;

        for y in 1..=HEIGHT {
            for x in 1..=WIDTH {
                let current_cell = current_value % 3;
                match current_cell {
                    1 => {
                        tokens.insert((x, y), Token::YELLOW);
                    }
                    2 => {
                        tokens.insert((x, y), Token::RED);
                    }
                    _ => (),
                }
                current_value = current_value / 3;
            }
        }

        Board { tokens: tokens }
    }

    pub fn add_token(&mut self, column: u8, color: &Token) -> Result<bool, &str> {
        if column < 1 || column > WIDTH {
            return Result::Err("out of bounds");
        }

        for h in 1..=HEIGHT {
            if !self.tokens.contains_key(&(column, h)) {
                self.tokens.insert((column, h), color.clone());
                return Result::Ok(true);
            }
        }

        Result::Ok(false)
    }

    pub fn get_color_at_cell(&self, x: u8, y: u8) -> Option<&Token> {
        self.tokens.get(&(x, y))
    }

    fn linear_step(coord: u8, dir: &Direction, bound: u8) -> Option<u8> {
        match match dir {
            STABLE => coord,
            INCREASING => coord + 1,
            DECREASING => coord - 1,
        } {
            x if x >= 1 && x <= bound => Some(x),
            _ => None,
        }
    }

    fn step((x, y): Index, dir: &Direction2D) -> Option<Index> {
        match (
            Board::linear_step(x, &dir.x, WIDTH),
            Board::linear_step(y, &dir.y, HEIGHT),
        ) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }

    pub fn have_winner_at_column(&self, column: u8) -> bool {
        for i in 1..=HEIGHT {
            let row = HEIGHT + 1 - i;
            if self.tokens.contains_key(&(column, row)) {
                return self.have_winner_at_index((column, row));
            }
        }
        false
    }

    fn have_winner_at_index(&self, idx: Index) -> bool {
        match self.tokens.get(&idx) {
            Some(color) => {
                // For every possible direction we count how many contiguous tokens of the same color
                // are present. To win we have to get at least 5 as the starting point will be counted
                // in both directions. Not the cleanest way.

                // Check horizontal
                if (self.count_same_color_in_direction(Some(idx), &Direction2D::left(), color)
                    + self.count_same_color_in_direction(Some(idx), &Direction2D::right(), color))
                    >= 5
                {
                    return true;
                }
                // Check vertical
                if (self.count_same_color_in_direction(Some(idx), &Direction2D::up(), color)
                    + self.count_same_color_in_direction(Some(idx), &Direction2D::down(), color))
                    >= 5
                {
                    return true;
                }

                // Check downward
                if (self.count_same_color_in_direction(Some(idx), &Direction2D::upleft(), color)
                    + self.count_same_color_in_direction(
                        Some(idx),
                        &Direction2D::downright(),
                        color,
                    ))
                    >= 5
                {
                    return true;
                }

                // Check upward
                if (self.count_same_color_in_direction(Some(idx), &Direction2D::upright(), color)
                    + self.count_same_color_in_direction(
                        Some(idx),
                        &Direction2D::downleft(),
                        color,
                    ))
                    >= 5
                {
                    return true;
                }

                false
            }
            None => false,
        }
    }

    fn count_same_color_in_direction(
        &self,
        idx: Option<Index>,
        dir: &Direction2D,
        color: &Token,
    ) -> u8 {
        if let Some(index) = idx {
            match self.tokens.get(&index) {
                Some(c) if c == color => {
                    1 + self.count_same_color_in_direction(Board::step(index, &dir), dir, color)
                }
                _ => 0,
            }
        } else {
            0
        }
    }

    pub fn is_full(&self) -> bool {
        self.tokens.len() == (WIDTH * HEIGHT) as usize
    }

    pub fn reset(&mut self) {
        self.tokens.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_winner() {
        let mut board = Board::new();
        for i in 1..=WIDTH {
            assert!(board.have_winner_at_column(i) == false);
        }
        board
            .add_token(4, &Token::YELLOW)
            .expect("could not add token");
        assert!(board.have_winner_at_column(4) == false);
        board
            .add_token(4, &Token::YELLOW)
            .expect("could not add token");
        assert!(board.have_winner_at_column(4) == false);
        board
            .add_token(4, &Token::YELLOW)
            .expect("could not add token");
        assert!(board.have_winner_at_column(4) == false);
        board
            .add_token(4, &Token::RED)
            .expect("could not add token");
        assert!(board.have_winner_at_column(4) == false);
        board
            .add_token(2, &Token::YELLOW)
            .expect("could not add token");
        assert!(board.have_winner_at_column(2) == false);
        board
            .add_token(2, &Token::YELLOW)
            .expect("could not add token");
        assert!(board.have_winner_at_column(2) == false);
        board
            .add_token(2, &Token::YELLOW)
            .expect("could not add token");
        assert!(board.have_winner_at_column(2) == false);
        board
            .add_token(2, &Token::YELLOW)
            .expect("could not add token");
        assert!(board.have_winner_at_column(2) == true);
    }

    #[test]
    fn test_is_full() {
        let mut board = Board::new();
        assert!(!board.is_full());
        for column in 1..=WIDTH {
            for _ in 1..=HEIGHT {
                assert!(!board.is_full());
                board
                    .add_token(column, &Token::YELLOW)
                    .expect("could not add token");
            }
        }
        assert!(board.is_full());
    }

    #[test]
    fn test_serializing() {
        let mut board = Board::new();
        let serialized = board.to_number();
        assert!(serialized == 0);
        assert!(Board::from_number(serialized).to_number() == serialized);
        for column in 1..=WIDTH {
            for row in 1..=HEIGHT {
                if (row + column) % 2 == 0 {
                    board
                        .add_token(column, &Token::YELLOW)
                        .expect("could not add token");
                } else {
                    board
                        .add_token(column, &Token::RED)
                        .expect("could not add token");
                }
            }
        }
        let board_full = Board::from_number(board.to_number());
        assert!(board_full.is_full());
        assert!(board_full.to_number() == board.to_number());
        assert!(board_full.to_number() > 1);
    }

    #[test]
    fn test_to_string() {
        let mut board = Board::new();
        assert!(board.to_string() == String::from("__________________________________________"));
        board
            .add_token(1, &Token::YELLOW)
            .expect("Could not add token");
        assert!(board.to_string() == String::from("Y_________________________________________"));
        board
            .add_token(1, &Token::RED)
            .expect("Could not add token");
        assert!(board.to_string() == String::from("YR________________________________________"));
        board
            .add_token(2, &Token::RED)
            .expect("Could not add token");
        assert!(board.to_string() == String::from("YR____R___________________________________"));
    }
}
