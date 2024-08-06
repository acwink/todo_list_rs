#[cfg(test)]
mod test_app {
    use todo::app_core::{ToDoList, TaskStatus};
    use tempfile::NamedTempFile;

    #[test]
    fn test_add_task() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let mut app = ToDoList::initial_app_by_path(file_path);
        let no = app.add_task("test");
        assert!(no.is_ok());
    }
    
    #[test]
    fn test_update_task() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let mut app = ToDoList::initial_app_by_path(file_path);
        let no = app.add_task("test_update_task");
        assert!(no.is_ok());
        let res = app.update_task_status(no.unwrap(), TaskStatus::Processing);
        assert!(res);
    }

    #[test]
    fn test_show_task_list() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let app = ToDoList::initial_app_by_path(file_path);
        app.show_task_list();
    }

    #[test]
    fn test_delete_task() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();
        let mut app = ToDoList::initial_app_by_path(file_path);
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
            let mut app1 = ToDoList::initial_app_by_path(file_path);
            let _ = app1.add_task(todo_desc);
        }
        let app2 = ToDoList::initial_app_by_path(file_path);
        app2.show_task_list();
        let task = app2.find_task_by_desc(todo_desc);
        assert!(task.is_some());
        assert!(task.unwrap().desc.eq(todo_desc));
    }

}
