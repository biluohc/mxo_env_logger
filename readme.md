[![Build status](https://travis-ci.org/biluohc//mxo_env_logger.svg?branch=master)](https://github.com/biluohc/mxo_env_logger)

## [`mxo_env_logger`](https://github.com/biluohc/mxo_env_logger)
Another `env_logger` wrapped.

### Cargo.toml:
```toml
[dependencies]
mxo_env_logger = "^0.1"
log = "^0.3.8"
```

### Code:
```rust
#[macro_use]
extern crate log;

extern crate mxo_env_logger;
use mxo_env_logger::*; // * contains StdErr(std::error::Error), so can use `e.description()` direct.

fn main() {
    init();

    trace!("Trace her"); // default closed.
    info!("Info u");
    warn!("Warning her");
    error!("Error world!");

    std::fs::File::open("Hentai.you").log_err(|e| error!("File open failed: {:?}", e.description()));
    std::env::args().nth(1000).ok_or("No 1000 Args").log_err(|e| error!("{}", e));
}
```

### Output:
```sh
INFO #main::mock:26: Info u
WARN #main::mock:27: Warning her
ERROR#main::mock:28: Error world!
ERROR#main:12: File open failed: "entity not found"
ERROR#main:16: No 1000 Args
```
