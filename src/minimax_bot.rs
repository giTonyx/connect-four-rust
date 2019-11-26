use super::interface;
use crate::board;
use crate::board::{Board, Token, WIDTH};
use crate::interface::Move;
use pancurses::Window;
use rand::Rng;
use std::collections::VecDeque;

pub struct MinimaxBot {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct BotMove {
    confidence: u8,
    position: u8,
}

const MAX_DEPTH: u8 = 4;

impl MinimaxBot {
    fn max(
        serialized_board: u128,
        player_color: Token,
        opponent_color: Token,
        depth: u8,
    ) -> BotMove {
        let mut moves = VecDeque::new();

        for x in 1..=WIDTH {
            //print!("Depth:{}, adding column {}\n", depth, x);
            let mut board = Board::from_number(serialized_board);
            //print!("Board: {}\n", board.to_string());
            if board
                .add_token(x, &player_color)
                .expect("Error adding token")
            {
                if board.have_winner_at_column(x) {
                    // Victory, we immediately return
                    return BotMove {
                        position: x,
                        confidence: 100,
                    };
                }
                if board.is_full() {
                    // Draw
                    moves.push_back(BotMove {
                        position: x,
                        confidence: 50,
                    });
                    continue;
                }
                if depth < MAX_DEPTH {
                    let recursive_move = MinimaxBot::min(
                        board.to_number(),
                        opponent_color.clone(),
                        player_color.clone(),
                        depth + 1,
                    );
                    if recursive_move.confidence == 100 {
                        return BotMove {
                            position: x,
                            confidence: 100,
                        };
                    }
                    moves.push_back(BotMove {
                        position: x,
                        confidence: recursive_move.confidence,
                    });
                } else {
                    moves.push_back(BotMove {
                        position: x,
                        confidence: 20 + rand::thread_rng().gen_range(1, 20),
                    });
                }
            }
        }
        match moves.iter().max() {
            Some(m) => m.clone(),
            None => BotMove {
                position: 0,
                confidence: 0,
            },
        }
    }

    fn min(
        serialized_board: u128,
        player_color: Token,
        opponent_color: Token,
        depth: u8,
    ) -> BotMove {
        let mut moves = VecDeque::new();

        for x in 1..=WIDTH {
            let mut board = Board::from_number(serialized_board);
            //print!("Min: Depth: {} Pos: {}\n", depth, x);
            if board
                .add_token(x, &player_color)
                .expect("Error adding token")
            {
                if board.have_winner_at_column(x) {
                    // Victory, we immediately return
                    return BotMove {
                        position: x,
                        confidence: 0,
                    };
                }
                if board.is_full() {
                    // Draw
                    moves.push_back(BotMove {
                        position: x,
                        confidence: 50,
                    });
                    continue;
                }
                if depth < MAX_DEPTH {
                    let recursive_move = MinimaxBot::max(
                        board.to_number(),
                        opponent_color.clone(),
                        player_color.clone(),
                        depth + 1,
                    );
                    if recursive_move.confidence == 0 {
                        return BotMove {
                            position: x,
                            confidence: 0,
                        };
                    }
                    moves.push_back(BotMove {
                        position: x,
                        confidence: recursive_move.confidence,
                    });
                } else {
                    moves.push_back(BotMove {
                        position: x,
                        confidence: 20 + rand::thread_rng().gen_range(1, 20),
                    });
                }
            }
        }
        match moves.iter().min() {
            Some(m) => m.clone(),
            None => BotMove {
                position: 0,
                confidence: 0,
            },
        }
    }
}

impl interface::GameInterface for MinimaxBot {
    fn name(&self) -> String {
        String::from("Bot")
    }

    fn play(
        &self,
        board: &board::Board,
        cursor_position: u8,
        color: Token,
        _: &Window,
    ) -> VecDeque<Move> {
        let mut moves = VecDeque::new();
        let opponent_color = match color {
            Token::YELLOW => Token::RED,
            Token::RED => Token::YELLOW,
        };

        let target_position =
            MinimaxBot::max(board.to_number(), color.clone(), opponent_color.clone(), 0).position;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max() {
        let mut board = Board::new();
        board
            .add_token(1, &Token::YELLOW)
            .expect("Could not add token");
        board
            .add_token(2, &Token::YELLOW)
            .expect("Could not add token");
        board
            .add_token(4, &Token::YELLOW)
            .expect("Could not add token");

        let bot_move = MinimaxBot::max(board.to_number(), Token::YELLOW, Token::RED, 0);
        assert!(bot_move.confidence == 100);
        assert!(bot_move.position == 3);
    }

    #[test]
    fn test_min() {
        let mut board = Board::new();
        board
            .add_token(1, &Token::YELLOW)
            .expect("Could not add token");
        board
            .add_token(2, &Token::YELLOW)
            .expect("Could not add token");
        board
            .add_token(4, &Token::YELLOW)
            .expect("Could not add token");

        let bot_move = MinimaxBot::min(board.to_number(), Token::YELLOW, Token::RED, 0);
        assert!(bot_move.confidence == 0);
        assert!(bot_move.position == 3);
    }

    #[test]
    fn test_minimax() {
        let mut board = Board::new();
        board
            .add_token(4, &Token::YELLOW)
            .expect("Could not add token");
        board
            .add_token(4, &Token::YELLOW)
            .expect("Could not add token");
        board
            .add_token(4, &Token::YELLOW)
            .expect("Could not add token");
        board
            .add_token(3, &Token::RED)
            .expect("Could not add token");
        board
            .add_token(7, &Token::RED)
            .expect("Could not add token");
        let bot_move = MinimaxBot::max(board.to_number(), Token::RED, Token::YELLOW, 0);
        assert!(bot_move.position == 4);
    }
}
