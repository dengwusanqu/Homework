use lazy_static::lazy_static;
use std::net::SocketAddr;
use tokio::io::{self, AsyncBufReadExt};

lazy_static! {
    static ref CLIENT: volo_gen::volo::redis::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::redis::RedisServiceClientBuilder::new("volo-redis")
            .address(addr)
            .build()
    };
}

enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Ping,
    Exit,
    Unknown,
}

impl Command {
    fn from_input(input: &str) -> Self {
        let args: Vec<&str> = input.split_whitespace().collect();
        if let Some(command_str) = args.get(0).map(|s| s.to_lowercase().to_string()) {
            match command_str.as_str() {
                "get" => {
                    if let Some(key) = args.get(1) {
                        Command::Get(key.to_string())
                    } else {
                        Command::Unknown
                    }
                }
                "set" => {
                    if let (Some(key), Some(value)) = (args.get(1), args.get(2)) {
                        Command::Set(key.to_string(), value.to_string())
                    } else {
                        Command::Unknown
                    }
                }
                "del" => {
                    if let Some(key) = args.get(1) {
                        Command::Del(key.to_string())
                    } else {
                        Command::Unknown
                    }
                }
                "ping" => Command::Ping,
                "exit" => Command::Exit,
                _ => Command::Unknown,
            }
        } else {
            Command::Unknown
        }
    }       
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    println!("Please enter the command (enter 'exit' to exit):");

    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin).lines();

    while let Some(line) = reader.next_line().await? {
        let command = Command::from_input(&line);

        match command {
            Command::Get(key) => {
                let req = volo_gen::volo::redis::RedisRequest {
                    cmd: volo_gen::volo::redis::RedisCommand::Get,
                    arguments: Some(vec![key.into()]),
                };
                process_request(req).await?;
            }
            Command::Set(key, value) => {
                let req = volo_gen::volo::redis::RedisRequest {
                    cmd: volo_gen::volo::redis::RedisCommand::Set,
                    arguments: Some(vec![key.into(), value.into()]),
                };
                process_request(req).await?;
            }
            Command::Del(key) => {
                let req = volo_gen::volo::redis::RedisRequest {
                    cmd: volo_gen::volo::redis::RedisCommand::Del,
                    arguments: Some(vec![key.into()]),
                };
                process_request(req).await?;
            }
            Command::Ping => {
                let req = volo_gen::volo::redis::RedisRequest {
                    cmd: volo_gen::volo::redis::RedisCommand::Ping,
                    arguments: None,
                };
                process_request(req).await?;
            }
            Command::Exit => {
                break;
            }
            Command::Unknown => {
                println!("Unknown Command");
            }
        }
    }
    Ok(())
}

async fn process_request(req: volo_gen::volo::redis::RedisRequest) -> Result<(), Box<dyn std::error::Error>> {
    let resp = CLIENT.redis_command(req).await?;

    if resp.ok {
        print!("{}", "Success: ");
    } else {
        print!("{}", "Error: ");
    }

    if let Some(data) = resp.data {
        println!("{}", data);
    }
    Ok(())
}
