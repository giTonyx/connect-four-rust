use crate::board::Direction::{DECREASING, INCREASING, STABLE};
use std::collections::HashMap;

pub const WIDTH: u8 = 7;
pub const HEIGHT: u8 = 6;

#[derive(Copy, Clone, PartialEq)]
pub enum Token {
    YELLOW,
    RED,
}

#[derive(Copy, Clone)]
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

    pub fn add_token(&mut self, column: u8, color: Token) -> Result<bool, &str> {
        if column < 1 || column > WIDTH {
            return Result::Err("out of bounds");
        }

        for h in 1..=HEIGHT {
            if !self.tokens.contains_key(&(column, h)) {
                self.tokens.insert((column, h), color);
                return Result::Ok(true);
            }
        }

        Result::Ok(false)
    }

    pub fn get_color_at_cell(&self, x: u8, y: u8) -> Option<&Token> {
        self.tokens.get(&(x, y))
    }

    fn linear_step(coord: u8, dir: Direction, bound: u8) -> Option<u8> {
        match match dir {
            STABLE => coord,
            INCREASING => coord + 1,
            DECREASING => coord - 1,
        } {
            x if x < 1 => None,
            x if x > bound => None,
            x => Some(x),
        }
    }

    fn step(idx: Index, dir: &Direction2D) -> Option<Index> {
        match (
            Board::linear_step(idx.0, dir.x, WIDTH),
            Board::linear_step(idx.1, dir.y, HEIGHT),
        ) {
            (None, _) => None,
            (_, None) => None,
            (Some(x), Some(y)) => Some((x, y)),
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
        return match self.tokens.get(&idx) {
            Some(color) => {
                // For every possible direction we count how many contigous tokens of the same color
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
        };
    }

    fn count_same_color_in_direction(
        &self,
        idx: Option<Index>,
        dir: &Direction2D,
        color: &Token,
    ) -> u8 {
        match idx {
            Some(index) => match self.tokens.get(&index) {
                Some(c) => {
                    if c == color {
                        1 + self.count_same_color_in_direction(Board::step(index, &dir), dir, color)
                    } else {
                        0
                    }
                }
                None => 0,
            },
            None => 0,
        }
    }

    pub fn reset(&mut self) {
        self.tokens.clear();
    }
}
