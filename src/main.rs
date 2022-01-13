use kubelet::config::Config;
use kubelet::Kubelet;

mod provider;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let provider = provider::MyProvider;

    // Get a configuration for the Kubelet
    let kubelet_config = Config::new_from_file_and_flags(env!("CARGO_PKG_VERSION"), None);

    // Load a kubernetes configuration
    let kubeconfig = kubelet::bootstrap(&kubelet_config, &kubelet_config.bootstrap_file, notify_bootstrap).await?;

    // Instantiate the Kubelet
    let kubelet = Kubelet::new(provider, kubeconfig, kubelet_config).await.unwrap();
    
    // Start the Kubelet and block on it
    Ok(kubelet.start().await.unwrap())
}

fn notify_bootstrap(message: String) {
    println!("BOOTSTRAP: {}", message);
}