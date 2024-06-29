fn main() {
    //Shadow variables
    /*let x: i8 = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Shadowing the x value: {x}");
    }

    println!("Current x value: {x}"); // returns x value to (x + 1)*/

    //Operations

    /*let sum = x + 2;
    let sub = x - 3;
    let mul = x * 2;
    let div = x / 5;
    let rem = x % 2;*/

    //Tuple and Array
    /*let person: (i8, &str, f32) = (1, "Miguel Nascimento dos Santos", 3.1415);

    let (cd, name, value) = person; // also, I can access the value with VARIABLE_NAME.INDEX. e.g. person.1
    println!("Person(cd: {cd}, name: {name}, value: {value})");*/

    /*let arr: [i8/* Type */; 5 /* Length */] = [1, 2, 3, 4, 5];
    let arr2: [i8; 5] = [3; 5]; // [3, 3, 3, 3, 3]
    let length = arr2.len();
    let mut i = 0;

    loop {
        if i < length {
            let v = arr2[i];
            println!("{i}: {v}");
            i += 1;
        } else {
            break;
        }
    }*/
}
