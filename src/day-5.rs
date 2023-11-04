#![allow(dead_code)]

use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day-4").unwrap();

    let result_1 = top_creates(&input);
    //let result_2 = number_of_partial_overlaps(&input);

    println!("Part 1: {result_1}");
    //println!("Part 2: {result_2}");
}

// PART 1

fn top_creates(input: &str) -> &str {
    "A"
}

// PART 2

//fn number_of_partial_overlaps(input: &str) -> u32 {}

// HELPER FUNCTIONS

struct CargoStorage {
    stacks: Vec<Vec<char>>,
}

impl CargoStorage {
    fn new(input: &str) -> Self {
        input.lines().rev().skip(1).for_each(|line| {
            //line.chars().collect::<Vec<char>>().chunks(4).map(parse_crate)
        });

        CargoStorage {
            stacks: vec![vec!['A']],
        }
    }
}

fn get_stack_count(line: &str) -> u8 {
    line.split_whitespace().count() as u8
}

fn parse_crate(crate_str: &str) -> char {
    'A'
}

// TESTS

#[cfg(test)]
mod tests_day5 {
    use super::*;

    mod test_number_of_full_overlaps {
        use super::*;
        use indoc::indoc;

        #[test]
        #[ignore]
        fn example() {
            let input = indoc! {"
                    [D]    
                [N] [C]    
                [Z] [M] [P]
                 1   2   3 

                move 1 from 2 to 1
                move 3 from 1 to 3
                move 2 from 2 to 1
                move 1 from 1 to 2
            "};

            assert_eq!(top_creates(&input), "CMZ");
        }
    }

    mod test_get_last_number {
        use super::*;

        #[test]
        fn it_returns_one_for_only_one_stack() {
            let input = " 1 ";

            assert_eq!(get_stack_count(&input), 1)
        }

        #[test]
        fn it_returns_a_higher_count() {
            let input = " 1  2  3 ";

            assert_eq!(get_stack_count(&input), 3)
        }
    }

    mod test_cargo_storage {
        use super::*;
        use indoc::indoc;

        #[test]
        fn it_creates_one_stack() {
            let input = indoc! {"
                [A]
                 1 
            "};

            let cargo_storage = CargoStorage::new(input);

            assert_eq!(cargo_storage.stacks[0][0], 'A')
        }

        #[test]
        fn it_creates_another_stack() {
            let input = indoc! {"
                [B]
                 1 
            "};

            let cargo_storage = CargoStorage::new(input);

            assert_eq!(cargo_storage.stacks[0][0], 'B')
        }
    }
}
