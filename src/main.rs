use sqlx::{postgres::PgPoolOptions, query};

fn test_sqlx() {}

#[derive(sqlx::Type, Debug, PartialEq, Eq)]
#[sqlx(type_name = "test_enum", rename_all = "snake_case")]
pub enum Test {
    TestExample,
    AnotherExample,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:dev_only_pwd@localhost:15432/test_enum")
        .await
        .unwrap();

    let mut tx = pool.clone().begin().await.unwrap();

    query!("SELECT 1 FROM test WHERE value = $1", Test::TestExample);
}
