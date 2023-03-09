use log::{error};

fn main() {
    env_logger::Builder::new().parse_filters("info").init();
    webserver::lib().unwrap_or_else(|e| {
        error!("webserver err: {e}");
    });
}
