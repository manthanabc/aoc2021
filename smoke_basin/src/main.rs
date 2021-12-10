use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Data {
    map: HashMap<Point, u8>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn neighbours(&self, map: &HashMap<Point, u8>) -> Vec<Point> {
        // offsets for UP, RIGHT, DOWN, LEFT
        // in form (row, col)
        let offsets: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut neighbours = Vec::new();

        for (row_offset, col_offset) in offsets {
            let row = self.row as isize + row_offset;
            let col = self.col as isize + col_offset;
            if row >= 0 && col >= 0 {
                let point = Point {
                    row: row as usize,
                    col: col as usize,
                };
                if map.contains_key(&point) {
                    neighbours.push(point);
                }
            }
        }
        neighbours
    }

    fn get_basin_size(&self, map: &HashMap<Point, u8>, seen: &mut HashSet<Point>) -> usize {
        if seen.contains(self) {
            return 0;
        }
        if map.get(self) == Some(&9) {
            return 0;
        }

        seen.insert(self.clone());

        let mut total = 1;

        for neighbour in self.neighbours(map) {
            // RECURSIOOOOOOOOOOOOOOOOOOOOOON
            total += neighbour.get_basin_size(map, seen);
        }
        // attempt to write this for loop as an iterator chain: fails, returns 1
        // self.neighbours(map).iter().fold(1, |mut acc, neighbour| {
        //     acc += neighbour.get_basin_size(map, seen);
        //     acc
        // })
        total
    }
}

impl Data {
    fn part_one(self) -> usize {
        let mut risks = Vec::new();

        for (point, depth) in &self.map {
            // compare current depth to all neighbour depths
            let is_lowest = point
                .neighbours(&self.map)
                .iter()
                .filter_map(|point| self.map.get(point))
                .all(|neighbour_depth| depth < neighbour_depth);
            // if it's smaller than them all, add to risks vector
            if is_lowest {
                risks.push(1 + *depth as usize);
            }
        }

        risks.iter().sum()
    }

    fn part_two(&self) -> usize {
        let mut sizes = Vec::new();
        let mut seen: HashSet<Point> = HashSet::new();

        for (point, depth) in &self.map {
            // compare current depth to all neighbour depths
            let is_lowest = point
                .neighbours(&self.map)
                .iter()
                .filter_map(|point| self.map.get(point))
                .all(|neighbour_depth| depth < neighbour_depth);

            // if it's smaller than them all, calculate size of basin by expanding from the lowest point
            if is_lowest {
                let size = point.get_basin_size(&self.map, &mut seen);
                sizes.push(size);
            }
        }

        sizes.sort();
        sizes.iter().rev().take(3).product()
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| parse_line(line, row_idx).into_iter())
            .collect();

        fn parse_line(input: &str, row_idx: usize) -> Vec<(Point, u8)> {
            input
                .chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    // can cast as u8 because every digit is 0 <= digit <= 9
                    let depth = c.to_digit(10).unwrap() as u8;
                    let point = Point {
                        row: row_idx,
                        col: col_idx,
                    };
                    (point, depth)
                })
                .collect()
        }

        Ok(Self { map })
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.clone().part_one());
    println!("Part two answer: {}", data.part_two());
}

/*use intcode::{parse_stdin_program, Environment, Program, Word};

fn main() {
    let data = parse_stdin_program();

    println!("stage1: {:?}", boost(&data, 1));
    println!("stage2: {:?}", boost(&data, 2));
}

fn boost(data: &[Word], input: Word) -> Word {
    let mut data = data.to_vec();
    let mut env = Environment::once(Some(input));

    Program::wrap(&mut data)
        .with_memory_expansion()
        .eval_with_env(&mut env)
        .unwrap();

    env.unwrap_input_consumed_once().unwrap()
}

#[test]
fn stage1_full() {
    intcode::with_parsed_program(|input| assert_eq!(boost(input, 1), 3638931938));
}

#[test]
fn stage2_full() {
    intcode::with_parsed_program(|input| assert_eq!(boost(input, 2), 86025));
}
*/
/*use std::fs::File;
use std::io::prelude::*;


// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input: &str) -> Vec<i64> {
    input.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect()
}

// --- problems

fn sum_of_two_is(sum: i64, vec: &[i64]) -> bool {
    vec.iter().enumerate()
        .flat_map(|(i, a)| vec.iter().skip(i+1).map(move |b| a + b))
        .find(|x| *x == sum)
        .is_some()
}

fn find_first_invalid(vec: &Vec<i64>, preamble: usize) -> Option<i64> {
    let index = (preamble..vec.len()).find(
        |index| !sum_of_two_is(vec[*index], &vec[index-preamble..*index])
    );

    index.map(|i| vec[i])
}

fn find_contiguous_set_summing_to(target: i64, vec: &[i64]) -> Option<&[i64]> {
    let mut start = 0;
    let mut end = 1;
    let mut sum: i64 = vec[start..=end].iter().sum();

    while end < vec.len() {
        if sum == target {
            return Some(&vec[start..=end]);
        }
        if sum > target {
            sum -= vec[start];
            start += 1;
        } else {
            end += 1;
            sum += vec[end];
        }
    }

    None
}

fn sum_of_min_and_max(vec: &[i64]) -> Option<i64> {
    vec.iter().min()
        .and_then(|min|
            vec.iter().max().map(|max| 
                min + max
            )
        )
}

fn part1(sequence: &Vec<i64>) -> Option<i64> {
    find_first_invalid(sequence, 25)
}

fn part2(sequence: &Vec<i64>) -> Option<i64> {
    let target = part1(sequence).unwrap();
    find_contiguous_set_summing_to(target, sequence)
        .and_then(sum_of_min_and_max)
}   


fn main() {
    let input = read_file("input.txt").unwrap();
    let sequence = parse_input(&input);
    println!("part1 {:?}", part1(&sequence));
    println!("part2 {:?}", part2(&sequence));
}
*/
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_input("1 2 3 4"), vec![1, 2, 3, 4]);
        assert_eq!(parse_input("15\n16\n0\n99"), vec![15, 16, 0, 99]);
    }

    #[test]
    fn test_sum_of_two_is() {
        assert!(sum_of_two_is(3, &vec![1,2,3,4,5]));
        assert!(sum_of_two_is(6, &vec![1,2,3,4,5]));
        assert!(sum_of_two_is(9, &vec![1,2,3,4,5]));
        assert!(!sum_of_two_is(20, &vec![9,8,7,6,5]));
    }

    #[test]
    fn test_find_first_invalid() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        assert_eq!(find_first_invalid(&input, 5), Some(127));
    }

    #[test]
    fn test_find_first_invalid_when_close_to_end() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219];
        assert_eq!(find_first_invalid(&input, 5), Some(127));
    }

    #[test]
    fn test_find_contiguous_set_summing_to() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        let expect: &[i64] = &vec![15,25,47,40];
        assert_eq!(find_contiguous_set_summing_to(127, &input), Some(expect));     
    }

    #[test]
    fn test_sum_of_min_and_max() {
        let input = vec![15,25,47,40];
        assert_eq!(sum_of_min_and_max(&input), Some(62));
    }
*/
/*
use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::process::exit;

struct Validator {
    buffer: RefCell<Vec<u64>>,
}

impl Validator {
    fn with_capacity(size: usize) -> Self {
        Self {
            buffer: RefCell::new(Vec::with_capacity(size)),
        }
    }

    fn find_sum(&self, numbers: &[u64]) -> Option<(u64, u64)> {
        let (&checksum, searchable) = numbers.split_last()?;
        let mut buf = self.buffer.borrow_mut();
        buf.clear();
        buf.extend_from_slice(searchable);
        buf.sort_unstable();

        let bound = {
            let b = buf.binary_search(&checksum);
            if b.is_ok() {
                b.ok()
            } else {
                b.err()
            }
        }?;

        for idx in (0..bound).rev() {
            let other = checksum - buf[idx];
            if buf[0..idx].binary_search(&other).is_ok() {
                return Some((buf[idx], other));
            }
        }

        None
    }
}

fn read_input() -> Vec<String> {
    let mut input = String::new();
    input = fs::read_to_string("input.txt").unwrap(); //std::io::stdin().read_to_string(&mut input).unwrap();
    //.read_to_string("input.txt").unwrap().lines().into_iter().collect()
    // input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
    input.lines().map(|s| s.to_string()).collect()
}

fn main() {
    let window_size = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(25)
        + 1;

    let encoding = read_input().iter()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>();

    let validator = Validator::with_capacity(window_size);

    let search = encoding
        .windows(window_size)
        .find_map(|w| validator.find_sum(w).is_none().then(|| w.last().unwrap()));

    if let Some(&answer) = search {
        println!("The answer for part 1 is {:?}.", answer);

        for set_size in 2..encoding.len() {
            let search = encoding
                .windows(set_size)
                .find(|&w| w.iter().sum::<u64>() == answer);
            if let Some(set) = search {
                let set_min = set.iter().min();
                let set_max = set.iter().max();
                let answer_part2 = set_min.zip(set_max).map(|(&f, &l)| f + l).unwrap();

                println!("The answer for part 2 is {:?}.", answer_part2);
                break;
            }
        }
    }
}
*/
/*
use std::fs;
// import hashmap
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Error");
    let mut horizontal: Vec<String>= vec![];
    let mut vertical: Vec<String>= vec![];
    let mut map: HashMap<(usize, usize), i32> = HashMap::new();
    let mut inputs: [u64; 100*100] = [0; 100*100];
    for (i, line) in input.lines().into_iter().enumerate() {
        let mut line_vec: Vec<u8> = line.as_bytes().to_vec();
        line_vec.into_iter().enumerate().for_each(|(j, x)| {
            inputs[i*100 + j] = x as u64;
        });
        //horizontal.push(inputs.iter().map(|x| *x as char).collect::<String>());
    }
    println!("yayy {}", part_2(&inputs));

    horizontal.push("9".repeat(input.trim().lines().collect::<Vec<&str>>()[1].len()+2));
    input.trim().lines().for_each(|line| {
        horizontal.push(("9".to_string().to_owned()+&(line.to_string().to_owned()+&"9".to_string())));
    });
    horizontal.push("9".repeat(input.trim().lines().collect::<Vec<&str>>()[1].len()+2));
    for i in 0..horizontal[0].len() {
        let mut column = "".to_string();
        for j in 0..horizontal.len() {
            column.push(horizontal[j].chars().nth(i).unwrap());
        }
        vertical.push(column.clone());
       // is.push(i as i32);
       // js.push(j as i32);
       // map.insert((i, j), 0);
    }
    //println!("{:?}", vertical);

    for i in 1..horizontal.len()-1 {
        for j in 1..vertical.len()-1 {
            let x = horizontal[i].chars().nth(j).unwrap();
            let xa = horizontal[i].chars().nth(j+1).unwrap();
            let xb = horizontal[i].chars().nth(j-1).unwrap();
            let ya = vertical[j].chars().nth(i+1).unwrap();
            let yb = vertical[j].chars().nth(i-1).unwrap();

            if x < xa && x < xb && x < ya && x < yb {
                map.insert((i, j), x.to_string().parse::<i32>().unwrap());
                //low.push(x.to_string().parse::<i32>().unwrap());
            }
        }
    }
    let size = get_basin(&horizontal, &vertical, 3, 3, 0);
    println!("{:?}", size);
   // map.iter().collect::<Vec<(&(usize, usize), &i32)>>().sort_by(|a, b| a.1.cmp(b.1));
   // println!("{:?}", map);
}

*/
/*
fn get_basin(horizontal: &Vec<String>, vertical: &Vec<String>, x:usize, y:usize, d:i32) -> i32 {
    if d == 2 {
        return 0;
    }
    println!("{:?} {}", x, y);
    let mut size = 0;
    let mut counter = 1;
    let mut xs = 0;
    let mut xe = 0;
    let mut ys = 0;
    let mut ye = 0;
    'hori: loop {
        if horizontal[y].chars().nth(x+counter).unwrap() == '9' {
            xs = x+counter;
            break 'hori;
        }
        counter += 1;
        size += 1;
    }
    counter = 1;
    'hori2: loop {
        if horizontal[y].chars().nth(x-counter).unwrap() == '9' {
            xe = x-counter;
            break 'hori2;
        }
        size += 1;
        counter += 1;
    }

    /*
    counter = 1;
    'vert: loop {
        if vertical[y+counter].chars().nth(x).unwrap() == '9' {
            break 'vert;
            ys = y+counter;
        }
        size += 1;
        counter += 1;
    }
    counter = 1;
    'vert2: loop {
        if vertical[y-counter].chars().nth(x).unwrap() == '9' {
            break 'vert2;
            ye = y-counter;
        }
        size += 1;
        counter += 1;
    }
    */
    // println!("aa {}", size);
    for i in xe+1..xs {
        let value = get_row(&vertical[i], y, x, &vertical, &horizontal, d);
        size += value;
        // println!("e {} {}", i, value);
    }
    size+1
}

fn get_row(vertical: &str, y: usize, x: usize, verticale: &Vec::<String>, horizontal: &Vec::<String>, d:i32) -> i32{
    let mut counter = 1;
    let mut size = 0;
    let mut sy = 0;
    let mut ey = 0;
    'hori: loop {
        if vertical.chars().collect::<Vec<char>>()[y+counter] == '9' {
            sy = y+counter;
            break 'hori;
        }
        counter += 1;
        size += 1;
    }
    counter = 1;
    'hori2: loop {
        if vertical.chars().collect::<Vec<char>>()[y-counter] == '9' {
            ey = y-counter;
            break 'hori2;
        }
        size += 1;
        counter += 1;
    }
    for k in ey+1..sy {
        let value = get_basin(verticale,horizontal, x, y, d+1);
        if value > size {
            size = value;
        }
        // println!("e {} {}", i, value);
    }
    size 
}

fn part_2(nums: &[u64]) -> u64 {
    let target = part_1(&nums);

    let mut low_idx = 0;
    let mut high_idx = 2;
    let mut sum: u64 = nums[low_idx..high_idx].iter().sum();
    loop {
        assert!(high_idx < nums.len());
        while low_idx < high_idx && sum + nums[high_idx] > target {
            sum -= nums[low_idx];
            low_idx += 1;
        }
        while high_idx < nums.len() && sum + nums[high_idx] <= target {
            sum += nums[high_idx];
            high_idx += 1;
        }
        if sum == target {
            break;
        }
    }

    let mut range = nums[low_idx..high_idx].to_owned();
    range.sort();
    range[0] + range[range.len() - 1]
}
*/
/*                if j == 0 {
                    let x = parse(horizontal[i].chars().nth(j).unwrap());
                    let xa = parse(horizontal[i+1].chars().nth(j).unwrap());
                    let xb = parse(horizontal[i-1].chars().nth(j).unwrap());
                    let ya = parse(horizontal[i].chars().nth(j+1).unwrap());
                //let yb = parse(horizontal[i].chars().nth(j-1).unwrap());
                    if x < xa && x < xb && x < ya {
                        println!("tt{}", x);
                        low.push(x);
                        if x < minimum {
                            lowestx = i;
                            lowesty = j;
                            minimum = x;
                        }
                    }
                } else if j == vertical.len() - 1 {
                    let x = parse(horizontal[i].chars().nth(j).unwrap());
                    let xa = parse(horizontal[i+1].chars().nth(j).unwrap());
                    let xb = parse(horizontal[i-1].chars().nth(j).unwrap());
                //let ya = parse(horizontal[i].chars().nth(j+1).unwrap());
                    let yb = parse(horizontal[i].chars().nth(j-1).unwrap());
                    if x < xa && x < xb && x < yb {
                        println!("tt{}", x);
                        low.push(x);
                        if x < minimum {
                            lowestx = i;
                            lowesty = j;
                            minimum = x;
                        }
                    }
                }
            } else if i == 0 {
                if j > 0 && j < vertical.len() -1 {
                    let x = parse(horizontal[i].chars().nth(j).unwrap());
                    let xa = parse(horizontal[i+1].chars().nth(j).unwrap());
                    let ya = parse(horizontal[i].chars().nth(j+1).unwrap());
                    let yb = parse(horizontal[i].chars().nth(j-1).unwrap());
                    if x < xa && x < ya && x < yb {
                        println!("tt{}", x);
                        low.push(x);
                        if x < minimum {
                            lowestx = i;
                            lowesty = j;
                            minimum = x;
                        }
                    }
                }
                if j == 0 {
                    let x = parse(horizontal[i].chars().nth(j).unwrap());
                    let xa = parse(horizontal[i+1].chars().nth(j).unwrap());
                    let ya = parse(horizontal[i].chars().nth(j+1).unwrap());
                    if x < xa && x < ya {
                        println!("tt{}", x);
                        low.push(x);
                        if x < minimum{
                            lowestx = i;
                            lowesty = j;
                            minimum = x;
                        }
                    }
                } else if j == vertical.len() - 1 {
                    let x = parse(horizontal[i].chars().nth(j).unwrap());
                    let xa = parse(horizontal[i+1].chars().nth(j).unwrap());
                    let yb = parse(horizontal[i].chars().nth(j-1).unwrap());
                    if x < xa && x < yb {
                        println!("tt{}", x);
                        low.push(x);
                        if x < minimum{
                            lowestx = i;
                            lowesty = j;
                            minimum = x;
                        }
                    }
                }
            } else if i == horizontal.len() - 1 {
                if j > 0 && j < vertical.len() -1 {
                    let x = parse(horizontal[i].chars().nth(j).unwrap());
                    let xb = parse(horizontal[i-1].chars().nth(j).unwrap());
                    let ya = parse(horizontal[i].chars().nth(j+1).unwrap());
                    let yb = parse(horizontal[i].chars().nth(j-1).unwrap());
                    if x < xb && x < ya && x < yb {
                        println!("tt{}", x);
                        low.push(x);
                        if x < minimum{
                            lowestx = i;
                            lowesty = j;
                            minimum = x;
                        }
                    }
                } else if j == 0 {
                    let x = parse(horizontal[i].chars().nth(j).unwrap());
                    let xb = parse(horizontal[i-1].chars().nth(j).unwrap());
                    let ya = parse(horizontal[i].chars().nth(j+1).unwrap());
                    if x < xb && x < ya {
                        println!("tt{}", x);
                        low.push(x);
                        if x < minimum{
                            lowestx = i;
                            lowesty = j;
                            minimum = x;
                        }
                    }
                } else if j == vertical.len() - 1 {
                    let x = parse(horizontal[i].chars().nth(j).unwrap());
                    let xb = parse(horizontal[i-1].chars().nth(j).unwrap());
                    let yb = parse(horizontal[i].chars().nth(j-1).unwrap());
                    if x < xb && x < yb {
                        println!("tt{}", x);
                        low.push(x);
                        if x < minimum{
                            lowestx = i;
                            lowesty = j;
                            minimum = x;
                        }
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for i in 0..low.len() {
        sum += low[i]+1;
    }
    low.sort();
    println!("sorted {:?}", low);
}

fn parse(c: char) -> i32 {
    c.to_string().parse::<i32>().unwrap()
}

fn get-basin(numb, horizontal, vertical, x, y) -> i32 {
    let mut size = 0;
    let mut counter = 0;
    let xs = 0;
    let xe = 0;
    let ys = 0;
    let ye = 0;
    'hori: loop {
        if horizontal[y].chars().nth(x+counter).unwrap() == '9' {
            break 'hori;
        }
        size += 1;
    }
}
*/
/*
fn part_1(nums: &[u64]) -> u64 {
    for window in nums.windows(26) {
        let mut has_property = false;
        let last = window[25];
        for summand in window[..25].iter().copied() {
            if last < summand { continue }
            let remainder = last - summand;
            if window[..25].contains(&remainder) {
                has_property = true;
                break;
            }
        }
        if !has_property {
            return last;
        }
    }
    unreachable!()
}
*/
