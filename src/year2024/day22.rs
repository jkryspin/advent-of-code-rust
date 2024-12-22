use std::collections::HashMap;

pub fn part1(input: &str) -> i64 {
    input.lines().map(|l| l.parse::<i64>().unwrap()).map(|secret| {
        let mut i=0;
        let mut secret = secret;
        while i<2000 {
            let new_secret = generate_secret(secret);
            secret = new_secret;
            i+=1;
        }
        secret
    }).map(|s|{
        s
    }).sum()
}

pub fn part2(input: &str) -> usize {
    let secrets:Vec<HashMap<[i64; 4],i64>> = input.lines().map(|l| l.parse::<i64>().unwrap()).map(|secret| {
        let mut i=0;
        let mut secret = secret;
        let mut deltas_to_cost:HashMap<[i64;4],i64> = HashMap::with_capacity(2000);
        let mut costs = Vec::with_capacity(2010);
        costs.push(price_of_secret(secret));
        while i<=2000 {
            let new_secret = generate_secret(secret);
            secret = new_secret;
            costs.push(price_of_secret(secret));
            if i >=3{
                let slice = &costs[i-3..=(i+1)];
                // get difference between each element in slice
                let array:[i64;4] = slice.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>().try_into().unwrap();

                // insert lower value into deltas_to_cost
                if deltas_to_cost.get(&array).is_none() {
                    deltas_to_cost.insert(array, price_of_secret(secret));
                }
            }
            i+=1;
        }
        deltas_to_cost
    }).collect();


    let mut total_amounts: HashMap<[i64; 4], i64> = HashMap::new();

    for secret in secrets {
        for (key, value) in &secret {
            *total_amounts.entry(*key).or_insert(0) += value;
        }
    }
    // get max value from total_amounts values
    let max = total_amounts.values().max().unwrap();

    *max as usize
}
fn price_of_secret(secret:i64) -> i64 {
    secret % 10
}

fn generate_secret(curr:i64) -> i64 {
    let res = mix_and_prune(curr, curr*64);
    let re2 = mix_and_prune(res, res/32);
    let res3 = mix_and_prune(re2, re2*2048);

    res3
}
fn mix_and_prune(secret:i64, new_secret:i64) -> i64 {
    let mixed_secret = mix_secret(secret, new_secret);
    prune_secret(mixed_secret)
}
fn prune_secret(new_secret:i64) -> i64 {
    new_secret % 16777216
}
pub fn mix_secret(secret:i64, new_secret:i64) -> i64 {
    secret ^ new_secret
}