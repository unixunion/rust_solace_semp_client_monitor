use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  about_api: Box<::apis::AboutApi>,
  acl_profile_api: Box<::apis::AclProfileApi>,
  authorization_group_api: Box<::apis::AuthorizationGroupApi>,
  bridge_api: Box<::apis::BridgeApi>,
  client_profile_api: Box<::apis::ClientProfileApi>,
  client_username_api: Box<::apis::ClientUsernameApi>,
  default_api: Box<::apis::DefaultApi>,
  distributed_cache_api: Box<::apis::DistributedCacheApi>,
  dmr_bridge_api: Box<::apis::DmrBridgeApi>,
  dmr_cluster_api: Box<::apis::DmrClusterApi>,
  jndi_api: Box<::apis::JndiApi>,
  mqtt_retain_cache_api: Box<::apis::MqttRetainCacheApi>,
  mqtt_session_api: Box<::apis::MqttSessionApi>,
  msg_vpn_api: Box<::apis::MsgVpnApi>,
  replay_log_api: Box<::apis::ReplayLogApi>,
  rest_delivery_point_api: Box<::apis::RestDeliveryPointApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      about_api: Box::new(::apis::AboutApiClient::new(rc.clone())),
      acl_profile_api: Box::new(::apis::AclProfileApiClient::new(rc.clone())),
      authorization_group_api: Box::new(::apis::AuthorizationGroupApiClient::new(rc.clone())),
      bridge_api: Box::new(::apis::BridgeApiClient::new(rc.clone())),
      client_profile_api: Box::new(::apis::ClientProfileApiClient::new(rc.clone())),
      client_username_api: Box::new(::apis::ClientUsernameApiClient::new(rc.clone())),
      default_api: Box::new(::apis::DefaultApiClient::new(rc.clone())),
      distributed_cache_api: Box::new(::apis::DistributedCacheApiClient::new(rc.clone())),
      dmr_bridge_api: Box::new(::apis::DmrBridgeApiClient::new(rc.clone())),
      dmr_cluster_api: Box::new(::apis::DmrClusterApiClient::new(rc.clone())),
      jndi_api: Box::new(::apis::JndiApiClient::new(rc.clone())),
      mqtt_retain_cache_api: Box::new(::apis::MqttRetainCacheApiClient::new(rc.clone())),
      mqtt_session_api: Box::new(::apis::MqttSessionApiClient::new(rc.clone())),
      msg_vpn_api: Box::new(::apis::MsgVpnApiClient::new(rc.clone())),
      replay_log_api: Box::new(::apis::ReplayLogApiClient::new(rc.clone())),
      rest_delivery_point_api: Box::new(::apis::RestDeliveryPointApiClient::new(rc.clone())),
    }
  }

  pub fn about_api(&self) -> &::apis::AboutApi{
    self.about_api.as_ref()
  }

  pub fn acl_profile_api(&self) -> &::apis::AclProfileApi{
    self.acl_profile_api.as_ref()
  }

  pub fn authorization_group_api(&self) -> &::apis::AuthorizationGroupApi{
    self.authorization_group_api.as_ref()
  }

  pub fn bridge_api(&self) -> &::apis::BridgeApi{
    self.bridge_api.as_ref()
  }

  pub fn client_profile_api(&self) -> &::apis::ClientProfileApi{
    self.client_profile_api.as_ref()
  }

  pub fn client_username_api(&self) -> &::apis::ClientUsernameApi{
    self.client_username_api.as_ref()
  }

  pub fn default_api(&self) -> &::apis::DefaultApi{
    self.default_api.as_ref()
  }

  pub fn distributed_cache_api(&self) -> &::apis::DistributedCacheApi{
    self.distributed_cache_api.as_ref()
  }

  pub fn dmr_bridge_api(&self) -> &::apis::DmrBridgeApi{
    self.dmr_bridge_api.as_ref()
  }

  pub fn dmr_cluster_api(&self) -> &::apis::DmrClusterApi{
    self.dmr_cluster_api.as_ref()
  }

  pub fn jndi_api(&self) -> &::apis::JndiApi{
    self.jndi_api.as_ref()
  }

  pub fn mqtt_retain_cache_api(&self) -> &::apis::MqttRetainCacheApi{
    self.mqtt_retain_cache_api.as_ref()
  }

  pub fn mqtt_session_api(&self) -> &::apis::MqttSessionApi{
    self.mqtt_session_api.as_ref()
  }

  pub fn msg_vpn_api(&self) -> &::apis::MsgVpnApi{
    self.msg_vpn_api.as_ref()
  }

  pub fn replay_log_api(&self) -> &::apis::ReplayLogApi{
    self.replay_log_api.as_ref()
  }

  pub fn rest_delivery_point_api(&self) -> &::apis::RestDeliveryPointApi{
    self.rest_delivery_point_api.as_ref()
  }


}
