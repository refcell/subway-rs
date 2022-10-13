use ansi_term::Colour::{Blue, Green};
use ethers::prelude::*;
use eyre::Result;
use subway_rs::{banner, utils};

#[tokio::main]
async fn main() -> Result<()> {
    // Clear the screen and print the banner
    print!("{}[2J", 27 as char);
    println!("{}", banner::CABRET);

    // Create the websocket clieant
    let client = utils::create_websocket_client().await?;

    // Get the http provider for flashbots use
    let http_provider = utils::get_http_provider()?;

    // Preload environment variable types
    let searcher_wallet = utils::get_searcher_wallet()?;
    let searcher_wallet_address = searcher_wallet.address();
    let wallet_balance = http_provider
        .get_balance(searcher_wallet_address, None)
        .await?;

    println!(
        "{}",
        Blue.paint(format!(
            "Listening for transactions from {:?}",
            searcher_wallet_address
        ))
    );
    println!(
        "{}",
        Blue.paint(format!("Wallet Balance: {:?}", wallet_balance))
    );

    // Create pending stream
    let stream = if let Ok(c) = client.watch_pending_transactions().await {
        c
    } else {
        panic!("Failed to create filter watcher for pending transactions!");
    };

    // Create transaction stream
    let mut tx_stream = stream.transactions_unordered(usize::MAX);

    // Listening to pending transactions
    while let Some(tx) = tx_stream.next().await {
        // Unwrap the transaction
        let tx = match tx {
            Ok(tx) => tx,
            Err(_) => {
                continue;
            }
        };

        // Match on tx.from
        if tx.from != searcher_wallet_address {
            println!(
                "{}",
                Green.paint(format!("[DETECTED] Transaction\n{:#?}", tx))
            );
        }
    }

    Ok(())
}
