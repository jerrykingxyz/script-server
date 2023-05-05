# script-server

A tiny http server to run scripts.

## Installation

Download the binary file from [release page](https://github.com/jerrykingxyz/script-server/releases).

## Building

`script-server` is written in Rust, so you will need to grab a [Rust installation](https://www.rust-lang.org/) in order to compile it.

To build script-server
``` bash
$ git clone https://github.com/jerrykingxyz/script-server
$ cd script-server
$ cargo build --release
$ ./target/release/script-server --help
```

To run the full test
``` bash
$ cargo test
```

## Usage

Run `script-server --help` to show command usage help.

``` bash
Usage: script-server [OPTIONS] <SCRIPTS_DIR>

Arguments:
  <SCRIPTS_DIR> Scripts dir

Options:
  -l, --listen <LISTEN>  Listen address [default: 0.0.0.0:8000]
  -t, --token <TOKEN>    Access token in header
  -h, --help             Print help
  -V, --version          Print version
```

The request API is
``` text
curl -XPOST -H "X-ACCESS-TOKEN: <TOKEN>" -d "<Args>" http://<LISTEN>/<SCRIPT_PATH>
```

* The method only supports `POST`, others will response with 404
* The `X-ACCESS-TOKEN` header is required to configure the access token if exist
* You can add script parameters to the http body and separate each parameter with `\n`
* The `<SCRIPT_PATH>` is the relative path of the execute script in `<SCRIPTS_DIR>`

The response is
| status code | description                                                               | body           |
|-------------|---------------------------------------------------------------------------|----------------|
| 404         | Request method is not POST                                                | -              |
| 401         | Incorrect access token                                                    | -              |
| 403         | The execution script does not exist or does not have execution permission | -              |
| 500         | The stderr is not empty after script execution                            | stderr content |
| 200         | Execute the script successfully                                           | stdout content |

See [the test case](/tests) for more usage examples

## License

MIT licensed
