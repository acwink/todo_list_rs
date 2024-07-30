use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use std::fmt::{self};
use std::num::ParseIntError;
use std::convert::From;

const FILE_PATH: &str = "D:\\Program\\to_do_list_rs\\data.txt";
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
    fn initial_app(file_path: &str) -> Self {
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

    pub fn add_task(&mut self, desc: &str) -> Result<u32, ParseTaskError> {
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

#[test]
fn test_add_task() {
    let mut app = ToDoList::initial_app(FILE_PATH);
    let no = app.add_task("学习学习");
    assert!(no.is_ok());
}

fn main() {
    let mut app = ToDoList::initial_app(FILE_PATH);
    let no = app.add_task("学习学习");
    println!("no: {}", no.unwrap());
    println!("{}", app);
    println!("Hello, world!");
}
