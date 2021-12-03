use std::fs;

fn main() {
    let mut bits = Vec::new();
    let mut lastremovedo = "a";
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    input.lines().for_each(|line| {
        bits.push(line);
    });

    let mut bitso = bits.clone();
    let mut oxygen_str = String::new();
    
    'outera: for i in 0..12 {
        let mut onec = 0;
        let mut zeroc = 0;
        for j in 0..bitso.len() {
            if bitso[j].chars().nth(i) == Some('1') {
                // oxygen_str.push_str("1");
                onec += 1;
            } else {
                // oxygen_str.push_str("0");
                zeroc += 1;
            }
        }
        println!("before added :{}" , oxygen_str);
        if onec >= zeroc {
            oxygen_str.push_str("1");
        } else {
            oxygen_str.push_str("0");
        }
        println!("after added :{}" , oxygen_str);
        let mut removed = 0;
        for j in 0..bitso.len() {
            println!("in array {}", bitso[j-removed]);
            if bitso[j-removed].chars().nth(i) != oxygen_str.chars().nth(i) {
                println!("removed :{}", bitso[j-removed]);
                lastremovedo = bitso[j-removed].clone();
                bitso.remove(j-removed);
                removed += 1;
                if bitso.len() == 0 {
                    oxygen_str =  lastremovedo.to_string().clone();
                    break 'outera;
                }
            }
        }
    }


    let mut lastremoved = "a";
    let mut bitsc = bits.clone();
    let mut carbon_str = String::new();
    'outer: for i in 0..12 {
        let mut onec = 0;
        let mut zeroc = 0;
        for j in 0..bitsc.len() {
            if bitsc[j].chars().nth(i) == Some('1') {
                // oxygen_str.push_str("1");
                onec += 1;
            } else {
                // oxygen_str.push_str("0");
                zeroc += 1;
            }
        }
        if onec > zeroc {
            carbon_str.push_str("0");
        } else {
            carbon_str.push_str("1");
        }
        let mut removed = 0;
        for j in 0..bitsc.len() {
            if bitsc[j-removed].chars().nth(i) != carbon_str.chars().nth(i) {
                // println!("{}", bitsc[j-removed]);
                lastremoved = bitsc[j-removed].clone();
                bitsc.remove(j-removed);
                removed += 1;
            }
            if bitsc.len() == 1 {
                carbon_str =  lastremoved.to_string().clone();
                break 'outer;
            }
        }
    }
    println!("{}, {}", oxygen_str, carbon_str);

    println!("{}", isize::from_str_radix(&carbon_str, 2).unwrap() * isize::from_str_radix(&oxygen_str, 2).unwrap());
}
