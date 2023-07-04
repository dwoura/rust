mod loop_print;
use crate::loop_print::print1::print_a_to_z;
use crate::loop_print::a2z::print2::print_z_to_a;

fn main() {
    println!("a_to_Z:");
    print_a_to_z();
    println!("Z_to_a:");
    print_z_to_a();
}
