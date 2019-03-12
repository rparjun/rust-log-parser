#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate regex;
#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::{self, Read, BufRead, BufReader, Result as StdIOResult };
use regex::Regex;


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
  //print the line which cannot be parsed and exit, default false
  #[structopt(short = "s", long = "stop")]
  stop: bool,

  //read from file, default is stdin
  #[structopt(short = "i", long = "input-file", parse(from_os_str))]
  file: Option<PathBuf>,

  #[structopt(short = "f", long = "format", default_value="json")]
  format: String
}

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

enum Input {
  S (io::Stdin),
  F (File)
}


fn main() -> StdIOResult<()> {

    let opt = Opt::from_args();
    //println!("{:?}", opt);
    //println!("{:?}", opt.file);

  let buffer: Box<BufRead> = match opt.file {
    None => Box::new(BufReader::new(io::stdin())),
    Some(file) => Box::new(BufReader::new(File::open(file).unwrap()))
  };


  let json_const: String = "json".to_string();

  let values_to_print  = opt.format.split(",");


  for x in values_to_print {
    //println!("{}", x)
  }

  for line in buffer.lines() {
    match line {
      Err(e) => { println!("Failed to read line"); }
      Ok(l) => {
        let val = parse(l.clone());
        //let val = parse(l);
        if val.is_none() {
          if opt.stop {
            println!("parse failed: {:?}", l);
            break;
          }
        }
        else {
          let entry = val.unwrap();

          if opt.format == json_const {
            let json = serde_json::to_string(&entry)?;
            println!("{}", json);
          }

          //println!("{} {} {}", entry.time, entry.path, entry.rt);
        }
      }
    }


    //let json = serde_json::to_string(&entry)?;
    //println!("    {:?}", entry);
    //println!("{}", json);
  }
  Ok(())
}

fn parse(l: String) -> Option<LogEntry> {
  let regex = String::from(r#"^(\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) \[(\S+ \+\d{4})\] "(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (\S+) (\S+)" (\d{3}) "rt=(\S+)" "(\S+)" "(.*)"$"#);
  let re = Regex::new(&regex).unwrap();

    let parsedValue = re.captures(&l);

    if parsedValue.is_none() {
      return None;
    }
    let caps = parsedValue.unwrap();

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
    return Some(entry);
}
