use std::io;

fn main() {

    let x = 123_000u32; // unsigned integer (also can be written  as 123_000_u32, 123_000 as u32)
    println!("x is {}", x);

    let mut x : u32 = 4; // muttable
    println!("x is {}", x);
    x=x+1;
    println!("x+1 is {}", x);
    const SECONDS_IN_MINUTE:u32 = 60; // constant should be capitalize
    println!("SECONDS_IN_MINUTE is {}", SECONDS_IN_MINUTE);

    let  x: i32 = -4; // signed integer, also immutable
    println!("x is {}", x);

    let  x: f32 = 4.86; // floating number
    println!("x is {}", x);

    let  x = (i32::MAX as i64) + 1;
    println!("(max i32) + 1 is {}", x);

    let mut tup:(i32, bool, char)=(1,true,'c');
    println!("tup.0 is {}", tup.0);
    tup=(2,true,'c');
    println!("tup.0 is {}", tup.0);

    let arr:[i32;5] = [1,2,3,4,5]; //array with 5 integer value
    println!("arr[0] is {}", arr[0]);

    let mut input = String :: new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let input:i64 = input.trim().parse().unwrap();

    println!("input+1 is {}", input+1);

    let num ={
        let x=3;
        x+1 // no semicolon
    };
    println!("num is {}", num);
    println!("add(2,3) is {}", add(2,3));

    let str=String::form("hello"); // value is stored on heap (name,value) as (str,"hello") bcoz its dynamic in size, also stored on stack  (name,value) as (str, pointer to str in heap)
    println!("str is {}",str)
}

fn add(x:i32,y:i32)->i32{
    if (x<0 || y<0){
        return 0
    }
    x+y
}