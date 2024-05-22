use snowflake::SnowflakeIdGenerator;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::env;

lazy_static! {
    pub static ref ID_GENERATOR: Mutex<SnowflakeIdGenerator> = {
        let node_id: i32 = env::var("NODE_ID").unwrap_or_else(|_| "1".to_string()).parse().expect("NODE_ID must be a number");
        let machine_id: i32 = env::var("MACHINE_ID").unwrap_or_else(|_| "1".to_string()).parse().expect("MACHINE_ID must be a number");
        Mutex::new(SnowflakeIdGenerator::new(node_id, machine_id))
    };
}

pub fn generate_id() -> i64 {
    ID_GENERATOR.lock().unwrap().real_time_generate()
}
