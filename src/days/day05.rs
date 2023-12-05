use std::usize;

use crate::Solution;

pub fn part_1(input: String) -> Solution {
    
    let maps = input.split("\n\n").collect::<Vec<&str>>();
    let unparsed_seeds = maps[0].split(": ").collect::<Vec<&str>>()[1].split(" ");
    let mut seeds: Vec<usize> = Vec::new();
    let mut new_seeds: Vec<usize> = Vec::new();
    for seed in unparsed_seeds{
        seeds.push(seed.parse().expect("failed to parse seed"));
    }
    for map in maps[1..].iter(){
        for seed in seeds.iter(){
            let mut mapped = false;
            for mapping in map.lines().collect::<Vec<&str>>()[1..].iter(){
                let mapping_parts: Vec<&str> = mapping.split(" ").collect();
                let source: usize= mapping_parts[1].parse().expect("failed to convert source to int");
                let dest: usize = mapping_parts[0].parse().expect("failed to convert dest to int");
                let range: usize = mapping_parts[2].parse().expect("failed to convert range to int");
                if &source<seed && seed< &(source + range){
                    new_seeds.push(dest+ (seed-source));
                    mapped = true;
                }
            }
            if !mapped{
                new_seeds.push(*seed);
            }
        }
        seeds = new_seeds;
        new_seeds = Vec::new();
    }
    let sol: usize = *seeds.iter().min().expect("failed to find min");
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let maps = input.split("\n\n").collect::<Vec<&str>>();
    let unparsed_seeds: Vec<&str> = maps[0].split(": ").collect::<Vec<&str>>()[1].split(" ").collect();
    let mut seeds: Vec<(usize,usize)> = Vec::new();
    let mut new_seeds: Vec<(usize,usize)> = Vec::new();
    for seed in (0..unparsed_seeds.len()).step_by(2){
        seeds.push((unparsed_seeds[seed].parse::<usize>().expect("failed to parse seeds"),unparsed_seeds[seed].parse::<usize>().expect("failed to parse seeds")+unparsed_seeds[seed+1].parse::<usize>().expect("failed to parse seeds")));
    }
    for map in maps[1..].iter(){
        for seed in seeds.iter(){
            let mut to_check = Vec::from_iter(seed.0..=seed.1);
            for mapping in map.lines().collect::<Vec<&str>>()[1..].iter(){
                let mapping_parts: Vec<&str> = mapping.split(" ").collect();
                let source: usize= mapping_parts[1].parse().expect("failed to convert source to int");
                let dest: usize = mapping_parts[0].parse().expect("failed to convert dest to int");
                let range: usize = mapping_parts[2].parse().expect("failed to convert range to int");
                let max_source = source + range;
                if seed.0< max_source && source <seed.1 {
                    //part of the seed region is in here. find out which
                    if seed.0<source {
                        if max_source<seed.1{ // the mapping is fully contained in the seed region
                            new_seeds.push((dest,dest+range));
                            to_check.retain(|x| (*x)<source || source+range<(*x))
                        } else { // the seed region ends in this mapping
                            new_seeds.push((dest,dest+(seed.1-source)));
                            to_check.retain(|x| (*x)<source || seed.1<(*x))
                        }
                    } else {
                        if max_source<seed.1{//the seed region starts in this mapping
                            new_seeds.push((dest+(seed.0-source),dest+range));
                            to_check.retain(|x| (*x)<seed.0 || source+range<(*x))
                        } else { // the seed region is fully contained within the mapping
                            new_seeds.push((dest+(seed.0-source),(dest+(seed.1-source))));
                            to_check.retain(|x| (*x)<seed.0 || seed.1<(*x))
                        }
                    }

                }
            }
            //find any remaining runs and include them
            let mut run_start = 0;
            let mut last :i64 = -1;
            for val in to_check{
                if val !=(last +1).try_into().unwrap()  && last>=0{
                    new_seeds.push((run_start,last.try_into().unwrap()));
                    run_start = val;
                }
                if last == -1 {
                    run_start = val;
                }
                last = val.try_into().unwrap();
            }
            if last!= -1{
                new_seeds.push((run_start,last.try_into().unwrap()));
            }
        }
        seeds = new_seeds;
        new_seeds = Vec::new();
    }
    let mut min_seeds = Vec::new();
    for seed in seeds{
        min_seeds.push(seed.0);
    }
    let sol: usize = *(min_seeds.iter().min().expect("failed to find min"));
    Solution::from(sol)
}
