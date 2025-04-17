use serde::Serialize;

#[derive(Serialize)]
pub struct VersionData {
    pub version: String,
}

#[derive(Serialize)]
pub struct ValidatorData {
    pub public_key: String,
    pub cluster_id: String,
    pub index: Option<usize>,
    pub graffiti: String,
}
#[derive(Serialize)]
pub struct ClusterData{
    /// Unique identifier for a Cluster
    pub cluster_id: String,
    /// The owner of the cluster and all of the validators
    pub owner: String,
    /// The Eth1 fee address for all validators in the cluster
    pub fee_recipient: String,
    /// If the Cluster is liquidated or active
    pub liquidated: bool,
    /// Operators in this cluster
    pub cluster_members: Vec<u64>,
}

#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub data: T,
}

impl<T> From<T> for GenericResponse<T> {
    fn from(data: T) -> Self {
        Self { data }
    }
}
