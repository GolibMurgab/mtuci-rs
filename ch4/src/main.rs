//4.1 Что такое владение?

// fn main() {
//     let s = "hello";
//
//     let s = String::from("hello");
//
//     let mut s = String::from("hello");
//
//     s.push_str(", world!"); // push_str() appends a literal to a String
//
//     println!("{}", s); // This will print `hello, world!`
//
//     {
//         let s = String::from("hello"); // s is valid from this point forward
//
//         // do stuff with s
//     }                                  // this scope is now over, and s is no
//                                        // longer valid
//     let x = 5;
//     let y = x;
//
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//
//     println!("{}, world!", s1);
//     println!("s1 = {}, s2 = {}", s1, s2);
//
// }

// fn main() {
//     let s = String::from("hello");  // s comes into scope
//
//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here
//
//     let x = 5;                      // x comes into scope
//
//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward
//
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.
//
// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.
//
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1
//
//     let s2 = String::from("hello");     // s2 comes into scope
//
//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.
//
// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it
//
//     let some_string = String::from("yours"); // some_string comes into scope
//
//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }
//
// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope
//
//     a_string  // a_string is returned and moves out to the calling function
// }

// fn main() {
//     let s1 = String::from("hello");
//
//     let (s2, len) = calculate_length(s1);
//
//     println!("The length of '{}' is {}.", s2, len);
// }
//
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String
//
//     (s, length)
// }

//4.2. Ссылки и заимствование

// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, len);
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

//4.3. Тип среза

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }
//
// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s); // word will get the value 5
//
//     s.clear(); // this empties the String, making it equal to ""
//
//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}




