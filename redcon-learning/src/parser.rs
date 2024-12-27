#[derive(Debug)]
pub enum RedisCommand {
    Set {
        key: String,
        value: String,
        options: SetOptions,
    },
    Unknown {
        command: String,
        args: Vec<String>,
    },
}

/// Could use the redis::SetOptions instead
#[derive(Debug, Default)]
pub struct SetOptions {
    nx: bool,                       // NX: Only set if the key does not exist
    xx: bool,                       // XX: Only set if the key exists
    get: bool,                      // GET: Return the old value
    expiration: Option<Expiration>, // Expiration options
}

#[derive(Debug)]
pub enum Expiration {
    /// Expire in seconds
    Ex(u64),
    /// Expire in milliseconds
    Px(u64),
    /// Expire at Unix timestamp in seconds
    ExAt(u64),
    /// Expire at Unix timestamp in milliseconds
    PxAt(u64),
    /// Keep the current TTL
    KeepTtl,
}

pub fn parse_set_command(args: &[String]) -> Result<RedisCommand, String> {
    if args.len() < 2 {
        return Err("SET command requires at least a key and a value.".to_string());
    }

    let key = args[0].clone();
    let value = args[1].clone();
    let mut options = SetOptions::default();

    let mut i = 2; // Start after key and value
    while i < args.len() {
        match args[i].to_uppercase().as_str() {
            "NX" => {
                if options.xx {
                    return Err("Cannot specify both NX and XX.".to_string());
                }
                options.nx = true;
            }
            "XX" => {
                if options.nx {
                    return Err("Cannot specify both NX and XX.".to_string());
                }
                options.xx = true;
            }
            "GET" => options.get = true,
            "EX" => {
                if i + 1 >= args.len() {
                    return Err("EX requires a seconds value.".to_string());
                }
                if let Ok(seconds) = args[i + 1].parse::<u64>() {
                    options.expiration = Some(Expiration::Ex(seconds));
                    i += 1;
                } else {
                    return Err("Invalid EX seconds value.".to_string());
                }
            }
            "PX" => {
                if i + 1 >= args.len() {
                    return Err("PX requires a milliseconds value.".to_string());
                }
                if let Ok(milliseconds) = args[i + 1].parse::<u64>() {
                    options.expiration = Some(Expiration::Px(milliseconds));
                    i += 1;
                } else {
                    return Err("Invalid PX milliseconds value.".to_string());
                }
            }
            "EXAT" => {
                if i + 1 >= args.len() {
                    return Err("EXAT requires a Unix timestamp in seconds.".to_string());
                }
                if let Ok(unix_time) = args[i + 1].parse::<u64>() {
                    options.expiration = Some(Expiration::ExAt(unix_time));
                    i += 1;
                } else {
                    return Err("Invalid EXAT Unix timestamp value.".to_string());
                }
            }
            "PXAT" => {
                if i + 1 >= args.len() {
                    return Err("PXAT requires a Unix timestamp in milliseconds.".to_string());
                }
                if let Ok(unix_time) = args[i + 1].parse::<u64>() {
                    options.expiration = Some(Expiration::PxAt(unix_time));
                    i += 1;
                } else {
                    return Err("Invalid PXAT Unix timestamp value.".to_string());
                }
            }
            "KEEPTTL" => {
                if options.expiration.is_some() {
                    return Err("Cannot specify KEEPTTL with other expiration options.".to_string());
                }
                options.expiration = Some(Expiration::KeepTtl);
            }
            _ => return Err(format!("Unknown option: {}", args[i])),
        }
        i += 1;
    }

    Ok(RedisCommand::Set {
        key,
        value,
        options,
    })
}
