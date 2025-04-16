//! The routes for the HTTP API

use crate::PeerId;
use crate::Shared;
use crate::ValidatorStore;
use api_types::{GenericResponse, VersionData};
use axum::{body::Body, routing::get, Json, Router};
use eth2::types::PeersData;
use message_receiver::MessageReceiver;
use network::peer_manager::PeerRecord;
use network::{peer_manager, Enr};
use parking_lot::RwLock;
use ssv_types::ValidatorMetadata;
use std::sync::Arc;
use types::EthSpec;
use validator_services::duties_service::{self, DutiesService};
use version::version_with_platform;
/// Creates all the routes for HTTP API

pub fn new<E: EthSpec /*R: MessageReceiver*/>(
    shared_state: Arc<RwLock<Shared<E /* ,R*/>>>,
) -> Router {
    // Default route
    Router::new()
        .route("/", get(root))
        .route("/anchor/version", get(get_version))
        // .route("/anchor/peers", get({
        //     let shared_state = shared_state.clone();
        //     move || async move { get_peers(shared_state).await }
        // }))
        .route(
            "/anchor/validators",
            get({
                let shared_state = shared_state.clone();
                move || async move { get_validators(shared_state).await }
            }),
        )
        .route(
            "/anchor/validators2",
            get({
                let shared_state = shared_state.clone();
                move || async move { get_validators_2(shared_state).await }
            }),
        )
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

// async fn get_peers<E: EthSpec>(
//     shared_state: Arc<RwLock<Shared<E>>>,
// ) -> Body {
//     let shared = shared_state.read();
//     if let Some(database_state) = &shared.database_state {
//         let state_ref = database_state.borrow();
//         let peers = state_ref.m
//     }
//     Body::new(peers.len().to_string())
// }
#[derive(serde::Serialize)]
struct ValidatorResponse {
    public_key: String,
    cluster_id: String,
    index: Option<usize>,
    // graffiti: String,
}
fn validator_to_response(validator: &ValidatorMetadata) -> ValidatorResponse {
    ValidatorResponse {
        public_key: validator.public_key.to_string(),
        cluster_id: format!("{:?}", validator.cluster_id),
        index: validator.index.map(|i| i.0),
        // graffiti: hex::encode(validator.graffiti.0),
    }
}

async fn get_validators<E: EthSpec>(
    shared_state: Arc<RwLock<Shared<E>>>,
) -> Json<GenericResponse<Vec<ValidatorResponse>>> {
    let shared = shared_state.read();

    if let Some(database_state) = &shared.database_state {
        let state_ref = database_state.borrow();
        let validators = state_ref
            .metadata()
            .values()
            .map(|v| validator_to_response(v))
            .collect::<Vec<_>>();

        Json(GenericResponse::from(validators))
    } else {
        Json(GenericResponse::from(Vec::new()))
    }
}

async fn get_validators_2<E: EthSpec>(
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
