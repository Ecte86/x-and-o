// Noughts and Crosses game with a simple AI opponent

use anyhow::{anyhow, bail, Result as AnyResult};

use modules::{game::Game, player_symbol::PlayerSymbol};
use std::{
    io::{self, Write},
    ops::Range,
};

mod modules;

use crate::modules::{
    game_mode,
    input_error::{self, InputError},
    player_symbol,
};

fn main() -> AnyResult<()> {
    run()?;
    Ok(())
}
fn run() -> AnyResult<()> {
    title();
    help();
    let mut game = Game::new();
    let board_range = [(0..3), (0..3)];
    // TODO: stop asking for 2nd player's move if the user enters
    //       single player mode, and setup Greedy AI for
    //       it.
    // loop to get how many players and the symbol of player 1
    let num_players = ask_for_num_players(&mut game, board_range.clone())?;
    let player_1_symbol = ask_symbol_for_player_1(&mut game, board_range.clone())?;
    if num_players == 1 {
        game.set_game_mode(game_mode::GameMode::SinglePlayer);
    } else {
        game.set_game_mode(game_mode::GameMode::TwoPlayer);
    }

    game.players_mut()[0].set_symbol(player_1_symbol);
    game.players_mut()[1].set_symbol(player_1_symbol.opposite());
    play_game(&mut game, board_range)?;
    Ok(())
}

fn ask_for_num_players(game: &mut Game, board_range: [Range<u8>; 2]) -> AnyResult<u8> {
    let mut valid_input = false;
    let prompt = "How many players? (1 or 2): ";
    let input = String::new();
    while !valid_input {
        match get_string_input(prompt.to_string()) {
            Ok(input) => {
                if let Err(e) = parse_input(&input) {
                    println!("Invalid input: {}", e);
                    continue;
                }
                if input == "1" || input == "2" {
                    break;
                } else {
                    valid_input = false;
                    println!("Invalid input");
                    continue;
                }
            }
            Err(e) if e.to_string() == anyhow!("Invalid input").to_string() => {
                println!("Invalid input");
                continue;
            }
            Err(e) => anyhow::bail!(e),
        }
    }
    // Once we get here, we know the input is valid
    if input == "1" {
        Ok(1)
    } else if input == "2" {
        Ok(2)
    } else {
        bail!(anyhow!("Invalid input"));
    }
}
fn ask_symbol_for_player_1(
    game: &mut Game,
    board_range: [Range<u8>; 2],
) -> AnyResult<PlayerSymbol> {
    let mut valid_input = false;
    let prompt = "Symbol for player 1? (X or O): ";
    let input = String::new();
    while !valid_input {
        match get_string_input(prompt.to_string()) {
            Ok(mut input) => {
                if let Err(e) = parse_input(&input) {
                    println!("Invalid input: {}", e);
                    continue;
                } else {
                    input = input.trim().to_string();
                }
            }
            Err(e) if e.to_string() == anyhow!("Invalid input").to_string() => {
                println!("Invalid input");
                continue;
            }
            Err(e) => anyhow::bail!(e),
        }
        valid_input = true;
    }
    // Once we get here, we know the input is valid
    let player_symbol = if input == "X" {
        PlayerSymbol::Cross
    } else {
        PlayerSymbol::Nought
    };
    Ok(player_symbol)
}

fn help() {
    println!("Enter coordinates in the format x,y.");
    println!("Enter q to quit.\n");
}

fn title() {
    let title = "Noughts and Crosses";
    println!("{:-^1$}\n", title, 80);
}

fn play_game(game: &mut Game, board_range: [Range<u8>; 2]) -> AnyResult<()> {
    let current_player_symbol = game.current_player().symbol();
    while !game.game_over() {
        let coords: (u8, u8) = (255, 255);
        let valid_input = false;

        do_current_players_turn(
            game,
            current_player_symbol,
            valid_input,
            coords,
            &board_range,
        )?;
        if game.board_mut().is_game_over() {
            break;
        }
        game.get_next_player();
    }
    if game.winner().is_some() {
        println!("Player {} wins!", game.winner().unwrap());
    } else {
        println!("It's a draw!");
    }
    Ok(())
}

fn do_current_players_turn(
    game: &mut Game,
    current_player_symbol: player_symbol::PlayerSymbol,
    valid_input: bool,
    coords: (u8, u8),
    board_range: &[Range<u8>; 2],
) -> Result<(), anyhow::Error> {
    println!("{}", game.board());
    // ask for and make move
    // TODO: rewrite this so that it's not so ugly, and so that
    //       it works for both single and two player modes

    // if game.game_mode() == &game_mode::GameMode::SinglePlayer
    //     && game.players()[game.current_player_index()] == game.players()[1] {
    // let prompt = format!("Player {}'s turn: ", current_player_symbol);
    // make_move(valid_input, prompt, board_range, coords, game)?;
    // Ok(())
    //     } else {
    //         let prompt = format!("Player {}'s turn: ", current_player_symbol);
    //         make_move(valid_input, prompt, board_range, coords, game)?;
    //         Ok(())
    //     }
}

fn make_move(
    mut valid_input: bool,
    prompt: String,
    board_range: &[Range<u8>; 2],
    mut coords: (u8, u8),
    game: &mut Game,
) -> Result<(), anyhow::Error> {
    while !valid_input {
        if let Err(e) = get_string_input(prompt.clone()) {
            if e.to_string() == anyhow!("Invalid input").to_string() {
                println!("Invalid input");
                valid_input = false;
                continue;
            } else {
                return Err(e);
            }
        } else {
            let parse_result = parse_input(&prompt);
            match parse_result {
                Err(e) if e.to_string() == "Invalid input" => {
                    bail!(InputError::InvalidInput);
                }
                Err(e) if e.to_string() == "Invalid coordinates" => {
                    bail!(InputError::InvalidCoordinates);
                }
                Err(e) => bail!(e),
                Ok((x_, y_)) => {
                    let x_in_range = board_range[0].contains(&x_);
                    let y_in_range = board_range[1].contains(&y_);
                    if !(x_in_range && y_in_range) {
                        bail!(InputError::InvalidCoordinates);
                    }
                    coords.0 = x_;
                    coords.1 = y_;
                }
            }
            if game.board().get(coords.0, coords.1).is_some() {
                println!("Position already taken");
                valid_input = false;
                continue;
            } else {
                valid_input = true;
                let current_player = game.current_player();
                game.board_mut().set(coords.0, coords.1, current_player)?;
            }
        }
    }
    Ok(())
}

fn get_string_input(prompt: String) -> AnyResult<String> {
    let mut input = String::new();
    if prompt.ends_with(' ') || prompt.ends_with(':') || prompt.ends_with('?') {
        print!("{}", prompt);
        io::stdout().flush()?;
    } else {
        println!("{}", prompt);
    }
    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(anyhow!(e));
    }
    let input_chars = input.trim().replace(' ', "").chars().collect::<Vec<char>>();
    let input = input.trim();
    if input.is_empty() || !(3..=4).contains(&input.len()) {
        bail!(input_error::InputError::InvalidInput);
    }
    if !(input_chars[0].is_numeric() && input_chars[1] == ',' && input_chars[2].is_numeric()) {
        bail!(input_error::InputError::InvalidCoordinates);
    }

    Ok(input.to_string())
}

fn parse_input(input: &str) -> AnyResult<(u8, u8)> {
    let input = input.trim().replace(' ', "");
    let mut parts = input.split(',').collect::<Vec<&str>>();
    let x = parts
        .pop()
        .ok_or(anyhow::anyhow!("Missing x coordinate"))?
        .to_string();
    let y = parts
        .pop()
        .ok_or(anyhow::anyhow!("Missing y coordinate"))?
        .to_string();
    let x = x.parse::<u8>()?;
    let y = y.parse::<u8>()?;
    if (1..=3).contains(&x) && (1..=3).contains(&y) {
        Ok((x - 1, y - 1))
    } else {
        Err(anyhow::anyhow!("Invalid input"))
    }
}
