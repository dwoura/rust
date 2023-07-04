# 第一周

+ **Rust的组织方式**

**Cargo-依赖管理工具**：类似于npm，Cargo 使用文件放置约定，即文件名就是模块名。

**crate-集装箱**：是一组功能的封装，类似于npm包。

rust中，模块不再通过文件路径的方式引入，而是通过cargo以及约定的模块声明方式来**构建模块树**，然后通过use关键字来使用。

## 模块声明&使用

假如我们想实现一个加法模块，并提供给其他地方使用。我们可以有如下三种组织方式

> Cargo 使用文件放置约定，因此模块查找以src目录下的rs文件或者目录为准，并且只会查找一级，嵌套文件夹下的rs文件不可以直接被其他文件使用。

### **方法一：直接在根文件下声明** **`add.rs`**

我们可以通过在src下添加模块名同名的文件，cargo就可以识别到add模块。

```csharp
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── add.rs
│   ├── lib.rs
```

### **方法二：声明add文件夹，文件夹下包含** **`mod.rs`**

如果模块是文件夹，则必须有`mod.rs`文件。这类似于javascript的`index.js`。cargo仍然可以识别到这是add模块

```vbnet
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── add
│   │   ├── mod.rs
│   ├── lib.rs
```

假设我们的代码内容如下,并位于文件`add.rs` 或者`add/mod.rs`内

```rust
pub fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}
```

那么在`lib.rs`中我们可以通过如下方式调用我们的`add`模块

```rust
// 声明模块并引用模块内的函数
mod add;
pub use crate::add::add_fn;

pub fn test_lib() {
  add_fn(1,2);
}
```

### **方法三：add.rs和add文件夹同时存在**

这种方式的目录结构看起来像下面这样

```csharp
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── add
│   │   └── add_m.rs
│   ├── add.rs // index.js
│   ├── lib.rs
```

`add.rs`负责入口模块的导入导出，`add`文件夹下则存放其余相关联的其他模块。这类似于javascript的`index.js`统一导出了多个其他模块。和上面不同的是这里 导入使用到了**mod**关键字来拆分模块；

文件内容看起来像下面这样

`add.rs`

```bash
pub mod add_m;
// 类似于 export * from './validate; export * from './helper'
```

`add/add_m.rs`

```rust
pub fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}
```

`lib.rs`

```rust
mod add;
pub use crate::add::add_m::add_fn;

pub fn test_lib() {
  add_fn(1,2);
}
```

上述三种方式使用较多的应该是前两种，并且在大型项目内第二种更为合理，可以更好的组织文件。那么当一个模块文件夹下拆分多个模块文件时该怎调用相邻文件呢？
