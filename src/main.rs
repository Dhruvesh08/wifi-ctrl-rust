use env_logger::Env;
use log::{error, info};
use wifi_ctrl::{sta, Result};

#[tokio::main]
async fn main() -> Result {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting wifi-sta example");

    let mut setup = sta::WifiSetup::new()?;

    let proposed_path = format!("/var/run/wpa_supplicant/wlan0");

    setup.set_socket_path(proposed_path);

    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, _app, ) = tokio::join!(
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

    let networks = requester.get_networks().await?;
    info!("Known networks");
    for networks in networks.iter() {
        info!("   {:?}", networks);
    }
    info!("Shutting down");
    requester.shutdown().await?;
    Ok(())
}




