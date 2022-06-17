// fn main() {
//     let x = (10,20);

//     let mut student_a = Student {
//         name : "Alice".to_string(),
//         age: 18,
//         class: "A1".to_string(),
//     };
//     student_a.name = "Eve".to_string();
//     println!("student_a name:{}", student_a.name);

//     let mut student_b = Student {
//         name : "Bob".to_string(),
//         age: 18,
//         class: "B1".to_string(),
//     };

//     //student_b.print_age();
//     student_b.upgrade_age();
//     println!("student_b age:{}", student_b.age);

//     student_a.clone().print_age();

//     Student::print_age(student_a.clone());

//     let student_c = Student::new();
//     let name = student_c.get_name();
//     student_c.get_name();
//     let student_d = Student {
//         name : "Age".to_string(),
//         age: 18,
//         class: "B1".to_string(),
//     };
//     student_d.get_name();


//     // enum

//     let class_a = Class::A;
//     Class::print_a();
//     let b = Class::get_b();
//     println!("B:{:?}",b);
//     Class::get_c(class_a);

// }

// // Student : class
// // name,class,age: trường dữ liệu
// // instance: bản thể (instance)


// #[derive(Clone)]
// struct Student {
//     name: String,
//     age: u8,
//     class: String,
// }

// impl Student {

//     fn new() -> Self{
//         Self { name: "Charlie".to_string(), age: 18, class: "C1".to_string()}
//     }
//     fn print_age(self) {
//         println!("Name:{}", self.name);
//     }
//     // ownership

//     fn upgrade_age(&mut self) {
//         self.age +=10;
//     }
//     // mutable reference

//     // let s1 =String::from("hello");
//     // let s2 =&s1;
//     fn get_name(&self) -> &String {
//         //(*self.name).to_string()
//         &self.name
//     }
//     // shared reference

// }


// // self vs Self
// // struct StudentA{} // unit-struct


// // struct StudentB {
// //     name: (String, String),
// //     age: u8,
// //     class: String,
// // }


// // ko cần khai báo kiểu dữ liệu 
// // giới hạn 
// // mục đích : lựa chọn 
// #[derive(Debug)]
// enum Class {
//     A,
//     B,
//     C,
// }

// // có implement dc cho enum
// impl Class {
//     fn print_a(){
//         println!("A:{:?}", Class::A);
//     }
    
//     fn get_b() -> Class {
//         Self::B
//     }

//     fn get_c(self) -> Class {
//         self
//     }
// }

// enum ClassA {
//     A(u32,u8),
//     B,
//     C
// }

// // lồng struct
// enum ClassC {
//     A(Student),
//     B
// }

// // struct lồng enum

// struct StudentD {
//     name: String,
//     age: u8,
//     class: ClassC,
// }



// enum Day {   
//     Monday, 
//     Tuesday, 
//     Wednesday, 
//     Thursday, 
//     Friday, 
//     Saturday, 
//     Sunday, 
// } 
 
// impl Day { 
//     fn mood(&self) { 
//         println!("{}", match *self { 
//             Day::Friday => "it's friday!", 
//             Day::Saturday | Day::Sunday => "weekend :-)", 
//             _ => "weekday...", 
//         }) 
//     } 
// } 
 
// fn main() { 
//     let today = Day::Tuesday; 
//     today.mood(); 
//     today.mood();
// }

// #[derive(PartialEq)]
// enum Direction { North, East, South, West }

// fn main (){
//     let x = Direction::North;
//     // match x {
//     //     Direction::North => println!("True"),
//     //     Direction::East => println!("False"),
//     //     _ => println!("False"),
//     // }

//     if x == Direction::North { 
//         println!("True");
//     }
//     else {
//         println!("False");
//     }
// }


//vec
// fn main(){
//     let mut a : Vec<u8> = Vec::new(); //1.Sử dụng new() method
//     let mut b: Vec<u32> = vec![]; //2. Sử dụng vec! macro

//     //Lấy giá trị và thay đổi giá trị
//     let mut c = vec![5, 4, 3, 2, 1];
//     c[0] = 1;
//     c[1] = 2;
//     //c[6] = 8;
//     println!("{}",c[6]);

//     let mut v = vec![1, 2, 3, 4, 5];
//     // for i in &v {
//     //     println!("A reference to {}", i);
//     // }

//     // for i in v {
//     //     println!("Take ownership of the vector and its element {}", i);
//     // }

//     // for i in &mut v {
//     //     println!("A mutable reference to {}", i);
//     // }

//     //iter(), into_iter(), iter_mut();

//     // for i in &v : shared reference
//     // for (i,v) in v.iter().enumerate() {
//     //     println!("Take ownership of the vector and its element {}", i);
//     // }
//     // println!("{:?}",v);

//     // iter_mute() -> mutable reference -> for i in &mut v

//     // into_iter -> ownership, mutable reference , shared reference

//     for i in v {
//         //*i +=1;
//         println!("i:{}",i);

//     }
//     //println!("{:?}",v);  
// }

// fn main(){
//     get_integer_u8(10u8);
//     let int_1 = get_integer(10u8);
//     let int_2 = get_integer(100u32);
//     let int_2 = get_integer("Hello".to_string());


//     let student_a = Student::<f64> {
//         name: "A".to_string(),
//         grade:1.2f64,
//     };

//     let student_b = Student::<String> {
//         name: "A".to_string(),
//         grade:"A+".to_string(),
//     };

//     let student_c = Student::<String>::new();


//     let x = Some(7);
//     let x_unwrap = x.unwrap();
//     let y: Option<u8> = None;
// }
// //generic type: kiểu dữ tổng hợp , truyền dữ liệu vào lấy dữ liệu đấy

// fn get_integer_u32(value: u32) {
//     println!("{}",value);
// }

// fn get_integer_u8(value:u8 ) {
//     println!("{}",value);
// }

// fn get_integer<T>(value: T) -> T{
//     //println!("{}",value);
//     value
// }


// struct Student<T> {
//     name:String,
//     grade: T,
// }

// impl<T> Student<T> {
//     fn new() -> Student<u8>{
//         Student { name: "H".to_string(), grade: 10u8 }
//     }
// }
// // fn get_integer<u32>(value: u32) -> u32{
// //     //println!("{}",value);
// //     value
// // }

// // fn get_integer<u8>(value: u8) -> u8{
// //     //println!("{}",value);
// //     value
// // }


// struct StudentA<T> {
//     name: T,
//     grade: T,
// }

// struct StudentB<T,V> {
//     name: V,
//     grade: T,
// }


// impl<T,V> StudentB<T,V> {}


// enum Class<T> {
//     A(T),
//     B,
//     C
// }

// Option : sự lựa chọn Some(value), None


// Exercise 1
// fn main() {
//     let mut shopping_list: Vec<String> = Vec::new();
//     shopping_list.push("milk".to_string());
// }

// fn main(){

// }
// struct Wrapper<T> {
//     value: T,
// }
// impl<T> Wrapper<T> {
//     pub fn new(value: T) -> Self {
//         Wrapper { value }
//     }
// // }


// fn main(){
//     let report_card_f64 = ReportCard{
//         grade: 1.2f64,
//         student_name:"A".to_string(),
//         student_age: 13,
//     };

//     let report_card_f64 = ReportCard{
//         grade: "A+".to_string(),
//         student_name:"A".to_string(),
//         student_age: 13,
//     };
// }

// #[derive(Debug)]
// pub struct ReportCard<T> {
//     pub grade: T,
//     pub student_name: String,
//     pub student_age: u8,
// }

// impl<T: std::fmt::Debug> ReportCard<T> {
//     fn print_grade(&self){
//         println!("{:?}", self.grade);
//     }
// }


// #[derive(Debug)]
// enum Message {
//     // TODO: define a few types of messages as used below
//     Quit,
//     Echo,
//     Move,
//     ChangeColor,
// }
// fn main() {
//     println!("{:?}", Message::Quit);
//     println!("{:?}", Message::Echo);
//     println!("{:?}", Message::Move);
//     println!("{:?}", Message::ChangeColor);
// }


// #[derive(Debug)]
// enum Message {
//     // TODO: define the different variants
//     Move{x:u32, y:u32},
//     Echo(String),
//     ChangeColor(u32,u32,u32),
//     Quit,
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", &self);
//     }
// }
// fn main() {
//     let messages = [
//         Message::Move { x: 10, y: 30 },
//         Message::Echo(String::from("hello world")),
//         Message::ChangeColor(200, 255, 255),
//         Message::Quit,
//     ];

//     for message in &messages {
//         message.call();
//     }
// }


// // option : Some(value), None
// fn print_number(maybe_number: Option<u16>) {
//     println!("printing: {}", maybe_number.unwrap());
// }

// fn main() {
//     print_number(Some(13));
//     print_number(Some(99));

//     let mut numbers: [Option<u16>; 5] = [None, None, None,None,None];
//     for iter in 0..5 {
//         let number_to_add: u16 = {
//             ((iter * 1235) + 2) / (4 * 16)
//         };

//         numbers[iter as usize] = Some(number_to_add);
//     }
// }



// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }
// impl MyData {
//     pub fn get_val1(&self) -> i32 {
//         return self.val1.clone();
//     }
//     pub fn get_val2(&mut self) -> String {
//         return self.val2.clone();
//     }
//     pub fn get_both(&self) -> (i32, String) {
//         return (self.val1, (*self.val2).to_string());
//     }
// }
// fn main() {
//     let mut d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };
//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();
// }


// fn main() {
//     let a = A {p: Some("p".to_string())};
//     a.a();
// }

// struct A {
//     p: Option<String>
// }

// impl A {
//     fn a(&self) -> &Self {
//         Self::b((*self.p.unwrap()).to_string());
//         self
//     }
//     fn b(b: String) {
//         print!("b: {}", b)
//     }
// }


fn main() {
    let a = A {p: Some("p".to_string())};
    a.a();
}

struct A {
    p: Option<String>
}


//&Option<String> -> Option<&String>
impl A {
    fn a(self) -> Self {
        Self::b(&self.p.as_ref().unwrap());
        self
    }
    fn b(b: &str) {
        print!("b: {}", b)
    }
}

//&mut &

// fn function_a(a: String) -> &String{
//     &a
// }