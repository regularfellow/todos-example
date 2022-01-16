use sqlx::postgres::PgPool;
use std::env;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    get_todos_with_things(&pool).await?;

    Ok(())
}

struct Todo {
    id: i64,
    description: String,
    done: bool,
}

async fn get_todos_with_things(pool: &PgPool) -> anyhow::Result<Vec<Todo>> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT * FROM todos WHERE id IN (SELECT todo_id FROM todo_things)"
    )
    .fetch_all(pool)
    .await?;
    Ok(todos)
}
