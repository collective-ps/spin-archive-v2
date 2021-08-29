use async_graphql::*;

#[derive(SimpleObject)]
struct User {
    username: String,
}

pub struct Query;

#[Object]
impl Query {
    async fn current_user(&self) -> User {
        User {
            username: "Test".to_string(),
        }
    }
}
