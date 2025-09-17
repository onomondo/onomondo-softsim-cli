# Onomondo SoftSIM CLI

A little toolbox to assist with provisioning of your SoftSIM-enabled device.

Provisioning is split into two distinct steps:

1. **Pre-production**: Fetch `n` profiles from the Onomondo API to avoid excessive load on the API and eliminate dependencies on stable internet. The profiles are encrypted using your public key.
2. **During production**: Continuously retrieve a new unique profile formatted to specifications.

These steps correspond one-to-one with the available commands in the CLI tool.

### Command: Fetch
This command fetches the specified number of profiles from the Onomondo API. Use the SoftSIM API key generated on our platform to gain access.

### Command: Next
This command finds the next unused profile on your local system. The profile is decrypted using the private key specified by the `--key` argument. After decryption and encoding, the file is prepended with `__` to invalidate the profile (a way of marking the profile as used).

## Installation
Pre-built binaries can be found under [releases](https://github.com/onomondo/onomondo-softsim-cli/releases).

Optionally, you can build the project from source. See the [build](#build) section below.

## Generate public/private key pair
To generate a SoftSIM API key, you will need a key pair. Use 4096-bit keys.

To generate a key pair:
```console
ssh-keygen -t rsa -m PEM -b 4096 -f <path_to_new_key>
```

> The Onomondo SoftSIM CLI tool does not support keys protected by a password.

The public key must be PEM-encoded:
```
-----BEGIN PUBLIC KEY-----
.....
-----END PUBLIC KEY-----
```
This can be obtained with:
```console
ssh-keygen -e -m PKCS8 -f <path_to_public_key>.pub
```

Use the public key to create an API key at https://app.onomondo.com/api-keys/softsim/new

*For testing purposes, https://cryptotools.net/rsagen can be helpful to quickly get started.*

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
Set the log level to `TRACE`:
```
softsim -vvv --help
```

### Fetch
Pull profiles from api.onomondo.com and write them to disk. Specify `count` to fetch multiple profiles. `softsim` breaks the count into batches of up to 1000.

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
Fetch 5678 profiles and store them under `./profiles/`:
```
softsim fetch -a <your_api_key> -n 5678
```

Specify an output path:
```
softsim fetch -a <your_api_key> -n 5678 -o "batch1"
```

### Next
Find the next available profile and output decrypted and decoded values. Specify `format` to change encoding.

- `HEX`: Suitable for SoftSIM integrations made by Onomondo.
- `RAW`: Suitable for those interested to decode the content of the SoftSIM HEX format
- `JSON`: Outputs profile data and relevant metadata in a JSON format

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

### SoftSIM profile illustration
The SoftSIM profile is represented in the following format when fetched from Onomondo. The SoftSIM is encrypted in this format:
```
{"iccid":"89457300000000000000","profile":"gwixsycJq295xfHxOvwjiNwj8feRHeDwIUsR8xhTBej31CxUKc9Axw1LGffdaIMGlBMx2XxGO1M7ZJHqG4kKcypmIc19vn8Iu4vthoxzRtMavTk+w+0yp1dZbZdhnsDZd96Zt3upKPXTNFoG+m8BOwmBR5lGlzdCuJytvHpPV5WcyL0Tdy5K2zyhZh2V9j+DhwrVrVyciJeWWRUzDSScaS+VhhrSo0EtsrfVamIJDv4XtWrseVnn6fh1ArlftTNbMcC/qpT/Q2UGc4lyVaDKjqZeFYoUR6cmVhlK55gRL+kPJ6qYUsbtgh1rcqjrs4S6xpIJnCgvR2wVpFJqGOhnyEtFFw5CgKvZol0ixNn6IPOyMyPHzyUe7UuyyFUPk5kDR29vjb+hZN1hh354lEOwMOpMFYBVt2Ug66Zs5eATVC5Vv7QdOsyTgOqvINmPUDvIwfTFMiG3t7rWXs7wFJKYLiU764rTGrTjS1yTzFIGpEqkze68b9Ehx6APB0KVeUQM2UB2439VUlcZ2CAwN+qvsycPfBlX1iIN2vjG7ZUWi0SQ9jrOA1xEvgBgqa1EDkkv5j1usEtm3Zu5EvZlsLbMdmai2GWX0p99BFf2WpwqPI4FMflntefZ9RdzPPc4XWp1PCBUfMDMCyeqJEb34aGAtASt+DlKLlXmcYczkQoe5mM="}
```
Following a successful decryption and formatting of the encrypted SoftSIM profile, the CLI tool exports the profile in the following format. It is this and only this format that is accepted by SoftSIM-enabled devices by Onomondo:
```
01120809101010325406360214980010325476981032140320000000000000000000000000000000000420000102030405060708090A0B0C0D0E0F0520000102030405060708090A0B0C0D0E0F0620000102030405060708090A0B0C0D0E0F
```

### Example
Write hex encoded profiles to stdout. Optionally, this can be piped directly to a device if the device is ready to receive a profile in this specific format.

`--key` should point to the private key generated in the previous steps:
```
softsim next --key <path_to_private_key>
```

Specify the format as `hex`:
```
softsim next --key resources/test/key --format=hex
```

`softsim next` can be called from manufacturing scripts as needed.

## Build
Build the project using Cargo:
```
cargo build --release
```

After building, the SoftSIM executable is located at: `target/release/softsim`

## Test
Run tests:
```
cargo test
```

## Benchmark
In the scenario of simulating profiles:

```
hyperfine --runs 1000 --warmup=1 --shell=none './target/release/softsim next --key resources/test/key'
Benchmark 1: ./target/release/softsim next --key resources/test/key
  Time (mean ± σ):       3.0 ms ±   0.8 ms    [User: 1.5 ms, System: 1.5 ms]
  Range (min … max):     2.4 ms …  24.4 ms    1000 runs
```

This should perform well on a standard production line but may encounter issues when processing a threshold of a million profiles.

## Installing commitlint + commit hook
Install commitlint and set up a commit hook:
```
npm install --save-dev @commitlint/{cli,config-conventional}
npx husky install
```
