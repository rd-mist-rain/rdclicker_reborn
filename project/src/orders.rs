pub enum RootOrders {
    Help,
    Config(ConfigOrders),
    Error,
}

pub enum ConfigOrders { // Root->Config
    Help,
    Ticks(u64),
    Show,
    Error,
}

pub fn serve(input: String,(reset,red,yellow,green):(&str,&str,&str,&str)) -> RootOrders {
    let orders_str: Vec<&str> = input.split_whitespace().collect();
    let len = orders_str.len();

    match orders_str[0] { // 根指令
        "/help" => {
            return RootOrders::Help;
        }
        "/config" => {
            if len >= 2 { // 满足1级子指令长度
                match orders_str[1] { // 1级子指令
                    "help" => { return RootOrders::Config(ConfigOrders::Help);}
                    "ticks" => {
                        if len >= 3 { // 满足2级子指令长度
                            match orders_str[2].trim().parse::<u64>() {
                                Ok(new_ticks) => {return RootOrders::Config(ConfigOrders::Ticks(new_ticks))},
                                Err(e) => 
                                {
                                    println!("{red}错误{reset}:{}{yellow}\n通常地,参数应该是一个数字。这个错误有可能是因为无效参数导致的。\n你可以重新输入指令。{reset}",e);
                                    return RootOrders::Config(ConfigOrders::Error);
                                }
                            }
                        }
                        else
                        {   
                            println!("{red}错误:{reset}/config ticks缺失一个参数");
                            return RootOrders::Config(ConfigOrders::Error);
                        }
                    }
                    "show" => {return RootOrders::Config(ConfigOrders::Show);}
                    other => 
                    {
                        println!("{red}错误:{reset}未知的命令{other}");
                        return RootOrders::Config(ConfigOrders::Error);
                    }
                }
            }
            else
            {   
                println!("{red}错误:{reset}/config缺失一个子指令");
                return RootOrders::Config(ConfigOrders::Error);
            }
        }
        other => 
        {
            println!("{red}错误:{reset}未知的命令{other}");
            return RootOrders::Error;
        }
    }
}
