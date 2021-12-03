use std::fs;

fn main() {
    let mut totalx = 0;
    let mut totaly = 0;
    let mut aim =0;
    let contents = fs::read_to_string("input.txt").unwrap();
    contents.lines().for_each(|line| {
        if line.contains("forward") {
            let value = line[8..].parse::<i32>().unwrap();
            totalx += value;
            totaly += aim*value;
        } else if line.contains("down") {
            let value = line[5..].parse::<i32>().unwrap();
            // totaly += value;
            aim += value;
        } else if line.contains("up") {
            let value = line[3..].parse::<i32>().unwrap();
            // totaly -= value;
            aim -= value;
        }
    });
    println!("{}, {}", totalx, totaly);
}
