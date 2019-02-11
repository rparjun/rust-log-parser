#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate regex;

use std::fs::File;
use std::io::{self, Read, BufRead, BufReader, Result as StdIOResult };
use regex::Regex;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct LogEntry {
  ip: String,
  time: String,
  method: String,
  path: String,
  version: String,
  status: i16,
  rt: f64,
  referer: String,
  user_agent: String,
}

fn main() -> StdIOResult<()> {

  let buffer;

  if (true) {
    buffer = BufReader::new(io::stdin());
    buffer = BufReader::new(File::open("nginx.log")?);
  }
  else {
  }

  //for line in BufReader::new(file).lines() {
  for line in buffer.lines() {
    let entry = parse(line?);

    //let json = serde_json::to_string(&entry)?;
    //println!("    {:?}", entry);
    //println!("{}", json);
    println!("{} {} {}", entry.time, entry.path, entry.rt);
  }
  Ok(())
}

fn parse(l: String) -> LogEntry {
  let regex = String::from(r#"^(\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) \[(\S+ \+\d{4})\] "(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (\S+) (\S+)" (\d{3}) "rt=(\S+)" "(\S+)" "(.*)"$"#);
  let re = Regex::new(&regex).unwrap();
    let caps = re.captures(&l).unwrap();

    let ip = caps.get(1).map_or("", |m| m.as_str()).to_string();
    let time = caps.get(2).map_or("", |m| m.as_str()).to_string();
    let method = caps.get(3).map_or("", |m| m.as_str()).to_string();
    let path = caps.get(4).map_or("", |m| m.as_str()).to_string();
    let version = caps.get(5).map_or("", |m| m.as_str()).to_string();
    let status: i16 = caps.get(6).map_or("", |m| m.as_str()).to_string().parse().unwrap();
    let rt: f64 = caps.get(7).map_or("", |m| m.as_str()).to_string().parse().unwrap();
    let referer = caps.get(8).map_or("", |m| m.as_str()).to_string();
    let user_agent = caps.get(9).map_or("", |m| m.as_str()).to_string();

    let entry = LogEntry { ip, time, method, path, version, status, rt, referer, user_agent };
    return entry;
}
