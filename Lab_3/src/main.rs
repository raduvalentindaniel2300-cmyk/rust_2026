
fn _next_prime(x: u16) -> Option<u16>
{
    let mut next = x + 1;
    while next as u32 <= u16::MAX  as u32 {
        let mut is_prime = true;
        for i in 2..=((next as f64).sqrt() as u16) {
            if next.is_multiple_of(i) {
                is_prime = false;
                break;
            }
        }
        if is_prime {
           return Some(next);
        }
        next += 1;
    }
    None
}
 fn addition(a:u32, b:u32) -> u32 {

if b>u32::MAX - a {
    panic!("Overflow  la adunare");
}
a+b}
fn multiplication(a:u32, b:u32) -> u32 {
      if a==0 || b==0 {
        return 0;
      }
      if b>u32::MAX / a {
        panic!("Overflow la inmultire");
      }
      a*b
}
#[derive(Debug)]
enum Eroare{
OverFlow,
}

fn addition1(a: u32, b: u32) -> Result<u32 , Eroare> {

    if b > u32::MAX - a {
        Err(Eroare::OverFlow)
    } else {
        Ok(a + b)
    }
}

fn multiplication1(a: u32, b: u32) -> Result<u32, Eroare> {
    if a == 0  ||b==0 {
        return Ok(0);
    }
    
   if b > u32::MAX / a {
        Err(Eroare::OverFlow)
    } else {
        Ok(a * b)
    }
}


fn aplicatie(a:u32 ,b :u32 , c:u32) -> Result<u32 , Eroare>
{
let suma =addition1(a ,b)?;

let m =multiplication1(suma , c)?;
Ok(m)

}


enum Char{

    Notascii,Notadigit,NotABase16,Notaletter,Notp
}

fn to_uppercase(c: char) ->Result<char ,Char>
{
if !c.is_ascii_lowercase() && !c.is_ascii_uppercase()
{return Err(Char::Notaletter);}

if c>'a' && c<'z'
{Ok((c as u8 - 32_u8) as char)}
else 
{Ok(c)}

}

fn to_lowercase(c: char) ->Result<char ,Char>
{
if !c.is_ascii_lowercase() && !c.is_ascii_uppercase()
{ return Err(Char::Notaletter);}

if c>'A' && c<'Z'
{Ok((c as u8 + 32_u8) as char)}
else 
{Ok(c)}


}

fn print_char(c:char)->Result<(),Char>
{

    let val=c as u32;
 if !(32..=126).contains(&val) {
        return Err(Char::Notp);
    }
    println!("{}", c);
    Ok(())
    
}

fn char_to_number(c:char)->Result<u32 ,Char>
{
let val =c as u32;
if val>127
{ return Err(Char::Notascii);}

if c.is_ascii_digit()
{Ok('c' as u32 - '0' as u32)}
else {
Err(Char::Notadigit)
}}



fn char_to_number_hex(c:char)->Result<char ,Char>
{
if !('A'..='F').contains(&c) && !c.is_ascii_digit()
{Err(Char::NotABase16)}
else
{Ok(c)}

}

fn print_error(err : Char){
match err {
        Char::Notascii => println!("Eroare: caracterul nu este ASCII"),
        Char::Notadigit => println!("Eroare: caracterul nu este o cifra"),
        Char::NotABase16 => println!("Eroare: caracterul nu este o cifra hexazecimala"),
        Char::Notaletter => println!("Eroare: caracterul nu este o litera"),
        Char::Notp => println!("Eroare: caracterul nu este afisbil"),
    }



}


fn main() {



let x = addition(3000,16000);
println!("{x}");
println!("\n");

let x = multiplication(3000,16000);
println!("{x}");

println!("\n");



   match aplicatie(1,2,3){
Ok(v)=>println!("{v}"),
Err(e)=>println!("Am dat de eroarea: {:?}", e),
   }

   println!("\nTestare to_uppercase");
    match to_uppercase('a') {
        Ok(c) => println!("Succes: {}", c),
        Err(e) => print_error(e),
    }
    match to_uppercase('7') { 
        Ok(c) => println!("Succes: {}", c),
        Err(e) => print_error(e),
    }

    println!("\nTestare to_lowercase");
    match to_lowercase('a') {
        Ok(c) => println!("Succes: {}", c),
        Err(e) => print_error(e),
    }
    match to_lowercase('7') { 
        Ok(c) => println!("Succes: {}", c),
        Err(e) => print_error(e),
    }

    println!("\nTestare print_char");
    match print_char('A') {
        Ok(()) => println!("(Afișat cu succes)"),
        Err(e) => print_error(e),
    }
    match print_char('\x07') { 
        Ok(()) => println!("(Afișat cu succes)"),
        Err(e) => print_error(e),
    }

    println!("\nTestare char_to_number");
    match char_to_number('5') {
        Ok(val) => println!("Număr obținut: {}", val),
        Err(e) => print_error(e),
    }
    match char_to_number('x') { 
        Ok(val) => println!("Număr obținut: {}", val),
        Err(e) => print_error(e),
    }

    println!("\nTestare char_to_number_hex ");
    match char_to_number_hex('F') {
        Ok(val) => println!("Valoare hex: {}", val),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('Z') { 
        Ok(val) => println!("Valoare hex: {}", val),
        Err(e) => print_error(e),
    }
}