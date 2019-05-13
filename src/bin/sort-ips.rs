use std::io::BufRead;
use std::net::IpAddr;
use std::str::FromStr;

fn main() {
    let stdin = std::io::stdin();
    let ips = sort_ips(stdin.lock());

    for ip in &ips {
        println!("{}", ip)
    }
}

fn sort_ips<R: BufRead>(r: R) -> Vec<IpAddr> {
    let mut ips = r
        .lines()
        .map(|s| s.expect("failed to get line from stdin"))
        .map(|s| IpAddr::from_str(&s).expect("failed to parse line"))
        .collect::<Vec<_>>();

    ips.sort();

    ips
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::net::Ipv4Addr;

    use super::*;

    const IPS: &str = r#"10.78.210.204
10.201.35.215
10.71.186.145
10.247.154.132
"#;
    #[test]
    fn test_sort_ips() {
        let cursor = io::Cursor::new(IPS);
        let ips = sort_ips(cursor);
        assert_eq!(
            ips,
            &[
                Ipv4Addr::new(10, 71, 186, 145),
                Ipv4Addr::new(10, 78, 210, 204),
                Ipv4Addr::new(10, 201, 35, 215),
                Ipv4Addr::new(10, 247, 154, 132),
            ]
        );
    }
}
