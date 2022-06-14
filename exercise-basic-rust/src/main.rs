// Bài tập 1: Cho 2 mảng có các phần tử là số nguyên dương, kiểm tra mảng này có phải là mảng con của mảng kia không ?(yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//             let sub_arr = [6,8,10];

// Bài tập 2 : Cho 1 chuỗi str Slice như dưới đây. Nhập bất kỳ ký tự nào  từ bàn phím, in ra số lượng ký từ này này xuất hiện trong chuỗi đã cho. 
// Nâng cao hơn : Tìm kiếm không phân biêt chữ hoa thường, theo dạng regex.
// https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.txt
// Ví dụ: cho chuỗi str = “This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.”
// Nhập từ bàn phím : “is is” => In ra kết quả số lượng “is is” là 5



// Bài tập 1
// fn main() {
//     let v = [1,3,2,10,5,6];
//     let v_child = [3,2,10,5];
//     println!("{}",v.len());
//     let mut i = 0;
//     let mut j =0 ;
//     let m = v.len();
//     let n = v_child.len();

//     while i< m && j < n {
//         if v[i] == v_child[j]{
//             i+=1;
//             j+=1;

//             if j ==n{
//                 println!("True");
//             }

//         }
//         else {
//             i = i-j +1;
//             j =0;
//         }
//     }

// }



//Exercise 2

// fn main(){
//     // Mọi người tự làm thêm phần để gọi các hàm
// }
// fn find_text() {

//     let input = input_from_stdin();


//     let text = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
//     let text = read_file("a.txt");
   
//     let first_char = input.chars().next().unwrap().to_lowercase().next().unwrap();

//     let chars = text.chars().collect::<Vec<char>>();
//     let input_chars = input.chars().collect::<Vec<char>>();

//     let mut index = 0;
//     let mut count = 0;
 
//     loop {

//         if index >= text.len() {
//             break;
//         }

//         if chars[index].to_lowercase().next().unwrap() == first_char { 

//             if index + input.len() > text.len() {
//                 println!("Khong hop le");
//                 break;
//             }

//             let mut check = true;
//             for i in 1..input.len() {
//                 if chars[index + i] != input_chars[i] {
//                     check = false;
//                 }
//             }

//             if check {
//                 index = index + input.len();
//                 count += 1;
//                 continue;
//             }
//         }

//         index += 1;
//     }

//     println!("count = {}", count);

// }

// use std::fs::File;
// fn read_file(path: &str) -> String {
//     let mut file = File::open(path).expect("File not found");
//     let mut data = String::new();
//     file.read_to_string(&mut data)
//                                     .expect("Error while reading file");
//     data
// }

// fn input_from_stdin() -> String {
//     let mut line = String::new();
//     println!("Enter any word: ");
//     io::stdin().read_line(&mut line).expect("Error at input");
//     line.pop();
//     line
//     // line.trim().to_string()
// }






