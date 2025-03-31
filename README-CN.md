# Luo9 SDK

[![Crates.io](https://img.shields.io/crates/v/luo9_sdk.svg)](https://crates.io/crates/luo9_sdk)
[![文档](https://img.shields.io/badge/docs-drluo.top|luo9sdk-blue.svg)](https://www.drluo.top/posts/luo9_sdk)
[![许可证: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

一个用于开发洛玖机器人插件的综合SDK，洛玖机器人是一个多功能的消息机器人框架。

## 特性

* 易于使用的API，用于处理各种消息类型
* 支持群组和私人消息
* 内置常见交互的事件处理
* 基于Tokio运行时的异步API设计
* 简化的配置管理
* 跨平台兼容性

## 安装

在您的`Cargo.toml`中添加：

```toml
[dependencies]
luo9_sdk = "1.0.0"
```

## 快速开始
以下是一个洛玖机器人插件的示例：

```rust
use std::sync::Arc;
use async_trait::async_trait;
use anyhow::{Result, anyhow};

use luo9_sdk::{
    Plugin, PluginMetadata,
    GroupMessage, PrivateMessage,
    Value, ApiManager,
    export_plugin
};

pub struct MyPlugin {
    metadata: PluginMetadata,
    config: Arc<Value>,
    api: ApiManager,
}

impl MyPlugin {
    pub async fn new(config: Arc<Value>) -> Result<Self> {
        let metadata = PluginMetadata {
            name: "my_plugin".to_string(),
            describe: "我的第一个洛玖机器人插件".to_string(),
            author: "您的名字".to_string(),
            version: "0.1.0".to_string(),
            message_types: vec![
                "group_message".to_string()
            ],
        };
        
        // 初始化api
        let api = match ApiManager::new(config.clone()) {
            Ok(api) => api,
            Err(e) => return Err(anyhow!("API初始化失败: {}", e)),
        }; 
        
        Ok(Self {
            metadata,
            config,
            api,
        })
    }
}

#[async_trait]
impl Plugin for MyPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    async fn handle_group_message(&self, message: &GroupMessage) -> Result<()> {
        if message.content.contains("hello") {
            match self.api.send_group_message(&message.group_id, "来自我的插件的问候！").await {
                Ok(_) => {}
                Err(e) => eprintln!("发送群消息失败：{}", e),
            }
        }
        Ok(())
    }
}

async fn create(config: Arc<Value>) -> Result<Box<dyn Plugin>> {
    let plugin = MyPlugin::new(config).await?;
    Ok(Box::new(plugin))
}

export_plugin!(create);

```

## 项目结构
一个典型的洛玖机器人插件项目结构如下：

```plaintext
my_plugin/
├── src/
│   ├── lib.rs
│   └── ... (其他源文件)
└── Cargo.toml
```

您的Cargo.toml应该包含：

```toml
[package]
name = "my_plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
luo9_sdk = "1.0.0"
async-trait = "0.1"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 构建您的插件
使用以下命令构建您的插件：

```bash
cargo build --release
```

编译后的插件将位于
target/release/my_plugin.dll（Windows）
target/release/libmy_plugin.so（Linux）或 target/release/## libmy_plugin.dylib（macOS）

将编译后的插件文件复制到洛玖机器人安装目录的plugins/my_plugin/目录下。

## 消息类型
SDK为不同的消息格式提供了结构化类型：

- GroupMessage：在群聊中接收的消息
- PrivateMessage：来自用户的私聊消息
- 以及更多...

## API管理器
ApiManager提供了与机器人框架交互的方法：

- send_group_message：向群组发送消息
- send_private_msg：向用户发送私聊消息
- send_group_ai_record：发送AI生成的语音消息
- send_group_image：向群组发送图片
- 以及更多...

## 示例插件

查看 **[示例插件](https://www.github.com/luoy-oss/luo9_exapmle_plugin)** 以获取更全面的演示。

## 文档
有关详细文档，请访问**[https://www.drluo.top/posts/luo9_sdk](https://www.drluo.top/posts/luo9_sdk)**

## 许可证
本项目采用**[GNU通用公共许可证v3.0](https://www.gnu.org/licenses/gpl-3.0)**许可
