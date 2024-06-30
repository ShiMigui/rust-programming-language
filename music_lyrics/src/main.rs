fn main() {
    let phrases = [
        "A partridge in a pear tree!",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let positions = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut index = 0;
    let phrases_length = phrases.len();

    while index < phrases_length {
        let position = positions[index];

        println!("On the {position} day of Christmas\nmy true love gave to me");

        let mut i: i8 = index as i8;
        while i >= 0 {
            let mut phrase = phrases[i as usize].to_string();

            if i == 0 && index != 0 {
                phrase = format!("And {}", phrase.to_lowercase());
            }

            println!("{phrase}");
            i -= 1;
        }
        println!();

        index += 1;
    }
}
