/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters \"q1\" and \"q2\" ; with values \"val1\" and \"val2\" respectively /SEMP/v2/monitor/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/monitor/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/monitor/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/monitor/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/monitor/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/monitor/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/monitor/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/monitor/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/monitor/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.    
 *
 * OpenAPI spec version: 2.12.00902000014
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnDistributedCacheClusterInstance {
  /// Indicates whether auto-start for the Cache Instance is enabled, and the Cache Instance will automatically attempt to transition from the Stopped operational state to Up whenever it restarts or reconnects to the message broker.
  #[serde(rename = "autoStartEnabled", skip_serializing_if="Option::is_none")]
  auto_start_enabled: Option<bool>,
  /// The name of the Distributed Cache.
  #[serde(rename = "cacheName", skip_serializing_if="Option::is_none")]
  cache_name: Option<String>,
  /// The name of the Cache Cluster.
  #[serde(rename = "clusterName", skip_serializing_if="Option::is_none")]
  cluster_name: Option<String>,
  #[serde(rename = "counter", skip_serializing_if="Option::is_none")]
  counter: Option<::models::MsgVpnDistributedCacheClusterInstanceCounter>,
  /// Indicates whether the Cache Instance is enabled.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The name of the Cache Instance.
  #[serde(rename = "instanceName", skip_serializing_if="Option::is_none")]
  instance_name: Option<String>,
  /// The timestamp of when the Cache Instance last registered with the message broker. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastRegisteredTime", skip_serializing_if="Option::is_none")]
  last_registered_time: Option<i32>,
  /// The timestamp of the last heartbeat message received from the Cache Instance. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastRxHeartbeatTime", skip_serializing_if="Option::is_none")]
  last_rx_heartbeat_time: Option<i32>,
  /// The timestamp of the last request for setting the lost message indication received from the Cache Instance. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastRxSetLostMsgTime", skip_serializing_if="Option::is_none")]
  last_rx_set_lost_msg_time: Option<i32>,
  /// The reason why the Cache Instance was last stopped.
  #[serde(rename = "lastStoppedReason", skip_serializing_if="Option::is_none")]
  last_stopped_reason: Option<String>,
  /// The timestamp of when the Cache Instance was last stopped. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastStoppedTime", skip_serializing_if="Option::is_none")]
  last_stopped_time: Option<i32>,
  /// The timestamp of the last request for clearing the lost message indication transmitted to the Cache Instance. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastTxClearLostMsgTime", skip_serializing_if="Option::is_none")]
  last_tx_clear_lost_msg_time: Option<i32>,
  /// The memory usage of the Cache Instance, in megabytes (MB).
  #[serde(rename = "memoryUsage", skip_serializing_if="Option::is_none")]
  memory_usage: Option<i32>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// Indicates whether one or more messages were lost by the Cache Instance.
  #[serde(rename = "msgsLost", skip_serializing_if="Option::is_none")]
  msgs_lost: Option<bool>,
  #[serde(rename = "rate", skip_serializing_if="Option::is_none")]
  rate: Option<::models::MsgVpnDistributedCacheClusterInstanceRate>,
  /// The operational state of the Cache Instance. The allowed values and their meaning are:  <pre> \"invalid\" - The Cache Instance state is invalid. \"down\" - The Cache Instance is operationally down. \"stopped\" - The Cache Instance has stopped processing cache requests. \"stopped-lost-msg\" - The Cache Instance has stopped due to a lost message. \"register\" - The Cache Instance is registering with the broker. \"config-sync\" - The Cache Instance is synchronizing its configuration with the broker. \"cluster-sync\" - The Cache Instance is synchronizing its messages with the Cache Cluster. \"up\" - The Cache Instance is operationally up. \"backup\" - The Cache Instance is backing up its messages to disk. \"restore\" - The Cache Instance is restoring its messages from disk. \"not-available\" - The Cache Instance state is not available. </pre> 
  #[serde(rename = "state", skip_serializing_if="Option::is_none")]
  state: Option<String>,
  /// Indicates whether stop-on-lost-message is enabled, and the Cache Instance will transition to the Stopped operational state upon losing a message. When Stopped, it cannot accept or respond to cache requests, but continues to cache messages.
  #[serde(rename = "stopOnLostMsgEnabled", skip_serializing_if="Option::is_none")]
  stop_on_lost_msg_enabled: Option<bool>
}

impl MsgVpnDistributedCacheClusterInstance {
  pub fn new() -> MsgVpnDistributedCacheClusterInstance {
    MsgVpnDistributedCacheClusterInstance {
      auto_start_enabled: None,
      cache_name: None,
      cluster_name: None,
      counter: None,
      enabled: None,
      instance_name: None,
      last_registered_time: None,
      last_rx_heartbeat_time: None,
      last_rx_set_lost_msg_time: None,
      last_stopped_reason: None,
      last_stopped_time: None,
      last_tx_clear_lost_msg_time: None,
      memory_usage: None,
      msg_vpn_name: None,
      msgs_lost: None,
      rate: None,
      state: None,
      stop_on_lost_msg_enabled: None
    }
  }

  pub fn set_auto_start_enabled(&mut self, auto_start_enabled: bool) {
    self.auto_start_enabled = Some(auto_start_enabled);
  }

  pub fn with_auto_start_enabled(mut self, auto_start_enabled: bool) -> MsgVpnDistributedCacheClusterInstance {
    self.auto_start_enabled = Some(auto_start_enabled);
    self
  }

  pub fn auto_start_enabled(&self) -> Option<&bool> {
    self.auto_start_enabled.as_ref()
  }

  pub fn reset_auto_start_enabled(&mut self) {
    self.auto_start_enabled = None;
  }

  pub fn set_cache_name(&mut self, cache_name: String) {
    self.cache_name = Some(cache_name);
  }

  pub fn with_cache_name(mut self, cache_name: String) -> MsgVpnDistributedCacheClusterInstance {
    self.cache_name = Some(cache_name);
    self
  }

  pub fn cache_name(&self) -> Option<&String> {
    self.cache_name.as_ref()
  }

  pub fn reset_cache_name(&mut self) {
    self.cache_name = None;
  }

  pub fn set_cluster_name(&mut self, cluster_name: String) {
    self.cluster_name = Some(cluster_name);
  }

  pub fn with_cluster_name(mut self, cluster_name: String) -> MsgVpnDistributedCacheClusterInstance {
    self.cluster_name = Some(cluster_name);
    self
  }

  pub fn cluster_name(&self) -> Option<&String> {
    self.cluster_name.as_ref()
  }

  pub fn reset_cluster_name(&mut self) {
    self.cluster_name = None;
  }

  pub fn set_counter(&mut self, counter: ::models::MsgVpnDistributedCacheClusterInstanceCounter) {
    self.counter = Some(counter);
  }

  pub fn with_counter(mut self, counter: ::models::MsgVpnDistributedCacheClusterInstanceCounter) -> MsgVpnDistributedCacheClusterInstance {
    self.counter = Some(counter);
    self
  }

  pub fn counter(&self) -> Option<&::models::MsgVpnDistributedCacheClusterInstanceCounter> {
    self.counter.as_ref()
  }

  pub fn reset_counter(&mut self) {
    self.counter = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnDistributedCacheClusterInstance {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_instance_name(&mut self, instance_name: String) {
    self.instance_name = Some(instance_name);
  }

  pub fn with_instance_name(mut self, instance_name: String) -> MsgVpnDistributedCacheClusterInstance {
    self.instance_name = Some(instance_name);
    self
  }

  pub fn instance_name(&self) -> Option<&String> {
    self.instance_name.as_ref()
  }

  pub fn reset_instance_name(&mut self) {
    self.instance_name = None;
  }

  pub fn set_last_registered_time(&mut self, last_registered_time: i32) {
    self.last_registered_time = Some(last_registered_time);
  }

  pub fn with_last_registered_time(mut self, last_registered_time: i32) -> MsgVpnDistributedCacheClusterInstance {
    self.last_registered_time = Some(last_registered_time);
    self
  }

  pub fn last_registered_time(&self) -> Option<&i32> {
    self.last_registered_time.as_ref()
  }

  pub fn reset_last_registered_time(&mut self) {
    self.last_registered_time = None;
  }

  pub fn set_last_rx_heartbeat_time(&mut self, last_rx_heartbeat_time: i32) {
    self.last_rx_heartbeat_time = Some(last_rx_heartbeat_time);
  }

  pub fn with_last_rx_heartbeat_time(mut self, last_rx_heartbeat_time: i32) -> MsgVpnDistributedCacheClusterInstance {
    self.last_rx_heartbeat_time = Some(last_rx_heartbeat_time);
    self
  }

  pub fn last_rx_heartbeat_time(&self) -> Option<&i32> {
    self.last_rx_heartbeat_time.as_ref()
  }

  pub fn reset_last_rx_heartbeat_time(&mut self) {
    self.last_rx_heartbeat_time = None;
  }

  pub fn set_last_rx_set_lost_msg_time(&mut self, last_rx_set_lost_msg_time: i32) {
    self.last_rx_set_lost_msg_time = Some(last_rx_set_lost_msg_time);
  }

  pub fn with_last_rx_set_lost_msg_time(mut self, last_rx_set_lost_msg_time: i32) -> MsgVpnDistributedCacheClusterInstance {
    self.last_rx_set_lost_msg_time = Some(last_rx_set_lost_msg_time);
    self
  }

  pub fn last_rx_set_lost_msg_time(&self) -> Option<&i32> {
    self.last_rx_set_lost_msg_time.as_ref()
  }

  pub fn reset_last_rx_set_lost_msg_time(&mut self) {
    self.last_rx_set_lost_msg_time = None;
  }

  pub fn set_last_stopped_reason(&mut self, last_stopped_reason: String) {
    self.last_stopped_reason = Some(last_stopped_reason);
  }

  pub fn with_last_stopped_reason(mut self, last_stopped_reason: String) -> MsgVpnDistributedCacheClusterInstance {
    self.last_stopped_reason = Some(last_stopped_reason);
    self
  }

  pub fn last_stopped_reason(&self) -> Option<&String> {
    self.last_stopped_reason.as_ref()
  }

  pub fn reset_last_stopped_reason(&mut self) {
    self.last_stopped_reason = None;
  }

  pub fn set_last_stopped_time(&mut self, last_stopped_time: i32) {
    self.last_stopped_time = Some(last_stopped_time);
  }

  pub fn with_last_stopped_time(mut self, last_stopped_time: i32) -> MsgVpnDistributedCacheClusterInstance {
    self.last_stopped_time = Some(last_stopped_time);
    self
  }

  pub fn last_stopped_time(&self) -> Option<&i32> {
    self.last_stopped_time.as_ref()
  }

  pub fn reset_last_stopped_time(&mut self) {
    self.last_stopped_time = None;
  }

  pub fn set_last_tx_clear_lost_msg_time(&mut self, last_tx_clear_lost_msg_time: i32) {
    self.last_tx_clear_lost_msg_time = Some(last_tx_clear_lost_msg_time);
  }

  pub fn with_last_tx_clear_lost_msg_time(mut self, last_tx_clear_lost_msg_time: i32) -> MsgVpnDistributedCacheClusterInstance {
    self.last_tx_clear_lost_msg_time = Some(last_tx_clear_lost_msg_time);
    self
  }

  pub fn last_tx_clear_lost_msg_time(&self) -> Option<&i32> {
    self.last_tx_clear_lost_msg_time.as_ref()
  }

  pub fn reset_last_tx_clear_lost_msg_time(&mut self) {
    self.last_tx_clear_lost_msg_time = None;
  }

  pub fn set_memory_usage(&mut self, memory_usage: i32) {
    self.memory_usage = Some(memory_usage);
  }

  pub fn with_memory_usage(mut self, memory_usage: i32) -> MsgVpnDistributedCacheClusterInstance {
    self.memory_usage = Some(memory_usage);
    self
  }

  pub fn memory_usage(&self) -> Option<&i32> {
    self.memory_usage.as_ref()
  }

  pub fn reset_memory_usage(&mut self) {
    self.memory_usage = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnDistributedCacheClusterInstance {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_msgs_lost(&mut self, msgs_lost: bool) {
    self.msgs_lost = Some(msgs_lost);
  }

  pub fn with_msgs_lost(mut self, msgs_lost: bool) -> MsgVpnDistributedCacheClusterInstance {
    self.msgs_lost = Some(msgs_lost);
    self
  }

  pub fn msgs_lost(&self) -> Option<&bool> {
    self.msgs_lost.as_ref()
  }

  pub fn reset_msgs_lost(&mut self) {
    self.msgs_lost = None;
  }

  pub fn set_rate(&mut self, rate: ::models::MsgVpnDistributedCacheClusterInstanceRate) {
    self.rate = Some(rate);
  }

  pub fn with_rate(mut self, rate: ::models::MsgVpnDistributedCacheClusterInstanceRate) -> MsgVpnDistributedCacheClusterInstance {
    self.rate = Some(rate);
    self
  }

  pub fn rate(&self) -> Option<&::models::MsgVpnDistributedCacheClusterInstanceRate> {
    self.rate.as_ref()
  }

  pub fn reset_rate(&mut self) {
    self.rate = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> MsgVpnDistributedCacheClusterInstance {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_stop_on_lost_msg_enabled(&mut self, stop_on_lost_msg_enabled: bool) {
    self.stop_on_lost_msg_enabled = Some(stop_on_lost_msg_enabled);
  }

  pub fn with_stop_on_lost_msg_enabled(mut self, stop_on_lost_msg_enabled: bool) -> MsgVpnDistributedCacheClusterInstance {
    self.stop_on_lost_msg_enabled = Some(stop_on_lost_msg_enabled);
    self
  }

  pub fn stop_on_lost_msg_enabled(&self) -> Option<&bool> {
    self.stop_on_lost_msg_enabled.as_ref()
  }

  pub fn reset_stop_on_lost_msg_enabled(&mut self) {
    self.stop_on_lost_msg_enabled = None;
  }

}



