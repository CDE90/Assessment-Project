use rand::Rng;
use std::{cmp::Reverse, fs, io, thread, time};

fn get_input<F>(display_text: &str, check: F, error_text: &str) -> String
where
    F: Fn(&str) -> bool,
{
    // Gets input from the user

    // initialize the input string & loop condition
    let mut valid = false;
    let mut inp: String = String::new();

    // loop until the input is valid
    while !valid {
        inp = String::new();

        println!("{}", display_text);
        // get the input and store it in the input string
        if io::stdin().read_line(&mut inp).is_err() {
            println!("{}", error_text);
            continue;
        }

        // remove the newline character
        inp = inp.trim().to_string();

        // check if the input is valid
        if check(&inp) {
            valid = true;
        } else {
            // print the error message if the input is invalid
            println!("{}", error_text);
            continue;
        };
    }

    // return the input
    inp
}

fn get_players() -> Vec<(String, i8)> {
    // Gets the players from the user

    // players will be stored in a vector of tuples as (name, score)
    let mut players: Vec<(String, i8)> = Vec::new();

    // get a list of allowed players form file
    let data = fs::read_to_string("../allowed_names.txt").expect("Unable to read file");
    let allowed_names: Vec<&str> = data.split("\r\n").collect();

    // value to check if there is another player to add
    let mut another_player = true;

    // makes sure there's at least 2 players (no-one wants to be alone)
    // loop until there are at least 2 players or the user doesn't want to add any more
    while players.len() < 2 || another_player {
        // get the player's name
        let name = get_input(
            &format!("Enter the name of player {}", players.len() + 1),
            // check if the name is in the list of allowed names
            |s| allowed_names.contains(&s.to_lowercase().as_str()),
            "Invalid name. Please try again.",
        );

        // set the player's score to 0
        players.push((name, 0));

        // if there are less than 2 players another player will be added
        if players.len() >= 2 {
            // ask player if they want to add another player
            // returns boolean: true if yes, false if no
            another_player = ["y", "yes"].contains(
                &get_input(
                    "Do you want to add another player? (y/n) ",
                    |s| ["yes", "y", "no", "n"].contains(&s.to_lowercase().as_str()),
                    "Invalid input. Please try again.",
                )
                .to_lowercase()
                .as_str(),
            );
        }
    }

    println!(); // newline

    // return the players
    players
}

fn check_win(players: &Vec<(String, i8)>) -> bool {
    // Checks if there is a winner
    // returns boolean: true if there is a winner, false if not

    let mut winners: Vec<(String, i8)> = Vec::new();

    // loop through player names and scores
    for (name, score) in players {
        // if the score is greater than 10
        if *score > 10 {
            // add the player to the winners vector
            winners.push((name.to_string(), *score));
        }
    }

    match winners.len() {
        0 => false, // no winner
        1 => {
            // there is a single winner
            println!(
                "Game over! {} is the winner!",
                winners[0].0 // name of winner
            );
            true
        }
        _num => {
            // sort winners by score in descending order
            winners.sort_by_key(|k| Reverse(k.1));

            if winners[0].1 == winners[1].1 {
                // there is a draw between two players
                println!(
                    "Game over! It's a draw between {} and {}!",
                    winners[0].0,
                    winners[1].0 // names of players
                );
                true
            } else {
                // there is a winner
                println!(
                    "Game over! {} is the winner!",
                    winners[0].0 // gets the name of the winner (player with highest score)
                );
                true
            }
        }
    }
}

fn player_rolls(players: &Vec<(String, i8)>) -> Vec<(String, i8)> {
    // Rolls the dice for each player

    // initialise a new vector to store the updated players
    let mut new_players: Vec<(String, i8)> = Vec::new();

    // loop through the players
    for (name, score) in players {
        // roll the dice
        let roll = rand::thread_rng().gen_range(1..=6);

        /*
            calculate value to change score by
            roll of 1 will deduct points, 2 will leave points unchanged
            3, 4, 5 or 6 will add points
        */
        let change = roll - 2;

        // print the roll
        println!(
            "{} rolled a {}, their score will change by {}!",
            name, roll, change
        );

        // calculate the new score
        new_players.push((name.to_string(), score + change));
    }

    // return the updated players
    new_players
}

fn display_scores(players: &Vec<(String, i8)>) {
    // Displays the scores for each player

    // loop through the players
    for (name, score) in players {
        println!("{}'s score is {}", name, score);
    }

    println!(); // newline
}

fn game() -> bool {
    // Runs the game

    // get the players
    let mut players = get_players();

    let mut win = false;

    // loop until there is a winner
    while !win {
        // roll the dice
        players = player_rolls(&players);

        // display the scores
        display_scores(&players);

        // check if there is a winner
        // returns boolean: true if there is a winner, false if not
        win = check_win(&players);

        // wait 1 second before starting the next round
        thread::sleep(time::Duration::from_secs(1));
    }

    // returns boolean value: true to play again, false to quit
    ["y", "yes"].contains(
        &get_input(
            "Do you want to play again? (y/n) ",
            // check if the input is yes or no
            |s| ["yes", "y", "no", "n"].contains(&s.to_lowercase().as_str()),
            "Invalid input. Please try again.",
        )
        .to_lowercase()
        .as_str(),
    )
}

fn main() {
    // Starts the game & controls game repeats

    let mut play = true;

    // loop until the user wants to quit
    while play {
        // game() returns a boolean: true if the user wants to play again, false if not
        play = game();

        println!(); // newline
    }

    println!("Thanks for playing!");
}
