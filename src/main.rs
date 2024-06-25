use std::collection;
use std::env;
use std::io;
use tokio;
use serde;

fn main(&mut setup: bool)
{

}

async fn listener(&target_ip: str)
{
let listener = TcpListener::bind(target_ip)
}

fn simple_db_setup(&mut setup: bool)
{

}

fn ha_db_setup(&mut setup: bool)
{

}

fn sentinel_setup(&mut setup: bool)
{

}

fn cluster_setup(&mut setup: bool)
{

}

fn custom_config(&mut setup: bool)
{

}

fn initial_setup()
{
    let setup_flag mut: Box::<bool> = Box::new(false)
    println!("Welcome to Rustdis. The free and open source alternative to Redis, rewritten in Rust.");
    println!
    ("
    Choose your setup configuration:
    1. Simple Database
    2. High-Availability (HA) Database
    3. Sentinel
    4. Cluster
    5. Custom Configuration
    ");
    let mut setup_config = New::u32();
    // Add CLI stuff here to accept arguments using Clap later
    match setup_config
    {
        1 => simple_db_setup(setup_flag)
        2 => ha_db_setup(setup_flag)
        3 => sentinel_setup(setup_flag)
        4 => cluster_setup(setup_flag)
        5 => custom_config(setup_flag)
    }
}
