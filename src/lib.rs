//! 洛玖机器人插件开发SDK
//! 
//! 这个库提供了开发洛玖机器人插件所需的所有工具和接口。

pub mod api;
pub mod plugin;
pub mod message;
pub mod config;
pub mod macros;
pub mod utils;

// 重新导出常用类型，方便用户使用
pub use plugin::{Plugin, PluginMetadata};
pub use message::{GroupMessage, PrivateMessage};
pub use config::Value;
pub use api::ApiTrait;
pub use api::ApiManager;