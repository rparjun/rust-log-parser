# Rust log parser

A CLI tool to parse log files and convert every line to a json, output certain values from logs

## Config file

Config file defines the pattern to look for and what all are the matches.

Sample config file:

```
{
  "regex": "^(\\d{1,3}.\\d{1,3}.\\d{1,3}.\\d{1,3}) \\[(\\S+ \\+\\d{4})\\] \"(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (\\S+) (\\S+)\" (\\d{3}) \"rt=(\\S+)\" \"(\\S+)\" \"(.*)\"$",
  "matches": {
    "1": "ip",
    "2": "date",
    "3": "method",
    "4": "path",
    "5": "version",
    "6": "code",
    "7": "rt",
    "8": "referer",
    "9": "ua"
  }
}
	
```
`matches` key provides names to the regex matches, this name can be used for filtering the output.

### Building


### Usage

#### Arguments
`-i: Log file to read, default is stdin`

`-f: output format,default is json. It should be one of the value from the matches hash in config file.`

`-c: config file`

`-s: exit program upon receiving a line which cannot be parsed with the regex provided.`


### Example

access.log
```
34.193.53.146 [11/Feb/2019:10:35:02 +0000] "GET /index.html HTTP/1.1" 200 "rt=0.016" "https://google.com/" "Google UA"
182.72.211.138 [11/Feb/2019:10:35:06 +0000] "GET /stats.json HTTP/2.0" 200 "rt=0.047" "https://amagi.com/index.html" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.110 Safari/537.36"
```

```
> parser -i access.log -c config.json

{"code":"200","date":"11/Feb/2019:10:35:02 +0000","ip":"34.193.53.146","method":"GET","path":"/index.html","referer":"https://google.com/","rt":"0.016","ua":"Google UA","version":"HTTP/1.1"}
{"code":"200","date":"11/Feb/2019:10:35:06 +0000","ip":"182.72.211.138","method":"GET","path":"/stats.json","referer":"https://amagi.com/index.html","rt":"0.047","ua":"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.110 Safari/537.36","version":"HTTP/2.0"}
```


```
> parser -i access.log -c config.json -f ua
Google UA
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.110 Safari/537.36
```
