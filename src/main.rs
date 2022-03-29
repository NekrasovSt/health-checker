use std::env;
use tokio::time;


async fn sleep(interval: u64) {
    time::sleep(time::Duration::from_secs(interval)).await
}
// https://pastebin.com/jmV6BQTA
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("Only two arguments are allowed");
    }
    let interval = &args[0].parse::<u64>().expect("Interval parse error");
    let url = &args[1];
    loop {
        let result = &reqwest::get(url).await;

        match result {
            Err(e) => panic!("URL Parse error"),
            Ok(res) => {
                if res.status().is_success() {
                    println!("OK(200)");
                } else {
                    println!("ERR({})", res.status())
                }
            }
        }
        sleep(*interval).await;
    }
}
