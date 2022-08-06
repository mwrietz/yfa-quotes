use std::env;
use std::process;

use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        process::exit(1);
    }

    let mut tickers = Vec::new();
    for i in 0..args.len() {
        tickers.push(&args[i]);
    }

    let provider = yahoo::YahooConnector::new();
    tickers.sort();

    println!();
    for i in 0..(tickers.len() - 1) {
        print!("{:10}", &tickers[i]);
    }
    println!();

    for i in 0..(tickers.len() - 1) {
        // get the latest quotes in 1 minute intervals
        let response = tokio_test::block_on(provider.get_latest_quotes(&tickers[i], "1d")).unwrap();
        // extract just the latest valid quote summery including timestamp,open,close,high,low,volume
        let quote = response.last_quote().unwrap();
        print!("${:<9.2}", quote.close);
    }
    println!("\n");
}

fn usage() {
    let prog_path = env::current_exe().unwrap();
    println!("\nError...\n");
    println!("Usage: {} ticker1 ticker2", prog_path.file_name().unwrap().to_str().unwrap());
}
