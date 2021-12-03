use std::fs;
fn main() {
    let mut pnumber = 100000000;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut increased = 0;
    let parsed_inputs = input.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    for i in 0..parsed_inputs.len()-2 {
        let sum = parsed_inputs[i] + parsed_inputs[i+1] + parsed_inputs[i+2];
        if sum > pnumber {
            increased += 1;
        }
        pnumber = sum;
    }
    //    let number = line.parse::<i32>().unwrap();
    //    increased += if number > pnumber {
    //        1
    //    } else {
    //        0
    //    };
    //    pnumber = number;
    // });
    println!("{}", increased);
}
