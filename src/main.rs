mod stack;
mod queue;
use stack::{Stack, StackTrait};
use queue::{Queue, QueueTrait};
use std::fmt;

#[derive(Copy, Clone)]
struct TestStruct {
    data: i32
}

impl fmt::Display for TestStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn main() {
    let mut stack: Stack<TestStruct> = Stack::new(10);
    stack.push(TestStruct{
        data: 1,
    });
    stack.push(TestStruct{
        data: 1,
    });
    stack.push(TestStruct{
        data: 1,
    });
    stack.push(TestStruct{
        data: 1,
    });
    stack.push(TestStruct{
        data: 1,
    });
    stack.print_stack();
}
