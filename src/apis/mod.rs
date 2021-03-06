use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod about_api;
pub use self::about_api::{ AboutApi, AboutApiClient };
mod acl_profile_api;
pub use self::acl_profile_api::{ AclProfileApi, AclProfileApiClient };
mod authorization_group_api;
pub use self::authorization_group_api::{ AuthorizationGroupApi, AuthorizationGroupApiClient };
mod bridge_api;
pub use self::bridge_api::{ BridgeApi, BridgeApiClient };
mod client_api;
pub use self::client_api::{ ClientApi, ClientApiClient };
mod client_profile_api;
pub use self::client_profile_api::{ ClientProfileApi, ClientProfileApiClient };
mod client_username_api;
pub use self::client_username_api::{ ClientUsernameApi, ClientUsernameApiClient };
mod config_sync_remote_node_api;
pub use self::config_sync_remote_node_api::{ ConfigSyncRemoteNodeApi, ConfigSyncRemoteNodeApiClient };
mod default_api;
pub use self::default_api::{ DefaultApi, DefaultApiClient };
mod distributed_cache_api;
pub use self::distributed_cache_api::{ DistributedCacheApi, DistributedCacheApiClient };
mod dmr_bridge_api;
pub use self::dmr_bridge_api::{ DmrBridgeApi, DmrBridgeApiClient };
mod dmr_cluster_api;
pub use self::dmr_cluster_api::{ DmrClusterApi, DmrClusterApiClient };
mod jndi_api;
pub use self::jndi_api::{ JndiApi, JndiApiClient };
mod mqtt_retain_cache_api;
pub use self::mqtt_retain_cache_api::{ MqttRetainCacheApi, MqttRetainCacheApiClient };
mod mqtt_session_api;
pub use self::mqtt_session_api::{ MqttSessionApi, MqttSessionApiClient };
mod msg_vpn_api;
pub use self::msg_vpn_api::{ MsgVpnApi, MsgVpnApiClient };
mod queue_api;
pub use self::queue_api::{ QueueApi, QueueApiClient };
mod replay_log_api;
pub use self::replay_log_api::{ ReplayLogApi, ReplayLogApiClient };
mod replicated_topic_api;
pub use self::replicated_topic_api::{ ReplicatedTopicApi, ReplicatedTopicApiClient };
mod rest_delivery_point_api;
pub use self::rest_delivery_point_api::{ RestDeliveryPointApi, RestDeliveryPointApiClient };
mod topic_endpoint_api;
pub use self::topic_endpoint_api::{ TopicEndpointApi, TopicEndpointApiClient };
mod transaction_api;
pub use self::transaction_api::{ TransactionApi, TransactionApiClient };

pub mod configuration;
pub mod client;
