//! 插件特性定义
//! 
//! 这个模块定义了插件需要实现的特性。

use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::message::{GroupMessage, PrivateMessage};

/// 插件元数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct PluginMetadata {
    /// 插件名称
    pub name: String,
    
    /// 插件描述
    pub describe: String,
    
    /// 插件作者
    pub author: String,
    
    /// 插件版本
    pub version: String,
    
    /// 插件支持的消息类型
    pub message_types: Vec<String>,
}

/// 插件特性，定义了插件需要实现的方法
#[async_trait]
pub trait Plugin: Send + Sync {
    /// 处理群消息
    async fn handle_group_message(&self, _message: &GroupMessage) -> Result<()> {
        // 默认实现，如果插件不支持此类消息，则直接返回 Ok
        println!("插件 {} 不支持处理群消息", self.metadata().name);
        Ok(())
    }
    
    /// 处理私聊消息
    async fn handle_private_message(&self, _message: &PrivateMessage) -> Result<()> {
        // 默认实现，如果插件不支持此类消息，则直接返回 Ok
        println!("插件 {} 不支持处理私聊消息", self.metadata().name);
        Ok(())
    }
    
    /// 处理群戳一戳事件
    async fn handle_group_poke(&self, _target_id: &str, _user_id: &str, _group_id: &str) -> Result<()> {
        // 默认实现，如果插件不支持此类事件，则直接返回 Ok
        println!("插件 {} 不支持处理群戳一戳事件", self.metadata().name);
        Ok(())
    }
}