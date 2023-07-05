/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номер строки: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {

    let i = find_term(SEARCH_TERM, QUOTE);
    println!("{i}");
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut num = 0;
    let mut answer=String::new();

    'outer: for i in quote.split("\n"){
        num += 1;
        for j in i.split(" ") {
            if search_term == j {
                    answer = num.to_string() + ": " + i;
                    break 'outer;
                };
        }
    }
    answer
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}
