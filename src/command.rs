pub enum PrefixMode {
    Required(char),      // 必须有指定前缀
    Optional(char),      // 可选前缀（如果有前缀则移除，没有则直接解析）
    None,                // 无前缀
}

pub struct Command {
    pub raw: String,        // 原始字符串 "/help arg1 arg2"（保留原始格式）
    pub name: String,       // "help"
    pub args: Vec<String>,  // ["arg1", "arg2"]
    pub prefix: char,       // '/' 或 '\0'（表示无前缀）
}

impl Command {
    /// 解析命令，支持多种前缀模式
    pub fn parse(msg: &str, mode: PrefixMode) -> Option<Self> {
        // 转换为 String 进行后续处理
        let raw = msg.to_string();
        let trimmed = raw.trim();
        
        if trimmed.is_empty() {
            return None;
        }
        
        match mode {
            PrefixMode::Required(p) => {
                if !trimmed.starts_with(p) {
                    return None;
                }
                Self::parse_with_prefix(raw, p)
            }
            PrefixMode::Optional(p) => {
                if trimmed.starts_with(p) {
                    Self::parse_with_prefix(raw, p)
                } else {
                    Self::parse_without_prefix(raw)
                }
            }
            PrefixMode::None => {
                Self::parse_without_prefix(raw)
            }
        }
    }
    
    /// 解析带前缀的命令
    fn parse_with_prefix(raw: String, prefix: char) -> Option<Self> {
        // 先 trim 处理逻辑，但保留原始 raw
        let trimmed = raw.trim();
        let without_prefix = &trimmed[prefix.len_utf8()..];
        let trimmed_cmd = without_prefix.trim_start();
        
        if trimmed_cmd.is_empty() {
            return None;
        }
        
        let mut parts = trimmed_cmd.split_whitespace();
        let name = parts.next()?.to_string();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();
        
        Some(Command {
            raw,
            name,
            args,
            prefix,
        })
    }
    
    /// 解析无前缀的命令
    fn parse_without_prefix(raw: String) -> Option<Self> {
        let trimmed = raw.trim();
        let mut parts = trimmed.split_whitespace();
        let name = parts.next()?.to_string();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();
        
        Some(Command {
            raw,
            name,
            args,
            prefix: '\0',
        })
    }
    
    pub fn args_raw(&self) -> String {
        self.args.join(" ")
    }
    
    pub fn has_args(&self) -> bool {
        !self.args.is_empty()
    }
    
    pub fn arg_at(&self, index: usize) -> Option<&String> {
        self.args.get(index)
    }
    
    pub fn has_prefix(&self) -> bool {
        self.prefix != '\0'
    }
    
    pub fn prefix_char(&self) -> Option<char> {
        if self.has_prefix() {
            Some(self.prefix)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_prefix() {
        let cmd = Command::parse("/help test", PrefixMode::Required('/')).unwrap();
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.args, vec!["test"]);
        assert_eq!(cmd.prefix, '/');
        
        assert!(Command::parse("help test", PrefixMode::Required('/')).is_none());
    }

    #[test]
    fn test_optional_prefix() {
        // 带前缀
        let cmd = Command::parse("/help test", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.prefix, '/');
        
        // 不带前缀
        let cmd = Command::parse("help test", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.prefix, '\0');
    }

    #[test]
    fn test_no_prefix() {
        let cmd = Command::parse("help test", PrefixMode::None).unwrap();
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.args, vec!["test"]);
        assert!(!cmd.has_prefix());
    }

    #[test]
    fn test_multiple_args() {
        let cmd = Command::parse("/echo hello world 123", PrefixMode::Required('/')).unwrap();
        assert_eq!(cmd.name, "echo");
        assert_eq!(cmd.args, vec!["hello", "world", "123"]);
        assert_eq!(cmd.args_raw(), "hello world 123");
        assert_eq!(cmd.arg_at(0), Some(&"hello".to_string()));
        assert_eq!(cmd.arg_at(2), Some(&"123".to_string()));
    }

    #[test]
    fn test_whitespace_handling() {
        let cmd = Command::parse("  /help   test   ", PrefixMode::Required('/')).unwrap();
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.args, vec!["test"]);
        assert_eq!(cmd.raw, "  /help   test   ");
    }

    #[test]
    fn test_empty_command() {
        assert!(Command::parse("", PrefixMode::Required('/')).is_none());
        assert!(Command::parse("   ", PrefixMode::Optional('/')).is_none());
        assert!(Command::parse("/", PrefixMode::Required('/')).is_none());
        assert!(Command::parse("/   ", PrefixMode::Required('/')).is_none());
    }
    
    #[test]
    fn test_raw_preservation() {
        // 测试各种原始格式的保留
        let test_cases = vec![
            "/cmd",
            "  /cmd  ",
            "/cmd arg1  arg2",
            "  /cmd  arg1    arg2  ",
        ];
        
        for input in test_cases {
            let cmd = Command::parse(input, PrefixMode::Required('/')).unwrap();
            assert_eq!(cmd.raw, input);
        }
    }
}
