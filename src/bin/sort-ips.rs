use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let stdin = std::io::stdin();
    let mut ips = stdin
        .lock()
        .lines()
        .map(|s| s.expect("failed to get line from stdin"))
        .map(|s| std::net::IpAddr::from_str(&s).expect("failed to parse line"))
        .collect::<Vec<_>>();

    ips.sort();

    for ip in &ips {
        println!("{}", ip)
    }
}
