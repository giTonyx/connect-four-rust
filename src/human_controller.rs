use crate::interface::Move;
use crate::{board, interface};
use pancurses::Window;
use std::collections::VecDeque;

pub struct HumanController {}

impl interface::GameInterface for HumanController {
    fn name(&self) -> String {
        String::from("Human")
    }

    fn play(&self, _: &board::Board, _: u8, _: board::Token, window: &Window) -> VecDeque<Move> {
        let mut moves = VecDeque::new();

        loop {
            match window.getch() {
                Some(pancurses::Input::Character('\x1B')) => {
                    window.ungetch(&pancurses::Input::Character('\x1B'));
                    break;
                }
                Some(pancurses::Input::Character('q')) => {
                    window.ungetch(&pancurses::Input::Character('q'));
                    break;
                }
                Some(pancurses::Input::KeyLeft) => {
                    moves.push_back(Move::LEFT);
                    break;
                }
                Some(pancurses::Input::KeyRight) => {
                    moves.push_back(Move::RIGHT);
                    break;
                }
                Some(pancurses::Input::Character(' ')) => {
                    moves.push_back(Move::DROP);
                    break;
                }
                _ => (),
            }
        }
        moves
    }
}
