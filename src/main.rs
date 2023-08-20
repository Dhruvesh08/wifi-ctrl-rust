use env_logger::Env;

use log::{error, info};

use wifi_ctrl::{sta, Result};

async fn scan_wifi() -> Result {
    let mut setup = sta::WifiSetup::new()?;

    // Set up Wi-Fi interface with fixed socket path
    let proposed_path = "/var/run/wpa_supplicant/wlan0";
    setup.set_socket_path(proposed_path);

    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, _app) = tokio::join!(
        async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {e}");
            }
        },
        app(requester),
    );
    Ok(())
}

async fn app(requester: sta::RequestClient) -> Result {
    info!("Requesting scan");
    let scan = requester.get_scan().await?;
    info!("Scan complete");
    for scan in scan.iter() {
        info!("   {:?}", scan);
    }

    requester.shutdown().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize logging and other setup
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting wifi-sta example");

    // Call the scan_wifi function to get the list of networks
    match scan_wifi().await {
        Ok(_) => info!("Scan complete"),
        Err(e) => error!("Error: {}", e),
    }
}
