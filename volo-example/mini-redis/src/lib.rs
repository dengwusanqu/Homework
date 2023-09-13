#![feature(impl_trait_in_assoc_type)]
use anyhow::anyhow;
use pilota::FastStr;
use std::collections::HashMap;
use std::sync::Mutex;
use volo_gen::volo::redis::{RedisCommand, RedisResponse};

pub struct S {
    pub map: Mutex<HashMap<String, String>>,
}

#[volo::async_trait]
impl volo_gen::volo::redis::RedisService for S {
    async fn redis_command(
        &self,
        req: volo_gen::volo::redis::RedisRequest,
    ) -> Result<volo_gen::volo::redis::RedisResponse, volo_thrift::AnyhowError>
    {
        println!("recive redis command:{:?} {:?}", req.cmd, req.arguments);
        
        match req.cmd {
            RedisCommand::Get => self.get_command(req),
            RedisCommand::Set => self.set_command(req),
            RedisCommand::Del => self.del_command(req),
            RedisCommand::Ping => Ok(RedisResponse {
                ok: true,
                data: Some(FastStr::from("Ping ok.")),
            }),
            _ => Ok(RedisResponse {
                ok: true,
                data: Some(FastStr::from("Not support.")),
            })
        }
    }
}

impl S {
    fn get_command(&self, req: volo_gen::volo::redis::RedisRequest) -> Result<RedisResponse, volo_thrift::AnyhowError> {
        if let Some(args) = req.arguments {
            if args.len() != 1 {
                return Ok(RedisResponse {
                    ok: false,
                    data: Some(FastStr::from("The number of parameters is not equal to 1.")),
                });
            } else {
                let key = args[0].as_str();
                match self.map.lock().unwrap().get(key) {
                    Some(value) => Ok(RedisResponse {
                        ok: true,
                        data: Some(FastStr::from(value.clone())),
                    }),
                    None => Ok(RedisResponse {
                        ok: false,
                        data: Some(FastStr::from("The corresponding key was not found.")),
                    }),
                }
            }
        } else {
            Err(anyhow!("There is a problem with the provided arguments.").into())
        }
    }

    fn set_command(&self, req: volo_gen::volo::redis::RedisRequest) -> Result<RedisResponse, volo_thrift::AnyhowError> {
        if let Some(args) = req.arguments {
            if args.len() != 2 {
                return Ok(RedisResponse {
                    ok: false,
                    data: Some(FastStr::from("The number of parameters is not equal to 2.")),
                });
            } else {
                let key = args[0].as_str();
                let value = args[1].as_str();

                self.map.lock().unwrap().insert(key.to_string(), value.to_string());
                Ok(RedisResponse {
                    ok: true,
                    data: Some(FastStr::from("Set successful.")),
                })
            }
        } else {
            Err(anyhow!("There is a problem with the provided arguments.").into())
        }
    }

    fn del_command(&self, req: volo_gen::volo::redis::RedisRequest) -> Result<RedisResponse, volo_thrift::AnyhowError> {
        if let Some(args) = req.arguments {
            if args.len() != 1 {
                return Ok(RedisResponse {
                    ok: false,
                    data: Some(FastStr::from("The number of parameters is not equal to 1.")),
                });
            } else {
                let key = args[0].as_str();
                self.map.lock().unwrap().remove(key);
                Ok(RedisResponse {
                    ok: true,
                    data: Some(FastStr::from("Del successful.")),
                })
            }
        } else {
            Err(anyhow!("There is a problem with the provided arguments.").into())
        }
    }
}
