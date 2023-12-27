use anyhow::{Context, Result};
use log::{self, LevelFilter};
use simple_logger::SimpleLogger;
use std::thread::sleep;
use std::time::Duration;
use zeromq_rust::{TradeEvent, PUSH_PORT};

#[allow(unreachable_code)]
fn main() -> Result<()> {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(LevelFilter::Info)
        .init()
        .with_context(|| "Creating logger.")?;

    let ctx: zmq::Context = zmq::Context::new();
    let socket: zmq::Socket = ctx
        .socket(zmq::SocketType::PUSH)
        .with_context(|| "Creating socket.")?;

    socket
        .bind(format!("tcp://*:{}", PUSH_PORT).as_str())
        .with_context(|| "Binding socket")?;
    log::info!("Server started at localhost:{}", PUSH_PORT);
    let mut trade_count: i32 = 0;
    loop {
        let random_trade = TradeEvent::new_random_trade();
        log::debug!("Random trade generated: {:?}", random_trade);

        if let Ok(serialised_date) = serde_json::to_string(&random_trade) {
            log::debug!("Sending trade event: {}", serialised_date);
            socket
                .send(&serialised_date, 0)
                .with_context(|| "Sending serialised trade")?;
            sleep(Duration::from_secs(1));
            log::debug!("Sent trade #{}", trade_count);
            trade_count += 1;
        } else {
            log::error!("Failed to serialize trade: {:?}", random_trade);
        }
    }
    Ok(())
}
