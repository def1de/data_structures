use std::fmt::Display;

pub struct Queue<T> {
    front: usize,
    rear: usize,
    queue: Vec<T>,
    max_size: usize
}

#[allow(dead_code)]
pub trait QueueTrait<T> {
    fn new(size: usize) -> Self;
    fn enquue(&mut self, item: T) -> Option<()>;
    fn dequeue(&mut self) -> Option<T>;
    fn print_queue(&mut self);
}

impl<T> QueueTrait<T> for Queue<T>
where
    T: Copy + Display,
{
    fn new(size: usize) -> Self {
        Queue {
            front: 0,
            rear: 1,
            queue: Vec::with_capacity(size),
            max_size: size-1
        }
    }

    fn enquue(&mut self, item: T) -> Option<()> {
        let next_rear = (self.rear + 1) % self.max_size;
        if next_rear == self.front {
            println!("Queue is full!");
            return None
        }
        if self.rear > self.queue.len(){
            self.queue.push(item);
        } else {
            self.queue[self.rear] = item;
        }
        self.rear = next_rear;
        return Some(());
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.front == self.rear {
            println!("Queue is empty!");
            return None
        }
        let item = self.queue[self.front];
        self.front = (self.front+1)%self.max_size;
        return Some(item);
    }

    fn print_queue(&mut self) {
        println!("Queue length {}", self.queue.len());
        for i in 0..self.queue.len() {
            print!("| {} ", self.queue[i])
        }
        println!("|");
        for i in 0..self.queue.len() {
            match i {
                _ if i == self.front => print!("| f "),
                _ if i == self.rear => print!("| r "),
                _ => print!("|   "),
            }
        }
        println!("|");
    }
}