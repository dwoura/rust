//枚举
pub enum Animal {
    Dog(String),
    Cat(String),
    Bird(String)
}

impl Animal {
    pub fn sound(&self) {
        match self{
            Animal::Dog(name) => println!("enum小狗{name} 叫: 汪汪汪!"),
            Animal::Cat(name) => println!("enum小猫{name} 叫: 喵喵喵!"),
            Animal::Bird(name) => println!("enum小鸟{name} 叫: 咕咕咕!")
        }
    }
}


//trait 记住架构
pub trait Animal_trait {
    fn sound(&self);
}

pub struct Dog(pub String);
pub struct Cat(pub String);
pub struct Bird(pub String);
 
impl Animal_trait for Dog {
    fn sound(&self){
        println!("trait小狗{} 叫: 汪汪汪!",self.0);
    }
}
impl Animal_trait for Cat {
    fn sound(&self){
        println!("trait小猫{} 叫: 喵喵喵!",self.0);
    }
}
impl Animal_trait for Bird {
    fn sound(&self){
        println!("trait小鸟{} 叫: 咕咕咕!",self.0);
    }
}

// pub fn animal_sound(animal:&dyn Animal_trait) {
//     animal.sound();
// }