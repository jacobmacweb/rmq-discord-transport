use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions,
};
use dotenv::dotenv;
use reqwest::multipart;
use serde_json::json;

use std::{fmt::{self, Formatter}, string::String};
use std::vec::Vec;
use std::{env, error, process};


mod interface;
use interface::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
struct NoUriError;

impl error::Error for NoUriError {}

impl fmt::Display for NoUriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "No webhook URI was given to the function, and no default URI is defined.")
    }
}

impl fmt::Debug for NoUriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "No webhook URI was given to the function, and no default URI is defined. {{ file: {} line: {} }}", file!(), line!())
    }
}


fn handle(body: &str, default_uri: &Option<String>) -> Result<()> {
    let client = reqwest::Client::new();
    let message: IncomingTransportData = serde_json::from_str(body)?;
    
    // Resolve target
    let target_uri = match message.webhook_uri {
        Some(x) => x,
        None => {
            match default_uri {
                Some(y) => y.to_owned(),
                None => {
                    eprintln!("No URI provided or ");
                    return Err(Box::new(NoUriError));
                }
            }
        }
    };
    
    let payload = json!(message.payload);
    let files: Vec<File> = message.files.unwrap_or_else(|| {
        Vec::new()
    });
    

    let request = if files.is_empty() {
        client.post(&target_uri)
            .header("Content-Type", "application/json")
            .body(payload.to_string())
    } else {
        let mut form = multipart::Form::new()
            .text("payload_json", payload.to_string());
        
        for (i, file) in files.into_iter().enumerate() {
            let part = multipart::Part::bytes(file.data)
                .file_name(file.filename)
                .mime_str("application/octet-stream")?;
            
            form = form.part(format!("file{}", i), part);
        }

        client.post(&target_uri)
            .multipart(form)
    };

    let mut res = request.send()?;
    let result: WebhookResponse = match res.json() {
        Ok(r) => r,
        Err(_) => return Ok(())
    };

    println!("Delivered {}", result.id);

    Ok(())
}

fn main() {
    dotenv().ok();

    let default_webhook = match env::var_os("DEFAULT_WEBHOOK_URI") {
        Some(s) => s.into_string().ok(),
        None => None
    };
    let rabbitmq_uri = env::var("RABBITMQ_URI").unwrap_or_else(|err| {
        println!(
            "No variable found RABBITMQ_URI in ENV, {:?}",
            err.to_string()
        );
        process::exit(1);
    });
    // Open connection to URI
    let mut connection = Connection::insecure_open(&rabbitmq_uri).unwrap_or_else(|err| {
        println!(
            "Unable to open connection insecurely to {}, {:?}",
            &rabbitmq_uri,
            err.to_string()
        );
        process::exit(1);
    });

    // Openn channel
    let channel = connection.open_channel(None).unwrap_or_else(|err| {
        println!(
            "Unable to open default channel on {}, {:?}",
            &rabbitmq_uri,
            err.to_string()
        );
        process::exit(1);
    });

    // Declare exchange
    let exchange = channel
        .exchange_declare(
            ExchangeType::Fanout,
            "discord",
            ExchangeDeclareOptions::default(),
        )
        .unwrap_or_else(|err| {
            println!(
                "Unable to open exchange on {}, {:?}",
                &rabbitmq_uri,
                err.to_string()
            );
            process::exit(1);
        });

    // Declare default queue
    let queue = channel
        .queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                ..QueueDeclareOptions::default()
            },
        )
        .unwrap_or_else(|err| {
            println!(
                "Unable to open queue on {} on exchange {}, {:?}",
                &rabbitmq_uri,
                exchange.name(),
                err.to_string()
            );
            process::exit(1);
        });

    // Bind queue to exchange
    queue
        .bind(&exchange, "", FieldTable::new())
        .unwrap_or_else(|err| {
            println!(
                "Unable to bind queue {} to exchange {} on {}, {:?}",
                queue.name(),
                exchange.name(),
                &rabbitmq_uri,
                err.to_string()
            );
            process::exit(1);
        });

    let consumer = queue.consume(ConsumerOptions {
        no_ack: true,
        ..ConsumerOptions::default()
    }).unwrap_or_else(|err| {
        println!(
            "Unable to start consumer on {} on exchange {}, {:?}",
            &rabbitmq_uri,
            exchange.name(),
            err.to_string()
        );
        process::exit(1);
    });

    // Iterate through messages
    println!(
        "Waiting for messages on {}/{}",
        queue.name(),
        exchange.name()
    );
    
    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("Got message of length {}", body.len());

                // Handle it
                if let Err(err) = handle(&String::from(body), &default_webhook) {
                    eprintln!(
                        "An error occurred handling a message: {:#?}",
                        err.to_string()
                    );
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}