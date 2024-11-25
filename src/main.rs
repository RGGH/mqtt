use rumqttc::{MqttOptions, Client, QoS, Event, Packet};

#[tokio::main]
async fn main() {
    // MQTT options setup
    let mut mqttoptions = MqttOptions::new("client_id", "test.mosquitto.org", 1883);
    mqttoptions.set_keep_alive(60);

    // Create the MQTT client and connection
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    // Subscribe to a topic
    match client.subscribe("home/livingroom/temperature", QoS::AtLeastOnce) {
        Ok(_) => println!("Subscribed to topic 'home/livingroom/temperature'"),
        Err(e) => {
            eprintln!("Failed to subscribe to topic: {:?}", e);
            return;
        }
    }

    // Publish a message to a topic
    match client.publish("home/livingroom/temperature", QoS::AtLeastOnce, false, "23.5°C") {
        Ok(_) => println!("Published message to topic 'home/livingroom/temperature'"),
        Err(e) => {
            eprintln!("Failed to publish message: {:?}", e);
            return;
        }
    }

    // Listen for incoming messages
    loop {
        match connection.eventloop.poll().await {
            Ok(event) => {
                // Handle incoming event
                match event {
                    Event::Incoming(Packet::Publish(publish)) => {
                        let message = String::from_utf8_lossy(&publish.payload);
                        println!("Received message: {}", message); // Debug message
                    }
                    _ => {
                        println!("Received unexpected event: {:?}", event);
                    }
                }
            }
            Err(e) => {
                // Handle error in the event loop
                println!("Error while processing MQTT events: {:?}", e);
                break;
            }
        }
    }
}

