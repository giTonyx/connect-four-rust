use super::interface;
use crate::board;
use crate::interface::Move;
use pancurses::Window;
use rand::Rng;
use std::collections::VecDeque;

pub struct RandomBot {}

// Just a simple bot playing at random

impl interface::GameInterface for RandomBot {
    fn name(&self) -> String {
        String::from("Random Bot")
    }

    fn play(
        &self,
        _: &board::Board,
        cursor_position: u8,
        _: board::Token,
        _: &Window,
    ) -> VecDeque<Move> {
        let mut moves = VecDeque::new();
        let target_position = rand::thread_rng().gen_range(1, board::WIDTH + 1);
        if target_position > cursor_position {
            for _ in cursor_position..target_position {
                moves.push_back(Move::RIGHT);
            }
        } else {
            for _ in target_position..cursor_position {
                moves.push_back(Move::LEFT)
            }
        }
        moves.push_back(Move::DROP);
        return moves;
    }
}
