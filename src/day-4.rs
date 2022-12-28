#![allow(dead_code)]

use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day-4").unwrap();

    let result_1 = number_of_full_overlaps(&input);
    let result_2 = number_of_partial_overlaps(&input);

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
}

// PART 1

fn number_of_full_overlaps(input: &str) -> u32 {
    input
        .lines()
        .map(|l| ElfPair::from(l).has_full_overlap())
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

// PART 2

fn number_of_partial_overlaps(input: &str) -> u32 {
    input
        .lines()
        .map(|l| ElfPair::from(l).has_overlap())
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

// HELPER FUNCTIONS

#[derive(PartialEq, Debug)]
struct AssignedSections {
    start: u32,
    end: u32,
}

impl From<&str> for AssignedSections {
    fn from(str: &str) -> Self {
        let (start, end) = str.split_once('-').unwrap();

        AssignedSections {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Debug)]
struct ElfPair {
    first: AssignedSections,
    second: AssignedSections,
}

impl From<&str> for ElfPair {
    fn from(str: &str) -> Self {
        let (first, second) = str.split_once(',').unwrap();

        ElfPair {
            first: first.into(),
            second: second.into(),
        }
    }
}

impl ElfPair {
    fn has_full_overlap(&self) -> bool {
        let ElfPair { first, second } = self;

        let sections_first = HashSet::<u32>::from_iter(first.start..=first.end);
        let sections_second = HashSet::<u32>::from_iter(second.start..=second.end);

        sections_first.is_subset(&sections_second) || sections_first.is_superset(&sections_second)
    }

    fn has_overlap(&self) -> bool {
        let ElfPair { first, second } = self;

        let sections_first = HashSet::<u32>::from_iter(first.start..=first.end);
        let sections_second = HashSet::<u32>::from_iter(second.start..=second.end);

        !sections_first.is_disjoint(&sections_second)
    }
}

// TESTS

#[cfg(test)]
mod tests_day4 {
    use super::*;

    mod test_number_of_full_overlaps {
        use super::*;
        use indoc::indoc;

        #[test]
        fn example() {
            let input = indoc! {"
                2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8
            "};

            assert_eq!(number_of_full_overlaps(&input), 2);
        }
    }

    mod test_number_of_fully_contains {
        use super::*;
        use indoc::indoc;

        #[test]
        fn example() {
            let input = indoc! {"
                2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8
            "};

            assert_eq!(number_of_partial_overlaps(&input), 4);
        }
    }

    mod test_assigend_sections {
        use super::*;

        #[test]
        fn it_can_be_converted_from_string() {
            let input = "1-2";
            let expected = AssignedSections { start: 1, end: 2 };

            assert_eq!(AssignedSections::from(input), expected)
        }
    }

    mod test_elf_pair {
        use super::*;

        #[test]
        fn it_can_be_converted_from_string() {
            let input = "1-2,3-4";
            let expected = ElfPair {
                first: AssignedSections { start: 1, end: 2 },
                second: AssignedSections { start: 3, end: 4 },
            };

            assert_eq!(ElfPair::from(input), expected)
        }

        #[test]
        fn it_reports_no_full_overlap_on_no_overlap() {
            let elf_pair = ElfPair::from("1-2,3-4");

            assert_eq!(elf_pair.has_full_overlap(), false)
        }

        #[test]
        fn it_reports_full_overlap_on_equal_sections() {
            let elf_pair = ElfPair::from("1-4,1-4");

            assert_eq!(elf_pair.has_full_overlap(), true)
        }

        #[test]
        fn it_reports_full_overlap_if_first_is_superset() {
            let elf_pair = ElfPair::from("1-4,3-4");

            assert_eq!(elf_pair.has_full_overlap(), true)
        }

        #[test]
        fn it_reports_full_overlap_if_second_is_superset() {
            let elf_pair = ElfPair::from("1-2,1-4");

            assert_eq!(elf_pair.has_full_overlap(), true)
        }

        #[test]
        fn it_reports_no_overlap_on_no_overlap() {
            let elf_pair = ElfPair::from("1-2,3-4");

            assert_eq!(elf_pair.has_overlap(), false)
        }

        #[test]
        fn it_reports_overlap_on_equal_sections() {
            let elf_pair = ElfPair::from("1-4,1-4");

            assert_eq!(elf_pair.has_overlap(), true)
        }

        #[test]
        fn it_reports_overlap_on_partial_overlap() {
            let elf_pair = ElfPair::from("1-3,3-4");

            assert_eq!(elf_pair.has_overlap(), true)
        }
    }
}
