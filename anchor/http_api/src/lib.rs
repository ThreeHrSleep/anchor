mod config;
mod router;

use std::{net::SocketAddr, path::PathBuf};

use anchor_validator_store::AnchorValidatorStore;
pub use config::Config;
use slot_clock::SlotClock;
use slot_clock::SystemTimeSlotClock;
use std::sync::Arc;
use task_executor::TaskExecutor;
use tokio::net::TcpListener;
use parking_lot::RwLock;
use tracing::info;
use types::EthSpec;
use eth2::types::{PeerData, PeersData};
use validator_services::duties_service::DutiesService;
use network::peer_manager::{PeerRecord};
use network::Enr;
use network::peer_manager::PeerId;
use network::Network;
use message_receiver::MessageReceiver;
/// A wrapper around all the items required to spawn the HTTP server.
///
/// The server will gracefully handle the case where any fields are `None`.
type ValidatorStore<E> = AnchorValidatorStore<SystemTimeSlotClock, E>;

pub struct Shared<E: EthSpec,/*R: MessageReceiver*/> {
    // pub network: Option<Network<R>>,  // Changed to Option<Network<R>>
    pub duties_service: Option<Arc<DutiesService<ValidatorStore<E>, SystemTimeSlotClock>>>,
    // pub peers: Option<Arc<RwLock<Vec<(&PeerId, &PeerRecord<Enr>)>>>>,
}
pub struct Context<T: SlotClock> {
    pub task_executor: TaskExecutor,
    // TODO: Protect the API endpoint
    // pub api_secret: ApiSecret,
    pub secrets_dir: Option<PathBuf>,
    // TODO: Handle graffiti
    // pub graffiti_file: Option<GraffitiFile>,
    // pub graffiti_flag: Option<Graffiti>,
    // TODO:Add differing chainspecs
    // pub spec: ChainSpec,
    pub config: Config,
    pub slot_clock: T,
}

/// Runs the HTTP API server
pub async fn run<E: EthSpec/* ,R: MessageReceiver*/>(config: Config, shared_state: Arc<RwLock<Shared<E/* ,R*/>>>) -> Result<(), String> {
    if !config.enabled {
        info!("HTTP API Disabled");
        return Ok(());
    }

    // Generate the axum routes
    let router = router::new(shared_state);

    // Set up a listening address

    let socket = SocketAddr::new(config.listen_addr, config.listen_port);
    let listener = TcpListener::bind(socket).await.map_err(|e| e.to_string())?;

    // Start the http api server
    axum::serve(listener, router)
        .await
        .map_err(|e| format!("{}", e))
}
