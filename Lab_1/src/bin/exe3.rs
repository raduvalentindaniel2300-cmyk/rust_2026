fn main() {
    let mut i = 99;

    while i > 0 {
        if i > 1 {
            println!("{} bottles of beer on the wall,", i);
            println!("{} bottles of beer.", i);
            println!("Take one down, pass it around,");
            
           
            if i - 1 == 1 {
                println!("1 bottle of beer on the wall.\n");
            } else {
                println!("{} bottles of beer on the wall.\n", i - 1);
            }
        } else {
          
            println!("1 bottle of beer on the wall,");
            println!("1 bottle of beer.");
            println!("Take one down, pass it around,");
            println!("No bottles of beer on the wall.\n");
        }
        
        i =i-1; 
    }

    println!("No bottles of beer on the wall,");
    println!("No bottles of beer.");
    println!("Go to the store, buy some more,");
    println!("99 bottles of beer on the wall.");
}