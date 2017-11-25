#[macro_use]
extern crate log;

extern crate mxo_env_logger;
use mxo_env_logger::*;

#[allow(unused_must_use)]
fn main() {

    init();

    std::fs::File::open("Hentai.you").log_err(|e| error!("File open failed: {:?}", e.description()));

    std::env::args().nth(1000).ok_or("No 1000 Args").log_err(
        |e| {
            error!("{}", e)
        },
    );

    mock::fun()
}

mod mock {
    pub fn fun() {
        trace!("Trace her");
        info!("Info u");
        warn!("Warning her");
        error!("Error world!");
    }
}

#[test]
fn test() {
    main()
}
