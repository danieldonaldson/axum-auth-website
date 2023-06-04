use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::error::Result;
use crate::Error::*;

use crate::user::User;

use super::SECTION;

pub async fn create_db_client() -> Client {
    let region_provider =
        RegionProviderChain::default_provider().or_else("af-south-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

pub async fn get_user_by_email(
    db_client: Client,
    username: String,
) -> Result<Option<User>> {
    let item = match db_client
        .get_item()
        .table_name("students")
        .key("email", AttributeValue::S(username))
        .key("cohort", AttributeValue::S(SECTION.to_string()))
        .send()
        .await
    {
        Ok(i) => i,
        Err(_) => return Err(DBConnectionFail),
    };

    if let Some(found) = item.item {
        let email = found
            .get("email")
            .ok_or(DBFailFieldNotFound("email".to_string()))?
            .as_s()
            .unwrap(); // handle this?
        let known_as = found
            .get("known_as")
            .ok_or(DBFailFieldNotFound("known_as".to_string()))?
            .as_s()
            .unwrap(); // handle this?
        let password_result = found
            .get("password")
            .ok_or(DBFailFieldNotFound("password".to_string()))?
            .as_s();
        let password = match password_result {
            Err(_) => return Err(DBFailFieldEmpty("password".to_string())),
            Ok(p) => p,
        };

        let user = User {
            email: email.to_string(),
            known_as: known_as.to_string(),
            password: password.to_string(),
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}
