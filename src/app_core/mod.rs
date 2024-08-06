pub mod task;
pub mod todo_list;
pub use todo_list::ToDoList;
pub use task::TaskStatus;

pub fn initial_app() -> ToDoList {
  ToDoList::initial_app()
}