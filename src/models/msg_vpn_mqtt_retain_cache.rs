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
pub struct MsgVpnMqttRetainCache {
  /// The name of the backup Cache Instance associated with this MQTT Retain Cache.
  #[serde(rename = "backupCacheInstance", skip_serializing_if="Option::is_none")]
  backup_cache_instance: Option<String>,
  /// The reason why the backup cache associated with this MQTT Retain Cache is operationally down, if any.
  #[serde(rename = "backupFailureReason", skip_serializing_if="Option::is_none")]
  backup_failure_reason: Option<String>,
  /// Whether the backup cache associated with this MQTT Retain Cache is operationally up.
  #[serde(rename = "backupUp", skip_serializing_if="Option::is_none")]
  backup_up: Option<bool>,
  /// The number of seconds that the backup cache associated with this MQTT Retain Cache has been operationally up.
  #[serde(rename = "backupUptime", skip_serializing_if="Option::is_none")]
  backup_uptime: Option<i32>,
  /// The name of the Cache Cluster associated with this MQTT Retain Cache.
  #[serde(rename = "cacheCluster", skip_serializing_if="Option::is_none")]
  cache_cluster: Option<String>,
  /// The name of the MQTT Retain Cache.
  #[serde(rename = "cacheName", skip_serializing_if="Option::is_none")]
  cache_name: Option<String>,
  /// The name of the Distributed Cache associated with this MQTT Retain Cache.
  #[serde(rename = "distributedCache", skip_serializing_if="Option::is_none")]
  distributed_cache: Option<String>,
  /// Enable or disable this MQTT Retain Cache. When the cache is disabled, neither retain messages nor retain requests will be delivered by the cache. However, live retain messages will continue to be delivered to currently connected MQTT clients.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The reason why this MQTT Retain Cache is operationally down, if any.
  #[serde(rename = "failureReason", skip_serializing_if="Option::is_none")]
  failure_reason: Option<String>,
  /// The message lifetime, in seconds. If a message remains cached for the duration of its lifetime, the cache will remove the message. A lifetime of 0 results in the message being retained indefinitely.
  #[serde(rename = "msgLifetime", skip_serializing_if="Option::is_none")]
  msg_lifetime: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The name of the primary Cache Instance associated with this MQTT Retain Cache.
  #[serde(rename = "primaryCacheInstance", skip_serializing_if="Option::is_none")]
  primary_cache_instance: Option<String>,
  /// The reason why the primary cache associated with this MQTT Retain Cache is operationally down, if any.
  #[serde(rename = "primaryFailureReason", skip_serializing_if="Option::is_none")]
  primary_failure_reason: Option<String>,
  /// Whether the primary cache associated with this MQTT Retain Cache is operationally up.
  #[serde(rename = "primaryUp", skip_serializing_if="Option::is_none")]
  primary_up: Option<bool>,
  /// The number of seconds that the primary cache associated with this MQTT Retain Cache has been operationally up.
  #[serde(rename = "primaryUptime", skip_serializing_if="Option::is_none")]
  primary_uptime: Option<i32>,
  /// Whether this MQTT Retain Cache is operationally up.
  #[serde(rename = "up", skip_serializing_if="Option::is_none")]
  up: Option<bool>,
  /// The number of seconds that the MQTT Retain Cache has been operationally up.
  #[serde(rename = "uptime", skip_serializing_if="Option::is_none")]
  uptime: Option<i32>
}

impl MsgVpnMqttRetainCache {
  pub fn new() -> MsgVpnMqttRetainCache {
    MsgVpnMqttRetainCache {
      backup_cache_instance: None,
      backup_failure_reason: None,
      backup_up: None,
      backup_uptime: None,
      cache_cluster: None,
      cache_name: None,
      distributed_cache: None,
      enabled: None,
      failure_reason: None,
      msg_lifetime: None,
      msg_vpn_name: None,
      primary_cache_instance: None,
      primary_failure_reason: None,
      primary_up: None,
      primary_uptime: None,
      up: None,
      uptime: None
    }
  }

  pub fn set_backup_cache_instance(&mut self, backup_cache_instance: String) {
    self.backup_cache_instance = Some(backup_cache_instance);
  }

  pub fn with_backup_cache_instance(mut self, backup_cache_instance: String) -> MsgVpnMqttRetainCache {
    self.backup_cache_instance = Some(backup_cache_instance);
    self
  }

  pub fn backup_cache_instance(&self) -> Option<&String> {
    self.backup_cache_instance.as_ref()
  }

  pub fn reset_backup_cache_instance(&mut self) {
    self.backup_cache_instance = None;
  }

  pub fn set_backup_failure_reason(&mut self, backup_failure_reason: String) {
    self.backup_failure_reason = Some(backup_failure_reason);
  }

  pub fn with_backup_failure_reason(mut self, backup_failure_reason: String) -> MsgVpnMqttRetainCache {
    self.backup_failure_reason = Some(backup_failure_reason);
    self
  }

  pub fn backup_failure_reason(&self) -> Option<&String> {
    self.backup_failure_reason.as_ref()
  }

  pub fn reset_backup_failure_reason(&mut self) {
    self.backup_failure_reason = None;
  }

  pub fn set_backup_up(&mut self, backup_up: bool) {
    self.backup_up = Some(backup_up);
  }

  pub fn with_backup_up(mut self, backup_up: bool) -> MsgVpnMqttRetainCache {
    self.backup_up = Some(backup_up);
    self
  }

  pub fn backup_up(&self) -> Option<&bool> {
    self.backup_up.as_ref()
  }

  pub fn reset_backup_up(&mut self) {
    self.backup_up = None;
  }

  pub fn set_backup_uptime(&mut self, backup_uptime: i32) {
    self.backup_uptime = Some(backup_uptime);
  }

  pub fn with_backup_uptime(mut self, backup_uptime: i32) -> MsgVpnMqttRetainCache {
    self.backup_uptime = Some(backup_uptime);
    self
  }

  pub fn backup_uptime(&self) -> Option<&i32> {
    self.backup_uptime.as_ref()
  }

  pub fn reset_backup_uptime(&mut self) {
    self.backup_uptime = None;
  }

  pub fn set_cache_cluster(&mut self, cache_cluster: String) {
    self.cache_cluster = Some(cache_cluster);
  }

  pub fn with_cache_cluster(mut self, cache_cluster: String) -> MsgVpnMqttRetainCache {
    self.cache_cluster = Some(cache_cluster);
    self
  }

  pub fn cache_cluster(&self) -> Option<&String> {
    self.cache_cluster.as_ref()
  }

  pub fn reset_cache_cluster(&mut self) {
    self.cache_cluster = None;
  }

  pub fn set_cache_name(&mut self, cache_name: String) {
    self.cache_name = Some(cache_name);
  }

  pub fn with_cache_name(mut self, cache_name: String) -> MsgVpnMqttRetainCache {
    self.cache_name = Some(cache_name);
    self
  }

  pub fn cache_name(&self) -> Option<&String> {
    self.cache_name.as_ref()
  }

  pub fn reset_cache_name(&mut self) {
    self.cache_name = None;
  }

  pub fn set_distributed_cache(&mut self, distributed_cache: String) {
    self.distributed_cache = Some(distributed_cache);
  }

  pub fn with_distributed_cache(mut self, distributed_cache: String) -> MsgVpnMqttRetainCache {
    self.distributed_cache = Some(distributed_cache);
    self
  }

  pub fn distributed_cache(&self) -> Option<&String> {
    self.distributed_cache.as_ref()
  }

  pub fn reset_distributed_cache(&mut self) {
    self.distributed_cache = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnMqttRetainCache {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_failure_reason(&mut self, failure_reason: String) {
    self.failure_reason = Some(failure_reason);
  }

  pub fn with_failure_reason(mut self, failure_reason: String) -> MsgVpnMqttRetainCache {
    self.failure_reason = Some(failure_reason);
    self
  }

  pub fn failure_reason(&self) -> Option<&String> {
    self.failure_reason.as_ref()
  }

  pub fn reset_failure_reason(&mut self) {
    self.failure_reason = None;
  }

  pub fn set_msg_lifetime(&mut self, msg_lifetime: i64) {
    self.msg_lifetime = Some(msg_lifetime);
  }

  pub fn with_msg_lifetime(mut self, msg_lifetime: i64) -> MsgVpnMqttRetainCache {
    self.msg_lifetime = Some(msg_lifetime);
    self
  }

  pub fn msg_lifetime(&self) -> Option<&i64> {
    self.msg_lifetime.as_ref()
  }

  pub fn reset_msg_lifetime(&mut self) {
    self.msg_lifetime = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnMqttRetainCache {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_primary_cache_instance(&mut self, primary_cache_instance: String) {
    self.primary_cache_instance = Some(primary_cache_instance);
  }

  pub fn with_primary_cache_instance(mut self, primary_cache_instance: String) -> MsgVpnMqttRetainCache {
    self.primary_cache_instance = Some(primary_cache_instance);
    self
  }

  pub fn primary_cache_instance(&self) -> Option<&String> {
    self.primary_cache_instance.as_ref()
  }

  pub fn reset_primary_cache_instance(&mut self) {
    self.primary_cache_instance = None;
  }

  pub fn set_primary_failure_reason(&mut self, primary_failure_reason: String) {
    self.primary_failure_reason = Some(primary_failure_reason);
  }

  pub fn with_primary_failure_reason(mut self, primary_failure_reason: String) -> MsgVpnMqttRetainCache {
    self.primary_failure_reason = Some(primary_failure_reason);
    self
  }

  pub fn primary_failure_reason(&self) -> Option<&String> {
    self.primary_failure_reason.as_ref()
  }

  pub fn reset_primary_failure_reason(&mut self) {
    self.primary_failure_reason = None;
  }

  pub fn set_primary_up(&mut self, primary_up: bool) {
    self.primary_up = Some(primary_up);
  }

  pub fn with_primary_up(mut self, primary_up: bool) -> MsgVpnMqttRetainCache {
    self.primary_up = Some(primary_up);
    self
  }

  pub fn primary_up(&self) -> Option<&bool> {
    self.primary_up.as_ref()
  }

  pub fn reset_primary_up(&mut self) {
    self.primary_up = None;
  }

  pub fn set_primary_uptime(&mut self, primary_uptime: i32) {
    self.primary_uptime = Some(primary_uptime);
  }

  pub fn with_primary_uptime(mut self, primary_uptime: i32) -> MsgVpnMqttRetainCache {
    self.primary_uptime = Some(primary_uptime);
    self
  }

  pub fn primary_uptime(&self) -> Option<&i32> {
    self.primary_uptime.as_ref()
  }

  pub fn reset_primary_uptime(&mut self) {
    self.primary_uptime = None;
  }

  pub fn set_up(&mut self, up: bool) {
    self.up = Some(up);
  }

  pub fn with_up(mut self, up: bool) -> MsgVpnMqttRetainCache {
    self.up = Some(up);
    self
  }

  pub fn up(&self) -> Option<&bool> {
    self.up.as_ref()
  }

  pub fn reset_up(&mut self) {
    self.up = None;
  }

  pub fn set_uptime(&mut self, uptime: i32) {
    self.uptime = Some(uptime);
  }

  pub fn with_uptime(mut self, uptime: i32) -> MsgVpnMqttRetainCache {
    self.uptime = Some(uptime);
    self
  }

  pub fn uptime(&self) -> Option<&i32> {
    self.uptime.as_ref()
  }

  pub fn reset_uptime(&mut self) {
    self.uptime = None;
  }

}



