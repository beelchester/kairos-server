use std::net::TcpListener;

use kairos_server::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:33333").expect("Failed to bind random port");
    println!(
        "Listening on port {}",
        listener.local_addr().unwrap().port()
    );
    run(listener).await
}
