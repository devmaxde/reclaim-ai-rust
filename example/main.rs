use reclaim_ai::apis::configuration::Configuration;
use reclaim_ai::apis::default_api::api_tasks_get;

#[tokio::main]
async fn main() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("your-api-key".to_string());
    println!("{:#?}",api_tasks_get(&config).await);
}