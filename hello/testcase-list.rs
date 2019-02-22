use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { 
                write!(f, ", ")?; 
            }

            write!(f, "{}", v)?;
        }

        // 'return' and semicolon are optional on the last line of a function
        // returning a value
        // write!(f, "]")
        return write!(f, "]");
    }
}

fn main() {
    let v = List(vec![4, 2, 0]);
    println!("{}", v);
}
