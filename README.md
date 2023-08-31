# onomondo softsim-cli (ss_cli)

## Build
`cargo build --release`
## Test
`cargo test`
## Benchmark
Assuming you faked a bunch of profiles:

```
hyperfine --runs 1000 --warmup=1 --shell=none './target/release/ss_cli next --key resources/test/key'
Benchmark 1: ./target/release/ss_cli next --key resources/test/key
  Time (mean ± σ):       3.0 ms ±   0.8 ms    [User: 1.5 ms, System: 1.5 ms]
  Range (min … max):     2.4 ms …  24.4 ms    1000 runs
```

This solustion doesn't scale well above millions of profiles, but good enough for the standard prodction line. 


## Installing commitlint + commit hook
```
npm install --save-dev @commitlint/{cli,config-conventional}
npx husky install
npx husky add .husky/commit-msg  'npx --no -- commitlint --edit ${1}'
```


## Usage
```
Usage: ss_cli [OPTIONS] <COMMAND>

Commands:
  fetch
          Fetch profiles from API
  next
          Find next available profile. Decrypt and decode the profile and mark it as used
  help
          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbosity...
          Verbosity level
  -h, --help
          Print help
  -V, --version
          Print version
```
### Examples
Set log level to `TRACE`
```
ss_cli -vvv ---help
```

### Fetch
Pulls profiles form api.onomondo.com and writes to disk. Specify `count` to fetch many for production usage. `ss_cli` breaks the count into batches of max 1000. 

```
Usage: ss_cli fetch [OPTIONS] --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>
          
  -n, --count <NUM_OF_PROFILES>
          [default: 1]
  -o, --out <OUTPUT>
          [default: profiles]
  -e, --endpoint <ENDPOINT>
          [default: https://api.onomondo.com/sims/profile]
  -h, --help
          Print help
```


### Examples

Get 10000 profiles and store under `./profile/` 
      
```
ss_cli fetch -a <your_api_key> -n 10000
```

Specify output path
```
ss_cli fetch -a <your_api_key> -n 1000 -o "batch1"
```

Optionally edit the api endpoint. Handy for local tests. 
```
ss_cli fetch -a <your_api_key> -n 1000 -e http:/localhost:44111
```


### Next 

Find next available profile and outputs decrypted and decoded values. Specify `format` to change encoding. 

`HEX` - suitable for SoftSIM integrations made by Onomondo

`JSON` - outputs RAW profile and relevant meta information. 

```
Usage: ss_cli next [OPTIONS] --key <KEY>

Options:
  -k, --key <KEY>
          Path to private key
  -i, --in <SET_OF_PROFILES>
          Path to encrypted profiles [default: ./profiles]
      --format[=<FORMAT>]
          Output format [default: hex] [possible values: hex, json]
  -h, --help
          Print help
```

### Example

Write hex encoded profiles to stdout. Optionally this can be piped directly to a device if ready to receive profile/
```
ss_cli next --key resources/test/key
```

Specify format to `json`
```
ss_cli next --key resources/test/key --format=json
```
