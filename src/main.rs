#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate regex;
#[macro_use]
extern crate structopt;


extern crate itertools;

use std::collections::BTreeMap;
use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Result as StdIOResult };
use regex::Regex;

mod config;

use config::Config;

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


fn main() -> StdIOResult<()> {
  let opt = Opt::from_args();

  let buffer: Box<BufRead> = match opt.file {
    None => Box::new(BufReader::new(io::stdin())),
    Some(file) => Box::new(BufReader::new(File::open(file).unwrap()))
  };

  let config = match opt.config {
    None => { config::default() },
    Some(f) => {
      match config::from_file(f) {
        Ok(config) => { config  }
        Err(v) => {
          eprintln!("{}", v);
          std::process::exit(-1);
        }
      }
    }
  };

  let json_const: String = "json".to_string();


  if opt.format != json_const {
    let mut found_format = false;
    for val in config.matches.values() {
      if val == &opt.format {
        found_format = true
      }
    }
    if found_format == false {
      let a: Vec<_> = config.matches.values().collect();
      eprintln!("Provided format '{}' does not exit, allowed values are {}", opt.format, itertools::join(a, ", "));
      std::process::exit(-1);
    }
  }

  for line in buffer.lines() {
    match line {
      Err(_e) => { println!("Failed to read line"); }
      Ok(l) => {
        let val = parse(l.clone(), &config);
        if val.is_none() {
          if opt.stop {
            eprintln!("parse failed: {:?}", l);
            break;
          }
        }
        else {
          let entry = val.unwrap();
          if opt.format == json_const {
            let json = serde_json::to_string(&entry).unwrap();
            println!("{}", json);
          }
          else {
            println!("{}", entry.get(&opt.format).unwrap());
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
