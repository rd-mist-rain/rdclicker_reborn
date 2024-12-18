use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, mouse_event, GetAsyncKeyState};

mod read_config;
mod orders;
mod config;
use orders::{RootOrders, ConfigOrders};

fn main() {
    println!("rdclicker_reborn version 0.6.1");
    let ((reset, red, yellow, green), lmode, rmode, allow_ansi) = crate::config::config();

    /*---------------核心部分--------------- */
    let ticks = Arc::new(AtomicU64::new(100));

    let lclone = Arc::clone(&ticks);
    let lclick = Arc::new(AtomicBool::new(false));
    let lclick_clone = Arc::clone(&lclick);
    
    match lmode {
        0 => {
            drop(lclick);
            drop(lclick_clone);
            thread::spawn(move || {
                loop {
                    if unsafe { GetAsyncKeyState(162) } < 0 {
                        unsafe {
                            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                        }
                    }
                    thread::sleep(Duration::from_millis(lclone.load(Ordering::Acquire)));
                }
            });
        },
        1 => {
            thread::spawn(move || {
                loop {
                    if unsafe { GetAsyncKeyState(162) } < 0 {
                        lclick.store(true, Ordering::Release);
                        thread::sleep(Duration::from_millis(200));
                        loop {
                            if unsafe { GetAsyncKeyState(162) } < 0 {
                                lclick.store(false, Ordering::Release);
                                thread::sleep(Duration::from_millis(200));
                                break;
                            }
                            thread::sleep(Duration::from_millis(100));
                        }
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            });

            thread::spawn(move || {
                loop {
                    if lclick_clone.load(Ordering::Acquire) {
                        unsafe {
                            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                        }
                    }
                    thread::sleep(Duration::from_millis(lclone.load(Ordering::Acquire)));
                }
            });
        },
        _ => {}
    }

    let rclone = Arc::clone(&ticks);
    let rclick = Arc::new(AtomicBool::new(false));
    let rclick_clone = Arc::clone(&rclick);

    match rmode {
        0 => {
            drop(rclick);
            drop(rclick_clone);
            thread::spawn(move || {
                loop {
                    if unsafe { GetAsyncKeyState(163) } < 0 {
                        unsafe {
                            mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                            mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                        }
                    }

                    thread::sleep(Duration::from_millis(rclone.load(Ordering::Acquire)));
                }
            });
        },
        1 => {
            thread::spawn(move || {
                loop {
                    if unsafe { GetAsyncKeyState(163) } < 0 {
                        rclick.store(true, Ordering::Release);
                        thread::sleep(Duration::from_millis(200));
                        loop {
                            if unsafe { GetAsyncKeyState(163) } < 0 {
                                rclick.store(false, Ordering::Release);
                                thread::sleep(Duration::from_millis(200));
                                break;
                            }
                            thread::sleep(Duration::from_millis(100));
                        }
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            });

            thread::spawn(move || {
                loop {
                    if rclick_clone.load(Ordering::Acquire) {
                        unsafe {
                            mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                            mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                        }
                    }
                    thread::sleep(Duration::from_millis(rclone.load(Ordering::Acquire)));
                }
            });
        },
        _ => {}
    }

    loop {
        if unsafe { GetAsyncKeyState(191) } < 0 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("读取输入失败");
            let orders: RootOrders = crate::orders::serve(input, (reset, red, yellow, green));
            match orders {
                RootOrders::Help => {
                    println!("指令列表--/");
                    println!("config [子指令]   用于在程序内部编辑某些配置");
                }
                RootOrders::Config(sub_orders) => {
                    match sub_orders {
                        ConfigOrders::Help => {
                            println!("指令列表--config");
                            println!("ticks [数字]    用于设置鼠标点击的间隔时间(单位:毫秒)");
                            println!("show    显示所有程序配置");
                        }
                        ConfigOrders::Show => {
                            println!("配置列表:");
                            println!("allow_ansi 是否允许ANSI转义字符={}", allow_ansi);
                            println!("ticks={}", ticks.load(Ordering::Acquire));
                            println!("左键-线程1点击模式:{}", lmode);
                            println!("右键-线程2点击模式:{}", rmode);
                        }
                        ConfigOrders::Ticks(new_ticks) => {
                            println!("成功操作:更新ticks为{}", new_ticks);
                            ticks.store(new_ticks, Ordering::Release);
                        }
                        ConfigOrders::Error => {}
                    }
                }
                RootOrders::Error => {}
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
