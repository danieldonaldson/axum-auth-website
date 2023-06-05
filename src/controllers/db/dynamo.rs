use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::app_config::AppConfig;
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
    db_client: &Client,
    username: String,
    config: &AppConfig,
) -> Result<Option<User>> {
    let item = match db_client
        .get_item()
        .table_name(&config.users_table)
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
            .cloned()
            .unwrap(); // handle this?
        let known_as = found
            .get("known_as")
            .ok_or(DBFailFieldNotFound("known_as".to_string()))?
            .as_s()
            .cloned()
            .unwrap(); // handle this?
        let group = found
            .get("group_id")
            .ok_or(DBFailFieldNotFound("group_id".to_string()))?
            .as_n()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        let password_result = found
            .get("password")
            .ok_or(DBFailFieldNotFound("password".to_string()))?
            .as_s()
            .cloned();
        let password = match password_result {
            Err(_) => return Err(DBFailFieldEmpty("password".to_string())),
            Ok(p) => p,
        };

        let user = User {
            email,
            known_as,
            password,
            group,
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub async fn create_user(
    db_client: &Client,
    username: String,
    password: String,
    config: &AppConfig,
) -> Result<()> {
    let user_av = AttributeValue::S(username);
    let password_av = AttributeValue::S(password);

    let request = db_client
        .put_item()
        .table_name(&config.users_table)
        .item("email", user_av)
        .item("password", password_av)
        .item("cohort", AttributeValue::S(SECTION.to_string()))
        .item("known_as", AttributeValue::S("GG".to_string()))
        .item("group_id", AttributeValue::N("0".to_string())); // WHYYYYY

    println!("Executing request [{request:?}] to add item...");

    match request.send().await {
        Ok(i) => i,
        Err(e) => {
            dbg!(e);
            return Err(DBConnectionFail);
        }
    };
    // dbg!(&resp);
    Ok(())
}
