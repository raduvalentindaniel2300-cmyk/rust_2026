fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    for _x in 0..n {
        s.push(c);
    }
    s
}
fn add_chars_nn(s: &mut String, c: char, n: i32) {
    for _x in 0..n {
        s.push(c);
    }
}
fn add_space(s: &mut String, n: u32) {
    for _x in 0..n {
        s.push(' ');
    }
}
fn add_str(s: &mut String, str: &String) {
    s.push_str(str);
}

fn add_integer(s: &mut String, n: u32) {
    if n == 0 {
        s.push_str("0");
    } else {
        let mut m = n;

        while m > 0 {
            let uc = m % 10;
            let c = (uc as u8 + b'0') as char;
            s.push(c);
            m = m / 10;
        }
    }
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    let mut ss = String::from("");
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        add_chars_nn(&mut ss, c, 26 - i);

        i += 1;
    }

    print!("{}\n\n", s);

    print!("{}\n", ss);
}
