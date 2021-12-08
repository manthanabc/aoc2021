use std::fs;
use std::collections::HashMap;

fn similar(s1: &str, s2:&str) -> i32 {
    let mut sum = 0;
    for c in s1.chars() {
        if s2.contains(c) {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut signals = Vec::new();
    let mut signalsf = Vec::new();
    // input.lines().enumerate().for_each(|(i, line)| { if i%2 ==1 {signals.push(line)} else {signalsf.push(line)} });
    input.lines().for_each(|line| {signals.push(line.split("|").into_iter().collect::<Vec<&str>>()[1])});
    input.lines().for_each(|line| {signalsf.push(line.split("|").into_iter().collect::<Vec<&str>>()[0])});
    let mut sum =0 ;
    let mut one = "";
    let mut four = "";
    let mut seven = "";
    let mut eight = "";
    let mut sum = 0;
    signalsf.iter().enumerate().for_each(|(i, signal)|{
        let mut out = "".to_string();
        signal.split_whitespace().for_each(|s|{
            if s.len() == 2 {
                one = s;
            }
            if s.len() == 4 {
                four = s;
            }
            if s.len() == 3 {
                seven = s;
            }
            if s.len() == 7 {
                eight = s;
            }
        });
        let word = signals[i];
        word.split_whitespace().for_each(|w|{
            println!("{}", w);
            if similar(w, one) == 2 && similar(w, eight) == 2 {
                out.push('1');
            }
            if  similar(w, one) == 1 && similar(w, four) == 2 {
                out.push('2');
            }
            if similar(w, one) == 2 && similar(w, eight) == 5 {
                out.push('3');
            }
            if similar(w, four) == 4 && similar(w, eight) == 4 {
                out.push('4');
            }
            if similar(w, seven) == 2 && similar(w, eight) == 5 && similar(w, four) == 3{
                out.push('5');
            }
            if similar(w, one) == 1 && similar(w, eight) == 6 {
                out.push('6');
            }
            if similar(w, eight) == 3 {
                out.push('7');
            }
            if similar(w, eight) == 7 {
                out.push('8');
            }
            if similar(w, four) == 4 &&  similar(w, eight) == 6 {
                out.push('9');
            }
            if similar(w, one) == 2 && similar(w, four) == 3 && similar(w, eight) == 6 {
                out.push('0');
            }
        });
        println!("{}", out);
        sum += out.parse::<i32>().unwrap();
        // println!("{} {} {} {}", out, four, seven, word);
    });
    println!("{}", sum);
}
/*
        // println!("{}", signal);
        let mut map:HashMap<char, Vec<char>> = HashMap::new();
        signal.split_whitespace().for_each(|word| {
            let word = word.trim();
            if word.len() ==3 {
                //map.insert('d', word.chars().into_iter().collect::<Vec<char>>());
                //map.get('d').unwrap();
                check_and_add('d', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('a', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('b', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('a', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('b', word.chars().into_iter().collect::<Vec<char>>());
            } else if word.len() == 4 {
                check_and_add('e', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('a', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('f', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('b', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('e', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('a', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('f', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('b', word.chars().into_iter().collect::<Vec<char>>());
            } else if word.len() == 7 {
                check_and_add('a', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('c', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('e', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('d', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('g', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('f', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('b', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('a', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('c', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('e', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('d', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('g', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('f', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('b', word.chars().into_iter().collect::<Vec<char>>());
            } else if word.len() == 2 {
                check_and_add('a', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                check_and_add('b', &mut map,  word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('a', word.chars().into_iter().collect::<Vec<char>>());
                // map.insert('b', word.chars().into_iter().collect::<Vec<char>>());
            }
        });
        println!("{:?}", map);
    });
    println!("{:?}", sum);
}

fn intersect(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut out:Vec<char> = Vec::new();
    for item in a.iter() {
        if b.contains(item) {
            out.push(*item);
        }
    }
    out
}

fn check_and_add(a: char, map: &mut HashMap<char, Vec<char>>, chs: Vec<char>) {
    if map.contains_key(&a) {
        let mut v = map.get(&a).unwrap().clone();
        let mut m = map.get_mut(&a).unwrap();
        *m = intersect(&v, &chs);
    } else {
        map.insert(a, chs);
    }
    println!("{:?} {:?}", a, map.get(&a).unwrap());
}

*/
