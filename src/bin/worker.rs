use anyhow::{Context, Result};
use log::{self, LevelFilter};
use simple_logger::SimpleLogger;
use zeromq_rust::{TradeEvent, PUSH_PORT};

#[allow(unreachable_code)]

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(LevelFilter::Info)
        .init()
        .with_context(|| "Creating logger.")?;

    let context: zmq::Context = zmq::Context::new();
    let socket = context
        .socket(zmq::PULL)
        .with_context(|| "Creating socket")?;

    socket
        .connect(format!("tcp://127.0.0.1:{}", PUSH_PORT).as_str())
        .with_context(|| "Connecting to server.")?;

    log::info!("Ready to receive trade events!");

    loop {
        if let Ok(data) = socket.recv_string(0) {
            if let Ok(serialised_trade) = data {
                let trade_event = serde_json::from_str::<TradeEvent>(serialised_trade.as_str())
                    .with_context(|| "Deserialising trade event")?;
                log::info!("Received trade event: {:?}", trade_event);
                log::info!("Process trade event here");
            } else {
                log::error!("Failed to receive message: {:?}", data);
            }
        }
    }
    Ok(())
}
