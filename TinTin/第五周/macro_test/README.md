## 第五周作业
+ 实现：一个简单的声明宏，并理解其**代码结构**，和**编译过程**。
+ 作业参考``` [Rust圣经](https://course.rs/advance/macro.html) ```
#### 声明宏

声明宏（Declarative Macro）是一种用于**创建代码模板**和**模式匹配**的宏。还有个叫过程宏课程没讲。

声明式宏允许我们写出类似 `match` 的代码。

match代码结构如下：

```rust
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}

```

宏结构如下：

```rust
macro_rules! 名 {
    //匹配模式1
    ($x:expr) => {
        //与$x有关的代码
    };
    //匹配模式2
    ($x:expr, $y:expr) => {
        //与$x和$y有关的代码
    };
}
```

**宏也是将一个值按照对应的模式进行匹配，并且对应模式会与特定代码关联。** 区别是宏里的值是一段Rust源代码。然后会去匹配符合这段源代码的结构的模式，最后被模式所关联的代码替换实现。一个模式就是一个分支。

实现例子（简易vector）如下：

```rust
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
    //区别原来的vec![]。
    let v = vector![1, 2, 3, 4];
    for i in v.iter() {
        println!("{}", i);
    }
}


```

+ 代码结构
  
  + `#[macro_export]` 注释将宏进行了导出，这样其他包才能引入该宏。
  
  + `macro_rule!` 宏定义，定义的时候不需要！，只有调用的时候才需要。
  
  + `( $( $x:expr ),* )` 模式解析。其中参数们先被一对 ( ) 括起来，小括号里写自定义的参数 $( ),然后`*`说明`*` 之前的模式会被匹配零次或任意多次。
  
  + `$x:expr` 的`expr` 表示rust中任意表达式，然后给出一个新模式叫`$x`。由于可以匹配任意表达式， 所以 `$x` 模式既可以跟整数 `1` 进行匹配，也可以跟字符串 "hello" 进行匹配: `vec!["hello", "world"]`。（好像有点泛型的感觉）
  
  + `=>{}`里的`$(temp_vec.push($x);)*` 这句也是遵从了`$()` 中写表达式，并且`*` 有任意多个含有不同`$x`的`$(temp_vec.push($x);)`
    
    当调用vector![1,2,3,4]时:
    
    ```rust
    {
        let mut temp_vec = Vec::new();
        $(temp_vec.push($x);)*
        temp_vec
    }
    //等同于
    {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    }
    ```

+ 编译过程
  
  main函数中调用vector![1, 2, 3, 4]时，rust编译器会解析vector宏并对其中整体元素进行模式匹配，匹配成功后就会把宏内部的代码片段替换为相应代码，最后执行相应代码并返回结果。


