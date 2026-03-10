use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut heartbeat = tokio::time::interval(Duration::from_secs(30));

    println!("banking worker started");

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("banking worker shutting down");
                break;
            }
            _ = heartbeat.tick() => {
                println!("banking worker heartbeat");
            }
        }
    }
}
