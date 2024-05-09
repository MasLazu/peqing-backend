use axum::Router;
use routes::auth_route;
use routes::user_route;
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, postgres::Postgres, Error, PgPool};
use tokio::net::TcpListener;

mod middleware;
mod models;
mod repositories;
mod routes;

pub type Result<T> = core::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = get_db_pool().await.unwrap();

    let routes = Router::new()
        .merge(user_route::routes(pool.clone()))
        .merge(auth_route::routes(pool.clone()))
        .layer(axum::middleware::from_fn_with_state(
            pool.clone(),
            middleware::auth_resolver::auth_resolver,
        ));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_db_pool() -> Result<PgPool> {
    let database_url = "postgres://postgres:postgres@localhost/peqing";

    if !Postgres::database_exists(database_url).await? {
        Postgres::create_database(database_url).await?;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
