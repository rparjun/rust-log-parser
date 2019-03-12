#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate regex;
#[macro_use]
extern crate structopt;

use std::collections::BTreeMap;
use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Result as StdIOResult };
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

  //use config file
  #[structopt(short = "c", long = "config", parse(from_os_str))]
  config: Option<PathBuf>,

  #[structopt(short = "f", long = "format", default_value="json")]
  format: String
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Config {
  regex: String,
  matches: BTreeMap<String, String>
}

fn main() -> StdIOResult<()> {

    let opt = Opt::from_args();
    //println!("{:?}", opt);
    //println!("{:?}", opt.file);

  let buffer: Box<BufRead> = match opt.file {
    None => Box::new(BufReader::new(io::stdin())),
    Some(file) => Box::new(BufReader::new(File::open(file).unwrap()))
  };

  let config = match opt.config {
    None => {
      let mut dummy = BTreeMap::new();

      dummy.insert("1".to_string(), "ip".to_string());
      dummy.insert("2".to_string(), "date".to_string());
      dummy.insert("3".to_string(), "method".to_string());
      dummy.insert("4".to_string(), "path".to_string());
      dummy.insert("5".to_string(), "version".to_string());
      dummy.insert("6".to_string(), "code".to_string());
      dummy.insert("7".to_string(), "rt".to_string());
      dummy.insert("8".to_string(), "referer".to_string());
      dummy.insert("9".to_string(), "ua".to_string());

      Config {
        regex: String::from(r#"^(\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) \[(\S+ \+\d{4})\] "(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (\S+) (\S+)" (\d{3}) "rt=(\S+)" "(\S+)" "(.*)"$"#),
        matches: dummy
      }
    },
    Some(f) => {
      let file = File::open(f).expect("file sh");
      let config: Config = serde_json::from_reader(file).expect("file should be proper JSON");;
      config
    }
  };

  let json_const: String = "json".to_string();
/*
  let values_to_print  = opt.format.split(",");
  for x in values_to_print {
    //println!("{}", x)
  }
*/

  for line in buffer.lines() {
    match line {
      Err(_e) => { println!("Failed to read line"); }
      Ok(l) => {
        let val = parse(l.clone(), &config);
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
            let json = serde_json::to_string(&entry).unwrap();
            println!("{}", json);
          }
        }
      }
    }
  }
  Ok(())
}

fn parse(l: String, config: &Config) -> Option<BTreeMap <String, String> > {
  let re = Regex::new(&config.regex).unwrap();

  let parsed_value = re.captures(&l);

  if parsed_value.is_none() {
    return None;
  }
  let caps = parsed_value.unwrap();

  let mut dummy: BTreeMap<String, String> = BTreeMap::new();

  for (k,v) in config.matches.iter() {
    dummy.insert(v.to_string(), caps.get(k.parse().unwrap()).map_or("", |m| m.as_str()).to_string());
  }
  Some(dummy)
}
