# SoftSIM CLI

Small tools to help out with provisioning before and during production of your SoftSIM enabled device. 

Provisioning is split in two distinct steps:

1. Pre-production. Fetch `n` profiles from the Onomondo API to avoid exessive load on API and to remove any dependencies on stable internet. The profiles are encrypted using your public key.
2. During production. Continiously get a new unique profile correctly formatted to specifications. 

These steps correspond one-to-one with the available commands in the CLI tool.

### Command: Fetch
This options fetches the specified number of profiles from the Onomondo API. Use the SoftSIM API generated in our platform to get access. 

### Command: Next
This option find the next unused profile on your local system. The profile is decrypted using the private key pointed to by the `--key` argument. After decryption and encoding the file is prepended with `__` to invalidate the profile.  


## Generate public / private key-pair
To generate a SoftSIM API key you need a key pair as well. 

To generate a key pair:
```console
ssh-keygen -t rsa -m PEM -b 4096 -f <path_to_new_key>
```

The public key is expected to be PEM encoded 
```
-----BEGIN PUBLIC KEY-----
.....
-----END PUBLIC KEY-----
```
Which can be obtained with 
```console
ssh-keygen -e -m PKCS8 -f <path_to_public_key>.pub
```

Use the to public key to create an API key on https://app.onomondo.com/

*For testing* https://cryptotools.net/rsagen can be helpful to quickly get started. Use at least 2048 bit keys. 

## Installation
Prebuild binaries can be found under releases. Optionally build from source. See relevant section below. 

## Usage
```
Usage: softsim [OPTIONS] <COMMAND>

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
### Log level
Set log level to `TRACE`
```
softsim -vvv ---help
```

### Fetch
Pulls profiles form api.onomondo.com and writes to disk. Specify `count` to fetch many for production usage. `softsim` breaks the count into batches of max 1000. 

```
Usage: softsim fetch [OPTIONS] --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>
          
  -n, --count <NUM_OF_PROFILES>
          [default: 1]
  -o, --out <OUTPUT>
          [default: profiles]
  -u, --url <url>
          [default: https://api.onomondo.com/sims/profiles]
  -h, --help
          Print help
```


### Examples

Get 5678 profiles and store under `./profile/` 
      
```
softsim fetch -a <your_api_key> -n 5678
```

Specify output path
```
softsim fetch -a <your_api_key> -n 1000 -o "batch1"
```

### Next 

Find next available profile and outputs decrypted and decoded values. Specify `format` to change encoding. 

`HEX` - suitable for SoftSIM integrations made by Onomondo

`JSON` - outputs RAW profile and relevant meta information. 

```
Usage: softsim next [OPTIONS] --key <KEY>

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

Write hex encoded profiles to stdout. Optionally this can be piped directly to a device if ready to receive profile.

`--key` should point to the private key generated in the previous steps. 


```
softsim next --key <path_to_private_key>
```

Specify format to `json`
```
softsim next --key resources/test/key --format=json
```

`softsim next` can be called from manufacturing scripts as needed.


## Build
`cargo build --release`
## Test
`cargo test`
## Benchmark
Assuming you faked a bunch of profiles:

```
hyperfine --runs 1000 --warmup=1 --shell=none './target/release/softsim next --key resources/test/key'
Benchmark 1: ./target/release/softsim next --key resources/test/key
  Time (mean ± σ):       3.0 ms ±   0.8 ms    [User: 1.5 ms, System: 1.5 ms]
  Range (min … max):     2.4 ms …  24.4 ms    1000 runs
```

This solustion doesn't scale well above millions of profiles, but good enough for the standard prodction line. 


## Installing commitlint + commit hook
```
npm install --save-dev @commitlint/{cli,config-conventional}
npx husky install
```
