use kubelet::config::Config;
use kubelet::Kubelet;
use std::sync::Arc;
use tracing::*;

//use oci_distribution::Client;

mod myprovider;
mod states;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger.
    // Set RUST_LOG environment variable on host to desired logging level (info,debug,error,etc.)
    tracing_subscriber::fmt().init();

    // Get a configuration for the Kubelet.
    info!("Preparing kubelet config.");
    let kubelet_config = Config::new_from_file_and_flags(env!("CARGO_PKG_VERSION"), None);

    // Load a kubernetes configuration.
    // The bootstrap process happens on first run, whereby TLS certificates and the certificate
    // signing request (CSR) are created before the node can join the cluster.
    info!("Bootstrapping process for kubelet.");
    let kubeconfig = kubelet::bootstrap(&kubelet_config, &kubelet_config.bootstrap_file, notify_bootstrap).await?;

    // Configure the file store to persist container images on the host
    info!("Configuring file store for images.");
    let mut store_path = kubelet_config.data_dir.join(".oci");
    store_path.push("modules");
    let store = Arc::new(
        kubelet::store::oci::FileStore::new(
            oci_distribution::Client::from_source(&kubelet_config),
            &store_path));

    // Create our provider that has our custom kubelet logic.
    info!("Creating provider instance.");
    let provider = myprovider::MyProvider::new(kubeconfig.clone(), store).await?;

    // Instantiate the Kubelet using an instance of our provider.
    info!("Creating kubelet instance.");
    let kubelet = Kubelet::new(provider, kubeconfig, kubelet_config).await.unwrap();
    
    // Start the Kubelet and block on it.
    info!("Staring kubelet.");
    Ok(kubelet.start().await.unwrap())
}

fn notify_bootstrap(message: String) {
    println!("BOOTSTRAP: {}", message);
}