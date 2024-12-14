fn main() {

    let gifts = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut day = 1;

    loop {
        let ordinal_day = match day {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => "invalid day",
        };

        println!("On the {ordinal_day} day of christmas, my true love gave to me");

        let mut i = day;

        while i != 0 {
            println!("{}", gifts[i - 1]);
            i -= 1;
        }

        if day == 12 {
            break;
        }
        day += 1;
    }
}
