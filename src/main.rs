use std::str::FromStr;
use std::fs::File;
use std::io::{self, Read};
use std::fmt::{self};
use std::num::ParseIntError;
use std::convert::From;

const FILE_PATH: &str = "D:\\Program\\to_do_list_rs\\data.txt";

enum TaskStatus {
    Pending,
    Processing,
    Completed,
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
                let status: TaskStatus = match args[2] {
                    "0" => TaskStatus::Pending,
                    "1" => TaskStatus::Processing,
                    "2" => TaskStatus::Completed,
                    _ => Err(ParseTaskError(format!("task's status is error")))?,
                };

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

fn main() {
    let mut app = ToDoList::initial_app(FILE_PATH);
    println!("{}", app);
    println!("Hello, world!");
}
