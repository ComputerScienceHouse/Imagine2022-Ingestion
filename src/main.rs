use mongodb::bson::doc;
use mongodb::options::{ClientOptions, Tls};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;

type BoxError = Box<dyn std::error::Error + Send + Sync>;
type BoxResult<T> = Result<T, BoxError>;

#[derive(Serialize, Deserialize, Debug)]
struct BluetoothFrame {
    macaddr: String,
    uename: String,
    rssi: i32,
    timestamp: u64,
}

fn main() -> BoxResult<()> {
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let runtime = Runtime::new()?;
    runtime.block_on(async_main())?;
    Ok(())
}

async fn async_main() -> BoxResult<()> {
    dotenv::dotenv().ok();
    println!("Booting ingestion server...");
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL variable not specified. Please specify the MongoDB's URL in this format: mongodb://user:password@some_mongo");
    let mut client_options = ClientOptions::parse(mongo_url).await?;
    client_options.app_name = Some("Sentinel Surveillance".to_string());
    match env::var("TLS") {
        Ok(tls) => match tls.as_str() {
            "true" => client_options.tls = Some(Tls::Enabled(Default::default())),
            _ => println!("TLS Disabled"),
        },
        _ => println!("TLS Disabled"),
    }
    let client = Client::with_options(client_options)?;
    let database = client.database(&env::var("DATABASE").expect("DATABASE variable not specified. Please specify DATABASE name."));
    database.run_command(doc!("ping": 1), None).await?;
    println!("Connected to mongo database.");
    let bluetooth_frames = database.collection::<BluetoothFrame>("bluetooth_frames");
    let socket_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS variable not specified. Please specify the server's address and port. (eg 0.0.0.0:8080)");
    let socket = UdpSocket::bind(&socket_address).await?;
    let mut buffer = [0; 1024];
    println!("Serving on {}", &socket_address);
    loop {
        let (_len, _address) = socket.recv_from(&mut buffer).await?;
        let frame = parse_frame(&buffer);
        if let Some(bluetooth_frame) = frame {
            println!("{:?}", bluetooth_frame);
            let result = bluetooth_frames.insert_one(&bluetooth_frame, None).await;
            match result {
                Ok(_) => println!("Saved bluetooth frame."),
                Err(_) => {
                    println!(
                        "Failed to save bluetooth frame to database - {:?}",
                        bluetooth_frame
                    );
                }
            }
        } else {
            eprintln!("Could not parse bluetooth frame.");
        }
    }
}

fn parse_frame(buffer: &[u8]) -> Option<BluetoothFrame> {
    let frame_string = std::str::from_utf8(&buffer).ok()?;
    let frame_vec: Vec<&str> = frame_string.split("|").collect();
    let parsed_frame = BluetoothFrame {
        macaddr: frame_vec[3].to_string(),
        uename: frame_vec[2].to_string(),
        rssi: frame_vec[4].parse::<i32>().ok()?,
        timestamp: frame_vec[1].parse::<u64>().ok()?
    };
    Some(parsed_frame)

}
