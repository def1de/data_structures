pub struct Stack {
    top: isize,
    stack: Vec<isize>,
    max_size: isize
}

#[allow(dead_code)]
pub trait StackTrait {
    fn new(size: usize) -> Self;
    fn push(&mut self, item: isize) -> Option<()>;
    fn pop(&mut self) -> Option<isize>;
    fn print_stack(&mut self);
}

impl StackTrait for Stack {
    fn new(size: usize) -> Self {
        Stack {
            top: -1,
            stack: Vec::with_capacity(size),
            max_size: size as isize -1
        }
    }

    fn push(&mut self, item: isize) -> Option<()> {
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

    fn pop(&mut self) -> Option<isize> {
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