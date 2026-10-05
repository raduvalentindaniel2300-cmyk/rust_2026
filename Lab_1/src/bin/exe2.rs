fn cmmdc(mut a: u32, mut b: u32) -> bool {
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    if a == 1 { true } else { false }
}

fn main() {
    for i in 0..=100 {
        for j in 0..=100 {
            if cmmdc(i, j) == true {
                println!("{} si {} sunt coprime", i, j);
            }
        }
    }
}
