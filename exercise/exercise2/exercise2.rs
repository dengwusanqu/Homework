use std::io;

fn compare_str(x:&str,y:&str)->bool{
    let mut x_chars=x.chars();
    let mut y_chars=y.chars();

    loop {
        match(x_chars.next(), y_chars.next()){
            (Some(x_char),Some(y_char))=>{
                if x_char<y_char{
                    return false;
                } else if x_char>y_char{
                    return true;
                }
            }
            (Some(_), None)=>return true,
            (None, Some(_))=>return false,
            (None, None)=>return false,
        }
    }
}

fn main() {
    println!("请输入字符串 x: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("读取输入失败");

    println!("请输入字符串 y: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("读取输入失败");

    let result = compare_str(&x, &y);
    println!("{}", result);
}