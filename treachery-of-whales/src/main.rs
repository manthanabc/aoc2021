use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers: Vec<i32> = input.trim()
        .split(",")
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut counter = 0;
    let mut minimum = u64::max_value();
    loop {
        let mut fuel:u64 = 0;
        for number in numbers.iter() {
            let temp = ((number-counter)as f32).abs()as u64;
            if temp > 0 {
                fuel += temp*(temp+1)/2;
                //println!("{} {} {}", temp, fuel, number);
            }
        }
        //println!("{}, {}", fuel, minimum);
        if fuel < minimum {
            minimum = fuel;
            println!("{}", minimum);
        }
        if counter == 16000 {
            break;
        }
        counter += 1;
    }
    println!("Hello, world!");
}
