use super::board;
use pancurses::Window;
use std::collections::VecDeque;

pub enum Move {
    LEFT,
    RIGHT,
    DROP,
}

pub trait GameInterface {
    fn name(&self) -> String;

    fn play(
        &self,
        board: &board::Board,
        cursor_position: u8,
        player_color: board::Token,
        window: &Window,
    ) -> VecDeque<Move>;
}

pub type InterfaceObject = Box<dyn GameInterface>;
