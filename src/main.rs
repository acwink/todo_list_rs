
use todo::app_core::{initial_app, TaskStatus};
use clap::{command, Arg, ArgAction};

const TODO_COMMAND: [&str; 4] = ["add", "delete", "update", "show"];
fn main() {
  let match_result = command!()
    .arg(
      Arg::new("add")
        .long("add")
        .short('a')
        .num_args(1)
        .value_name("DESC")
        .exclusive(true)
        .help("Add a task.")
    )
    .arg(
      Arg::new("update")
        .long("update")
        .short('u')
        .action(ArgAction::Set)
        .num_args(2)
        .value_names(["NO", "STATUS"])
        .exclusive(true)
        .help("Update the task's status.")
    )
    .arg(
      Arg::new("delete")
        .long("delete")
        .short('d')
        .num_args(1)
        .value_name("NO")
        .exclusive(true)
        .help("Delete a task by the task's NO.")
    )
    .arg(
      Arg::new("show")
        .long("show")
        .short('s')
        .num_args(0)
        .help("Show all your tasks.")
    )
    .get_matches();

  let mut app = initial_app();
  let operation_option = TODO_COMMAND.into_iter().find(|op| match_result.contains_id(op));
  if let Some(op) = operation_option {
    match op {
      "add" => {
        let desc = match_result.get_one::<String>("add").expect("Except that the NO is an unsigned number");
        let _ = app.add_task(desc);
        app.show_task_list();
      }
      "delete" => {
        let no = match_result.get_one::<String>("delete").unwrap();
        app.delete_task(no.parse::<u32>().expect("Except that the NO is an unsigned number"));
        app.show_task_list();
      },
      "show" => {
        app.show_task_list();
      },
      "update" => {
        let tuple = match_result.get_many::<String>("update").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
        let no = tuple.get(0).expect("Except that the NO is an unsigned number").parse::<u32>().unwrap();
        let status = tuple.get(1).unwrap();
        app.update_task_status(no, TaskStatus::from(*status));
        app.show_task_list();
      },
      _ => todo!(),
    }
    
  }
  
}
