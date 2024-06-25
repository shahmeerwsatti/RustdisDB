use std::collection;
use std::env;
use std::io;
use tokio;
use serde;
use clap;

fn main(&mut setup: bool)
{
    let mut dbtype: Box::<String> = String::new();
    struct db
    {
        hash: HashMap<str, str>;
        list: Vec!<str>;
        set: HashSet<str, str>;
        geospatial: HashMap<str, Vec!<str>>;
    }
    impl db
    {
        fn new() -> Self
        {
            let mut dbstructure: String = String::new();
            println!("Choose a data structure. 
            1. Map
            2. List
            3. Set
            4. Geospatial Data Store
            
            Choose the corresponding number. Remember, all sets in Rustdis are always sorted ");
            //Input functionality here later with Clap to take CLI arguments
            match dbstructure
            {
                1 => db
                {
                    hash: HashMap::new();
                }
                2 => db
                {
                    list: Vec!<str>::new();
                }
                3 => db
                {
                    set: HashSet::new()
                }
                4 => db
                {
                    geospatial: HashMap::new()
                }
            }
        }
    }

    impl db
    {
        fn set()
        {

        }
    }

    impl db
    {
        fn get()
        {

        }
    }
}

async fn listener(&target_ip: str)
{
    let listener = TcpListener::bind(target_ip).await.unwrap();
    let userdb = Arc::new(Mutex::new(db::New()))
    loop
    {
      let (socket, _) = listener.accept().await.unwrap();
      let db = Arc::clone(&*db);

      tokio::spawn(async move {
        
      })
    }
}

fn simple_db_setup(&mut setup: bool, &mut databasetype: str)
{

}

fn ha_db_setup(&mut setup: bool, &mut databasetype: str)
{

}

fn sentinel_setup(&mut setup: bool, &mut databasetype: str)
{

}

fn cluster_setup(&mut setup: bool, &mut databasetype: str)
{

}

fn custom_config(&mut setup: bool, &mut databasetype: str)
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

    struct choicetype
    {
        setup_config: u8
    }

    if setup_config > 5
    {
       println!("Error! Expected number less than or equal to 5");
       initial_setup();
    }
    else if setup_config < 1
    {
        println!("Error! Expected number greater than or equal to 1");
        initial_setup();
    }
    let args = choicetype::Parse();
    //Finish up Clap CLI stuff here

    match setup_config
    {
        1 => simple_db_setup(setup_flag),
        2 => ha_db_setup(setup_flag),
        3 => sentinel_setup(setup_flag),
        4 => cluster_setup(setup_flag),
        5 => custom_config(setup_flag);
    }
}
