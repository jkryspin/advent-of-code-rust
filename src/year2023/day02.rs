use regex::Regex;

pub fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    let re: Regex = Regex::new(r"\d+").unwrap();
    let cubed: Regex = Regex::new(r"\d+ \w+").unwrap();
    lines.for_each(|l| {
        let (cubes, game_id) = parse(l, &re, &cubed);
        if is_valid_(&cubes) {
            sum += game_id;
        }
    });

    sum
}

fn parse(l: &str, re: &Regex, cubed: &Regex) -> (Vec<Vec<CubeSet>>, u32) {
    let games = l.split(";");
    let id_str = games.clone().next().unwrap();
    let game_id = re.find(id_str).unwrap().as_str().parse::<u32>().unwrap();
    let cubes: Vec<_> = games
        .map(|g| {
            return cubed
                .find_iter(g)
                .map(|m| {
                    let mut split_m = m.as_str().split(" ");
                    let amount = split_m.next().unwrap();
                    let color = split_m.next().unwrap();
                    return CubeSet {
                        amount: amount.parse().unwrap(),
                        color: color.to_string(),
                    };
                })
                .collect::<Vec<CubeSet>>();
        })
        .collect();
    return (cubes, game_id);
}
fn is_valid_(cubes: &Vec<Vec<CubeSet>>) -> bool {
    for c in cubes.iter() {
        for cube in c.iter() {
            if cube.color == "red" && cube.amount > 12 {
                return false;
            }
            if cube.color == "green" && cube.amount > 13 {
                return false;
            }
            if cube.color == "blue" && cube.amount > 14 {
                return false;
            }
        }
    }
    return true;
}
#[derive(Debug)]
struct CubeSet {
    amount: u32,
    color: String,
}

pub fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let re = Regex::new(r"\d+").unwrap();
    let cubed = Regex::new(r"\d+ \w+").unwrap();
    let mut sum = 0;
    lines.for_each(|l| {
        let (cubes, _) = parse(l, &re, &cubed);
        sum += min_power(&cubes);
    });

    sum
}

fn min_power(cubes: &Vec<Vec<CubeSet>>) -> u32 {
    let mut min_red = 1;
    let mut min_green = 1;
    let mut min_blue = 1;
    for c in cubes.iter() {
        for cube in c.iter() {
            if cube.color == "red" {
                min_red = min_red.max(cube.amount);
            }
            if cube.color == "green" {
                min_green = min_green.max(cube.amount);
            }
            if cube.color == "blue" {
                min_blue = min_blue.max(cube.amount);
            }
        }
    }
    return min_red * min_blue * min_green;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(input);
        assert_eq!(result, 2286);
    }
}
