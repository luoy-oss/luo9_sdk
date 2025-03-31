//! API模块
//! 
//! 这个模块定义了与不同机器人框架交互的API。

pub mod napcat;
// 未来可以添加更多框架支持
// pub mod onebot;

use std::sync::Arc;
use async_trait::async_trait;
use anyhow::{Result, anyhow};

use super::config::Value;

/// API特性，定义了与机器人框架交互的方法
#[async_trait]
pub trait ApiTrait: Send + Sync {
    /// 发送群消息
    async fn send_group_message(&self, group_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 发送私聊消息
    async fn send_private_msg(&self, user_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 发送群AI语音
    async fn send_group_ai_record(&self, group_id: &str, voice: &str, message: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 发送群@消息
    async fn send_group_at(&self, group_id: &str, qq: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 发送群图片
    async fn send_group_image(&self, group_id: &str, file: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 发送群文件
    async fn send_group_file(&self, group_id: &str, file: &str, name: &str, folder_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 发送群戳一戳
    async fn send_group_poke(&self, group_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 获取群文件夹中的文件
    async fn get_group_files_by_folder(&self, group_id: &str, folder_id: &str, file_count: i32) -> Result<String, Box<dyn std::error::Error>>;
    
    /// 获取群根目录文件
    async fn get_group_root_files(&self, group_id: &str) -> Result<String, Box<dyn std::error::Error>>;
}

/// API管理器，用于创建和管理API实例
#[derive(Clone)]
pub struct ApiManager {
    api: Arc<dyn ApiTrait>
}

impl ApiManager {
    /// 创建一个新的API管理器
    pub fn new(config: Arc<Value>) -> Result<Self, Box<dyn std::error::Error>> {
        let _config = config.clone();
        
        if _config.napcat {
            let napcat = napcat::NapCat::new(_config.base_url(), _config.access_token());
            return Ok(Self {
                api: Arc::new(napcat)
            });
        }
        
        // 如果需要支持其他框架，可以在这里添加
        // if _config.onebot {
        //     let onebot = onebot::OneBot::new(_config.base_url(), _config.access_token());
        //     return Ok(Self {
        //         api: Arc::new(onebot)
        //     });
        // }
        
        Err("No API enabled in config".into())
    }
    
    /// 发送群消息
    pub async fn send_group_message(&self, group_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.api.send_group_message(group_id, message).await
    }
    
    /// 发送群AI语音
    pub async fn send_group_ai_record(&self, group_id: &str, voice: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.api.send_group_ai_record(group_id, voice, message).await
    }
    
    /// 发送群@消息
    pub async fn send_group_at(&self, group_id: &str, qq: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.api.send_group_at(group_id, qq).await
    }
    
    /// 发送群图片
    pub async fn send_group_image(&self, group_id: &str, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.api.send_group_image(group_id, file).await
    }
    
    /// 发送群文件
    pub async fn send_group_file(&self, group_id: &str, file: &str, name: &str, folder_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.api.send_group_file(group_id, file, name, folder_id).await
    }
    
    /// 发送群戳一戳
    pub async fn send_group_poke(&self, group_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.api.send_group_poke(group_id, user_id).await
    }
    
    /// 发送私聊消息
    pub async fn send_private_msg(&self, user_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.api.send_private_msg(user_id, message).await
    }
    
    /// 获取群文件夹中的文件
    pub async fn get_group_files_by_folder(&self, group_id: &str, folder_id: &str, file_count: i32) -> Result<String, Box<dyn std::error::Error>> {
        self.api.get_group_files_by_folder(group_id, folder_id, file_count).await
    }
    
    /// 获取群根目录文件
    pub async fn get_group_root_files(&self, group_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.api.get_group_root_files(group_id).await
    }
}