use std::cell::RefCell;

struct SimpleStack<T>{
    stack:RefCell<Vec<T>>,
}

impl<T> SimpleStack<T>{
    fn new()->SimpleStack<T>{
        SimpleStack{
            stack:RefCell::new(Vec::new()),
        }
    }

    fn push(&self,value:T){
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self)->Option<T>{
        self.stack.borrow_mut().pop()
    }
}

fn main(){
    let stack=SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Popped value: {:?}",stack.pop());
    println!("Popped value: {:?}",stack.pop());

    stack.push(4);

    println!("Popped value: {:?}",stack.pop());
    println!("Popped value: {:?}",stack.pop());
    println!("Popped value: {:?}",stack.pop());
}