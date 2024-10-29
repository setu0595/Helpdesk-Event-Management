use std::sync::Arc;
mod auth;
mod events;
mod queue;
mod models;

use crate::auth::authenticate_user;
use crate::events::prioritize_event;
use crate::queue::{handle_events, process_events, EventQueue, Event};
use tokio::sync::mpsc;
use reqwest::Client;


#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(64); // Larger buffer for scalability
    let event_queue = EventQueue::new(); // Create a new event queue

    // Start event handling and processing tasks
    tokio::spawn(handle_events(rx, event_queue.clone())); // Event handling in its own task
    tokio::spawn(process_events(event_queue.clone())); // Start processing events


    // Shared HTTP client wrapped in Arc for scalability
    let client = Arc::new(Client::new());

    // Simulate multiple user logins concurrently
    let mut tasks = Vec::new();
    let users = vec![
        ("user", "user"),
        ("support", "support"),
        ("admin@gmail.com", "adminpass"),
        // Add more users for scalability testing
    ];

    for (username, password) in users {
        let client = Arc::clone(&client);
        let tx = tx.clone();
        tasks.push(tokio::spawn(async move {handle_user_request(client, username, password, tx).await}));
    }

    // Keep the main function alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

// Function to handle each user's request independently
async fn handle_user_request(client: Arc<Client>, username: &str, password: &str, tx: mpsc::Sender<Event>) {
    let token = authenticate_user(&client, username, password).await.unwrap();
    let token_data = auth::decode_token(&token).unwrap();

    let mut role = String::new();

    // Check if resource_access is available and print client-specific roles
    if let Some(resource_access) = &token_data.claims.resource_access {
        for (_client_name, client_roles) in resource_access {
            //println!("Roles for {}: {:?}", client_name, client_roles.roles);

            // Example: Check if roles contain a specific role
            if let Some(first_role) = client_roles.roles.first() {
                role = first_role.clone();
                //println!("First role for {}: {}", client_name, first_role);
            }
        }
    }
        
    //let mut events = Vec::new(); // Collect events here
    
    let priority = prioritize_event(&role);

    let event_description = format!("Handling the event for user: {} with role: {}", username, role);

    // Create and send the event to the queue
    let event = Event {
        priority,
        description: event_description,
    };


    tx.send(event).await.unwrap();
    
    /* events.push(event); // Add event to the vector

    // Send all events concurrently
    let send_futures: Vec<_> = events.into_iter()
        .map(|event| tx.send(event))
        .collect();

    // Await all send operations to complete
    join_all(send_futures).await; */
}

