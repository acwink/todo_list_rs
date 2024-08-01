use std::path::Path;
use std::str::FromStr;
use std::fs::File;
use std::io::{Read, Take, Write};
use std::fmt::{self};
use std::num::ParseIntError;
use std::convert::From;
use std::io;

const FILE_PATH: &str = "data.txt";
// todo：模块化该代码，并只暴露出TodoList结构, 使用crate尝试吧，先把整体的逻辑搭建起来
enum TaskStatus {
    Pending,
    Processing,
    Completed,
}

impl From<&str> for TaskStatus {
    fn from(value: &str) -> Self {
        match value {
            "0" => Self::Pending,
            "1" => Self::Processing,
            "2" => Self::Completed,
            _ => Self::Pending,
        }
    }
}

impl From<TaskStatus> for &str {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Pending => "0",
            TaskStatus::Completed => "1",
            TaskStatus::Processing => "2",
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Pending => "Pending",
            Self::Processing => "Processing",
            Self::Completed => "Completed",
        })
    }
}

struct Task {
    no: u32,
    desc: String,
    status: TaskStatus,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<20}{:<30}{:<20}", self.no, self.desc, self.status)
    }
}

#[derive(Debug)]
struct ParseTaskError(String);

impl fmt::Display for ParseTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ParseIntError> for ParseTaskError {
    fn from(_: ParseIntError) -> Self {
        ParseTaskError(format!("no is invalid digit"))
    }
}

impl FromStr for Task {
    type Err = ParseTaskError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<_> = s.split(',').collect();
        match args.len() {
            3 => {
                let no: u32 = args[0].parse()?;
                let desc = args[1].to_string();
                let status: TaskStatus = args[2].into();
                Ok(Self {
                    no,
                    desc,
                    status,
                })
            },
            _ => Err(ParseTaskError("argument's number is error".to_string())),
        }
    }
}

struct ToDoList {
    task_list: Vec<Task>,
}

impl ToDoList {
    fn initial_app<T: AsRef<Path>>(file_path: T) -> Self {
        let mut task_list: Vec<Task> = Vec::new();
        // 通过filepath找到指定的数据文件, 然后将其添加进todoList
        let mut file = File::open(file_path).unwrap();
        let mut tasks_data = String::new();
        // 把文件内容读入到buffer
        let _ = file.read_to_string(&mut tasks_data);
        
        for task in tasks_data.lines() {
            let task = Task::from_str(task).unwrap();
            task_list.push(task);
        }

        Self {
            task_list,
        }
    }

    fn add_task(&mut self, desc: &str) -> Result<u32, ParseTaskError> {
        let no = match self.task_list.last() {
            Some(task) => task.no + 1,
            None => 1u32,  
        };
        let s = format!("{},{},{}", no, desc, TaskStatus::Pending);
        println!("{}", s);
        let task = Task::from_str(&s)?;
        self.task_list.push(task);
        Ok(no)
    }

    fn update_task_status(&mut self, no: u32, status: TaskStatus) -> bool {
        let task = self.task_list.iter_mut().find(|task| task.no == no);
        match task {
            Some(task) => {
                task.status = status;
                true
            },
            None => false,
        }
    }

    fn delete_task(&mut self, no: u32) -> bool {
        let res = self.task_list.iter().enumerate().find(|(_, task)| task.no == no);
        if let Some((index, _)) = res {
            self.task_list.remove(index);
            true
        } else {
            false
        }
    }

    fn find_task_by_desc(&self, desc: &str) -> Option<&Task> {
        self.task_list.iter().find(|task| task.desc.eq(desc))
    }

    fn show_task_list(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for ToDoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:<20}{:<30}{:<20}", "no", "desc", "status")?;
        for ele in self.task_list.iter() {
            writeln!(f, "{}", ele)?;
        }
        write!(f, "")
    }
}
// todo：开一个线程定时去保存数据
impl Drop for ToDoList {
    fn drop(&mut self) {
        println!("drop todoList");
        match File::create(FILE_PATH) {
            Ok(mut f) => {
                for ele in &self.task_list {
                    let _ = writeln!(f, "{},{},{}", ele.no, ele.desc, ele.status); 
                }
            },
            Err(_) => {
                println!("save failed");
            }
        }

    }
}

#[cfg(test)]
mod test_app {
    use crate::{ToDoList, TaskStatus};
    use tempfile::NamedTempFile;

    #[test]
    fn test_add_task() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let mut app = ToDoList::initial_app(file_path);
        let no = app.add_task("test");
        assert!(no.is_ok());
    }
    
    #[test]
    fn test_update_task() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let mut app = ToDoList::initial_app(file_path);
        let no = app.add_task("test_update_task");
        assert!(no.is_ok());
        let res = app.update_task_status(no.unwrap(), TaskStatus::Processing);
        assert!(res);
    }

    #[test]
    fn test_show_task_list() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let app = ToDoList::initial_app(file_path);
        app.show_task_list();
    }

    #[test]
    fn test_delete_task() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let mut app = ToDoList::initial_app(file_path);
        let no = app.add_task("test delete");
        assert!(no.is_ok());
        assert!(app.delete_task(no.unwrap()));
    }

    // 这个测试用例有问题，待修改
    #[test]
    fn test_save_data() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let todo_desc = "test_save_data";
        {
            let mut app1 = ToDoList::initial_app(file_path);
            app1.add_task(todo_desc);
        }
        let mut app2 = ToDoList::initial_app(file_path);
        app2.show_task_list();
        let task = app2.find_task_by_desc(todo_desc);
        assert!(task.is_some());
        assert!(task.unwrap().desc.eq(todo_desc));
    }

}

fn main() {
    let mut app = ToDoList::initial_app(FILE_PATH);
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim_end();
        println!("{}", input);
    }
}
