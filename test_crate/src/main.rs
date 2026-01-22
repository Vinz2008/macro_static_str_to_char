use macro_static_str_to_char::str_to_char;

fn main() {
    let v = vec!['a', 'b', 'c', 'f', 'e'];

    let buf: &[char] = &v;
    let a = match buf {
        str_to_char!("test") => 1,
        str_to_char!("allo") => 3,
        str_to_char!("???") => 4,
        str_to_char!("abcfe") => 10,
        _ => 5,
    };

    dbg!(a);
}
