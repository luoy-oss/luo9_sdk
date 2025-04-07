//! 消息类型定义
//! 
//! 这个模块定义了各种消息类型。

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;


/// 群消息
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupMessage {
    /// 消息ID
    pub message_id: String,
    /// 消息内容
    pub content: String,
    /// 发送者ID
    pub sender_id: String,
    /// 群ID
    pub group_id: String,
    /// 原始消息数据
    pub raw_data: JsonValue,
}
/// 私聊消息
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateMessage {
    /// 消息ID
    pub message_id: String,
    /// 消息内容
    pub content: String,
    /// 发送者ID
    pub sender_id: String,
    /// 原始消息数据
    pub raw_data: JsonValue,
}
