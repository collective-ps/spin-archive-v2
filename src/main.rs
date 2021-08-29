use std::env;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use dotenv::dotenv;
use poem::web::Html;
use poem::{handler, route, IntoResponse, Server};
use sqlx::{migrate::Migrator, postgres::Postgres, Pool};

use query::GraphQL;
use schema::{MutationRoot, QueryRoot};

mod config;
mod jwt;
mod models;
mod query;
mod schema;

static MIGRATOR: Migrator = sqlx::migrate!();

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL was not provided");
    let pool = Pool::<Postgres>::connect(&database_url).await?;
    MIGRATOR.run(&pool).await?;

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();

    let mut app = route();

    app.at("/")
        .get(graphql_playground)
        .post(GraphQL::new(schema));

    Server::bind("0.0.0.0:8000")
        .await
        .unwrap()
        .run(app)
        .await
        .unwrap();

    Ok(())
}
