mod work1;
mod work2;
use work1::Animal;

use work1::Animal_trait;
use work1::Dog;
use work1::Cat;
use work1::Bird;
//use work1::animal_sound;

use work2::Add;
use work2::Number;

fn main() {
    //work1测试
    // let animals: Vec<Animal> = vec![
    //     Animal::Dog("旺财".to_string()),
    //     Animal::Cat("咪咪".to_string()),
    //     Animal::Bird("鸽鸽".to_string())
    // ];

    // for animal in animals.iter() {
    //     animal.sound();
    // }

    // let animals: Vec<Box<dyn Animal_trait>> = vec![
    //     Box::new(Dog("旺财".to_string())),
    //     Box::new(Cat("咪咪".to_string())),
    //     Box::new(Bird("鸽鸽".to_string()))
    // ];

    // for animal in animals.iter() {
    //     //animal.sound();
    //      
    // }


    //work2测试
    let num1 = Number(10);
    let num2 = Number(20);

    let result = num1.add(num2);
    println!("结果和为: {:?}", result);
}
