#[macro_use]
extern crate rocket;

use rocket::time::Date;

mod todoTask {

    use rocket::serde::json::Json;
    use rocket::serde::Deserialize;
    use rocket::serde::Serialize;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct TodoData {
        
        user_id: String,
        task_id: String,
        task: String,
        date: String,
        completed: bool,
    }
    
    #[post("/new", format = "json", data = "<todo_data>")]
    pub fn new_todo(todo_data: Json<TodoData>) -> String {
        let todo = todo_data.into_inner();

        // Print the input to the console
        println!("User ID: {}", todo.user_id);
        println!("Task ID: {}", todo.task_id);
        println!("Task: {}", todo.task);
        println!("Date: {:?}", todo.date);
        println!("Completed: {}", todo.completed);

        // Respond with the input details
        format!(
            "The input user_id is {}, task_id is {}, task is {}, date is {:?}, completed is {}",
            todo.user_id, todo.task_id, todo.task, todo.date, todo.completed
        )
    }

    #[get("/new/<id>")]
    pub fn get_todo(id: &str) -> String {
        format!("hello, {}", id)
    }
}


use todoTask::new_todo;
use todoTask::get_todo;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![new_todo, get_todo])
}
