use std::fs;

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Failed to read input");
    let mut boardso = inputs.split("\n\n").into_iter().collect::<Vec<&str>>();
    let numbers = boardso[0];
    // let mut boards = boardso.clone();
    let mut numbers_parsed = vec![];
    numbers.split(",").into_iter().for_each(|number| {
        numbers_parsed.push(number.parse::<i32>().unwrap());
    });
    boardso.remove(0);
    let mut boards = boardso.clone();
    let mut current_numbers: Vec<i32> = vec![];
    let mut done = false;
    let mut first_board = 0;
    let mut last_draw =0;
    let mut last_board = boards[0].clone();
    'outer: for number in 0..numbers_parsed.len() {
        // println!("{}", number);
        let mut removed =0;
        current_numbers.push(numbers_parsed[number]);
        for board in 0..boards.len() {
            if checkrow(boards[board-removed], &current_numbers) ||  checkcolomn(boards[board-removed], &current_numbers) {
                done = true;
                //println!("i {:?}", board);
                last_board = boards[board-removed].clone();
                boards.remove(board-removed);
                removed += 1;
                first_board = board;
                last_draw = current_numbers.len()-1;
                }
                // break 'outer;
            }
        } 
    let mut sum =0;
    for row in last_board.split("\n") {
        for number in row.split_whitespace() {
            // println!("{}", number);
            if !current_numbers[0..last_draw+1].contains(&number.parse::<i32>().unwrap()) {
                // println!("{}", number);
                sum += number.parse::<i32>().unwrap();
            }
        }
    }
    println!("{:?} {:?}", current_numbers[last_draw], sum);
    // println!("{:#?} {}" , boards, numbers); 
    //let example_board = "14 21 17 24 4 \n 10 16 15 9 19 \n 18 8 23 26 20 \n 22 11 13 6 5 \n 2 0 12 3 7";
    //let example_numbers = vec![7];
    //println!("{}", checkrow(example_board, &example_numbers));
}

fn checkcolomn(s: &str, numbers_done: &Vec<i32>) -> bool {
    // println!("got input {} {:?}", s, numbers_done);
    let s = s.trim();
    let mut rows = s.split("\n");
    for i in 0..5 {
        let mut done = true;
        for j in rows.clone() {
            let elements = j.split_whitespace();
            let elements = elements.collect::<Vec<&str>>();
            // println!("{:?}", elements);
            if elements.len() == 5{
                // println!("{:?}", elements);
                if !numbers_done.contains(&elements[i].parse::<i32>().unwrap()) {
                    done = false;
                }
            }
        }
        if done {
            return true;
        }
    }
    return false;
}

fn checkrow(boardin: &str, numbers_done: &Vec<i32>) -> bool {
    // println!("got input \na:{}:\nb:{:?}:", boardin, numbers_done);
    let board = boardin.trim();
    let mut rows = board.split("\n");
    for i in rows.clone() {
        let elements = i.split_whitespace();
        let elements = elements.collect::<Vec<&str>>();
        let mut done = true;
        for j in elements.clone() {
            if !numbers_done.contains(&j.parse::<i32>().unwrap()) {
                done = false;
            }
        }
        if done {
            return true;
        }
    }
    return false;
}
