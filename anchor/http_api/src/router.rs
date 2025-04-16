//! The routes for the HTTP API

use api_types::{GenericResponse, VersionData};
use axum::{body::Body, routing::get, Json, Router};
use std::sync::Arc;
use parking_lot::RwLock;
use types::EthSpec;
use validator_services::duties_service::{self, DutiesService};
use crate::Shared;
use network::{peer_manager, Enr};
use network::peer_manager::PeerRecord;
use version::version_with_platform;
use eth2::types::{PeersData};
use crate::PeerId;
use message_receiver::MessageReceiver;
use crate::ValidatorStore;
/// Creates all the routes for HTTP API 

pub fn new<E: EthSpec,/*R: MessageReceiver*/>(shared_state: Arc<RwLock<Shared<E/* ,R*/>>>) -> Router {
    // Default route
    Router::new()
        .route("/", get(root))
        .route("/anchor/version", get(get_version))
        // .route("/anchor/peers", get({
        //     let shared_state = shared_state.clone();
        //     move || async move { get_peers(shared_state).await }
        // }))
        .route("/anchor/validators", get({
            let shared_state = shared_state.clone();
            move || async move { get_validators(shared_state).await }
        }))
}

// Temporary return value.
async fn root() -> &'static str {
    "Anchor client"
}

async fn get_version() -> Json<GenericResponse<VersionData>> {
    Json(GenericResponse::from(VersionData {
        version: version_with_platform(),
    }))
}

// async fn get_peers<E: EthSpec>(shared_state: Arc<RwLock<Shared<E>>>) -> Json<GenericResponse<Vec<PeersData>>> {
//     let peers = shared_state.read().peers.clone().unwrap_or_default();
//     let peers_data = peers.read().clone();
//     Json(GenericResponse::from(peers_data))

// }

// async fn get_peers<E: EthSpec,R: MessageReceiver>(shared_state: Arc<RwLock<Shared<E,R>>>) -> Json<GenericResponse<Vec<(String,String)>>> {
//     // let peers = shared_state.read().peers.clone().unwrap_or_default();
//     // let peers_data = peers.read().clone();
//     // Json(GenericResponse::from(peers_data)) 
//     let peers = shared_state.read().network.clone().unwrap(); 


// }

async fn get_validators<E: EthSpec>(
    shared_state: Arc<RwLock<Shared<E>>>,
) -> Body {
    let shared = shared_state.read(); 

    if let Some(database_state) = &shared.database_state {
        let state_ref = database_state.borrow();
        let validators = state_ref.metadata().values().collect::<Vec<_>>();
        let num_validators = validators.len();
        
        Body::new(num_validators.to_string())
        // if let Some(duties_service) = &shared.duties_service {
        // let validators = duties_service.validator_store;
        // let num_validators = duties_service::ValidatorStore::num_voting_validators(&validators); 

        // let body = serde_json::to_string(&num_validators).unwrap();
        // Body::new(body)
    } else {
        Body::new("nothing".to_string())
    }
}
