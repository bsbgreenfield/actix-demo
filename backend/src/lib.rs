use actix_web::web;
use deadpool_postgres::{
    Client, Config, GenericClient, ManagerConfig, Pool, RecyclingMethod, Runtime,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTask, Task};
use serde::Deserialize;
use std::env;
use std::error::Error;

use tokio_postgres::NoTls;

pub mod models;
pub mod schema;

pub fn get_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set in .env");
    let mut cnfig = Config::new();

    let max_connections = env::var("DB_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "16".to_string())
        .parse::<usize>()
        .expect("DB_MAX_CONNECTIONS must be a valid number");

    cnfig.url = Some(database_url);
    cnfig.user = Some("benjigreenfield".to_string());
    cnfig.password = Some("123".to_string());
    cnfig.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cnfig.pool = Some(deadpool_postgres::PoolConfig {
        max_size: max_connections,
        ..Default::default()
    });

    cnfig
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create pool")
}

pub fn create_new_task(conn: &mut PgConnection, title: &str, body: Option<&str>) -> Task {
    use crate::schema::tasks;

    let new_task = NewTask { title, body };

    diesel::insert_into(tasks::table)
        .values(new_task)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("should have a db connection")
}

pub async fn get_five_tasks(pool: web::Data<Pool>) -> Result<Vec<Task>, Box<dyn Error>> {
    let client: Client = pool.get().await?;

    let query = "SELECT * FROM tasks";

    let rows = client.query(query, &[]).await?;

    let tasks = rows
        .into_iter()
        .map(|row| Task {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
        })
        .collect();

    Ok(tasks)
}

#[derive(Deserialize, Debug)]
pub struct TaskJson {
    pub title: String,
    pub body: String,
}
pub async fn make_new_task(
    pool: web::Data<Pool>,
    new_task: TaskJson,
) -> Result<u64, Box<dyn Error>> {
    let client: Client = pool.get().await?;

    let query = "INSERT INTO tasks (title, body) VALUES ($1, $2)";
    client
        .query(query, &[&new_task.title, &new_task.body])
        .await?;

    Ok(1)
}
