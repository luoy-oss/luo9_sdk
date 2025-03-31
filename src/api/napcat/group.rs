//! Napcat群组API
//! 
//! 这个模块实现了与Napcat框架交互的群组相关API。

use reqwest::Client;
use serde_json::{json, Value};

/// 发送群消息
pub async fn send_group_message(base_url: &str, access_token: &str, group_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/send_group_msg", base_url);
    let client = Client::new();
    
    let params = json!({
        "group_id": group_id,
        "message": message,
        "access_token": access_token
    });
    println!("{params}");

    client.post(&url)
        .query(&params)
        .send()
        .await?;
    
    Ok(())
}

/// 发送群AI语音
pub async fn send_group_ai_record(base_url: &str, access_token: &str, group_id: &str, character: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/send_group_ai_record", base_url);
    let client = Client::new();
    
    let params = json!({
        "group_id": group_id,
        "character": character,
        "text": text,
        "access_token": access_token
    });
    
    client.post(&url)
        .query(&params)
        .send()
        .await?;
    
    Ok(())
}

/// 发送群@消息
pub async fn send_group_at(base_url: &str, access_token: &str, group_id: &str, qq: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/send_group_at", base_url);
    let client = Client::new();
    
    let params = json!({
        "group_id": group_id,
        "message": format!("[CQ:at,qq={qq}]", qq=qq),
        "access_token": access_token
    });
    
    client.post(&url)
        .query(&params)
        .send()
        .await?;
    
    Ok(())
}

/// 发送群图片
pub async fn send_group_image(base_url: &str, access_token: &str, group_id: &str, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/send_group_msg", base_url);
    let client = Client::new();
    
    let params = json!({
        "group_id": group_id,
        "message": format!("[CQ:image,file={file}]", file=file),
        "access_token": access_token
    });

    let response = client.post(&url)
        .query(&params)
        .send()
        .await?;
    
    Ok(())
}

/// 上传群文件
pub async fn upload_group_file(base_url: &str, access_token: &str, group_id: &str, file: &str, name: &str, folder_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/upload_group_file", base_url);
    let client = Client::new();
    
    let params = json!({
        "group_id": group_id,
        "file": file,
        "name": name,
        "folder_id": folder_id,
        "access_token": access_token
    });
    
    client.post(&url)
        .query(&params)
        .send()
        .await?;
    
    Ok(())
}

/// 发送群戳一戳
pub async fn send_group_poke(base_url: &str, access_token: &str, group_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/send_group_poke", base_url);
    let client = Client::new();
    
    let params = json!({
        "group_id": group_id,
        "user_id": user_id,
        "access_token": access_token
    });
    
    client.post(&url)
        .query(&params)
        .send()
        .await?;
    
    Ok(())
}

/// 获取群文件夹中的文件
pub async fn get_group_files_by_folder(base_url: &str, access_token: &str, group_id: &str, folder_id: &str, file_count: i32) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}/get_group_files_by_folder", base_url);
    let client = Client::new();
    
    let params = json!({
        "group_id": group_id,
        "folder_id": folder_id,
        "file_count": file_count,
        "access_token": access_token
    });
    
    let response = client.post(&url)
        .query(&params)
        .send()
        .await?
        .json::<Value>()
        .await?;
    
    Ok(response.to_string())
}

/// 获取群根目录文件
pub async fn get_group_root_files(base_url: &str, access_token: &str, group_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}/get_group_root_files", base_url);
    let client = Client::new();
    
    let params = json!({
        "group_id": group_id,
        "access_token": access_token
    });
    
    let response = client.post(&url)
        .query(&params)
        .send()
        .await?
        .json::<Value>()
        .await?;
    
    Ok(response.to_string())
}