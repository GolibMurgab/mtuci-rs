// fn main() {
// let x = 5;
// println!("The value of x is: {}", x);
// x = 6;
// println!("The value of x is: {}", x);
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// }

// fn main() {
//     let x = 5;
//
//     let x = x + 1;
//
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("{spaces}");
// }

// fn main() {
//     let mut spaces = "   ";
//     spaces = spaces.len();
// }

// fn main() {
// let guess: u32 = "42".parse().expect("Not a number!");
// }

// fn main() {
//     let x = 2.0; // f64
//
//     let y: f32 = 3.0; // f32
// }

// fn main() {
//     // addition
//     let sum = 5 + 10;
//
//     // subtraction
//     let difference = 95.5 - 4.3;
//
//     // multiplication
//     let product = 4 * 30;
//
//     // division
//     let quotient = 56.7 / 32.2;
//     println!("{quotient}");
//     let truncated = -5 / 3; // Results in -1
//
//     // remainder
//     let remainder = 43 % 5;
// }

// fn main() {
//     let t = true;
//
//     let f: bool = false; // with explicit type annotation
// }

// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// fn main() {
//     let tup = (500, 6.4, 1);
//
//     let (x, y, z) = tup;
//
//     println!("The value of y is: {y}");
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//
//     let five_hundred = x.0;
//
//     let six_point_four = x.1;
//
//     let one = x.2;
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }
//
// fn main() {
// let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];
// }

// fn main() {
// let a: [i32; 5] = [1, 2, 3, 4, 5];
// }

// fn main() {
// let a = [3; 5];
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//
//     let first = a[0];
//     let second = a[1];
// }



// fn main() {
//     let a = [1, 2, 3, 4, 5];
//
//     println!("Please enter an array index.");
//
//     let mut index = String::new();
//
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
//
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
//
//     let element = a[index];
//
//     println!("The value of the element at index {index} is: {element}");
// }

//3.3 функции

// fn main() {
//     println!("Hello, world!");
//
//     another_function();
// }
//
// fn another_function() {
//     println!("Another function.");
// }

// fn main() {
//     another_function(5);
// }
//
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }
//
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main() {
//     let y = 6;
// }

// fn main() {
//     let x = (let y = 6);
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("The value of y is: {y}");
// }

// fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = plus_one(5);
//
//     println!("The value of x is: {x}");
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

//3.4 комментарии

// Hello, world.

// Итак, мы делаем что-то сложное, настолько длинное, что нам нужно
// несколько строк комментариев, чтобы сделать это! Ух! Надеюсь, этот комментарий
// объясняет, что происходит.

// fn main() {
//     let lucky_number = 7; // I’m feeling lucky today
// }

// fn main() {
//     // I’m feeling lucky today
//     let lucky_number = 7;
// }

//3.5 Управляющие конструкции

// fn main() {
//     let number = 7;
//
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// fn main() {
//     let number = 3;
//
//     if number {
//         println!("number was three");
//     }
// }

// fn main() {
//     let number = 3;
//
//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// fn main() {
//     let number = 6;
//
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//
//     println!("The value of number is: {number}");
// }

// fn main() {
//     let condition = true;
//
//     let number = if condition { 5 } else { "six" };
//
//     println!("The value of number is: {number}");
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//
//     println!("The result is {result}");
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}!");
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//
//     while index < 5 {
//         println!("the value is: {}", a[index]);
//
//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//
//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }

