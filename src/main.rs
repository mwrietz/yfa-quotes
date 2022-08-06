//use std::fs::File;
//use std::io::{self, BufRead};
//use std::path::Path;

use std::env;
use std::process;
//use std::fs;

//use chrono::prelude::*;
//use std::time::{Duration, UNIX_EPOCH};
use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    //i_o::cls();
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
/*
    println!(
        "{:30} {:>6} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "Time", "Symbol", "Open", "Close", "High", "Low", "Volume"
    );
*/

    println!();
    for i in 0..(tickers.len() - 1) {
        print!("{:10}", &tickers[i]);
    }
    println!();

    for i in 0..(tickers.len() - 1) {
        // get the latest quotes in 1 minute intervals
        //let response = tokio_test::block_on(provider.get_latest_quotes("AAPL", "1m")).unwrap();
        let response = tokio_test::block_on(provider.get_latest_quotes(&tickers[i], "1d")).unwrap();
        // extract just the latest valid quote summery
        // including timestamp,open,close,high,low,volume
        let quote = response.last_quote().unwrap();
        //let time: DateTime<Utc> = DateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
        /*
        println!(
            "{:30} {:6} ${:9.3} ${:9.3} ${:9.3} ${:9.3} {:10}",
            time.to_rfc3339(),
            &tickers[i],
            quote.open,
            quote.close,
            quote.high,
            quote.low,
            quote.volume
        );
        */
        print!("${:<9.2}", quote.close);
    }
    println!("\n");
}

fn usage() {
    let prog_path = env::current_exe().unwrap();
    println!("\nError...\n");
    println!("Usage: {} ticker1 ticker2", prog_path.file_name().unwrap().to_str().unwrap());
}
