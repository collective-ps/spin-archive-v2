use std::env;

#[allow(dead_code)]
pub fn secret_key() -> String {
    env::var("SECRET_KEY").unwrap_or("SECRET_KEY_PLEASE_CHANGE_ME".to_owned())
}

#[allow(dead_code)]
pub fn get_webhook_url() -> String {
    env::var("DISCORD_WEBHOOK_URL").unwrap_or_default()
}

#[allow(dead_code)]
pub fn get_contributor_webhook_url() -> String {
    env::var("DISCORD_CONTRIBUTOR_WEBHOOK_URL").unwrap_or_default()
}

#[allow(dead_code)]
pub fn get_twitter_consumer_key() -> String {
    env::var("TWITTER_CONSUMER_KEY").unwrap_or_default()
}

#[allow(dead_code)]
pub fn get_twitter_consumer_secret() -> String {
    env::var("TWITTER_CONSUMER_SECRET").unwrap_or_default()
}

#[allow(dead_code)]
pub fn get_aws_access_key_id() -> String {
    env::var("AWS_ACCESS_KEY_ID").unwrap_or_default()
}

#[allow(dead_code)]
pub fn get_aws_secret_access_key() -> String {
    env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default()
}
