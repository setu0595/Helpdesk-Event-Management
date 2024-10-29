use tokio::sync::mpsc;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

#[derive(Eq, PartialEq, Debug, Clone)] // Add Clone to derive
pub struct Event {
    pub priority: u8,
    pub description: String,
}

// Ensure EventQueue is defined to manage the event queue
#[derive(Clone)] // Add Clone to derive
pub struct EventQueue {
    queue: Arc<Mutex<BinaryHeap<Event>>>, // Store the queue in an Arc<Mutex>
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
        }
    }

    pub fn push(&self, event: Event) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(event);
    }

    pub fn pop(&self) -> Option<Event> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop()
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Reverse to make it a max-heap
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub async fn handle_events(mut rx: mpsc::Receiver<Event>, event_queue: EventQueue) {
    // Listen for new events from the channel
    while let Some(event) = rx.recv().await {
        println!("Received event: {:?}", event);
        event_queue.push(event); // Add event to the priority queue
    }
}

pub async fn process_events(event_queue: EventQueue) {
    loop {
        // Sleep briefly to avoid busy waiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Process events if there are any in the queue
        if let Some(event) = event_queue.pop() {
            println!("Handling event with priority {}: {}", event.priority, event.description);
            // Simulate event processing time
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}
