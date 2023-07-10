pub mod test{
    pub fn test_ownership_01(){
        // let s1 = String::from("I am a superman.");
        // let s2 =s1;
        // println!("{s1}");
        // println!("{s2}");
        //报错：这个String s1没有实现Copy的trait。
        //更改：s2赋值时，s1-> s1.clone();
        //总结：赋值相当于move了该资源的所有权，这需要变量在mut情况下完成。
        //     默认浅拷贝，str.clone()相当于只拷贝了个值。
    }

    pub fn test_immutable(){
        // let a = 10u32;
        // let b = a;
        // println!("{a}");
        // println!("{b}");
        //报错：无
        //更改：无
        //总结：固定字节数类型的赋值是复制了新的资源，因此是两个资源，分别两个不同所有者
        //      这种栈上的小资源，便于直接完全拷贝的资源就完全拷贝。

        let a = 10u32;
        let b = &a;
        let c = &&&&&a;
        let d = &b;
        let e = b;
        println!("{a}");
        println!("{b}");
        println!("{c}");
        println!("{d}");
        println!("{e}");
        //报错：无
        //输出：全是10
        //总结：rust关注人类的业务，认为程序员只会用一个&，因此多个&也会被识别为1个，不存在多重引用。
        //      语义层面上看，就是把最开始引用&的变量拿来用。 
        //      由于引用也是一种值变量，因此赋值的时候也是做引用拷贝的，而不是move和引用指向的值拷贝。
    }   

}