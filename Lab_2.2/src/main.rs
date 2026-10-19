struct Note {
    title: String,
    body: String,
}

// ---------- Part 1: borrow to read (&str) ----------

// How many times does `c` appear in `s`?
fn count_char(s: &str, c: char) -> usize {
    let mut count = 0;
    for x in s.chars() {
        if x == c {
            count += 1;
        }
    }
    count
}

// The first word of `s` (up to the first space, or all of `s`), as a slice of `s`.
// No copying: return a &str that points into `s`. Use as_bytes and a while loop.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i] != b' ' {
        i += 1;
    }
    &s[..i]
}

// A new String with the characters of `s` in reverse order.
fn reverse(s: &str) -> String {
    let mut new_string = String::new();
    for c in s.chars().rev() {
        new_string.push(c);
    }
    new_string
}

// True if `s` reads the same backwards. Reuse reverse().
fn is_palindrome(s: &str) -> bool {
    s == reverse(s)
}

// A new String made of `s` repeated n times.
fn repeat_str(s: &str, n: usize) -> String {
    let mut new_string = String::new();
    for _ in 0..n {
        new_string.push_str(s);
    }
    new_string
}

// ---------- Part 2: mutable borrow (&mut String) ----------

// Append "[tag]" to the caller's string.
fn add_tag(s: &mut String, tag: &str) {
    s.push('[');
    s.push_str(tag);
    s.push(']');
}

// Replace every occurrence of `c` with '*' in the caller's string.
// Build a new String, then assign it back with *s = ...
fn censor(s: &mut String, c: char) {
    let mut new_string = String::new();
    for ch in s.chars() {
        if ch == c {
            new_string.push('*');
        } else {
            new_string.push(ch);
        }
    }
    *s = new_string;
}

// ---------- Part 3: ownership in, ownership out (String) ----------

// Take a string, make it uppercase, and give it back.
fn shout(s: String) -> String {
    let mut new_string = String::new();
    for c in s.chars() {
        if c > 'a' && c < 'z' {
            let uc = c as u8 - 32;
            let c = uc as char;
            new_string.push(c);
        } else {
            new_string.push(c);
        }
    }
    new_string
}

// Take two strings and return the longer one (the first on a tie). The other is dropped.
fn longer_of(a: String, b: String) -> String {
    if a.len() >= b.len() { a } else { b }
}

// ---------- Part 4: Notebook (a struct that owns its strings) ----------

// Build a Note from two string slices. The Note must own its text.
fn make_note(title: &str, body: &str) -> Note {
    Note {
        title: String::from(title),
        body: String::from(body),
    }
}

// Number of words in the body, counted by hand with chars().
fn word_count(note: &Note) -> usize {
    let mut count = 0;
    for x in note.body.chars() {
        if x == ' ' {
            count += 1;
        }
    }
    count + 1



}

// Add `text` to the end of the body, separated by one space
// (no leading space if the body is empty).
fn append(note: &mut Note, text: &str) {
    note.body.push(' ');
    note.body.push_str(text);
}

// Use the note up and return "[ARCHIVED] <title>: <body>".
fn archive(note: Note) -> String {
    let mut new_string = String::from("[ARCHIVED] ");
    new_string.push_str(&note.title);
    new_string.push_str(": ");
    new_string.push_str(&note.body);
    new_string
}

// Combine two notes. The first is consumed and reused, the second is only read.
// Title becomes "<a>+<b>", body becomes "<a body> <b body>".
fn merge(a: Note, b: &Note) -> Note {
    Note {
        title: format!("{}+{}", a.title, b.title),
        body: format!("{} {}", a.body, b.body),
    }
}

fn main() {
    // Part 1
    let text = String::from("level up your rust");
    println!("{}", count_char(&text, 'e'));
    println!("{}", count_char("banana", 'a'));
    println!("{}", first_word(&text));
    println!("{}", first_word("rust"));
    println!("{}", reverse(&text));
    println!("{} {}", is_palindrome("level"), is_palindrome(&text));
    println!("{}", repeat_str("ab", 3));

    // Part 2
    let mut s = String::from("hello world");
    add_tag(&mut s, "v2");
    println!("{s}");
    censor(&mut s, 'o');
    println!("{s}");

    // Part 3
    let loud = shout(s);
    println!("{loud}");
    let winner = longer_of(String::from("tiny"), loud);
    println!("{winner}");

    // Part 4
    let title = String::from("shopping");
    let mut n1 = make_note(&title, "milk");
    let n2 = make_note("ideas", "learn rust borrowing");
    println!("{title}");
    println!("{} word(s)", word_count(&n1));
    append(&mut n1, "eggs");
    append(&mut n1, "bread");
    println!("{}: {}", n1.title, n1.body);
    let merged = merge(n1, &n2);
    println!("{}: {}", merged.title, merged.body);
    println!("{} word(s)", word_count(&merged));
    println!("{}", archive(n2));
}
