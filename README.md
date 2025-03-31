# Luo9 SDK

[![Crates.io](https://img.shields.io/crates/v/luo9_sdk.svg)](https://crates.io/crates/luo9_sdk)
[![Documentation](https://img.shields.io/badge/docs-drluo.top\|luo9_sdk-blue.svg)](https://www.drluo.top/posts/luo9_sdk)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A comprehensive SDK for developing plugins for Luo9 Bot, a versatile messaging bot framework.

## Features

* Easy-to-use API for handling various message types
* Support for group and private messages
* Built-in event handling for common interactions
* Asynchronous API design with Tokio runtime
* Simplified configuration management
* Cross-platform compatibility

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
luo9_sdk = "1.0.0"
```

## Quick Start
Here's a minimal example of a Luo9 Bot plugin:

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
            describe: "My first Luo9 Bot plugin".to_string(),
            author: "Your Name".to_string(),
            version: "0.1.0".to_string(),
            message_types: vec![
                "group_message".to_string()
            ],
        };
        
        // init api
        let api = match ApiManager::new(config.clone()) {
            Ok(api) => api,
            Err(e) => return Err(anyhow!("API initialization failed: {}", e)),
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
            match self.api.send_group_message(&message.group_id, "Hello from my plugin!").await {
                Ok(_) => {}
                Err(e) => eprintln!("send group message failed:{}", e),
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

## Project Structure
A typical Luo9 Bot plugin project structure looks like this:

```plaintext
my_plugin/
├── src/
│   ├── lib.rs
│   └── ... (other source files)
└── Cargo.toml
```

Your Cargo.toml should include:

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

## Building Your Plugin
Build your plugin with:

```bash
cargo build --release
```

The compiled plugin will be located at
target/release/my_plugin.dll (Windows)
target/release/libmy_plugin.so (Linux), or target/release/libmy_plugin.dylib (macOS).

Copy the compiled plugin file to the plugins/my_plugin/ directory of your Luo9 Bot installation.

## Message Types
The SDK provides structured types for different message formats:

- GroupMessage : Messages received in group chats
- PrivateMessage : Direct messages from users
- And more...

## API Manager
The ApiManager provides methods to interact with the bot framework:

- send_group_message : Send a message to a group
- send_private_msg : Send a direct message to a user
- send_group_ai_record : Send AI-generated voice messages
- send_group_image : Send images to a group
- And more...

## Example Plugin
Check out the **[example plugin](https://www.github.com/luoy-oss/luo9_exapmle_plugin)** for a more comprehensive demonstration.

## Documentation
For detailed documentation, visit **[https://www.drluo.top/posts/luo9_sdk](https://www.drluo.top/posts/luo9_sdk)**

## License
This project is licensed under the **[GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0)**