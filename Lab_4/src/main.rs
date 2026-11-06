use std::fs;

fn p1(fisier: &str) {
    if let Ok(text) = fs::read_to_string(fisier) {
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
    } else {
        println!("Eroare la citirea fisierului: {}", fisier);
    }
}

fn p2(f_in: &str, f_out: &str) {
    if let Ok(text) = fs::read_to_string(f_in) {
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
        
        if  fs::write(f_out, &rez).is_ok() {
            println!("p2: salvat in {}", f_out);
        } else {
            println!("Eroare la scrierea in fisierul: {}", f_out);
        }
    } else {
        println!("Eroare la citirea fisierului: {}", f_in);
    }
}

fn p3(fisier: &str) {
    let abrevs = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    if let Ok(text) = fs::read_to_string(fisier) {
        if text.is_empty() { 
            return; 
        }

        let mut rez = String::new();
        for cuv in text.split(' ') {
            let mut schimbat = false;
            
            for pereche in abrevs {
                if cuv == pereche.0 {
                    rez.push_str(pereche.1);
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
    } else {
        println!("Eroare la citirea fisierului: {}", fisier);
    }
}

fn p4(fisier: &str) {
    if let Ok(text) = fs::read_to_string(fisier) {
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
    } else {
        println!("Eroare la citirea fisierului: {}", fisier);
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