mod SMS;
use SMS::Menu;
use std::string::ToString;
use std::io;

fn main() {
    let mut menu = Menu::new();
    menu.run();
}
