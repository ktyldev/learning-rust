#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

fn main() {
    // println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    
    // right-aligment!
    // println!("{number:>width$}{number:>width$}", number=123, width=6);

    // let pi = 3.141592;
    // println!("{:.3}", pi);

    println!("Now {:?} will print!", DebugPrintable(3));

    println!("Now {:#?} will print!", Deep(DebugPrintable(4))); 
}



