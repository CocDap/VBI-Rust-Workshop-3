// //

// // fn main(){
// //     {
// //         let r;
// //         // {
// //         //     let x =5;
// //         //     r = &x;
// //         // }

// //         let x =5;
// //         r = &x;

// //         //dangling reference
// //         println!("r:{}", r);
// //     }
// // }

// // fn main() {
// //     let string1 = String::from("abcd");
// //     let string2 = "xyz";

// //     let result = longest(&string1, string2);
// //     println!("The longest string is {}", result);
// // }
// // &Vec và Vec<&T>
// // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// //     if x.len() > y.len() {
// //         return "abc";
// //     } else {
// //         return "xyz";
// //     }
// // }

// // fn longest<'a,'b>(x: &'a str, y: &'b str) -> &'a str {

// //     if x.len() > y.len() {
// //         x
// //     } else {
// //         y
// //     }
// // }

// // Trait

// // fn main() {
// //     let x: u32 = 100;
// //     let res = x.say_hello();
// //     println!("{}", res);
// //     let person = Person {};
// //     let person2 = Person2 {};
// //     x.get_hello();
// //     person.get_hello();
// //     person2.get_hello();

// //     give_greeting(person);
// // }

// // struct Person {}

// // impl Person {
// //     fn say_hello_1(&self) -> String{
// //         "Hello".to_string()
// //     }
// // }
// // struct Person2 {}
// // impl Person2 {
// //     fn say_hello_2(&self) -> String{
// //         "World".to_string()
// //     }
// // }

// // struct Person3 {}
// // impl Person3 {
// //     fn say_hello_3(&self) -> String{
// //         "TOI".to_string()
// //     }
// // }
// trait Speak {
//     fn say_hello(&self) -> String;
//     fn get_hello(&self) {
//         println!("Hello World Default");
//     }
// }

// // trait Speak {
// //     fn say_hello(&self) -> String;
// // }

// impl Speak for String {
//     fn say_hello(&self) -> String {
//         "String".to_string()
//     }
// }

// impl Speak for u32 {
//     fn say_hello(&self) -> String {
//         "String".to_string()
//     }
// }
// struct Person {}
// impl Speak for Person {
//     fn say_hello(&self) -> String {
//         "Hello".to_string()
//     }
// }
// struct Person2 {}

// impl Speak for Person2 {
//     fn say_hello(&self) -> String {
//         "World".to_string()
//     }
//     fn get_hello(&self) {
//         println!("Hello Person2");
//     }
// }

// // trait + function : ko có hành vi rõ ràng

// struct Person4 {}

// trait Listen {
//     fn say_listen(&self) -> String;
// }
// fn give_greeting(p: impl Listen) {
//     println!("{}", p.say_listen());
// }

// // fn give_greeting_trait_bound<T: Listen + Speak + Hi + Function, G: Listen+ Debug>(p: T) {
// //     println!("{}", p.say_listen());
// // }

// // fn give_greeting_trait_bound_where<T>(p: T)
// // where
// //     T: Listen + Speak + Hi + Function,
// //     G: Listen+ Debug
// // {
// //     println!("{}", p.say_listen());
// // }

// // trait bound

// impl Listen for Person {
//     fn say_listen(&self) -> String {
//         "Hello Listen".to_string()
//     }
// }
// // fn give_greeting() {
// //     println!("{}","Hello World");
// // }


// #[derive(Debug)]
// struct Class {
//     x: u32
// }


// struct Mate{

// }

// #[derive(Debug)]
// enum A {

// }

// // pub use std::fmt;
// // impl fmt::Debug for Class {
// //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// //         f.debug_struct("Class").field("x", &self.x).finish()
// //     }
// // }
// fn main (){
//     // let class = Class{x: 10};
//     // println!("{}",19);
//     // println!("{:?}", class);

//     // let mut counter = Counter::<u32>{x:0, y:1};
//     // let res1= counter.next();
//     // println!("res1: {}",res1);

//     // let res2= counter.next();
//     // println!("res2: {}",res2);

//     // let res3= counter.next();
//     // println!("res2: {}",res3);

//     let pointer1 = Point{x:10,y:20};
//     let pointer2 = Point{x:10,y:20};
//     let pointer3 = pointer1+ pointer2;

// }

// //pub use std::ops::AddAssign;
// pub use std::ops::Add;
// struct Counter {
//     x:u32,
//     y:u32
// }

// //use std::ops::Add;
// #[derive(Debug, Copy, Clone, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// // impl<T: Copy+ Add<Output = T>> Iterator<T> for Counter<T>{
// //     type Output = Self;
// //     fn next(&mut self) ->T{
// //         self.x +=self.y;
// //         self.x
// //         //return self.x+ self.y;
// //     }
// // }
// // pub trait Iterator<T>{
// //     fn next(&mut self) ->T;
// // }



// use std::io;
// fn main() {
//     let mut input: Vec<String>;
//     loop {
//         let mut input_text = String::new();
//         println!("Type instruction in the format Add <name> to <department>:");
//         io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
//         let trimmed_text: String = input_text.trim().to_string();
//         //Vec<String> chứ ko phải Vec<&str>
//         input = trimmed_text.split(" ").map(|x|x.to_string()).collect();
//         if input[0] == "Add" && input[2] == "to" {
//             break;
//         } else {
//             println!("Invalid format.");
//         }
//     }
//     println!("{:?}", input);
// }


// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for String {
//     //Add your code here
//     fn append_bar(self) -> Self{
//         //format!("{}Bar",self)
//         self + "Bar"
//     }
// }

// fn main() {
//     let s = String::from("Foo");
//     let s = s.append_bar();
//     println!("s: {}", s);
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_foo_bar() {
//         assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
//     }

//     #[test]
//     fn is_bar_bar() {
//         assert_eq!(
//             String::from("").append_bar().append_bar(),
//             String::from("BarBar")
//         );
//     }
// }


trait AppendBar {
    fn append_bar(self) -> Self;
}
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self{
        // self.push("Bar".to_string());
        // self.iter().map(|x| x.to_string()).collect()

        let mut res = self.clone();
        res.push("Bar".to_string());
        res

        // self.push("Bar".to_owned());
        // self

        // let vec = String::from("Bar".to_string());
        // self.append(&mut vec);
        // self

    }
}
//TODO: Add your code here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
