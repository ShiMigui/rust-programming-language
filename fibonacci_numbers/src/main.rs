use std::io::stdin;

fn main() {
    println!("Type a number(max. 255): ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");

    let num = input.trim().parse().expect("Type a number!");
    println!();
    gen_fibonacci_table(num);
}

fn gen_fibonacci_table(length: u8) {
    let mut counter = 0;
    let mut current_number: u32 = 1;
    let mut previous_number = 0;

    while counter < length {
        let c = format!("{:02}", counter + 1);
        println!("{c} - {current_number}");
        let tmp = previous_number;
        previous_number = current_number;
        current_number += tmp;

        counter += 1;
    }
}
