use std::cell::Cell;

struct MyRc<T>{
    data:Option<Box<T>>,
    count:Cell<usize>,
}

impl<T:Clone> MyRc<T>{
    fn new(value:T)->Self{
        Self{
            data:Some(Box::new(value)),
            count:Cell::new(1),
        }
    }

    fn clone(&self)->Self{
        self.count.set(self.count.get()+1);
        Self {
            data:self.data.clone(),
            count:Cell::new(self.count.get()),
        }
    }

    fn strong_count(&self)->usize{
        self.count.get()
    }

    fn deref(&self)->&T{
        self.data.as_ref().unwrap()
    }
}

impl<T> Drop for MyRc<T>{
    fn drop(&mut self){
        let count=self.count.get()-1;
        self.count.set(count);

        if count==0{
            self.data = None;
        }
    }
}

fn main() {
    let five=MyRc::new(5);
    let fivel=five.clone();

    println!("{}", fivel.deref());
    println!("{}", fivel.strong_count());
}