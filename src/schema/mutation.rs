use async_graphql::*;
use sqlx::PgPool;

use crate::models::user;

#[derive(SimpleObject)]
struct LoginResult {
    token: Option<String>,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<LoginResult> {
        let pool = ctx.data_unchecked::<PgPool>();
        let user = user::login(&pool, &username, &password).await;

        match user {
            Ok(_user) => Ok(LoginResult {
                token: Some("logged-in".to_owned()),
            }),
            _ => Ok(LoginResult { token: None }),
        }
    }
}
