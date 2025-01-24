mod stack;
mod queue;
// use stack::{Stack, StackTrait};
use queue::{Queue, QueueTrait};

fn main() {
    let mut queue = Queue::new(10);

    queue.enquue(1);
    queue.enquue(2);
    queue.dequeue();
    queue.enquue(3);
    queue.enquue(3);
    queue.enquue(3);
    queue.enquue(3);
    queue.dequeue();
    queue.dequeue();
    queue.dequeue();
    queue.enquue(1);
    queue.enquue(1);
    queue.enquue(1);
    queue.enquue(1);
    queue.print_queue();
}
