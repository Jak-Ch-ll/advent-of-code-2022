#![allow(dead_code)]

use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day-3").unwrap();

    let result_1 = calculate_priority_of_rucksacks(&input);
    let result_2 = calculate_priority_of_groups(&input);

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
}

// PART 1

fn calculate_priority_of_rucksacks(input: &str) -> u32 {
    input
        .lines()
        .map(find_intersection_in_rucksack)
        .map(char_to_priority)
        .sum()
}

// PART 2

fn calculate_priority_of_groups(input: &str) -> u32 {
    split_text_into_lines_of_3(input)
        .map(find_intersection_in_group)
        .map(char_to_priority)
        .sum()
}

// HELPER FUNCTIONS

fn find_intersection_in_group(group: &str) -> char {
    group
        .lines()
        .map(|rucksack| HashSet::from_iter(rucksack.chars()))
        //.map(|rucksack| rucksack.chars())
        .reduce(|accum, current| HashSet::<_>::from_iter(accum.intersection(&current).cloned()))
        .unwrap()
        .drain()
        .next()
        .unwrap()
}

fn find_intersection_in_rucksack(rucksack: &str) -> char {
    let half_len = rucksack.len() / 2;

    let (first_half_rucksack, second_half_rucksack) = rucksack.split_at(half_len);

    let first_half_chars = HashSet::<_>::from_iter(first_half_rucksack.chars());
    let second_half_chars = HashSet::<_>::from_iter(second_half_rucksack.chars());

    *first_half_chars
        .intersection(&second_half_chars)
        .next()
        .unwrap()
}

fn char_to_priority(char: char) -> u32 {
    match char {
        ('a'..='z') => (char as u32) - 96,
        ('A'..='Z') => (char as u32) - 38,
        _ => 0,
    }
}

fn split_text_into_lines_of_3(text: &str) -> impl Iterator<Item = &str> {
    let mut newline_counter = 0;

    text.split(move |c| {
        if c != '\n' {
            return false;
        }

        if newline_counter == 2 {
            newline_counter = 0;
            return true;
        }

        newline_counter += 1;
        return false;
    })
    .filter(|text| !text.is_empty())
}

// TESTS

#[cfg(test)]
mod tests_day3 {
    use super::*;

    mod test_calculate_priority_of_groups {
        use super::*;
        use indoc::indoc;

        #[test]
        fn example() {
            let input = indoc! {"
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw
            "};

            assert_eq!(calculate_priority_of_groups(&input), 70);
        }
    }

    mod test_find_intersection_in_group {
        use super::*;
        use indoc::indoc;

        #[test]
        fn it_finds_the_intersection() {
            let input = indoc! {"
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
            "};

            assert_eq!(find_intersection_in_group(input), 'r');
        }
    }

    mod test_split_text_into_lines_of_3 {
        use super::*;
        use indoc::indoc;

        #[test]
        fn it_splits_into_lines_of_3() {
            let input = indoc! {"
                aa
                bb
                cc
                dd
                ee
                ff
            "};

            let expected_output_1 = indoc! {"
                aa
                bb
                cc"};

            let expected_output_2 = indoc! {"
                dd
                ee
                ff"};

            let mut output_iterator = split_text_into_lines_of_3(input);
            let output_1 = output_iterator.next().unwrap();
            let output_2 = output_iterator.next().unwrap();
            let should_be_none = output_iterator.next();

            assert_eq!(output_1, expected_output_1);
            assert_eq!(output_2, expected_output_2);
            assert_eq!(should_be_none, None);
        }
    }

    mod test_calculate_priority_of_rucksacks {
        use super::*;
        use indoc::indoc;

        #[test]
        fn example() {
            let input = indoc! {"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "};

            assert_eq!(calculate_priority_of_rucksacks(&input), 157);
        }
    }

    mod test_find_intersection_in_rucksack {
        use super::*;

        #[test]
        fn it_finds_the_intersection_of_single_letters() {
            assert_eq!(find_intersection_in_rucksack("aa"), 'a');
        }

        #[test]
        fn it_finds_the_intersection_of_example_rucksack() {
            assert_eq!(
                find_intersection_in_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"),
                'p'
            );
        }
    }

    mod test_char_to_priority {
        use super::*;

        #[test]
        fn it_transforms_a_to_1() {
            assert_eq!(char_to_priority('a'), 1);
        }

        #[test]
        fn it_transforms_z_to_26() {
            assert_eq!(char_to_priority('z'), 26);
        }

        #[test]
        fn it_transforms_capital_a_to_27() {
            assert_eq!(char_to_priority('A'), 27);
        }

        #[test]
        fn it_transforms_capital_z_to_52() {
            assert_eq!(char_to_priority('Z'), 52);
        }
    }
}
