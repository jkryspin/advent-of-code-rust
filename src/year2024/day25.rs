use std::cmp::PartialEq;
use crate::year2024::day25::SchematicType::{Key, Lock};

pub fn part1(input: &str) -> u32 {
    let grid:Vec<Schematic> = input
        .split("\n\n")
        .map(|group| {
            let schematic = Schematic {
                grid: group
                    .lines()
                    .map(|line| line.chars().collect())
                    .collect::<Vec<Vec<char>>>(),
            };
            schematic
        }).collect();
    let mut count = 0;
    let needed = grid[0].grid.len() - 2;
    println!("{:?}", needed);
    grid.iter().filter(|x|x.get_type() == Lock).for_each(|schematic| {
        println!("{:?}", schematic);
        println!("{:?}", schematic.get_type());
        let total = schematic.total_per_column();
        grid.iter().filter(|x|x.get_type()== Key).for_each(|schematic2| {
            let total2 = schematic2.total_per_column();
            println!("{:?}", total);
            println!("{:?}", total2);
            if total.iter().zip(total2.iter()).all(|(a, b)| a + b <= needed as u32) {
                count += 1;
            }
        });
    });
    count
}
pub fn part2(_input: &str) -> u32 {
    return 0;
}

#[derive(Debug)]
struct Schematic{
    grid: Vec<Vec<char>>,
}

impl Schematic {
    fn get_type(&self)-> SchematicType{
        // if row 0 is all # its a lock
        if self.grid[0].iter().all(|&c| c == '#'){
            return SchematicType::Lock;
        }else if self.grid[0].iter().all(|&c| c == '.'){
            return SchematicType::Key;
        }
        panic!("Invalid Schematic");
    }
    fn total_per_column(&self)-> Vec<u32>{
        let mut total = vec![0; self.grid[0].len()];
        for row in &self.grid{
            for (i, &c) in row.iter().enumerate(){
                if c == '#'{
                    total[i] += 1;
                }
            }
        }
        // remove the all # border (which exists on key and lock)
        total.iter_mut().for_each(|x| *x -= 1);
        total
    }
}

#[derive(Debug, Eq, PartialEq)]
enum SchematicType{
    Lock,
    Key
}