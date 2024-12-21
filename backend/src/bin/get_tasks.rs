use backend::models::{self, Task};
use backend::*;
use diesel::prelude::*;

fn main() {
    use backend::schema::tasks::dsl::*;

    //let connection = &mut establish_connection();

    //let results = tasks
    //   .select(Task::as_select())
    //  .limit(5)
    // .load(connection)
    //.expect("Should be a db connection");

    //println!("{}", results.len());
    //for task in results {
    //   println!("{}", task.title);
    //  println!("------------");
    // if let Some(body_content) = task.body {
    //    println!("{}", body_content);
    //}
    //}
}
