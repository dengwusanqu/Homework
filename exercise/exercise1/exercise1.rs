use std::io;

struct Buffer<T> {
    data: Vec<T>,
}

impl<T> Buffer<T>
where T: std::ops::Add<Output=T>+Default+Clone,
{
    fn new()->Self {
        Buffer{ 
            data: Vec::new() 
        }
    }

    fn push(&mut self, item:T) {
        self.data.push(item);
    }

    fn sum(&self)->T{
        let mut sum=T::default();
        for item in &self.data{
            sum=sum+item.clone();
        }
        sum
    }    
}

fn main() {
    let mut buffer=Buffer::new();

    loop {
        println!("请输入一个实数 或输入 'end' 退出: ");
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let trimmed=input.trim();
        if trimmed=="end" {
            break;
        }
        
        match trimmed.parse::<f64>() {
            Ok(num)=>{
                buffer.push(num);
            }
            Err(_)=>{
                println!("无效的输入，请重新输入一个实数。");
            }
        }
    }

    let result=buffer.sum();
    println!("Sum: {}", result);
}