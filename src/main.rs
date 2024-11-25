use rumqttc::{MqttOptions, Client, QoS};
use tokio::runtime::Runtime;

fn main() {
    // Create a runtime for async execution
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        // MQTT options setup
        let mut mqttoptions = MqttOptions::new("client_id", "test.mosquitto.org", 1883);
        mqttoptions.set_keep_alive(60);

        // Create an MQTT client
        let (mut client, mut connection) = Client::new(mqttoptions, 10);

        // Subscribe to a topic
        client.subscribe("home/livingroom/temperature", QoS::AtMostOnce).unwrap(); // Removed `.await`
        println!("Subscribed to topic 'home/livingroom/temperature'");

        // Publish a message to a topic
        client.publish("home/livingroom/temperature", QoS::AtMostOnce, false, "23.5°C").unwrap(); // Removed `.await`
        println!("Published message to topic 'home/livingroom/temperature'");

        // Listen for incoming messages
        loop {
            match connection.eventloop.poll().await {
                // Handle event
                Ok(event) => {
                    println!("Received message: {:?}", event);
                }
                // Handle error
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
            }
        }
    });
}

