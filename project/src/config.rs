pub fn config()->((&'static str,&'static str,&'static str,&'static str),u8,u8,bool)
{
    let reset: &str;
    let red: &str;
    let yellow: &str;
    let green: &str;
    let lmode:u8; // 0长按 1单击切换
    let rmode:u8;
    let allow_ansi:bool; 
    match crate::read_config::read_config() {
        Ok(configs) => {
            if configs.allow_ansi { allow_ansi=true; } 
            else { allow_ansi=false; }
            lmode=configs.left_mode;
            rmode=configs.right_mode;
        },
        Err(e) => {
            eprintln!("Failed to read configs: {}", e);
            allow_ansi=false;
            lmode=0;
            rmode=0;
        }
    }
    if allow_ansi
    {
        reset = "\x1b[0m";
        red = "\x1b[31m";
        yellow = "\x1b[33m";
        green = "\x1b[32m";
    }
    else 
    {
        reset="";
        red="";
        yellow="";
        green="";
    }
    return ((reset,red,yellow,green),lmode,rmode,allow_ansi);
}
