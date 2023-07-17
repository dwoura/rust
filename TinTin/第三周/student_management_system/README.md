# 简易版学生信息管理系统

SMS:(StudentManagementSystem) by Dwoura



数据结构

+ struct
  
  + 学生 Student
  
  + 班级 Class
  
  + 社团 Club
  
  + 课程 Course
  
  + 菜单 Menu
  
  + 系统对象StudentManagementSystem

+ impl
  
  + 前四个类的CRUD
  
  + 学生与班级、社团和课程之间的关系
  
  + 菜单的逻辑实现

+ 结构：
  
  Menu->StudentManagementSystem->四个类



#### 心得：

写rust给我最大的感受就是过了编译器这关，就会写得很舒服，逻辑结构在视觉上和脑海里面都超级清晰。大一的时候用c++写同样的东西光改bug就改了很久，这少了点，那多了点，一不注意就内存溢出了。Rust虽然麻烦了点，但是对我这种菜鸡，从开始到改完所有bug的时间上来看花费的确实提升了不少，而且敲这种细致代码还能加深结构的印象。



**小记一下坑** ：

+ Vec<某种类型>不是写Vector

+ &self有点类似this的指向本体感觉，用到self才能调用主体里边的数据结构，如果要改变数据，传参传&mut self。函数里第一个参数得是&self。

+ match很好用，match可以当switch用也能当“二极管”用，不过里边是函数式的写法。
  
  match 一个动态返回值的东西（比如self.students.get_mut(&id)）{
  
    会把动态返回的对象给Some里，Some(自定义对象名)=》{这里可以用返回的对象}
  
    什么都没有返回的话则是None=》{}
  
  }

+ ```rust
  self.students.iter().for_each(|(id,student)| {
              println!("##{}:{}",id,student.name);
          });
  ```
  
  这个有意思，两个竖线 | 对象，对象，...| 可以把迭代的对象的属性解构出来。

+ ```rust
  let course = Course {id, name, ..Default::default()};
  ```
  
  ..Default::default() 这个写在新对象属性最后，可以起到让后面几个属性设为类型的默认值。 当然前提要在struct头上加注解：#[derive(Default)]

+ ```rust
  impl StudentManagementSystem{
      //初始化函数
      pub fn new() -> Self {
          StudentManagementSystem {
              students: HashMap::new(),
              clubs: HashMap::new(),
              classes: HashMap::new(),
              courses: HashMap::new()
          }
      }
  }
  ```
  
  里边的self是返回当前的实例，即新new出来的StudentManagementSystem类。

+ ```rust
  self.students.get_mut(&sid)
  ```
  
  students是一个hashmap，插入insert()，以键取值获取get()/get_mut()，要改值就得用后者。
  
  
