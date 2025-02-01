use std::fmt::Display;

pub struct Stack<T> {
    top: isize,
    stack: Vec<T>,
    max_size: isize
}

#[allow(dead_code)]
pub trait StackTrait<T> {
    fn new(size: usize) -> Self;
    fn push(&mut self, item: T) -> Option<()>;
    fn pop(&mut self) -> Option<T>;
    fn print_stack(&mut self);
}

impl<T> StackTrait<T> for Stack<T>
where
    T: Copy + Display,
{
    fn new(size: usize) -> Self {
        Stack {
            top: -1,
            stack: Vec::with_capacity(size),
            max_size: size as isize -1
        }
    }

    fn push(&mut self, item: T) -> Option<()> {
        if self.top >= self.max_size{
            return None
        }
        self.top+=1;
        if self.top as usize >= self.stack.len(){
            self.stack.push(item);
        } else {
            self.stack[self.top as usize] = item;
        }
        return Some(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == -1 {
            return None
        }
        let return_value = self.stack[self.top as usize];
        self.top-=1;
        return Some(return_value);
    }

    fn print_stack(&mut self) {
        println!("\n");
        for i in (0..=self.top).rev(){
            println!("{} | {}", i, self.stack[i as usize]);
        }
    }
}