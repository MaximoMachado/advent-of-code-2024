use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::iter;

///////////////////////////////////////////////////////////////////////////////

fn parse_two_lists(solution_1_text: &str) -> (Vec<i64>, Vec<i64>) {
    let mut first_list = vec![];
    let mut second_list = vec![];
    solution_1_text.split("\n").for_each(|line| {
        line.split_whitespace()
            .collect::<Vec<_>>()
            .windows(2)
            .for_each(|els| {
                if els[0] != "" {
                    first_list.push(els[0].parse::<i64>().expect("should be number"));
                }
                if els[1] != "" {
                    second_list.push(els[1].parse::<i64>().expect("should be number"));
                }
            });
    });
    (first_list, second_list)
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: i64 = {
        let mut input_lists = parse_two_lists(
            &read_to_string("./input/day01-problem01.txt").expect("File should exist"),
        );
        input_lists.0.sort();
        input_lists.1.sort();

        iter::zip(input_lists.0, input_lists.1).fold(0, |total_distance, (el1, el2)| {
            total_distance + (el1 - el2).abs()
        })
    };

    let sol2: i64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
