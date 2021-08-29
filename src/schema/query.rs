use async_graphql::*;

#[derive(SimpleObject)]
struct User {
    username: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_user(&self) -> User {
        User {
            username: "Test".to_string(),
        }
    }
}
