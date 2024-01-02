use std::io;

fn main() {
    //variables
    println!("Hello, world!");
    let x = 4;
    println!("x is: {}", x);
    {
        //pipeline-esque
        let x = x - 2;
        println!("x is: {}", x);
    }
    let x = x + 1;
    println!("x is: {}", x);
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("There are {} seconds in a minute.", SECONDS_IN_MINUTE);

    //data types
    let _x: i32 = 2; //whole numbers, i = int, 32 means 32 bytes other i8/16/32/64/128
    let _y: u32 = 3; //unit same as int, cant be negative
    let _zfloat: f32 = 4.1; //float point value
    let _torf: bool = false; //boolean
    let _letter: char = ';';

    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.1);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {} {} {} {}", arr[0], arr[1], arr[2], arr[3], arr[4]);
    arr[2] = arr[2] + 4;
    println!("{}", arr[2]);
    println!("{} {} {} {} {}", arr[0], arr[1], arr[2], arr[3], arr[4]);

    //userinput
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);

    user_input()
}

fn user_input() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}