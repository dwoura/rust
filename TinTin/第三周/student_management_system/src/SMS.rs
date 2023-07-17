//学生信息管理系统 StudentManagementSystem by Dwoura
use std::collections::HashMap;
use std::vec::Vec;
use std::io;

#[derive(Default)]
pub struct Student {
    id: i32,
    name: String,
    class_id: i32,
    club_id: i32,
    courses_id: Vec<i32>
}

#[derive(Default)]
pub struct Club {
    id: i32,
    name: String,
    members_id: Vec<i32>
}

#[derive(Default)]
pub struct Class {
    id: i32,
    name: String,
    students_id: Vec<i32>
}

#[derive(Default)]
pub struct Course {
    id: i32,
    name: String,
    students_id: Vec<i32>
}

//菜单类
pub struct Menu {
    sms: StudentManagementSystem,
}

//操作对象
pub struct StudentManagementSystem {
    students: HashMap<i32,Student>,
    clubs: HashMap<i32,Club>,
    classes: HashMap<i32,Class>,
    courses: HashMap<i32,Course>
}


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

//实现学生操作
impl StudentManagementSystem {
    //添加学生
    pub fn add_student(&mut self, id:i32, name:String) {
        let student = Student {id, name, ..Default::default()};
        match self.students.get_mut(&id) {
            Some(student) => {
                println!("添加失败，已经存在");
            }
            None => {
                self.students.insert(id, student);
                println!("添加成功");
            }
        }
    }
    //删除学生
    pub fn delete_student(&mut self, id:i32) {
        match self.students.get_mut(&id){
            Some(student) => {
                self.students.remove(&id);
                println!("删除成功");
            }
            None => {
                println!("删除失败，已不存在");
            }
        }
        
    }
    //修改学生
    pub fn modify_student(&mut self, id:i32, name:String) {
        if let Some(student) = self.students.get_mut(&id){
            student.name = name;
            println!("修改成功");
            return;
        } else {
            println!("修改失败，学生不存在");
        }
    }
    //加入班级
    pub fn student_class_selection(&mut self, sid:i32, cid:i32){
        if let Some(student) = self.students.get_mut(&sid){
            //对应课程也要加入学生
            if let Some(class) = self.classes.get_mut(&cid){
                student.class_id = cid;
                class.students_id.push(sid);
                println!("加入成功");
            }else{
                println!("班级不存在");
            }
            return;
        }else{
            println!("学生不存在");
        }
    }
    //加入社团
    pub fn student_club_selection(&mut self, sid:i32, cid:i32){
        if let Some(student) = self.students.get_mut(&sid){
            //对应社团也要加入学生
            if let Some(club) = self.clubs.get_mut(&cid){
                student.club_id = cid;
                club.members_id.push(sid);
                println!("加入成功");
            }else{
                println!("社团不存在");
            }
            return;
        }else{
            println!("学生不存在");
        }
    }
    //学生选课
    pub fn student_course_selection(&mut self, sid:i32, cid:i32){
        if let Some(student) = self.students.get_mut(&sid){
            //对应课程也要加入学生
            if let Some(course) = self.courses.get_mut(&cid){
                student.courses_id.push(cid);
                course.students_id.push(sid);
                println!("选课成功");
            }else{
                println!("课程不存在");
            }
            return;
        }else{
            println!("学生不存在");
        }
    }

    //private
    //根据id查询学生姓名
    fn get_student_name_by_id(&self, student_id:i32) -> &str {
        match self.students.get(&student_id) {
            Some(student) => return &student.name,
            None => return "无此姓名"
        }
    }

    //private
    //根据id查询所在班级名字
    fn get_class_name_by_id(&self, class_id:i32) -> &str {
        match self.classes.get(&class_id) {
            Some(class) => return &class.name,
            None => return "无此班级"
        }
    }

    //private
    //根据id查询所在社团名字
    fn get_club_name_by_id(&self, club_id:i32) -> &str {
        match self.clubs.get(&club_id) {
            Some(club) => return &club.name,
            None => return "无此社团"
        }
    }

    //private
    //根据id查询课程名字
    fn get_course_name_by_id(&self, course_id:i32) -> &str {
        match self.courses.get(&course_id) {
            Some(course) => return &course.name,
            None => return "无此课程"
        }
    }

    //private
    //根据id查询学生所选课程 并打印
    fn print_course_name_by_student_id(&self, id:i32){
        match self.students.get(&id) {
            Some(student) => {
                let ids = &student.courses_id;//获取学生id的课程号vec，用于遍历查询出课程名
                println!("#所选课程：↓↓");
                ids.iter().for_each(|id| {
                    println!("##{}",self.get_course_name_by_id(*id));
                });
                println!("#所选课程：↑↑");
            },
            None =>{
                println!("#所选课程：无课程",);
            }
        }
    }

    //查询所有学生的id和姓名
    pub fn print_all_student_id_and_name(&self){
        println!("#所有学生：↓↓");
        self.students.iter().for_each(|(id,student)| {
            println!("##{}:{}",id,student.name);
        });
        println!("#所有学生：↑↑");
        //改进：分页 
    }

    //查询学生
    pub fn query_student(&self, id:i32) {
        match self.students.get(&id) {
            Some(student) => {
                let class_name = self.get_class_name_by_id(student.class_id);
                let club_name = self.get_club_name_by_id(student.club_id);
                
                
                //let courses_name = get_courses_name_by_id(student.courses_id);
                println!("↓↓查询成功↓↓");
                println!("#学生id：{}",student.id);
                println!("#学生姓名：{}",student.name);
                println!("#所在班级：{}",class_name);
                println!("#所在社团：{}",club_name);
                self.print_course_name_by_student_id(student.id);
                println!("↑↑查询成功↑↑");
                
            }
            None => {
                println!("查询失败");
            }
        }
    }

}

//实现社团操作
impl StudentManagementSystem {
    //添加社团
    pub fn add_club(&mut self, id:i32, name:String) {
        let club = Club {id, name, ..Default::default()};
        match self.clubs.get_mut(&id) {
            Some(club) => {
                println!("添加失败，已经存在");
            }
            None => {
                self.clubs.insert(id, club);
                println!("添加成功");
            }
        }
    }
    //删除社团
    pub fn delete_club(&mut self, id:i32) {
        match self.clubs.get_mut(&id){
            Some(student) => {
                self.clubs.remove(&id);
                println!("删除成功");
            }
            None => {
                println!("删除失败，已不存在");
            }
        }
    }
    //修改社团
    pub fn modify_club(&mut self, id:i32, name:String) {
        if let Some(club) = self.clubs.get_mut(&id){
            club.name = name;
            println!("修改成功");
            return;
        }else {
            println!("修改失败，社团不存在");
        }
    }
    
    //private
    //根据id查询社团所有成员 并打印
    fn print_club_members_by_id(&self,id:i32) {
        match self.clubs.get(&id) {
            Some(club) => {
                let ids = &club.members_id;
                println!("#社团成员：↓↓");
                ids.iter().for_each(|id| {
                    println!("##{}",self.get_student_name_by_id(*id));
                });
                println!("#社团成员：↑↑");
            },
            None =>{
                println!("#社团成员：无成员",);
            }
        }
    }

    //查询所有社团的id和名字
    pub fn print_all_club_id_and_name(&self){
        println!("#所有学生：↓↓");
        self.clubs.iter().for_each(|(id,club)| {
            println!("##{}:{}",id,club.name);
        });
        println!("#所有学生：↑↑");
        //改进：分页 
    }

    //查询社团
    pub fn query_club(&self, id:i32) {
        match self.clubs.get(&id) {
            Some(club) => {
                let class_name = self.get_class_name_by_id(club.id);
                println!("↓↓查询成功↓↓");
                println!("#社团id：{}",club.id);
                println!("#社团名：{}",club.name);
                self.print_club_members_by_id(club.id);
                println!("↑↑查询成功↑↑");
            }
            None => {
                println!("查询失败");
            }
        }
    }

}

//实现班级操作
impl StudentManagementSystem {
    //添加班级
    pub fn add_class(&mut self, id:i32, name:String) {
        let class = Class {id, name, ..Default::default()};
        match self.classes.get_mut(&id) {
            Some(class) => {
                println!("添加失败，已经存在");
            }
            None => {
                self.classes.insert(id, class);
                println!("添加成功");
            }
        }
    }
    //删除班级
    pub fn delete_class(&mut self, id:i32) {
        match self.classes.get_mut(&id){
            Some(class) => {
                self.classes.remove(&id);
                println!("删除成功");
            }
            None => {
                println!("删除失败，已不存在");
            }
        }
    }
    //修改班级
    pub fn modify_class(&mut self, id:i32, name:String) {
        if let Some(class) = self.classes.get_mut(&id){
            class.name = name;
            println!("修改成功");
            return;
        }else {
            println!("修改失败，班级不存在");
        }
    }
    
    //private
    //根据id查询班级所有学生 并打印
    fn print_class_students_by_id(&self,id:i32) {
        match self.classes.get(&id) {
            Some(class) => {
                let ids = &class.students_id;
                println!("#班级学生：↓↓");
                ids.iter().for_each(|id| {
                    println!("##{}",self.get_student_name_by_id(*id));
                });
                println!("#班级学生：↑↑");
            },
            None =>{
                println!("无此班级");
            }
        }
    }

    //根据学生id加入班级 与学生类中的一样

    //打印全部班级的id号和名字
    pub fn print_all_class_id_and_name(&self){
        println!("#所有班级：↓↓");
        self.classes.iter().for_each(|(id,class)| {
            println!("##{}:{}",id,class.name);
        });
        println!("#所有班级：↑↑");
        //改进：分页 
    }
    //查询班级
    pub fn query_class(&self, id:i32) {
        match self.classes.get(&id) {
            Some(class) => {
                let class_name = self.get_class_name_by_id(class.id);
                println!("↓↓查询成功↓↓");
                println!("#班级id：{}",class.id);
                println!("#班级名：{}",class.name);
                self.print_class_students_by_id(class.id);
                println!("↑↑查询成功↑↑");
            }
            None => {
                println!("查询失败");
            }
        }
    }
}

//实现课程操作
impl StudentManagementSystem {
    //添加课程
    pub fn add_course(&mut self, id:i32, name:String) {
        let course = Course {id, name, ..Default::default()};
        match self.courses.get_mut(&id) {
            Some(course) => {
                println!("添加失败，已经存在");
            }
            None => {
                self.courses.insert(id, course);
                println!("添加成功");
            }
        }
    }
    //删除课程
    pub fn delete_course(&mut self, id:i32) {
        match self.courses.get_mut(&id){
            Some(course) => {
                self.courses.remove(&id);
                println!("删除成功");
            }
            None => {
                println!("删除失败，已不存在");
            }
        }
    }
    //修改课程
    pub fn modify_course(&mut self, id:i32, name:String) {
        if let Some(course) = self.courses.get_mut(&id){
            course.name = name;
            println!("修改成功");
            return;
        } else {
            println!("修改失败，课程不存在");
        }
    }
    
    //private
    //根据id查询课程中所有已选学生 并打印
    fn print_course_students_by_id(&self,id:i32) {
        match self.courses.get(&id) {
            Some(course) => {
                let ids = &course.students_id;//获取学生id的vec，用于遍历查询出学生姓名
                println!("#课程学生：↓↓");
                ids.iter().for_each(|id| {
                    println!("##{}",self.get_student_name_by_id(*id));
                });
                println!("#课程学生：↑↑");
            },
            None =>{
                println!("#课程学生：无学生",);
            }
        }
    }

    //查询所有课程的id和名字
    pub fn print_all_course_id_and_name(&self){
        println!("#所有课程：↓↓");
        self.courses.iter().for_each(|(id,course)| {
            println!("##{}:{}",id,course.name);
        });
        println!("#所有课程：↑↑");
        //改进：分页 
    }

    //查询课程
    pub fn query_course(&self, id:i32) {
        match self.courses.get(&id) {
            Some(course) => {
                let course_name = self.get_course_name_by_id(course.id);

                println!("↓↓查询成功↓↓");
                println!("#课程id：{}",course.id);
                println!("#课程名：{}",course.name);
                self.print_course_students_by_id(course.id);
                println!("↑↑查询成功↑↑");
            }
            None => {
                println!("查询失败");
            }
        }
    }

}

//实现菜单类
impl Menu {
    pub fn new() -> Self {
        Menu {
            sms: StudentManagementSystem::new(),
        }
    }

    //输入函数封装
    pub fn get_input_2(prompt: &str) -> Result<(i32, String), String> {
        println!("{}", prompt);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取用户输入时出错");
        let mut parts = input.trim().split_whitespace();
        
        if let Some(id_str) = parts.next() {
            if let Some(name) = parts.next() {
                if let Ok(id) = id_str.parse::<i32>() {
                    return Ok((id, name.trim().to_string()));
                } else {
                    return Err("无效的id".to_string());
                }
            }
        }
        
        Err("输入格式错误".to_string())
    }
    pub fn get_input_1(prompt: &str) -> Result<i32, String> {
        println!("{}", prompt);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取用户输入时出错");
        let mut parts = input.trim().split_whitespace();
        
        if let Some(id_str) = parts.next() {
            if let Ok(id) = id_str.parse::<i32>() {
                return Ok(id);
            } else {
                return Err("无效的id".to_string());
            }
        }
        
        Err("输入格式错误".to_string())
    }

    pub fn run(&mut self) {
        loop {
            println!("###菜单选项###");
            println!("1. 学生操作");
            println!("2. 班级操作");
            println!("3. 社团操作");
            println!("4. 课程操作");
            println!("0. 退出");
            println!("###菜单选项###");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("读取用户输入时出错");

            match choice.trim() {
                "1" => self.student_menu(),
                "2" => self.class_menu(),
                "3" => self.club_menu(),
                "4" => self.course_menu(),
                "0" => {
                    println!("感谢使用Rust简易学生信息管理系统，下次见！");
                    break;
                }
                _ => println!("无效选择，请重新输入"),
            }
        }
    }

    fn student_menu(&mut self) {
        loop {
            println!("###学生操作###");
            println!("1. 添加学生");
            println!("2. 删除学生");
            println!("3. 修改学生");
            println!("4. 查询学生");
            println!("5. 全部学生");
            println!("6. 加入班级");
            println!("7. 加入社团");
            println!("8. 学生选课");
            println!("0. 返回上层");
            println!("###学生操作###");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("读取用户输入时出错");

            match choice.trim() {
                "1" => {
                    // 添加学生的逻辑操作
                    // 例如：self.sms.add_student(id, name);
                    if let Ok((id, name)) = Menu::get_input_2("请输入学生id号和名字，用空格分开：") {
                        self.sms.add_student(id, name);
                        //--继续？返回？
                    } else {
                        println!("输入错误，添加失败！");
                    }
                }
                "2" => {
                    // 删除学生的逻辑操作
                    // 例如：self.sms.delete_student(id);
                    if let Ok(id) = Menu::get_input_1("请输入删除学生id号：") {
                        self.sms.delete_student(id);
                    } else {
                        println!("输入错误，删除失败！");
                    }
                }
                "3" => {
                    // 修改学生的逻辑操作
                    // 例如：self.sms.modify_student(id, name);
                    if let Ok((id, name)) = Menu::get_input_2("请输入修改学生id号和修改后的学生名字，用空格分隔：") {
                        self.sms.modify_student(id, name);
                    } else {
                        println!("输入错误，修改失败！");
                    }
                }
                "4" => {
                    // 查询学生的逻辑操作
                    // 例如：self.sms.query_student(id);
                    if let Ok(id) = Menu::get_input_1("请输入查询学生id号：") {
                        self.sms.query_student(id);
                    } else {
                        println!("输入错误，查询失败！");
                    }
                }
                "5" => {
                    // 打印全部学生的逻辑操作
                    // 显示id和name
                    self.sms.print_all_student_id_and_name();
                }
                "6" => {
                    // 加入班级的逻辑操作
                    // 输入id号和班级号加入
                    if let Ok(sid) = Menu::get_input_1("请输入需要加入班级的学生id号：") {
                        if let Ok(cid) = Menu::get_input_1("请输入需要为该学生添加的班级号：") {
                            self.sms.student_class_selection(sid, cid);
                        } else {
                            println!("输入错误，查询失败！");
                        }
                    } else {
                        println!("输入错误，查询失败！");
                    }
                }
                "7" => {
                    // 加入社团的逻辑操作
                    // 输入id号和社团号加入
                    if let Ok(sid) = Menu::get_input_1("请输入需要加入社团的学生id号：") {
                        if let Ok(cid) = Menu::get_input_1("请输入需要为该学生添加的社团号：") {
                            self.sms.student_club_selection(sid, cid);
                        } else {
                            println!("输入错误，查询失败！");
                        }
                    } else {
                        println!("输入错误，查询失败！");
                    }
                }
                "8" => {
                    // 学生选课的逻辑操作
                    // 输入id号和课程号选课
                    if let Ok(sid) = Menu::get_input_1("请输入需要选课的学生id号：") {
                        if let Ok(cid) = Menu::get_input_1("请输入需要为该学生添加的课程号：") {
                            self.sms.student_course_selection(sid, cid);
                        } else {
                            println!("输入错误，查询失败！");
                        }
                    } else {
                        println!("输入错误，查询失败！");
                    }
                }
                "0" => break,
                _ => println!("无效选择，请重新输入"),
            }
        }
    }

    fn class_menu(&mut self) {
        // 班级操作菜单的逻辑实现
        loop {
            println!("###班级操作###");
            println!("1. 添加班级");
            println!("2. 删除班级");
            println!("3. 修改班级");
            println!("4. 查询班级");
            println!("5. 全部班级");
            println!("6. 添加学生");
            println!("0. 返回上层");
            println!("###学生操作###");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("读取用户输入时出错");

            match choice.trim() {
                "1" => {
                    // 添加班级的逻辑操作
                    if let Ok((id, name)) = Menu::get_input_2("请输入班级id号和名字，用空格分开：") {
                        self.sms.add_class(id, name);
                        //--继续？返回？
                    } else {
                        println!("输入错误，添加失败！");
                    }
                }
                "2" => {
                    // 删除班级的逻辑操作
                    if let Ok(id) = Menu::get_input_1("请输入删除班级id号：") {
                        self.sms.delete_class(id);
                    } else {
                        println!("输入错误，删除失败！");
                    }
                }
                "3" => {
                    // 修改班级的逻辑操作
                    if let Ok((id, name)) = Menu::get_input_2("请输入修改班级id号和修改后的班级名字，用空格分隔：") {
                        self.sms.modify_class(id, name);
                    } else {
                        println!("输入错误，修改失败！");
                    }
                }
                "4" => {
                    // 查询班级的逻辑操作
                    if let Ok(id) = Menu::get_input_1("请输入查询班级id号：") {
                        self.sms.query_class(id);
                    } else {
                        println!("输入错误，查询失败！");
                    }
                }
                "5" => {
                    // 打印全部班级的逻辑操作
                    // 显示id和name
                    self.sms.print_all_class_id_and_name();
                }
                "6" => {
                    // 添加学生的逻辑操作
                    // 输入班级id号和学生id号加入
                    if let Ok(cid) = Menu::get_input_1("请输入需要加入学生的班级id号：") {
                        if let Ok(sid) = Menu::get_input_1("请输入需要添加的学生id号：") {
                            self.sms.student_class_selection(sid, cid);
                        } else {
                            println!("输入错误，添加失败！");
                        }
                    } else {
                        println!("输入错误，添加失败！");
                    }
                }
                "0" => break,
                _ => println!("无效选择，请重新输入"),
            }
        }
    }

    fn club_menu(&mut self) {
        // 社团操作菜单的逻辑实现
        loop {
            println!("###社团操作###");
            println!("1. 添加社团");
            println!("2. 删除社团");
            println!("3. 修改社团");
            println!("4. 查询社团");
            println!("5. 全部社团");
            println!("6. 添加学生");
            println!("0. 返回上层");
            println!("###学生操作###");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("读取用户输入时出错");

            match choice.trim() {
                "1" => {
                    // 添加社团的逻辑操作
                    if let Ok((id, name)) = Menu::get_input_2("请输入社团id号和名字，用空格分开：") {
                        self.sms.add_club(id, name);
                        //--继续？返回？
                    } else {
                        println!("输入错误，添加失败！");
                    }
                }
                "2" => {
                    // 删除社团的逻辑操作
                    if let Ok(id) = Menu::get_input_1("请输入删除社团id号：") {
                        self.sms.delete_club(id);
                    } else {
                        println!("输入错误，删除失败！");
                    }
                }
                "3" => {
                    // 修改社团的逻辑操作
                    if let Ok((id, name)) = Menu::get_input_2("请输入修改社团id号和修改后的社团名字，用空格分隔：") {
                        self.sms.modify_club(id, name);
                    } else {
                        println!("输入错误，修改失败！");
                    }
                }
                "4" => {
                    // 查询社团的逻辑操作
                    if let Ok(id) = Menu::get_input_1("请输入查询社团id号：") {
                        self.sms.query_club(id);
                    } else {
                        println!("输入错误，查询失败！");
                    }
                }
                "5" => {
                    // 打印全部社团的逻辑操作
                    // 显示id和name
                    self.sms.print_all_club_id_and_name();
                }
                "6" => {
                    // 添加学生的逻辑操作
                    // 输入社团id号和学生id号加入
                    if let Ok(cid) = Menu::get_input_1("请输入需要加入学生的社团id号：") {
                        if let Ok(sid) = Menu::get_input_1("请输入需要添加的学生id号：") {
                            self.sms.student_club_selection(sid, cid);
                        } else {
                            println!("输入错误，添加失败！");
                        }
                    } else {
                        println!("输入错误，添加失败！");
                    }
                }
                "0" => break,
                _ => println!("无效选择，请重新输入"),
            }
        }
    }

    fn course_menu(&mut self) {
        // 课程操作菜单的逻辑实现
        loop {
            println!("###课程操作###");
            println!("1. 添加课程");
            println!("2. 删除课程");
            println!("3. 修改课程");
            println!("4. 查询课程");
            println!("5. 全部课程");
            println!("6. 添加学生");
            println!("0. 返回上层");
            println!("###学生操作###");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("读取用户输入时出错");

            match choice.trim() {
                "1" => {
                    // 添加课程的逻辑操作
                    if let Ok((id, name)) = Menu::get_input_2("请输入课程id号和名字，用空格分开：") {
                        self.sms.add_course(id, name);
                        //--继续？返回？
                    } else {
                        println!("输入错误，添加失败！");
                    }
                }
                "2" => {
                    // 删除课程的逻辑操作
                    if let Ok(id) = Menu::get_input_1("请输入删除课程id号：") {
                        self.sms.delete_course(id);
                    } else {
                        println!("输入错误，删除失败！");
                    }
                }
                "3" => {
                    // 修改课程的逻辑操作
                    if let Ok((id,name)) = Menu::get_input_2("请输入修改课程id号和修改后的课程名字，用空格分隔：") {
                        self.sms.modify_course(id, name);
                    } else {
                        println!("输入错误，修改失败！");
                    }
                }
                "4" => {
                    // 查询课程的逻辑操作
                    if let Ok(id) = Menu::get_input_1("请输入查询课程id号：") {
                        self.sms.query_course(id);
                    } else {
                        println!("输入错误，查询失败！");
                    }
                }
                "5" => {
                    // 打印全部课程的逻辑操作
                    // 显示id和name
                    self.sms.print_all_course_id_and_name();
                }
                "6" => {
                    // 添加学生的逻辑操作
                    // 输入课程id号和学生id号加入
                    if let Ok(cid) = Menu::get_input_1("请输入需要加入学生的课程id号：") {
                        if let Ok(sid) = Menu::get_input_1("请输入需要添加的学生id号：") {
                            self.sms.student_course_selection(sid, cid);
                        } else {
                            println!("输入错误，添加失败！");
                        }
                    } else {
                        println!("输入错误，添加失败！");
                    }
                }
                "0" => break,
                _ => println!("无效选择，请重新输入"),
            }
        }
    }
}