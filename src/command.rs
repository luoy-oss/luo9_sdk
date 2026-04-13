pub enum PrefixMode {
    Required(char),      // 必须有指定前缀
    Optional(char),      // 可选前缀（如果有前缀则移除，没有则直接解析）
    None,                // 无前缀
}

#[derive(Debug)]
pub struct Command {
    pub raw: String,        // 原始字符串 "/echo hello world"（保留原始格式）
    pub name: String,       // 命令名，如 "echo"
    pub args: Vec<String>,  // 按空白分割的参数列表
    pub args_raw: String,   // 去掉命令名后的原始内容（未分割，保留原样包括空格）
    pub prefix: char,       // '/' 或 '\0'（表示无前缀）
}

impl Command {
    pub fn parse(msg: &str, cmd_name: &str, mode: PrefixMode) -> Option<Self> {
        let raw = msg.to_string();
        let trimmed = raw.trim();
        
        if trimmed.is_empty() {
            return None;
        }
        
        // 1. 根据前缀模式，提取实际内容（去掉前缀）
        let (content, actual_prefix, _prefix_len_in_trimmed) = match mode {
            PrefixMode::Required(p) => {
                if !trimmed.starts_with(p) {
                    return None;
                }
                let prefix_len = p.len_utf8();
                let after_prefix = &trimmed[prefix_len..];
                (after_prefix.trim_start(), p, prefix_len)
            }
            PrefixMode::Optional(p) => {
                if trimmed.starts_with(p) {
                    let prefix_len = p.len_utf8();
                    let after_prefix = &trimmed[prefix_len..];
                    (after_prefix.trim_start(), p, prefix_len)
                } else {
                    (trimmed, '\0', 0)
                }
            }
            PrefixMode::None => {
                (trimmed, '\0', 0)
            }
        };
        
        // 检查是否以命令名开头
        if !content.starts_with(cmd_name) {
            return None;
        }
        
        //    计算在原始 trimmed 字符串中，命令名结束的位置
        //    需要考虑前缀和命令名前的空格
        let trimmed_parts: Vec<&str> = trimmed.splitn(2, |c: char| !c.is_whitespace()).collect();
        if trimmed_parts.is_empty() {
            return None;
        }

        let raw_trimmed_start = raw.find(trimmed).unwrap_or(0);
        
        // 计算前缀在 raw 中的长度
        let prefix_part_len = if actual_prefix != '\0' {
            // 在 trimmed 中，前缀在开头，但 raw 中可能有前导空格
            // 找到 trimmed 在 raw 中的位置，前缀就是从那里开始的
            // let trimmed_start_in_raw = raw.find(trimmed).unwrap_or(0);
            // 前缀字符在 trimmed 中的位置是 0
            // 所以在 raw 中，前缀的位置就是 trimmed_start_in_raw
            // 前缀长度就是实际前缀字符的长度
            actual_prefix.len_utf8()
        } else {
            0
        };
        
        // 命令名在 trimmed 中的起始位置
        let cmd_start_in_trimmed = content.find(cmd_name).unwrap_or(0);
        
        // 命令名在 raw 中的起始位置
        let cmd_start_in_raw = raw_trimmed_start + prefix_part_len + cmd_start_in_trimmed;
        
        // 命令名在 raw 中的结束位置
        let cmd_end_in_raw = cmd_start_in_raw + cmd_name.len();
        
        // 提取命令名后面的所有内容（保留原始格式，包括所有空格）
        let args_raw = if cmd_end_in_raw < raw.len() {
            raw[cmd_end_in_raw..].to_string()
        } else {
            String::new()
        };
        
        // 分割参数：去掉前导空白后按空白分割
        let trimmed_args = args_raw.trim_start();
        let args: Vec<String> = if trimmed_args.is_empty() {
            Vec::new()
        } else {
            trimmed_args.split_whitespace().map(|s| s.to_string()).collect()
        };
        
        Some(Command {
            raw,
            name: cmd_name.to_string(),
            args,
            args_raw,
            prefix: actual_prefix,
        })
    }
    
    pub fn args_raw(&self) -> &str {
        &self.args_raw
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
    fn test_optional_prefix_echo() {
        // 带前缀
        let cmd = Command::parse("/echo hello world", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.name, "echo");
        assert_eq!(cmd.args_raw, " hello world");
        assert_eq!(cmd.args, vec!["hello", "world"]);
        assert_eq!(cmd.prefix, '/');
        
        // 不带前缀
        let cmd = Command::parse("echo hello world", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.name, "echo");
        assert_eq!(cmd.args_raw, " hello world");
        assert_eq!(cmd.args, vec!["hello", "world"]);
        assert_eq!(cmd.prefix, '\0');
    }

    #[test]
    fn test_no_spaces_between_cmd_and_args() {
        // echo内容 -> "内容"
        let cmd = Command::parse("echo内容", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "内容");
        assert_eq!(cmd.args, vec!["内容"]);
        
        // /echo内容
        let cmd = Command::parse("/echo内容", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "内容");
        assert_eq!(cmd.args, vec!["内容"]);
    }

    #[test]
    fn test_multiple_spaces() {
        // echo{0-n个空格}内容
        let cmd = Command::parse("echo    内容", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "    内容");
        assert_eq!(cmd.args, vec!["内容"]);
        
        // echo{0-n个空格}内容1{0-n个空格}内容2
        let cmd = Command::parse("echo   内容1    内容2", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "   内容1    内容2");
        assert_eq!(cmd.args, vec!["内容1", "内容2"]);
    }

    #[test]
    fn test_mixed_without_spaces() {
        // echo内容1内容2（没有空格，视为一个整体）
        let cmd = Command::parse("echo内容1内容2", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "内容1内容2");
        assert_eq!(cmd.args, vec!["内容1内容2"]);
        
        // echo内容1 内容2（有空格，拆分成两个）
        let cmd = Command::parse("echo内容1 内容2", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "内容1 内容2");
        assert_eq!(cmd.args, vec!["内容1", "内容2"]);
    }

    #[test]
    fn test_command_boundary() {
        assert!(Command::parse("echo123", "echo", PrefixMode::Optional('/')).is_some());
        assert!(Command::parse("echo 123", "echo", PrefixMode::Optional('/')).is_some());
        assert!(Command::parse("echo内容", "echo", PrefixMode::Optional('/')).is_some());
        assert!(Command::parse("echoabc", "echo", PrefixMode::Optional('/')).is_some());
        assert!(Command::parse("echo abc", "echo", PrefixMode::Optional('/')).is_some());
    }

    #[test]
    fn test_required_prefix() {
        let cmd = Command::parse("/echo test", "echo", PrefixMode::Required('/')).unwrap();
        assert_eq!(cmd.args_raw, " test");
        
        // 没有前缀应该失败
        assert!(Command::parse("echo test", "echo", PrefixMode::Required('/')).is_none());
    }

    #[test]
    fn test_none_prefix() {
        let cmd = Command::parse("echo test", "echo", PrefixMode::None).unwrap();
        assert_eq!(cmd.args_raw, " test");
        
        // 带前缀应该失败
        assert!(Command::parse("/echo test", "echo", PrefixMode::None).is_none());
    }

    #[test]
    fn test_empty_args() {
        let cmd = Command::parse("/echo", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "");
        assert!(cmd.args.is_empty());
        assert!(!cmd.has_args());
    }

    #[test]
    fn test_whitespace_preservation() {
        let cmd = Command::parse("  /echo   hello   world  ", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.raw, "  /echo   hello   world  ");
        assert_eq!(cmd.args_raw, "   hello   world  ");  // 保留所有空格，包括尾随空格
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }
    
    #[test]
    fn test_first_arg() {
        let cmd = Command::parse("/echo arg1 arg2", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.arg_at(0), Some(&"arg1".to_string()));
        assert_eq!(cmd.arg_at(1), Some(&"arg2".to_string()));
        assert_eq!(cmd.arg_at(2), None);
    }
    
    #[test]
    fn test_raw_preservation() {
        let test_cases = vec![
            "/echo",
            "  /echo  ",
            "/echo arg1  arg2",
            "  /echo  arg1    arg2  ",
        ];
        
        for input in test_cases {
            let cmd = Command::parse(input, "echo", PrefixMode::Required('/')).unwrap();
            assert_eq!(cmd.raw, input);
        }
    }
    
    #[test]
    fn test_edge_cases() {
        // 命令名后面直接跟内容，没有空格
        let cmd = Command::parse("/echohello", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "hello");
        assert_eq!(cmd.args, vec!["hello"]);
        
        // 只有命令名
        let cmd = Command::parse("/echo", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.args_raw, "");
        assert!(cmd.args.is_empty());
    }
}