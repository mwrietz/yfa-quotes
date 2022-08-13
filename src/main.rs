use std::env;
use std::process;

use colored::Colorize;
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

    let mut header = String::from("\n");
    for i in 0..(tickers.len() - 1) {
        header += &format!("{:10}", &tickers[i]);
    }

    let mut values = String::from("");
    for i in 0..(tickers.len() - 1) {
        // get the latest quotes in 1 minute intervals
        let response = tokio_test::block_on(provider.get_latest_quotes(&tickers[i], "1d")).unwrap();
        // extract just the latest valid quote summery including timestamp,open,close,high,low,volume
        let quote = response.last_quote().unwrap();
        values += &format!("${:<9.2}", quote.close);
    }
    values += "\n";
    println!("{}", header.green());
    println!("{}", values);
}

fn usage() {
    println!("{} v{}", get_prog_name(), env!("CARGO_PKG_VERSION"));
    println!("Usage: {} [TICKER_SYMB] [TICKER_SYMB]...", get_prog_name());
}

fn get_prog_name() -> String {
    let prog_name = env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    prog_name
}
