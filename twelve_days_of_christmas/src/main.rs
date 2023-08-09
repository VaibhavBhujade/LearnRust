// On the first day of Christmas
// My true love brought to me
// A partridge in a pear tree

// On the second day of Christmas
// My true love brought to me
// Two turtle doves
// And a partridge in a pear tree

// On the third day of Christmas
// My true love brought to me
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the fourth day of Christmas
// My true love brought to me
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the fifth day of Christmas
// My true love brought to me
// Five goldenen rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the sixth day of Christmas
// My true love brought to me
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the seventh day of Christmas
// My true love brought to me
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the eighth day of Christmas
// My true love brought to me
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the ninth day of Christmas
// My true love brought to me
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the tenth day of Christmas
// my true love sent to me
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the eleventh day of Christmas, my true love sent to me
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the twelfth day of Christmas
// my true love sent to me
// Twelve drummers drumming
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
fn main() {

    let gift_lines = ["A partridge in a pear tree","Two turtle doves, and","Three french hens","Four calling birds",
                            "Five golden rings","Six geese a-laying","Seven swans a-swimming","Eight maids a-milking",
                            "Nine ladies dancing","Ten lords a-leaping","Eleven pipers piping","Twelve drummers drumming"];

    let second_line = "my true love sent to me";

    let first_line = ["On the", "day of Christmas"];

    let days = ["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];

    let mut count = 1;

    for element in days {

        println!("{} {} {}", first_line[0], element , first_line[1]);
        println!("{}" , second_line);

        for index in 0..count {
            println!("{}",gift_lines[count-index-1]);
        }
        println!("");
        count+=1;
    }
}
