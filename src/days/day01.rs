use crate::Solution;

pub fn part_1(input: String) -> Solution {
    let mut sol = 0;
    for line in input.lines() {
        let mut tens = None;
        let mut ones = None;
        for char in line.chars() {
            if char.is_digit(10) {
                ones = Some(char.to_digit(10).expect("this is bad"));
                if tens.is_none() {
                    tens = Some(char.to_digit(10).expect("how the hell has this happened"))
                }
            }
        }
        sol += 10 * tens.expect("failed to find tens") + ones.expect("failed to find ones")
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let mut sol = 0;
    for line in input.lines() {
        let mut tens = None;
        let mut ones = None;
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                ones = Some(char.to_digit(10).expect("this is bad"));
                if tens.is_none() {
                    tens = Some(char.to_digit(10).expect("how the hell has this happened"))
                }
            } else {
                let actual_digits = [1, 2, 6, 4, 5, 9, 3, 7, 8];
                for (j, digit) in [
                    "one", "two", "six", "four", "five", "nine", "three", "seven", "eight",
                ]
                .into_iter()
                .enumerate()
                {
                    if i + digit.len() > line.len() {
                        break;
                    }
                    if &line[i..(i + digit.len())] == digit {
                        ones = Some(actual_digits[j]);
                        if tens.is_none() {
                            tens = Some(actual_digits[j]);
                        }
                    }
                }
            }
        }
        sol += 10 * tens.expect("failed to find tens") + ones.expect("failed to find ones")
    }
    Solution::from(sol)
}
