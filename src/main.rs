use paho_mqtt as mqtt;
use std::time::Duration;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
        
    let update_topic = env::var("MQTT_TOPIC").unwrap_or_else(|_| "updates".to_string());
    let broker_address = env::var("MQTT_ENDPOINT").unwrap_or_else(|_| "tcp://127.0.0.1:1885".to_string());

    // Create MQTT client options
    let host = &broker_address; // Public test broker
    let client_id = "rust_mqtt_client";
 
    // Create a client & define connect options
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(client_id)
        .finalize();
 
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        std::process::exit(1);
    });
 
    // Set up connection options
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();
 
    // Connect to the broker
    println!("Connecting to MQTT broker at {}...", host);
    match cli.connect(conn_opts) {
        Ok(_) => println!("Connected successfully!"),
        Err(e) => {
            println!("Unable to connect: {:?}", e);
            std::process::exit(1);
        }
    }
 
    // Subscribe to a topic
    let topic = update_topic;
    println!("Subscribing to topic: {}", topic);
    cli.subscribe(&topic, 1).unwrap();
 
    // Start consuming messages
    let rx = cli.start_consuming();
 
    // Publish a message
    let msg_text = "Hello from Rust MQTT client!";
    let msg = mqtt::Message::new(&topic, msg_text, 1);
    println!("Publishing message: {}", msg_text);
    cli.publish(msg).unwrap();
 
    // Receive messages for 10 seconds
    println!("Waiting for messages (10 seconds)...");
    for _ in 0..10 {
        if let Ok(Some(msg)) = rx.recv_timeout(Duration::from_secs(1)) {
            println!("Received: {} on topic: {}", 
                     msg.payload_str(), 
                     msg.topic());
        }
    }
 
    // Disconnect
    println!("Disconnecting...");
    cli.unsubscribe(&topic).unwrap();
    cli.disconnect(None).unwrap();
    println!("Disconnected!");
}