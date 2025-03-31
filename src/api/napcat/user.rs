//! Napcat用户API
//! 
//! 这个模块实现了与Napcat框架交互的用户相关API。

use reqwest::Client;
use serde_json::json;

/// 发送私聊消息
pub async fn send_private_msg(base_url: &str, access_token: &str, user_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/send_private_msg", base_url);
    let client = Client::new();
    
    let params = json!({
        "user_id": user_id,
        "message": message,
        "access_token": access_token
    });
    
    client.post(&url)
        .query(&params)
        .send()
        .await?;
    
    Ok(())
}