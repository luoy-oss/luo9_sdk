//! 宏定义
//! 
//! 这个模块定义了一些有用的宏。

/// 注册插件的宏
#[macro_export]
macro_rules! register_plugin {
    ($name:expr, $factory:expr) => {
        #[cfg(feature = "plugin_registry")]
        #[ctor::ctor]
        fn register_plugin() {
            let mut registry = $crate::plugin_registry::PLUGIN_REGISTRY.lock().unwrap();
            registry.register($name, $factory);
        }
    };
}

/// 导出插件创建函数的宏
#[macro_export]
macro_rules! export_plugin {
    ($create_fn:expr) => {
        #[no_mangle]
        pub extern "C" fn create_plugin(
            config: std::sync::Arc<luo9_sdk::config::Value>,
        ) -> anyhow::Result<Box<dyn luo9_sdk::plugin::Plugin>> {
            // 检查是否在 Tokio 运行时上下文中
            match tokio::runtime::Handle::try_current() {
                // 如果在 Tokio 运行时上下文中，直接使用当前运行时
                Ok(handle) => handle.block_on($create_fn(config)),
                // 如果不在 Tokio 运行时上下文中，创建一个新的运行时
                Err(_) => {
                    // 创建一个新的 Tokio 运行时
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .expect("无法创建 Tokio 运行时");
                    
                    // 在新创建的运行时中执行异步函数
                    rt.block_on($create_fn(config))
                }
            }
        }
    };
}