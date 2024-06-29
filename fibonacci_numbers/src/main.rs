fn main() {
    print_fibonacci_at(30, None);
}

fn print_fibonacci_at(nth: u8, str: Option<&str>) {
    let str = str.unwrap_or("\n");

    let mut current_index = 0;
    let mut current_number: u32 = 1;
    let mut previous_number = 0;

    while current_index < nth {
        print!("{current_number}{str}");
        let temp_previous = previous_number;
        previous_number = current_number;
        current_number += temp_previous;

        current_index += 1;
    }
}
