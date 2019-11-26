mod board;
mod human_controller;
mod interface;
mod minimax_bot;
mod random_bot;
mod ui;
extern crate clap;

use crate::human_controller::HumanController;
use crate::interface::InterfaceObject;
use crate::minimax_bot::MinimaxBot;
use crate::random_bot::RandomBot;

fn choose_player_type(param: Option<&str>) -> InterfaceObject {
    match param {
        Some("random") => Box::new(RandomBot {}) as InterfaceObject,
        Some("minimax") => Box::new(MinimaxBot {}) as InterfaceObject,
        _ => Box::new(HumanController {}) as InterfaceObject,
    }
}

fn main() {
    let arg_matches = clap::App::new("Connect 4 Rust")
        .arg(
            clap::Arg::with_name("player1_type")
                .long("player1")
                .short("1")
                .takes_value(true)
                .default_value("human")
                .possible_values(&["human", "random", "minimax"]),
        )
        .arg(
            clap::Arg::with_name("player2_type")
                .long("player2")
                .short("2")
                .takes_value(true)
                .default_value("human")
                .possible_values(&["human", "random", "minimax"]),
        )
        .get_matches();

    ui::UI::new(
        choose_player_type(arg_matches.value_of("player1_type")),
        choose_player_type(arg_matches.value_of("player2_type")),
    )
    .run();
}
