extern crate pancurses;
use super::board;
use std::time;

pub struct UI {
    board: board::Board,
    window: pancurses::Window,
    score_yellow: u8,
    score_red: u8,
    cursor_position: u8,
    current_player: board::Token,
}

impl UI {
    const INPUT_TIMEOUT: i32 = 500;

    pub fn new() -> UI {
        UI {
            board: board::Board::new(),
            window: pancurses::initscr(),
            score_red: 0,
            score_yellow: 0,
            cursor_position: 1,
            current_player: board::Token::YELLOW,
        }
    }

    fn draw_horizontal_line(&self) {
        self.window.attrset(pancurses::COLOR_PAIR(1));
        for _ in 1..=board::WIDTH {
            self.window.addch('-');
            self.window.addch('-');
            self.window.addch('-');
        }
        self.window.addch('-');
        self.window.addch('\n');
    }

    fn draw(&self) {
        self.window.clear();
        self.window.printw("Connect Four Game\n\n");

        // Draw players scores
        if self.current_player == board::Token::YELLOW {
            self.window.addstr("\u{2192}");
        } else {
            self.window.addstr(" ");
        }
        self.window
            .printw(format!(" Player 1: {:02} ", self.score_yellow));
        self.window.attrset(pancurses::COLOR_PAIR(2));
        self.window.addstr("  \n");
        self.window.attrset(pancurses::COLOR_PAIR(1));

        if self.current_player == board::Token::RED {
            self.window.addstr("\u{2192}");
        } else {
            self.window.addstr(" ");
        }
        self.window
            .printw(format!(" Player 2: {:02} ", self.score_red));
        self.window.attrset(pancurses::COLOR_PAIR(3));
        self.window.addstr("  \n");
        self.window.attrset(pancurses::COLOR_PAIR(1));

        for i in 1..=board::WIDTH {
            self.window.addch(' ');
            if i == self.cursor_position {
                self.window.addstr(" \u{2193}");
            } else {
                self.window.addstr("  ");
            }
        }
        self.window.addch('\n');

        // Draw grid
        for i in 1..=board::HEIGHT {
            let h = board::HEIGHT - i + 1;
            self.draw_horizontal_line();

            for x in 1..=board::WIDTH {
                self.window.attrset(pancurses::COLOR_PAIR(1));
                self.window.addch('|');

                match self.board.get_color_at_cell(x, h) {
                    Some(board::Token::RED) => {
                        self.window.attrset(pancurses::COLOR_PAIR(3));
                    }
                    Some(board::Token::YELLOW) => {
                        self.window.attrset(pancurses::COLOR_PAIR(2));
                    }
                    None => (),
                }

                self.window.addch(' ');
                self.window.addch(' ');
            }
            self.window.attrset(pancurses::COLOR_PAIR(1));
            self.window.addch('|');
            self.window.addch('\n');
        }
        self.draw_horizontal_line();

        // Footer with instructions
        self.window.addstr("\nPress \u{2190} and \u{2192} to move the arrow, SPACE to play a piece, Q or ESC to quit\n");

        self.window.refresh();
    }

    fn move_left(&mut self) {
        if self.cursor_position > 1 {
            self.cursor_position -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor_position < board::WIDTH {
            self.cursor_position += 1;
        }
    }

    fn reset(&mut self) {
        std::thread::sleep(time::Duration::from_secs(3));

        // Consume all input
        self.window.timeout(0);
        loop {
            match self.window.getch() {
                Some(_) => (),
                None => break,
            }
        }
        self.window.timeout(UI::INPUT_TIMEOUT);
        self.board.reset();
    }

    fn drop_token(&mut self) {
        match self
            .board
            .add_token(self.cursor_position, self.current_player)
        {
            Ok(true) => {
                // ToDo: add an animation?

                // Check if current player won
                if self.board.have_winner_at_column(self.cursor_position) {
                    match self.current_player {
                        board::Token::YELLOW => self.score_yellow += 1,
                        board::Token::RED => self.score_red += 1,
                    }

                    self.draw();

                    match self.window.subwin(13, 13, 10, 15) {
                        Ok(victory_window) => {
                            victory_window.addstr("*************");
                            victory_window.addstr("*           *");
                            victory_window.addstr("* ");

                            match self.current_player {
                                board::Token::YELLOW => {
                                    victory_window.addstr("PLAYER 1");
                                    victory_window.attrset(pancurses::COLOR_PAIR(2));
                                    victory_window.addstr(" ");
                                }
                                board::Token::RED => {
                                    victory_window.addstr("PLAYER 2");
                                    victory_window.attrset(pancurses::COLOR_PAIR(3));
                                    victory_window.addstr(" ");
                                }
                            }
                            victory_window.attrset(pancurses::COLOR_PAIR(1));
                            victory_window.addstr(" *");

                            victory_window.addstr("* VICTORY!! *");
                            victory_window.addstr("*           *");
                            victory_window.addstr("*************");
                            victory_window.refresh();
                        }
                        Err(_) => (),
                    }

                    self.reset();
                }

                // Check if there is a draw
                if self.board.is_full() {
                    self.draw();

                    match self.window.subwin(13, 13, 10, 15) {
                        Ok(draw_window) => {
                            draw_window.addstr("*************");
                            draw_window.addstr("*           *");
                            draw_window.addstr("*   DRAW!   *");
                            draw_window.addstr("*           *");
                            draw_window.addstr("*************");
                            draw_window.refresh();
                        }
                        Err(_) => (),
                    }
                    self.reset();
                }

                // Switch current player (note: we do even if a player won)
                self.current_player = match self.current_player {
                    board::Token::YELLOW => board::Token::RED,
                    board::Token::RED => board::Token::YELLOW,
                }
            }
            Ok(false) => (),
            Err(_) => (),
        }
    }

    pub fn run(&mut self) {
        self.window.refresh();
        self.window.keypad(true);
        pancurses::noecho();

        pancurses::start_color();
        pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
        pancurses::init_pair(2, pancurses::COLOR_YELLOW, pancurses::COLOR_YELLOW);
        pancurses::init_pair(3, pancurses::COLOR_RED, pancurses::COLOR_RED);

        self.window.timeout(UI::INPUT_TIMEOUT);

        loop {
            UI::draw(self);

            match self.window.getch() {
                Some(pancurses::Input::Character('\x1B')) => break,
                Some(pancurses::Input::Character('q')) => break,
                Some(pancurses::Input::KeyLeft) => self.move_left(),
                Some(pancurses::Input::KeyRight) => self.move_right(),
                Some(pancurses::Input::Character(' ')) => self.drop_token(),
                Some(_) => (),
                None => (),
            }
        }

        pancurses::endwin();
    }
}
