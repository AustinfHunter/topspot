mod models;
mod setup;
mod server;
mod http;

use mysql::*;
use mysql::prelude::*;
use clap::*;


#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value_t=false,short,long)]
    setup: bool,
    #[arg(default_value="root",long)]
    mysql_user: String,
    #[arg(default_value="root",long)]
    mysql_pass: String,
    #[arg(default_value="localhost",long)]
    mysql_host: String,
    #[arg(default_value="3306",long)]
    mysql_port: String,
    #[arg(default_value="topspot",long)]
    mysql_db: String
}

fn main() { 
    let args = Args::parse();
    let url = format!("mysql://{}:{}@{}:{}/{}",args.mysql_user,args.mysql_pass,args.mysql_host,args.mysql_port,args.mysql_db);
    let opts = Opts::from_url(&url).unwrap();
    let pool = Pool::new(opts);
    let mut conn  = pool.expect("err pool").get_conn().unwrap(); 
    if args.setup == true {
        if let Err(err) = setup::setup_msql(conn){
        eprintln!("{}",err);
        }
    } else {
        println!("Skipping setup");
    }

    server::init("8080",&mut std::io::stdout());
}
