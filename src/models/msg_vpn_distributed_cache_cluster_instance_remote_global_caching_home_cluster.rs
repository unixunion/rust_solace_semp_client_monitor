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
pub struct MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
  /// The name of the Distributed Cache.
  #[serde(rename = "cacheName", skip_serializing_if="Option::is_none")]
  cache_name: Option<String>,
  /// The number of cache requests forwarded to the remote Home Cache Cluster.
  #[serde(rename = "cacheRequestForwardedCount", skip_serializing_if="Option::is_none")]
  cache_request_forwarded_count: Option<i32>,
  /// The number of cache requests received from the remote Home Cache Cluster.
  #[serde(rename = "cacheRequestRxCount", skip_serializing_if="Option::is_none")]
  cache_request_rx_count: Option<i32>,
  /// The name of the Cache Cluster.
  #[serde(rename = "clusterName", skip_serializing_if="Option::is_none")]
  cluster_name: Option<String>,
  /// The number of heartbeat messages received from the remote Home Cache Cluster.
  #[serde(rename = "heartbeatRxCount", skip_serializing_if="Option::is_none")]
  heartbeat_rx_count: Option<i64>,
  /// The operational state of the heartbeat with the remote Home Cache Cluster is up.
  #[serde(rename = "heartbeatUp", skip_serializing_if="Option::is_none")]
  heartbeat_up: Option<bool>,
  /// The name of the remote Home Cache Cluster.
  #[serde(rename = "homeClusterName", skip_serializing_if="Option::is_none")]
  home_cluster_name: Option<String>,
  /// The name of the Cache Instance.
  #[serde(rename = "instanceName", skip_serializing_if="Option::is_none")]
  instance_name: Option<String>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>
}

impl MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
  pub fn new() -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
      cache_name: None,
      cache_request_forwarded_count: None,
      cache_request_rx_count: None,
      cluster_name: None,
      heartbeat_rx_count: None,
      heartbeat_up: None,
      home_cluster_name: None,
      instance_name: None,
      msg_vpn_name: None
    }
  }

  pub fn set_cache_name(&mut self, cache_name: String) {
    self.cache_name = Some(cache_name);
  }

  pub fn with_cache_name(mut self, cache_name: String) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.cache_name = Some(cache_name);
    self
  }

  pub fn cache_name(&self) -> Option<&String> {
    self.cache_name.as_ref()
  }

  pub fn reset_cache_name(&mut self) {
    self.cache_name = None;
  }

  pub fn set_cache_request_forwarded_count(&mut self, cache_request_forwarded_count: i32) {
    self.cache_request_forwarded_count = Some(cache_request_forwarded_count);
  }

  pub fn with_cache_request_forwarded_count(mut self, cache_request_forwarded_count: i32) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.cache_request_forwarded_count = Some(cache_request_forwarded_count);
    self
  }

  pub fn cache_request_forwarded_count(&self) -> Option<&i32> {
    self.cache_request_forwarded_count.as_ref()
  }

  pub fn reset_cache_request_forwarded_count(&mut self) {
    self.cache_request_forwarded_count = None;
  }

  pub fn set_cache_request_rx_count(&mut self, cache_request_rx_count: i32) {
    self.cache_request_rx_count = Some(cache_request_rx_count);
  }

  pub fn with_cache_request_rx_count(mut self, cache_request_rx_count: i32) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.cache_request_rx_count = Some(cache_request_rx_count);
    self
  }

  pub fn cache_request_rx_count(&self) -> Option<&i32> {
    self.cache_request_rx_count.as_ref()
  }

  pub fn reset_cache_request_rx_count(&mut self) {
    self.cache_request_rx_count = None;
  }

  pub fn set_cluster_name(&mut self, cluster_name: String) {
    self.cluster_name = Some(cluster_name);
  }

  pub fn with_cluster_name(mut self, cluster_name: String) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.cluster_name = Some(cluster_name);
    self
  }

  pub fn cluster_name(&self) -> Option<&String> {
    self.cluster_name.as_ref()
  }

  pub fn reset_cluster_name(&mut self) {
    self.cluster_name = None;
  }

  pub fn set_heartbeat_rx_count(&mut self, heartbeat_rx_count: i64) {
    self.heartbeat_rx_count = Some(heartbeat_rx_count);
  }

  pub fn with_heartbeat_rx_count(mut self, heartbeat_rx_count: i64) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.heartbeat_rx_count = Some(heartbeat_rx_count);
    self
  }

  pub fn heartbeat_rx_count(&self) -> Option<&i64> {
    self.heartbeat_rx_count.as_ref()
  }

  pub fn reset_heartbeat_rx_count(&mut self) {
    self.heartbeat_rx_count = None;
  }

  pub fn set_heartbeat_up(&mut self, heartbeat_up: bool) {
    self.heartbeat_up = Some(heartbeat_up);
  }

  pub fn with_heartbeat_up(mut self, heartbeat_up: bool) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.heartbeat_up = Some(heartbeat_up);
    self
  }

  pub fn heartbeat_up(&self) -> Option<&bool> {
    self.heartbeat_up.as_ref()
  }

  pub fn reset_heartbeat_up(&mut self) {
    self.heartbeat_up = None;
  }

  pub fn set_home_cluster_name(&mut self, home_cluster_name: String) {
    self.home_cluster_name = Some(home_cluster_name);
  }

  pub fn with_home_cluster_name(mut self, home_cluster_name: String) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.home_cluster_name = Some(home_cluster_name);
    self
  }

  pub fn home_cluster_name(&self) -> Option<&String> {
    self.home_cluster_name.as_ref()
  }

  pub fn reset_home_cluster_name(&mut self) {
    self.home_cluster_name = None;
  }

  pub fn set_instance_name(&mut self, instance_name: String) {
    self.instance_name = Some(instance_name);
  }

  pub fn with_instance_name(mut self, instance_name: String) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.instance_name = Some(instance_name);
    self
  }

  pub fn instance_name(&self) -> Option<&String> {
    self.instance_name.as_ref()
  }

  pub fn reset_instance_name(&mut self) {
    self.instance_name = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeCluster {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

}



