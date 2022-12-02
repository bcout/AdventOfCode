/*
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament 
    is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, 
    Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats 
    Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. 
    "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is 
    called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be 
    suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for 
    a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round 
    (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you 
        chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
*/

use std::collections::HashMap;

fn main(){
    // I want a dictionary of point values for each move and outcome
    // Seems a hashmap is the way to go
    // 
    // I use String instead of &str so that I can index the hashmap later using a string variable
    let mut points: HashMap<String, i32> = HashMap::new();
    populate_hashmap(&mut points);

    let mut total_points = 0;
    let input = include_str!("../input.txt");

    // Loop through every round in the strategy guide
    for line in input.split("\n"){
        if line.is_empty(){
            continue;
        }
        // We know the input is in ASCII, so we can read it as bytes to let us index the string
        let line_as_bytes = line.as_bytes();
        // The opponent's move is the first character in the line, ours is the third
        let opponent_move = line_as_bytes[0] as char; 
        let player_move = line_as_bytes[2] as char;

        let status: i32 = determine_round_outcome(opponent_move, player_move, &points);
        
        total_points = total_points + points[&player_move.to_string()];
        total_points = total_points + points[&status.to_string()];
    }

    println!("Total points: {}", total_points);
}

fn determine_round_outcome(opponent_move: char, player_move: char, points: &HashMap<String, i32>) -> i32{
    let mut player_move_points: i32 = points[&player_move.to_string()];
    let mut opponent_move_points: i32 = points[&opponent_move.to_string()];
    player_move_points = player_move_points - 1;
    opponent_move_points = opponent_move_points - 1;
    
    //https://stackoverflow.com/a/2795421
    // I'm stealing a one-liner for this logic
    let status: i32 = (3 + player_move_points - opponent_move_points) % 3;

    // Status will be 1 if player wins, 2 if opponent wins, 0 for a draw
    return status;
}

fn populate_hashmap(points: &mut HashMap<String, i32> ){
    // Insert player moves
    points.insert("X".to_string(), 1); // rock
    points.insert("Y".to_string(), 2); // paper
    points.insert("Z".to_string(), 3); //scissors
    // Insert opponent moves
    points.insert("A".to_string(), 1); // rock
    points.insert("B".to_string(), 2); // paper
    points.insert("C".to_string(), 3); //scissors
    // Insert outcomes
    points.insert("1".to_string(), 6); // win
    points.insert("2".to_string(), 0); // lose
    points.insert("0".to_string(), 3); // draw
}