use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::Error::DBFailFieldNotFound

use crate::user::User;

pub async fn create_db_client() -> Client {
    let region_provider =
        RegionProviderChain::default_provider().or_else("af-south-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

pub async fn get_user_by_email(db_client: Client) -> Option<User> {
    let item = db_client
        .get_item()
        .table_name("students")
        .key("email", AttributeValue::S("dnldan001".to_string()))
        .key("cohort", AttributeValue::S("UCTOHS".to_string()))
        .send()
        .await
        .unwrap();

    if let Some(found) = item.item {
        let email = found
            .get("email")
            .ok_or(DBFailFieldNotFound("email".to_string()))?
            .as_s()
            .unwrap(); //TODO: fix this??
        let known_as = found
            .get("known_as")
            .ok_or(DBFailFieldNotFound("known_as".to_string()))?
            .as_s()
            .unwrap(); //TODO: fix this??
                       // dbg!(email_result);

        let user = User {
            email: email.to_string(),
            known_as: known_as.to_string(),
        };
        return user;
    } else {
        return None;
    }
}
