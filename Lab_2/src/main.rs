
fn add_chars_n(mut s:String ,c:char, n:i32) ->String
{for _x in 0..n{
    s.push(c);
}
s

}
fn add_chars_nn(s:&mut String ,c:char, n:i32)
{for _x in 0..n{
    s.push(c);
}

}
fn add_spaces(s:&mut String ,n:u32)
{for _x in 0..n{
    s.push(' ');
}
}
fn add_str(s:&mut String, str:&String)
{
    s.push_str(str);
    }

fn add_integer(s:&mut String ,n:i32)
{if n==0
    {s.push('0');}
    else
    {let mut copie=n;

     let mut m=0;

        while copie>0
        {m=copie%10 +m*10;
        copie=copie/10;
        }
        let mut i=0;
        while m>0
        {
            if i>0 && i%3==0
            {s.push('_');
                
            }
            i+=1;
            let uc=m%10;
        let c= (uc as u8 + b'0') as char;
            s.push(c);
            m=m/10;
        } 



    }

}

fn add_float(s:&mut String , n:f32)
{ let parte_int = n as i32;
    let parte_frac = n - parte_int as f32;
    add_integer(s,parte_int);
    s.push('.');
    let mut copie_frac = (parte_frac * 1000.0) as i32;
    if copie_frac < 100 {
        s.push('0');
        if copie_frac < 10 {
            s.push('0');
        }
    }
    add_integer(s,copie_frac);
}
fn main() {
    let mut sss = String::from("");
    let mut i = 0;
    let mut ss = String::from("");
    while i < 26 {
        let c = (i as u8 + b'a') as char;
sss = add_chars_n(sss, c, 26-i);

        add_chars_nn(&mut ss, c, 26 - i);

        i += 1;
    }

    print!("{}\n\n", sss);


    print!("{}\n\n\n", ss);


     let mut s = String::new();
     let mut s1 = String::new();
     let mut s2 = String::new();
 

 add_spaces(&mut s,35);
 add_str(&mut s,&String::from("I 💚"));
 s.push('\n');
 add_spaces(&mut s,35);
 add_str(&mut s,&String::from("RUST."));
 s.push('\n');


println!("{}", s);
let c1 = String::from("Most");
    add_str(&mut s1, &c1);
    add_spaces(&mut s1, 1);
    add_spaces(&mut s2, c1.len() as u32 + 1);

    let c2 = String::from("downloaded");
    add_str(&mut s2, &c2);
    add_spaces(&mut s2, 1);
    add_spaces(&mut s1, c2.len() as u32 + 1);

    let c3 = String::from("crate");
    add_str(&mut s1, &c3);
    add_spaces(&mut s1, 1);
    add_spaces(&mut s2, c3.len() as u32 + 1);

    let c4 = String::from("has");
    add_str(&mut s2, &c4);
    add_spaces(&mut s2, 1);
    add_spaces(&mut s1, c4.len() as u32 + 1);

    let lungime_inainte = s1.len();
    add_integer(&mut s1, 306437968);
    let caractere_adaugate = (s1.len() - lungime_inainte) as u32;
    
    add_spaces(&mut s1, 1);
    add_spaces(&mut s2, caractere_adaugate + 1);

    let c5 = String::from("downloads");
    add_str(&mut s2, &c5);
    add_spaces(&mut s2, 1);
    add_spaces(&mut s1, c5.len() as u32 + 1);

    let c6 = String::from("and");
    add_str(&mut s1, &c6);
    add_spaces(&mut s1, 1);
    add_spaces(&mut s2, c6.len() as u32 + 1);

    let c7 = String::from("the");
    add_str(&mut s2, &c7);
    add_spaces(&mut s2, 1);
    add_spaces(&mut s1, c7.len() as u32 + 1);

    let c8 = String::from("lastest");
    add_str(&mut s1, &c8);
    add_spaces(&mut s1, 1);
    add_spaces(&mut s2, c8.len() as u32 + 1);

    let c9 = String::from("version");
    add_str(&mut s2, &c9);
    add_spaces(&mut s2, 1);
    add_spaces(&mut s1, c9.len() as u32 + 1);

    let c10 = String::from("is");
    add_str(&mut s1, &c10);
    add_spaces(&mut s1, 1);
    add_spaces(&mut s2, c10.len() as u32 + 1);

    let lungime_inainte_float = s2.len();
    add_float(&mut s2, 2.038);
    let punct = String::from(".");
    add_str(&mut s2, &punct);
    let caractere_adaugate_float = (s2.len() - lungime_inainte_float) as u32;

    add_spaces(&mut s1, caractere_adaugate_float);

    println!("{}", s1);
    println!("{}", s2);
}

