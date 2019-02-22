use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[allow(dead_code)]
#[derive(Debug)]
struct Deep(Structure);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    // println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    
    // right-aligment!
    // println!("{number:>width$}{number:>width$}", number=123, width=6);

    // let pi = 3.141592;
    // println!("{:.3}", pi);

    println!("Now {} will print!", Structure(3));
    // println!("Now {:#?} will print!", Deep(Structure(4))); 
}



