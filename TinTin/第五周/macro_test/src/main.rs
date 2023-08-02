//简易vector
#[macro_export]
macro_rules! vector {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

fn main() {
    let v = vector![1, 2, 3, 4];
    for i in v.iter() {
        println!("{}", i);
    }
}
