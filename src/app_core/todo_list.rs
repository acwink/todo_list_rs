
use std::io::{Read, Write};
use std::{fs::File, path::Path};
use std::fmt;
use super::task::{Task, TaskStatus, ParseTaskError};
use std::str::FromStr;


const FILE_PATH: &str = "data.txt";
pub struct ToDoList {
  task_list: Vec<Task>,
}

impl ToDoList {
  pub fn initial_app_by_path<T: AsRef<Path>>(file_path: T) -> Self {
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

  pub fn initial_app() -> Self {
    return Self::initial_app_by_path(FILE_PATH);
  }


  pub fn add_task(&mut self, desc: &str) -> Result<u32, ParseTaskError> {
      let no = match self.task_list.last() {
          Some(task) => task.no + 1,
          None => 1u32,  
      };
      let s = format!("{},{},{}", no, desc, TaskStatus::Pending);
      let task = Task::from_str(&s)?;
      self.task_list.push(task);
      Ok(no)
  }

  pub fn update_task_status(&mut self, no: u32, status: TaskStatus) -> bool {
      let task = self.task_list.iter_mut().find(|task| task.no == no);
      match task {
          Some(task) => {
              task.status = status;
              true
          },
          None => false,
      }
  }

  pub fn delete_task(&mut self, no: u32) -> bool {
      let res = self.task_list.iter().enumerate().find(|(_, task)| task.no == no);
      if let Some((index, _)) = res {
          self.task_list.remove(index);
          true
      } else {
          false
      }
  }

  pub fn find_task_by_desc(&self, desc: &str) -> Option<&Task> {
      self.task_list.iter().find(|task| task.desc.eq(desc))
  }

  pub fn show_task_list(&self) {
      println!("{}", self);
  }

  pub fn show_task_by_status(&self, status: TaskStatus) {
    let list = self.task_list.iter().filter(|task| task.status == status);
    println!("{:<20}{:<30}{:<20}", "no", "desc", "status");
    list.for_each(|task| println!("{}", task));
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
impl Drop for ToDoList {
  fn drop(&mut self) {
      match File::create(FILE_PATH) {
          Ok(mut f) => {
              for ele in &self.task_list {
                  let status: &str = ele.status.into();
                  let _ = writeln!(f, "{},{},{}", ele.no, ele.desc, status); 
              }
          },
          Err(_) => {
              println!("save failed");
          }
      }

  }
}
