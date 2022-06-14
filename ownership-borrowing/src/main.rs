use std::f32::consts::E;

// // 
// fn main() {
//     let mut s1 = String::from("hello");

//     // println!("s1:{}", s1);
//     // let s2 = &mut s1;
//     // println!("{}", s1);// lỗi
//     // println!("{:p}", s2);
//     //shared reference
//     // s2.push_str("world");
//     // println!("s2:{}", s2);

//     // mutable reference

//     //let s2 = &s1;
//     //println!("s2:{}", s2);
//     // {
//     //     let s5 = &mut s1;
//     // }
//     // let s5 = &mut s1;
//     // let s6 = &mut s1;
    
//     // s5.push_str("world");

//     // println!("s5:{}", s6);
//     let s2 = &s1;
//     let s3 = &s1;
//     println!("{}",s2);
//     println!("{}",s3);
//     // println!("s6:{}", s6);
//     //let s6 = &mut s1;



// }


//fn main(){
    // let x = 10;
    // let y = &x;
    // println!("{}",x);
    // println!("{}",y);
    // println!("address x:{:p}",&x);
    // println!("{:p}",y);

    // let x = test(1, &mut 30);
    // println!("gia tri cua x:{}",x);

    // let s = String::from("hello");
    // let len = test_string(s);
    //let s1 =s;
    // let s1 = &s;
    // let s3 = "hello";
    // let len_string = test_string(s3);
    // println!("{}",len_string);

    //dangling reference
    //let str = test();


//}

// fn test(input:u32, output :&mut u32) -> u32 {
//     if input ==1 {
//         *output = 10;
//     }
//     else {
//         *output = 5;
//     }

//     *output
// }

// fn test_string(x: String) -> usize {
//     x.len()
// }

// fn test()-> String{
//     let s = String::from("hello");
//     s
// }

// fn test_string(x: String) -> (usize, String) {
//     (x.len(), x)
// }

// fn main() {
//     let reference_to_nothing = dangle();
// }
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }


//Exercise 1
// fn main() {
    
//     let x = change_value(10,&mut 20);
// }
// fn change_value(input:u32, output: &mut u32) -> u32{
//     if input ==1 {
//         *output =3;
//     }
//     else {
//         *output = 4;
//     }

//     *output
// }

// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     while count < 10 {
//         num += 2;
//         if vector_is_prime(num, &primes) {
//             count += 1;
//             primes.push(num);
//         }
//     }
//     println!("{:?}", primes);

//     let array = &[10,20];
// }
// // p = &vec[1,2,3,4] -> &1, &2, &3

// fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
//     for &i in p {
//         if num > i && num % i != 0 {
//             return false;
//         }
//     }

//     true
// }


//Exercise 3
fn main() {
    let mut values = vec![10, 11, 12];
    let v = &mut values;

    let mut max = 0;
    //let v1 = &v;
    
    for n in v.iter() {
        max = std::cmp::max(max, *n);
    }
    //println!("{:?}",v);
    println!("max is {}", max);
    println!("Converting to percentages of maximum value...");
    
    for n in v {
        *n = 100 * (*n) / max;
    }
    println!("values: {:#?}", values);
}