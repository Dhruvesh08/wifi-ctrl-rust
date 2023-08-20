use std::error::Error;

use env_logger::Env;
use log::info;
use wifi_ctrl::sta;



async fn get_wifi_networks() -> Result<Vec<wifi_ctrl::sta::ScanResult>, Box<dyn Error>> {
    // Initialize logging and other setup
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting get_wifi_networks example");

    // Set up Wi-Fi interface with fixed socket path
    let mut setup = sta::WifiSetup::new()?;
    let proposed_path = "/var/run/wpa_supplicant/wlan0";

    setup.set_socket_path(proposed_path);

    let requester = setup.get_request_client();
    let networks = requester.get_scan().await?;

    // Shutdown and return the networks
    requester.shutdown().await?;
    
    Ok(networks.to_vec())
}
#[tokio::main]
async fn main() {
    match get_wifi_networks().await {
        Ok(networks) => {
            println!("Available Wi-Fi Networks:");
            for network in networks {
                println!("{:?}", network);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
