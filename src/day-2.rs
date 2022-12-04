#![allow(dead_code)]

use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day-2").unwrap();

    let result_1 = calculate_score_1(&input);
    let result_2 = calculate_score_2(&input);

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
}

const WIN_POINTS: u32 = 6;
const DRAW_POINTS: u32 = 3;
const LOSE_POINTS: u32 = 0;
const ROCK_POINTS: u32 = 1;
const PAPER_POINTS: u32 = 2;
const SCISSORS_POINTS: u32 = 3;

fn calculate_score_1(games: &str) -> u32 {
    games
        .lines()
        .map(|game| match game {
            "A X" => ROCK_POINTS + DRAW_POINTS,
            "B X" => ROCK_POINTS + LOSE_POINTS,
            "C X" => ROCK_POINTS + WIN_POINTS,
            "A Y" => PAPER_POINTS + WIN_POINTS,
            "B Y" => PAPER_POINTS + DRAW_POINTS,
            "C Y" => PAPER_POINTS + LOSE_POINTS,
            "A Z" => SCISSORS_POINTS + LOSE_POINTS,
            "B Z" => SCISSORS_POINTS + WIN_POINTS,
            "C Z" => SCISSORS_POINTS + DRAW_POINTS,
            _ => 0,
        })
        .sum()
}

fn calculate_score_2(games: &str) -> u32 {
    games
        .lines()
        .map(|game| match game {
            "A X" => LOSE_POINTS + SCISSORS_POINTS,
            "B X" => LOSE_POINTS + ROCK_POINTS,
            "C X" => LOSE_POINTS + PAPER_POINTS,
            "A Y" => DRAW_POINTS + ROCK_POINTS,
            "B Y" => DRAW_POINTS + PAPER_POINTS,
            "C Y" => DRAW_POINTS + SCISSORS_POINTS,
            "A Z" => WIN_POINTS + PAPER_POINTS,
            "B Z" => WIN_POINTS + SCISSORS_POINTS,
            "C Z" => WIN_POINTS + ROCK_POINTS,
            _ => 0,
        })
        .sum()
}
