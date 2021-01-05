use clap::{App, Arg};
use quotes::Quote;
use quotes::SinaQuote;

#[async_std::main]
async fn main() {
    let quote = SinaQuote::new().unwrap();

    let matches = App::new("quotes")
        .version("0.1.0")
        .arg(
            Arg::with_name("tick")
                .short("t")
                .long("tick")
                .value_name("code")
                .help("Sets a code to get tick"),
        )
        .arg(
            Arg::with_name("current_price")
                .short("c")
                .long("current_price")
                .value_name("code")
                .help("Sets a code to get current price"),
        )
        .arg(
            Arg::with_name("pct_change")
                .short("p")
                .long("pct_change")
                .value_name("code")
                .help("Sets a code to get pct change"),
        )
        .arg(
            Arg::with_name("pre_close")
                .short("P")
                .long("pre_close")
                .value_name("code")
                .help("Sets a code to get pre close price"),
        )
        .arg(
            Arg::with_name("open")
                .short("o")
                .long("open")
                .value_name("code")
                .help("Sets a code to get open price"),
        )
        .get_matches();

    if let Some(code) = matches.value_of("tick") {
        match quote.tick(code).await {
            Ok(t) => println!("{:#?}", t),
            Err(e) => eprintln!("{}", e),
        }
    }

    if let Some(code) = matches.value_of("current_price") {
        match quote.current_price(code).await {
            Ok(p) => println!("{}", p),
            Err(e) => eprintln!("{}", e),
        }
    }

    if let Some(code) = matches.value_of("pre_close") {
        match quote.pre_close(code).await {
            Ok(p) => println!("{}", p),
            Err(e) => eprintln!("{}", e),
        }
    }

    if let Some(code) = matches.value_of("pct_change") {
        match quote.pct_change(code).await {
            Ok(p) => println!("{}%", p),
            Err(e) => eprintln!("{}", e),
        }
    }

    if let Some(code) = matches.value_of("open") {
        match quote.open(code).await {
            Ok(p) => println!("{}", p),
            Err(e) => eprintln!("{}", e),
        }
    }
}
