use rumqttc::{MqttOptions, Client, QoS};

#[tokio::main]
async fn main() {
    // MQTT options setup
    let mut mqttoptions = MqttOptions::new("client_id", "test.mosquitto.org", 1883);
    mqttoptions.set_keep_alive(60);

    // Create an MQTT client
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    // Subscribe to a topic
    client.subscribe("home/livingroom/temperature", QoS::AtMostOnce).unwrap();
    println!("Subscribed to topic 'home/livingroom/temperature'");

    // Publish a message to a topic
    client.publish("home/livingroom/temperature", QoS::AtMostOnce, false, "23.5°C").unwrap();
    println!("Published message to topic 'home/livingroom/temperature'");

    // Listen for incoming messages
    loop {
        match connection.eventloop.poll().await {
            Ok(event) => {
                // Handle event
                println!("Received message: {:?}", event);
            }
            Err(e) => {
                // Handle error
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}


