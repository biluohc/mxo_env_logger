/*!
# [`mxo_env_logger`](https://github.com/biluohc/mxo_env_logger)
Another `env_logger` wrapped.

## Cargo.toml: 
```toml
[dependencies]
mxo_env_logger = "^0.1"
log = "^0.3.8"
```

## Code: 
```
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

## Output: 
```sh
INFO #main::mock:26: Info u
WARN #main::mock:27: Warning her
ERROR#main::mock:28: Error world!
ERROR#main:12: File open failed: "entity not found"
ERROR#main:16: No 1000 Args
```
*/
extern crate log;
use log::{LogLevelFilter, LogRecord, SetLoggerError};

extern crate env_logger;
use env_logger::LogBuilder;

use std::result::Result;
use std::env;

pub use std::error::Error as StdErr;

/// Initializes the global logger with an env logger.
pub fn init() -> Result<(), SetLoggerError> {
    let format = |record: &LogRecord| {
        format!(
            "{:5}#{}:{}: {}",
            record.level(),
            record.target(),
            record.location().line(),
            record.args()
        )
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    env::var("RUST_LOG").ok().map(|v| builder.parse(&v));
    builder.init()
}

/**
 Use like `map_err`, but it return the origin `E` rather than a new `E`(The `FnOnce` `F` not eat `E`).

 Ps: the macros belongs to `log` crate.

 ```sh
    std::fs::File::open("Hentai.you").log_err(|e| error!("File open failed: {:?}", e.description()));

    std::env::args().nth(1000).ok_or("No 1000 Args").log_err(|e| error!("{}", e));
 ```
 */
pub trait LogErr<E> {
    fn log_err<F: FnOnce(&E)>(self, f: F) -> Self;
}

impl<T, E> LogErr<E> for Result<T, E> {
    fn log_err<F: FnOnce(&E)>(self, f: F) -> Self {
        match self {
            Ok(o) => Ok(o),
            Err(e) => {
                f(&e);
                Err(e)
            }
        }
    }
}
