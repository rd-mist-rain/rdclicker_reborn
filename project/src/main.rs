use std::sync::atomic::{AtomicU64, Ordering,AtomicBool};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, mouse_event, GetAsyncKeyState};
use std::fs::File;
use std::io::Read;
use toml;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config
{
    allow_ansi:bool,
    left_mode:u8,
    right_mode:u8
}
fn read_config() -> Result<Config,Box<dyn std::error::Error>>{
    // 打开 TOML 文件
    let mut file = File::open("configs.toml")?;
    
    // 读取文件内容
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 反序列化 TOML 文件内容为 Config 结构体
    let configs: Config = toml::from_str(&contents)?;
    Ok(configs)
}
fn main() {
    println!("Release-O2: rdclicker_reborn with version 0.4.0");
    let reset: &str;
    let red: &str;
    let yellow: &str;
    let green: &str;
    let lmode:u8; // 0长按 1单击切换
    let rmode:u8;
    
    match read_config() {
        Ok(configs) => {
            if configs.allow_ansi 
            {
                reset = "\x1b[0m";
                red = "\x1b[31m";
                yellow = "\x1b[33m";
                green = "\x1b[32m";
            } 
            else 
            {
                reset = "";
                red = "";
                yellow = "";
                green = "";
            }
            lmode=configs.left_mode;
            rmode=configs.right_mode;
        },
        Err(e) => {
            eprintln!("Failed to read configs: {}", e);
            reset = "\x1b[0m";
            red = "\x1b[31m";
            yellow = "\x1b[33m";
            green = "\x1b[32m";
            lmode=0;
            rmode=0;
        }
    }

    // 创建原子变量 ticks
    let ticks = Arc::new(AtomicU64::new(100));

    // 启动一个线程处理鼠标左键点击事件
    let lclone = Arc::clone(&ticks);

    let lclick=Arc::new(AtomicBool::new(false));
    let lclick_clone=Arc::clone(&lclick);
    match lmode 
    {
        0=>{
        drop(lclick);
        drop(lclick_clone);    
        thread::spawn(move || {
            loop {
                if unsafe { GetAsyncKeyState(162) } < 0 { // 左Ctrl
                    unsafe {
                        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    }
                }
    
                // 从原子变量获取 ticks 的值
                thread::sleep(Duration::from_millis(lclone.load(Ordering::Acquire)));
            }
        });
        },
        1=>{
            thread::spawn(move || {
                loop {
                    if unsafe{GetAsyncKeyState(162)} < 0
                    {
                        lclick.store(true, Ordering::Relaxed);
                        thread::sleep(Duration::from_millis(200));
                        loop {
                            if unsafe{GetAsyncKeyState(162)} < 0
                            {
                                lclick.store(false, Ordering::Relaxed);
                                thread::sleep(Duration::from_millis(200));
                                break;
                            }
                        }
                    }
                }
            });
            thread::spawn(move || {
                loop {
                if lclick_clone.load(Ordering::Relaxed) 
                {
                    unsafe {
                        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    }
                    thread::sleep(Duration::from_millis(lclone.load(Ordering::Acquire)));
                }
                }
            });
        },
        _=>{}
    }

    // 启动另一个线程处理鼠标右键点击事件
    let rclone = Arc::clone(&ticks);
    let rclick=Arc::new(AtomicBool::new(false));
    
    let rclick_clone=Arc::clone(&rclick);
    match rmode 
    {
        0=>{
        drop(rclick);
        drop(rclick_clone);    
        thread::spawn(move || {
            loop {
                if unsafe { GetAsyncKeyState(163) } < 0 { // 右Ctrl
                    unsafe {
                        mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                        mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                    }
                }
    
                // 从原子变量获取 ticks 的值
                thread::sleep(Duration::from_millis(rclone.load(Ordering::Acquire)));
            }
        });
        },
        1=>{
            thread::spawn(move || {
                loop {
                    if unsafe{GetAsyncKeyState(163)} < 0
                    {
                        rclick.store(true, Ordering::Relaxed);
                        thread::sleep(Duration::from_millis(200));
                        loop {
                            if unsafe{GetAsyncKeyState(163)} < 0
                            {
                                rclick.store(false, Ordering::Relaxed);
                                thread::sleep(Duration::from_millis(200));
                                break;
                            }
                        }
                    }
                }
            });
            thread::spawn(move || {
                loop {
                if rclick_clone.load(Ordering::Relaxed) 
                {
                    unsafe {
                        mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                        mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                    }
                    thread::sleep(Duration::from_millis(rclone.load(Ordering::Acquire)));
                }
                }
            });
        },
        _=>{}
    }

    // 主线程负责更新 ticks 变量
    loop 
    {
        if unsafe { GetAsyncKeyState(18) } < 0 { // Alt
            let mut input = String::new();

        // 使用 expect 直接处理读取输入
            std::io::stdin().read_line(&mut input).expect("读取输入失败");

        // 尝试解析输入值为u64类型
            match input.trim().parse::<u64>() {
                Ok(new_ticks) => {
                // 更新原子变量的值
                    println!("{green}成功操作{reset}:更新ticks为{new_ticks}");
                    ticks.store(new_ticks, Ordering::Release);
                },
                Err(e) => {
                // 输入无效提醒用户
                    println!("{red}错误{reset}:{}{yellow}\n通常地,输入应该是一个数字。这个错误有可能是因为无效输入导致的。\n你可以再次按下Alt并重新输入。{reset}"
                            ,e);
                },
            }
        }
    }
}
