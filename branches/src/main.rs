fn main() {
    // Hello, world!

    if_else();

    loops_whiles();
}

fn if_else() -> () {
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

fn loops_whiles() -> () {
    println!("\n[LOOP START]");
    let mut count: i8 = 0;
    'counting: loop {
        println!("count... {count}");
        let mut remaining: i8 = 10;

        loop {
            println!("remaining... {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("count... {count}");
    println!("[LOOP END]\n");

    for num in (1..10).rev(){
        println!("...{num}");
    }
    /*let mut x: u8 = 10;
    while x > 0 {
        println!("... {x}");
        x -= 1;
    }*/
    println!("LIFTOFF!! 🚀\n");

    let a = [10, 20, 30, 40, 50];

    println!("[WHILE START]");
    let mut index = 0;
    while index < a.len() {
        let v = a[index];
        println!("the value is... {v}");
        index += 1;
    }
    println!("[WHILE END]\n");
    // or
    println!("[FOR START]");
    for v in a {
        println!("the value is... {v}");
    }
    println!("[FOR END]\n");
}
