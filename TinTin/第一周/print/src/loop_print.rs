pub mod a2z;
pub mod print1 {
    pub fn print_a_to_z() {
        for i in ('Z'..='a').rev(){
            println!("{i}");
        }
    }
}



