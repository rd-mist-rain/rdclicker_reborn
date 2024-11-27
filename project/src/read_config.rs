use std::fs::File;
use std::io::Read;
use toml;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config
{
    pub allow_ansi:bool,
    pub left_mode:u8,
    pub right_mode:u8
}
pub fn read_config() -> Result<Config,Box<dyn std::error::Error>>{
    // 打开 TOML 文件
    let mut file = File::open("configs.toml")?;
    
    // 读取文件内容
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 反序列化 TOML 文件内容为 Config 结构体
    let configs: Config = toml::from_str(&contents)?;
    Ok(configs)
}