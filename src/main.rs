use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("at least 1 argument required");
        process::exit(1);
    }

    let words = &args[0..];

    println!(">INPUT: {}", words.join(" "));
    print!("OUTPUT: ");

    for word in words.iter() {
        let lowercase_word = word.to_lowercase();
        let mut chars = lowercase_word.chars().peekable();

        loop {
            match chars.next() {
                Some(letter) => {
                    if let Some(n) = wennsshein(letter, chars.peek()) {
                        print!("{}", n);
                    } else {
                        print!(" ");
                    }
                }
                None => break,
            }
        }
        print!(" ");
    }
    println!("");
}

fn wennsshein(letter: char, next_letter: Option<&char>) -> Option<usize> {
    match (letter, next_letter) {
        ('s', Some('h')) | ('c', Some('h')) => Some(6),
        _ => match letter {
            's' | 'z' => Some(0),
            't' | 'd' => Some(1),
            'n' => Some(2),
            'm' => Some(3),
            'r' => Some(4),
            'l' => Some(5),
            'g' | 'j' => Some(6),
            'k' | 'c' => Some(7),
            'f' | 'v' => Some(8),
            'b' | 'p' => Some(9),

            _ => None,
        },
    }
}
