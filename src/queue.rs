pub struct Queue {
    front: usize,
    rear: usize,
    queue: Vec<isize>,
    max_size: usize
}

#[allow(dead_code)]
pub trait QueueTrait {
    fn new(size: usize) -> Self;
    fn enquue(&mut self, item: isize) -> Option<()>;
    fn dequeue(&mut self) -> Option<isize>;
    fn print_queue(&mut self);
}

impl QueueTrait for Queue {
    fn new(size: usize) -> Self {
        Queue {
            front: 0,
            rear: 1,
            queue: Vec::with_capacity(size),
            max_size: size-1
        }
    }

    fn enquue(&mut self, item: isize) -> Option<()> {
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

    fn dequeue(&mut self) -> Option<isize> {
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