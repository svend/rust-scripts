use regex::Regex;
use std::io::BufRead;
use std::net::IpAddr;
use std::str::FromStr;

fn main() {
    let stdin = std::io::stdin();
    let lines = sort_lines(stdin.lock());

    for line in &lines {
        println!("{}", line)
    }
}

fn sort_lines<R: BufRead>(r: R) -> Vec<String> {
    let mut lines: Vec<_> = r
        .lines()
        .map(|s| s.expect("failed to get line from stdin"))
        .collect();
    lines.sort_by_key(|line| get_ip(&line).expect("failed to fine IP address in line"));
    lines
}

fn get_ip(s: &str) -> Option<IpAddr> {
    let re = Regex::new(r"(((([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]))|((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(%.+)?)").unwrap();
    let caps = re.captures(s)?;
    let ip = caps.get(1).map(|c| c.as_str())?;
    IpAddr::from_str(ip).ok()
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::net::Ipv4Addr;

    use super::*;

    const IPS: &str = r#"10.78.210.204
10.247.154.132
10.201.35.215
10.71.186.145"#;

    const IPS_SORTED: &str = r#"10.71.186.145
10.78.210.204
10.201.35.215
10.247.154.132"#;

    #[test]
    fn test_sort_lines() {
        let cursor = io::Cursor::new(IPS);
        let lines = sort_lines(cursor);
        assert_eq!(lines.join("\n"), IPS_SORTED);
    }

    #[test]
    fn test_get_ip() {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(get_ip("127.0.0.1"), Some(ip));
        assert_eq!(get_ip("_127.0.0.1_"), Some(ip));
        assert_eq!(get_ip(""), None);
    }
}
