/*

  This file is a module with name 'config', unlike ruby we need not declare the module. Filename is the module name

*/
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs::File;
use serde_json;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Config {
  pub regex: String,
  pub matches: BTreeMap<String, String>
}

pub fn default() -> self::Config {
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
}

pub fn from_file(f: PathBuf) -> Result<self::Config, String> {
  match File::open(f.clone()) {
    Ok(file) => {
      let config: Result<Config, serde_json::Error> = serde_json::from_reader(file);
      match config {
        Ok(c) => { Ok(c) }
        Err(e) => {
          Err(format!("Failed to parse config: '{}'", e))
        }
      }
    }
    Err(e) => {
      Err(format!("Failed to read config file '{}': {}", f.to_str().unwrap(), e))
    }
  }

}
