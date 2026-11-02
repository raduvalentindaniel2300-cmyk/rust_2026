use std::fs;

fn p1(fisier: &str) {
    let text = fs::read_to_string(fisier).unwrap();
    if text.is_empty() {
        println!("p1: fisier gol");
        return;
    }

    let mut b_max = 0;
    let mut c_max = 0;
    let mut lin_b = "";
    let mut lin_c = "";

    for lin in text.lines() {
        if lin.len() > b_max {
            b_max = lin.len();
            lin_b = lin;
        }
        if lin.chars().count() > c_max {
            c_max = lin.chars().count();
            lin_c = lin;
        }
    }
    println!("cel mai lung in octeti: {}", lin_b);
    println!("cel mai lung in caractere: {}", lin_c);
}

fn p2(f_in: &str, f_out: &str) {
    let text = fs::read_to_string(f_in).unwrap();
    let mut rez = String::new();

    for c in text.chars() {
        if !c.is_ascii() {
            println!("eroare: am dat de non-ascii ('{}')", c);
            return;
        }
        
        if c.is_ascii_alphabetic() {
            let baza = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let cuv = ((c as u8 - baza + 13) % 26) + baza;
            rez.push(cuv as char);
        } else {
            rez.push(c);
        }
    }
    
    fs::write(f_out, rez).unwrap();
    println!("p2: salvat in {}", f_out);
}

fn p3(fisier: &str) {
    let abrevs = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    let text = fs::read_to_string(fisier).unwrap();
    if text.is_empty() { 
        return; 
    }

    let mut rez = String::new();
    for cuv in text.split(' ') {
        let mut schimbat = false;
        
        for pereche in abrevs {
            if cuv == pereche.0 {         // .0 este abrevierea (ex: "pt")
                rez.push_str(pereche.1);  // .1 este cuvantul intreg (ex: "pentru")
                schimbat = true;
                break;
            }
        }
        
        if !schimbat {
            rez.push_str(cuv);
        }
        rez.push(' ');
    }
    println!("{}", rez.trim_end());
}
fn p4(fisier: &str) {
    let text = fs::read_to_string(fisier).unwrap(); 
    let mut gasit = false;

    for lin in text.lines() {
        let lin = lin.trim();
        if lin.starts_with('#') || lin.is_empty() {
            continue;
        }

        let mut parts = lin.split_whitespace();
        if let (Some(ip), Some(host)) = (parts.next(), parts.next()) {
            println!("{} => {}", host, ip);
            gasit = true;
        }
    }
    
    if !gasit {
         println!("p4: nimic de afisat");
    }
}

fn main() {

    
    println!(" Test P1 ");
    p1("src/p1.txt");

    println!("\n Test P2 ");
    p2("src/p2.txt", "src/out_p2.txt");

    println!("\n Test P3 ");
    p3("src/p3.txt");

    println!("\n Test P4 ");
    p4("src/p4.txt");
}