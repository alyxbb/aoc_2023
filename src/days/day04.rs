use std::array;

use crate::{Solution, SolutionPair};

fn part_1(input: String) -> Solution {
    let mut tot = 0;
    for mut line in input.lines(){
        let mut card_score = 0;
        line = line.split(": ").last().expect("card number is missing");
        let subparts = line.split(" | ").collect::<Vec<&str>>();
        let winners: Vec<&str> = subparts[0].split_whitespace().collect();
        let mine = subparts[1];
        for num in mine.split_whitespace() {
            if winners.contains(&num) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        tot += card_score;
    }
    Solution::from(tot)
}

fn part_2(input: String) -> Solution {
    let mut card_counts = [0; 200];
    for (i,mut line) in input.lines().enumerate(){
        card_counts[i] +=1;
        let card_count = card_counts[i];
        let mut card_score = 0;
        line = line.split(": ").last().expect("card number is missing");
        let subparts = line.split(" | ").collect::<Vec<&str>>();
        let winners: Vec<&str> = subparts[0].split_whitespace().collect();
        let mine = subparts[1];
        for num in mine.split_whitespace() {
            if winners.contains(&num) {
                card_score += 1;
            }
        }
        for j in 0..card_score  {
            card_counts[i+j+1]+=card_count;
        }
    }
    Solution::from(card_counts.iter().sum::<u32>())
}
///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: String) -> SolutionPair {
    (part_1(input.clone()), part_2(input.clone()))
}
