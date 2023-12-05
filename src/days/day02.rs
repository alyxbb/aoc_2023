use crate::Solution;

pub fn part_1(input: String) -> Solution {
    let mut sol = 0;
    'invalid: for (i, line) in input.lines().enumerate(){
        let game_data = line.split(": ").last().expect("Could not find game data");
        for round in game_data.split("; "){
             for colour in round.split(", "){
                println!("{colour}");
                let part: Vec<&str> = colour.split(" ").collect();
                    let count = part[0].parse::<i32>().expect("Found a non number where a number was expected");
                    match part[1] {
                        "red" =>{
                            if count > 12{
                                continue 'invalid;
                            }
                        },
                        "green" => {
                            if count > 13 {
                                continue 'invalid;
                            }
                        },
                        "blue" => {
                            if count > 14 {
                                continue 'invalid;
                            }
                        }
                        _ => panic!("found an invalid colour"),
                    }
                
            }
        }
        sol += 1+i;
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let mut sol = 0;
    for line in input.lines(){
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let game_data = line.split(": ").last().expect("Could not find game data");
        for round in game_data.split("; "){
             for colour in round.split(", "){
                println!("{colour}");
                let part: Vec<&str> = colour.split(" ").collect();
                    let count = part[0].parse::<i32>().expect("Found a non number where a number was expected");
                    match part[1] {
                        "red" =>{
                            if count > max_red {
                                max_red=count;
                            }
                        },
                        "green" => {
                            if count > max_green {
                                max_green = count
                            }
                        },
                        "blue" => {
                            if count > max_blue {
                                max_blue = count
                            }
                        }
                        _ => panic!("found an invalid colour"),
                    }
                
            }
        }
        sol += max_red * max_green * max_blue;
    }
    Solution::from(sol)
}
