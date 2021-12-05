use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut inputs = input.lines().into_iter().collect::<Vec<_>>();
    
    for input in inputs.iter() {
        // println!("jere");
        let mut input = input.to_string();
        let (x1,y1,x2,y2) = check_contains(&input);
        if x1 == x2 || y1 == y2 {
            for i in x1..x2+1 {
                for j in y1..y2+1 {
                    let mut count = map.entry((i, j)).or_insert(0);
                    *count += 1;
                }
            }
        } else {
            // println!("********{} {} {} {}", x1, y1, x2, y2);
            for i in x1..x2+1 {
                let mut yo: i32;
                if y1 < y2 {
                    for j in y1..y2+1 {
                        if i-x1 == j-y1 {
                            // println!("diagna {} {}", i, j);
                            let mut count = map.entry((i, j)).or_insert(0);
                            *count += 1;
                        }
                    }
                } else {
                    yo = y1-y2;
                    for j in y2..y1+1 {
                        if i-x1 == yo {
                            // println!("other diagonal {} {}", i, j);
                            let mut count = map.entry((i, j)).or_insert(0);
                            *count += 1;
                        }
                        yo-=1;
                    }
                }
            }
        }
    }
    println!("{:?}", map);
    let mut second_greatest = 0;
    for i in map.iter() {
        if i.1 > &1 && i.0.1 != -1 {
            second_greatest += if *i.1 >=2 {1} else {0};
        }
    }
    let number_of_trees = map.iter().filter(|x| x.1 >= &2).count();
    println!("{:?}", second_greatest);
    let line = "1,1 -> 1,2".to_string();
    let mut x = 1;
    let mut y = 2;
}

fn check_contains(line: &str) -> (i32, i32, i32, i32){
    let tokens = line.split(" ").collect::<Vec<_>>();
    let x1y1 = tokens[0].split(",").collect::<Vec<_>>();
    let x1 = x1y1[0].parse::<i32>().unwrap();
    let y1 = x1y1[1].parse::<i32>().unwrap();
    let x2y2 = tokens[2].split(",").collect::<Vec<_>>();
    let x2 = x2y2[0].parse::<i32>().unwrap();
    let y2 = x2y2[1].parse::<i32>().unwrap();
    if x1 == x2 || y1 == y2 {
        if x1 < x2 {
            if y1 < y2 {
                return (x1, y1, x2, y2);
            } else {
                return (x1, y2, x2, y1);
            }
        } else {
            if y1 < y2 {
                return (x2, y1, x1, y2);
            } else {
                return (x2, y2, x1, y1);
            }
        }
        //println!("{} {} {} {}", x1, y1, x2, y2);
        //return (x1, y1, x2, y2);
    }
    else {
        if x1 < x2 {
            return (x1, y1, x2, y2);
        } else {
            return (x2, y2, x1, y1);
        }
    }
    return (-1, -1, -1, -1);
}
