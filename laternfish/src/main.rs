use std::fs;
//import hashmap
use std::collections::HashMap;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut inputs:Vec<usize> = vec![];
    input.trim().split(",").for_each(|x|{
        inputs.push(x.parse::<usize>().unwrap());
    });
    println!("{}", simulate_fishes(inputs, 256usize));
}

fn simulate_fishes(fishes: Vec<usize>, gen: usize) -> usize{
    let mut states = vec![0usize; 9];
    fishes.iter().for_each(|fish| {
        states[*fish] += 1;
    });

    for _ in 0..gen {
        states.rotate_left(1);
        states[6] += states[8];
    }
    states.iter().sum()
}
    /*
    input.trim().split(",").for_each(|number| {
        let number = number.parse::<i64>().unwrap();
        if map.contains_key(&number) {
            *map.get_mut(&number).unwrap() += 1;
        } else {
            map.insert(number, 1);
        }
        // map.insert(number.parse::<i32>(), line.split_whitespace().nth(2).unwrap().to_string());
    });
    for i in 0..26 {
        println!("{:#?}", map);
        let mut map2 = map.clone();
        for keyo in 0..9 {
            let key = &(9-keyo);
            println!("{}", key);
            if *key -1 < 0 {
                if map2.contains_key(&(8)) {
                    *map2.get_mut(&(8)).unwrap() += *map.get(&(*key)).unwrap();
                } else {
                    map2.insert(8, *map.get(&(*key)).unwrap());
                }
                if map2.contains_key(&(6)) {
                    *map2.get_mut(&(6)).unwrap() += *map.get(&(*key)).unwrap();
                } else {
                    map2.insert(6, *map.get(&(*key)).unwrap());
                }
                *map2.get_mut(&(key)).unwrap() = 0;
                // map2.insert(8, *map.get(key).unwrap());
            } else {
                if map2.contains_key(&(key - 1)) {
                    if *key - 1 != 0 {
                        *map2.get_mut(&(key - 1)).unwrap() += *map.get(&(*key)).unwrap();
                    } else {
                        *map2.get_mut(&(key - 1)).unwrap() = *map.get(&(*key)).unwrap();
                    }
                } else {
                    map2.insert(key - 1, *map2.get(key).unwrap());
                } 
                *map2.get_mut(key).unwrap() = 0;
            }
        }
        map = map2;
    }
    println!("sum of values is {:?}", map.values().sum::<i32>());
}
*/
    /*
    let input = fs::read_to_string("input1.txt").expect("Error");
    let mut population:Vec<i32> = Vec::new();
    input.trim().split(',').for_each(|line| {
        let fish = line.parse::<i32>().unwrap();
        population.push(fish);
    });
    for i in 0..18 {
        population.iter_mut().for_each(|fish| {
            *fish -= 1;
        });
        let mut new_population:Vec<i32> = population.clone();
        for i in 0..new_population.len() {
            let mut fish = population[i].clone();
            if fish < 0 {
                fish = 6;
                // print!("{} ", fish.get_days());
                population.push(8);
            }
            population[i] = fish;
        }
        // population = new_population;
    }
    //let mut sum = 0;
    //population.iter().for_each(|fish| {
    //    sum += getfishes(fish.get_days());
    //});
    println!("{}", population.len());
}

fn getfishes(age: i32) -> i32 {
    let mut sum = 0;
    let created = (18 - age) / 6;
    sum += created;
    for i in 0..created {
        let fish = getfishes(age + i * 6);
        sum += fish;
    }
    sum
}
*/
