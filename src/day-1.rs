#![allow(dead_code)]

use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day-1").unwrap();

    let result_1 = max_calories(&input);
    let result_2 = max_3_calories(&input);

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
}

// Part 1

fn max_calories(input: &str) -> u32 {
    input.split("\n\n").map(sum_calories).max().unwrap()
}

// Part 2

fn max_3_calories(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(sum_calories)
        .fold([0, 0, 0], |mut acc, num| {
            let (idx, min_value) = acc
                .iter()
                .enumerate()
                .min_by_key(|(_, &value)| value)
                .unwrap();

            if min_value < &num {
                acc[idx] = num;
            }

            acc
        })
        .iter()
        .sum()
}

// Helper

fn sum_calories(input: &str) -> u32 {
    input.lines().filter_map(|n| n.parse::<u32>().ok()).sum()
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    mod sum_calories {
        use super::*;
        use indoc::indoc;

        #[test]
        fn it_gives_back_a_single_number() {
            let input = "1000";

            assert_eq!(sum_calories(input), 1000);
        }

        #[test]
        fn it_gives_back_the_sum_of_two_numbers() {
            let input = indoc! {"
                            1000
                            2000
                        "};

            assert_eq!(sum_calories(input), 3000);
        }
    }

    mod most_calories {
        use super::*;
        use indoc::indoc;

        #[test]
        fn it_gives_back_a_single_number() {
            let input = "1000";

            assert_eq!(max_calories(input), 1000);
        }

        #[test]
        fn it_gives_back_the_bigger_of_two_single_numbers() {
            let input = indoc! {"
                            1000

                            2000
                        "};

            assert_eq!(max_calories(input), 2000);
        }

        #[test]
        fn example() {
            let input = indoc! {"
                            1000
                            2000
                            3000

                            4000

                            5000
                            6000

                            7000
                            8000
                            9000

                            10000
                        "};

            assert_eq!(max_calories(input), 24000);
        }
    }
}
