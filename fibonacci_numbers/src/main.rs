fn main() {
    gen_fibonacci_table(30);
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