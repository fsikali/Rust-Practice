// Using the ? operator on an Option<T> value

fn last_char_of_first_line(text: &str) -> Option<char> { 
    text.lines().next()?.chars().last()
} 

fn main() { 
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today"),
        Some('d)
    ); 

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\hi"), None);
}
