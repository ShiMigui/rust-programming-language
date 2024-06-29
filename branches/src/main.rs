fn main() {
    // Hello, world!

    let num: u8 = 13;

    if num >= 5 {
        println!("A plate of wheat for three sad tigers");
    } else {
        println!("The mouse gnawed the king of Rome's clothes");
    }

    if num != 0 {
        println!("The number is something other than zero");
    }

    if num % 2 == 0 {
        println!("The number is divisible by 2");
    } else if num % 3 == 0 {
        println!("The number is divisible by 3");
    } else {
        println!("The number isn't divisible by 2 or 3");
    }

    let i_no_have_idea = if num == 13 { "Hello" } else { "World" };

    println!("{i_no_have_idea}");
}
