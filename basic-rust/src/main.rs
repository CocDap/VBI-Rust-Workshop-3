
use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let x = 5;
    let mut y = 10;

    y = 20;
    // x = 30;

    const PI : f32 = 3.14;

    let a = 5;
    {

        let a = 10;
        println!("{}", a);
    }

    let a = "Tom";
    println!("{}", a);


    let num = 1000_000_000;

    let num1 = num as i8;

    let arr : [u32; 3] = [1, 2, 3];

    println!("{:?}", arr);
    println!("{}", arr[0]);

    let [a, b, c] = arr;


    let name = String::from("Hello World");

    let str = &name[0..4];


    println!("name = {}, str = {}", name, str);


    let a = 5;

    if a % 2 == 0 {
        println!("{} la so chan", a);
    } else {
        println!("{} la so le", a);
    }

    let str = match a {
        5 => "la so 5", 
        _ => "khong phai la 5"
    };


    loop {

        if a == 5 {
            break;
        }




    }

    // for 


    // let mut user_name = String::from("Johnny Depp");
    // let name = &mut user_name[0..4];
    // // user_name.push_str( 'A');
    // name.replace(from, to)

    // println!("{}", name);


    // let mut name = String::from("Hello world!");
    // let str = &name[0..4];

    // println!("hello:{}", name );
    // name.push_str("ABVC");
    // println!("hello:{}", name );


    let mut name = String::from("hello"); 
    
    // let temp = &mut name[0..4];
    let temp = &mut name;
    temp.push_str("AAA");
    println!("temp = {}", temp);

    println!("name = {}", name);
   




    let input = "નમસ્તે";

    let mut output = String::new();

    for ch in input.chars() {
        output.insert(0, ch);
    }

    println!("Reverse string = {} to {}", input, output);
    println!("Reverse string = {} to {}", input, input.chars().rev().collect::<String>());


    loop {
        let mut line = String::new();
        println!("Enter a number:");
        io::stdin().read_line(&mut line).unwrap();
    
        
        // println!("input = {}", line);
    
        line.pop(); // \n
    
        println!("input = {}", line);
    
        let number = line.parse::<i32>().unwrap();
    
        if number < 0 {
            break;
        }

        let rand_number = rand::thread_rng().gen_range(0..100);
        println!("Result = {}", rand_number);

        if number == rand_number {
            println!("You'r winner");
        } else {
            println!("Try again!");
        }


    }
   

    println!("End game");






}
