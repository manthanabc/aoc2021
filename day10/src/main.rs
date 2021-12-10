use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut scores = Vec::new();
    input.trim().lines().for_each(|line| {
        let mut score: u64 = 0;
        let mut holder = Vec::new();
        line.chars().for_each(|c| {
            if c == '(' {
                holder.push(1);
            } else if c == ')' {
                let k = holder.pop();
                if k != Some(1) {
                    // score += 3;
                    holder.pop();
                }
            }
            if c == '[' {
                holder.push(2);
            } else if c == ']' {
                let k = holder.pop();
                if k != Some(2) {
                    // score += 57;
                    holder.pop();
                }
            }
            if c == '{' {
                holder.push(3);
            } else if c == '}' {
                let k = holder.pop();
                if k != Some(3) {
                    // score += 1197;
                    holder.pop();
                }
            }
            if c == '<' {
                holder.push(4);
            } else if c == '>' {
                let k = holder.pop();
                if k != Some(4) {
                    // score += 25137;
                    holder.pop();
                }
            }
        });
        while holder.len() > 0 {
            // score += holder.pop().unwrap() * holder.pop().unwrap();
            let value = holder.pop();
            score *= 5;
            score += if value == Some(1) {
                1
            } else if value == Some(2) {
                2
            } else if value == Some(3) {
                3
            } else if value == Some(4) {
                4
            } else {
                0
            };
        }
        scores.push(score);
    });
    scores.sort();
    println!("scores {:?}", scores);
    println!("{:?}", scores[((scores.len()+1)/2) -1]);
    // println!("Hello, world!");
}
