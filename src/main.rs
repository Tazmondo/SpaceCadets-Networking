use std::env;

mod client;
mod server;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mode = args
        .get(1)
        .expect("Must provide a server or client argument.")
        .to_lowercase();

    match mode.as_str() {
        "client" => client::start(),
        "server" => server::start(),
        _ => panic!("Mode must be client or server"),
    }
}
