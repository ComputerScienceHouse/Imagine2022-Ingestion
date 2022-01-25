use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;

type BoxError = Box<dyn std::error::Error + Send + Sync>;
type BoxResult<T> = Result<T, BoxError>;

#[derive(Serialize, Deserialize, Debug)]
struct BluetoothFrameWithoutTimestamp {
    macaddr: String,
    uename: String,
    rssi: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BluetoothFrame {
    macaddr: String,
    uename: String,
    rssi: f64,
    timestamp: u64,
}

fn main() -> BoxResult<()> {
    let runtime = Runtime::new()?;
    runtime.block_on(async_main())?;
    Ok(())
}

async fn async_main() -> BoxResult<()> {
    let mongo_url = env::var("MONGO_URL")?;

    let mut client_options = ClientOptions::parse(mongo_url).await?;
    client_options.app_name = Some("Sentinel Surveillance".to_string());

    let client = Client::with_options(client_options)?;

    let database = client.database("develop");

    database.run_command(doc!("ping": 1), None).await?;

    println!("Connected to mongo database.");

    let bluetooth_frames = database.collection::<BluetoothFrame>("bluetooth_frames");

    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buffer = [0; 1024];

    loop {
        let (len, _address) = socket.recv_from(&mut buffer).await?;

        let bluetooth_frame =
            serde_json::from_slice::<BluetoothFrameWithoutTimestamp>(&buffer[0..len]);

        match bluetooth_frame {
            Ok(bluetooth_frame) => {
                let now = SystemTime::now();
                let timestamp = now.duration_since(UNIX_EPOCH).unwrap();

                let bluetooth_frame = BluetoothFrame {
                    macaddr: bluetooth_frame.macaddr,
                    uename: bluetooth_frame.uename,
                    rssi: bluetooth_frame.rssi,
                    timestamp: timestamp.as_millis() as u64,
                };

                println!("Received bluetooth frame - {:?}", bluetooth_frame);

                let result = bluetooth_frames.insert_one(&bluetooth_frame, None).await;

                match result {
                    Ok(_) => {
                        println!("Saved bluetooth frame.");
                    }
                    Err(_) => {
                        println!(
                            "Failed to save bluetooth frame to database - {:?}",
                            bluetooth_frame
                        );
                    }
                }
            }
            Err(error) => {
                println!("Invalid message received - {:?}", error);
            }
        }
    }
}
