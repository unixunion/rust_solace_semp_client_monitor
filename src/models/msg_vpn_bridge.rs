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
pub struct MsgVpnBridge {
  /// Indicates whether the Bridge is bound to the queue in the remote Message VPN.
  #[serde(rename = "boundToQueue", skip_serializing_if="Option::is_none")]
  bound_to_queue: Option<bool>,
  /// The name of the Bridge.
  #[serde(rename = "bridgeName", skip_serializing_if="Option::is_none")]
  bridge_name: Option<String>,
  /// The virtual router of the Bridge. The allowed values and their meaning are:  <pre> \"primary\" - The Bridge is used for the primary virtual router. \"backup\" - The Bridge is used for the backup virtual router. \"auto\" - The Bridge is automatically assigned a router. </pre> 
  #[serde(rename = "bridgeVirtualRouter", skip_serializing_if="Option::is_none")]
  bridge_virtual_router: Option<String>,
  /// The name of the Client for the Bridge.
  #[serde(rename = "clientName", skip_serializing_if="Option::is_none")]
  client_name: Option<String>,
  /// Indicates whether messages transmitted over the Bridge are compressed.
  #[serde(rename = "compressed", skip_serializing_if="Option::is_none")]
  compressed: Option<bool>,
  #[serde(rename = "counter", skip_serializing_if="Option::is_none")]
  counter: Option<::models::MsgVpnBridgeCounter>,
  /// Indicates whether the Bridge is enabled.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// Indicates whether messages transmitted over the Bridge are encrypted with TLS.
  #[serde(rename = "encrypted", skip_serializing_if="Option::is_none")]
  encrypted: Option<bool>,
  /// The establisher of the Bridge connection. The allowed values and their meaning are:  <pre> \"local\" - The Bridge connection was established by the local Message VPN. \"remote\" - The Bridge connection was established by the remote Message VPN. </pre> 
  #[serde(rename = "establisher", skip_serializing_if="Option::is_none")]
  establisher: Option<String>,
  /// The reason for the inbound connection failure from the Bridge.
  #[serde(rename = "inboundFailureReason", skip_serializing_if="Option::is_none")]
  inbound_failure_reason: Option<String>,
  /// The state of the inbound connection from the Bridge. The allowed values and their meaning are:  <pre> \"init\" - The connection is initializing. \"disabled\" - The connection is disabled by configuration. \"enabled\" - The connection is enabled by configuration. \"not-ready\" - The connection is operationally down. \"not-ready-wait-to-connect\" - The connection is down and waiting to connect. \"not-ready-connecting\" - The connection is trying to connect. \"not-ready-handshaking\" - The connection is handshaking. \"not-ready-wait-next\" - The connection failed to connect and is waiting to retry. \"not-ready-wait-reuse\" - The connection is closing in order to reuse an existing connection. \"not-ready-wait-cleanup\" - The connection is closed and cleaning up. \"ready\" - The connection is operationally up. \"ready-subscribing\" - The connection is up and synchronizing subscriptions. \"ready-in-sync\" - The connection is up and subscriptions are synchronized. </pre> 
  #[serde(rename = "inboundState", skip_serializing_if="Option::is_none")]
  inbound_state: Option<String>,
  /// The ID of the last message transmitted to the Bridge.
  #[serde(rename = "lastTxMsgId", skip_serializing_if="Option::is_none")]
  last_tx_msg_id: Option<i64>,
  /// The physical interface on the local Message VPN host for connecting to the remote Message VPN.
  #[serde(rename = "localInterface", skip_serializing_if="Option::is_none")]
  local_interface: Option<String>,
  /// The name of the local queue for the Bridge.
  #[serde(rename = "localQueueName", skip_serializing_if="Option::is_none")]
  local_queue_name: Option<String>,
  /// The maximum time-to-live (TTL) in hops. Messages are discarded if their TTL exceeds this value.
  #[serde(rename = "maxTtl", skip_serializing_if="Option::is_none")]
  max_ttl: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The state of the outbound connection to the Bridge. The allowed values and their meaning are:  <pre> \"init\" - The connection is initializing. \"disabled\" - The connection is disabled by configuration. \"enabled\" - The connection is enabled by configuration. \"not-ready\" - The connection is operationally down. \"not-ready-wait-to-connect\" - The connection is down and waiting to connect. \"not-ready-connecting\" - The connection is trying to connect. \"not-ready-handshaking\" - The connection is handshaking. \"not-ready-wait-next\" - The connection failed to connect and is waiting to retry. \"not-ready-wait-reuse\" - The connection is closing in order to reuse an existing connection. \"not-ready-wait-cleanup\" - The connection is closed and cleaning up. \"ready\" - The connection is operationally up. \"ready-subscribing\" - The connection is up and synchronizing subscriptions. \"ready-in-sync\" - The connection is up and subscriptions are synchronized. </pre> 
  #[serde(rename = "outboundState", skip_serializing_if="Option::is_none")]
  outbound_state: Option<String>,
  #[serde(rename = "rate", skip_serializing_if="Option::is_none")]
  rate: Option<::models::MsgVpnBridgeRate>,
  /// The FQDN or IP address of the remote Message VPN.
  #[serde(rename = "remoteAddress", skip_serializing_if="Option::is_none")]
  remote_address: Option<String>,
  /// The Client Username the Bridge uses to login to the remote Message VPN.
  #[serde(rename = "remoteAuthenticationBasicClientUsername", skip_serializing_if="Option::is_none")]
  remote_authentication_basic_client_username: Option<String>,
  /// The authentication scheme for the remote Message VPN. The allowed values and their meaning are:  <pre> \"basic\" - Basic Authentication Scheme (via username and password). \"client-certificate\" - Client Certificate Authentication Scheme (via certificate file or content). </pre> 
  #[serde(rename = "remoteAuthenticationScheme", skip_serializing_if="Option::is_none")]
  remote_authentication_scheme: Option<String>,
  /// The maximum number of retry attempts to establish a connection to the remote Message VPN. A value of 0 means to retry forever.
  #[serde(rename = "remoteConnectionRetryCount", skip_serializing_if="Option::is_none")]
  remote_connection_retry_count: Option<i64>,
  /// The number of seconds to delay before retrying to connect to the remote Message VPN.
  #[serde(rename = "remoteConnectionRetryDelay", skip_serializing_if="Option::is_none")]
  remote_connection_retry_delay: Option<i64>,
  /// The priority for deliver-to-one (DTO) messages transmitted from the remote Message VPN. The allowed values and their meaning are:  <pre> \"p1\" - The 1st or highest priority. \"p2\" - The 2nd highest priority. \"p3\" - The 3rd highest priority. \"p4\" - The 4th highest priority. \"da\" - Ignore priority and deliver always. </pre> 
  #[serde(rename = "remoteDeliverToOnePriority", skip_serializing_if="Option::is_none")]
  remote_deliver_to_one_priority: Option<String>,
  /// The name of the remote Message VPN.
  #[serde(rename = "remoteMsgVpnName", skip_serializing_if="Option::is_none")]
  remote_msg_vpn_name: Option<String>,
  /// The name of the remote router.
  #[serde(rename = "remoteRouterName", skip_serializing_if="Option::is_none")]
  remote_router_name: Option<String>,
  /// The ID of the transmit flow for the connected remote Message VPN.
  #[serde(rename = "remoteTxFlowId", skip_serializing_if="Option::is_none")]
  remote_tx_flow_id: Option<i32>,
  /// The colon-separated list of cipher-suites supported for TLS connections to the remote Message VPN. The value \"default\" implies all supported suites ordered from most secure to least secure.
  #[serde(rename = "tlsCipherSuiteList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_list: Option<String>,
  /// Indicates whether the Bridge is configured to use the default cipher-suite list.
  #[serde(rename = "tlsDefaultCipherSuiteList", skip_serializing_if="Option::is_none")]
  tls_default_cipher_suite_list: Option<bool>,
  /// Indicates whether the TTL (hops) exceeded event has been raised.
  #[serde(rename = "ttlExceededEventRaised", skip_serializing_if="Option::is_none")]
  ttl_exceeded_event_raised: Option<bool>,
  /// The amount of time in seconds since the Bridge connected to the remote Message VPN.
  #[serde(rename = "uptime", skip_serializing_if="Option::is_none")]
  uptime: Option<i64>
}

impl MsgVpnBridge {
  pub fn new() -> MsgVpnBridge {
    MsgVpnBridge {
      bound_to_queue: None,
      bridge_name: None,
      bridge_virtual_router: None,
      client_name: None,
      compressed: None,
      counter: None,
      enabled: None,
      encrypted: None,
      establisher: None,
      inbound_failure_reason: None,
      inbound_state: None,
      last_tx_msg_id: None,
      local_interface: None,
      local_queue_name: None,
      max_ttl: None,
      msg_vpn_name: None,
      outbound_state: None,
      rate: None,
      remote_address: None,
      remote_authentication_basic_client_username: None,
      remote_authentication_scheme: None,
      remote_connection_retry_count: None,
      remote_connection_retry_delay: None,
      remote_deliver_to_one_priority: None,
      remote_msg_vpn_name: None,
      remote_router_name: None,
      remote_tx_flow_id: None,
      tls_cipher_suite_list: None,
      tls_default_cipher_suite_list: None,
      ttl_exceeded_event_raised: None,
      uptime: None
    }
  }

  pub fn set_bound_to_queue(&mut self, bound_to_queue: bool) {
    self.bound_to_queue = Some(bound_to_queue);
  }

  pub fn with_bound_to_queue(mut self, bound_to_queue: bool) -> MsgVpnBridge {
    self.bound_to_queue = Some(bound_to_queue);
    self
  }

  pub fn bound_to_queue(&self) -> Option<&bool> {
    self.bound_to_queue.as_ref()
  }

  pub fn reset_bound_to_queue(&mut self) {
    self.bound_to_queue = None;
  }

  pub fn set_bridge_name(&mut self, bridge_name: String) {
    self.bridge_name = Some(bridge_name);
  }

  pub fn with_bridge_name(mut self, bridge_name: String) -> MsgVpnBridge {
    self.bridge_name = Some(bridge_name);
    self
  }

  pub fn bridge_name(&self) -> Option<&String> {
    self.bridge_name.as_ref()
  }

  pub fn reset_bridge_name(&mut self) {
    self.bridge_name = None;
  }

  pub fn set_bridge_virtual_router(&mut self, bridge_virtual_router: String) {
    self.bridge_virtual_router = Some(bridge_virtual_router);
  }

  pub fn with_bridge_virtual_router(mut self, bridge_virtual_router: String) -> MsgVpnBridge {
    self.bridge_virtual_router = Some(bridge_virtual_router);
    self
  }

  pub fn bridge_virtual_router(&self) -> Option<&String> {
    self.bridge_virtual_router.as_ref()
  }

  pub fn reset_bridge_virtual_router(&mut self) {
    self.bridge_virtual_router = None;
  }

  pub fn set_client_name(&mut self, client_name: String) {
    self.client_name = Some(client_name);
  }

  pub fn with_client_name(mut self, client_name: String) -> MsgVpnBridge {
    self.client_name = Some(client_name);
    self
  }

  pub fn client_name(&self) -> Option<&String> {
    self.client_name.as_ref()
  }

  pub fn reset_client_name(&mut self) {
    self.client_name = None;
  }

  pub fn set_compressed(&mut self, compressed: bool) {
    self.compressed = Some(compressed);
  }

  pub fn with_compressed(mut self, compressed: bool) -> MsgVpnBridge {
    self.compressed = Some(compressed);
    self
  }

  pub fn compressed(&self) -> Option<&bool> {
    self.compressed.as_ref()
  }

  pub fn reset_compressed(&mut self) {
    self.compressed = None;
  }

  pub fn set_counter(&mut self, counter: ::models::MsgVpnBridgeCounter) {
    self.counter = Some(counter);
  }

  pub fn with_counter(mut self, counter: ::models::MsgVpnBridgeCounter) -> MsgVpnBridge {
    self.counter = Some(counter);
    self
  }

  pub fn counter(&self) -> Option<&::models::MsgVpnBridgeCounter> {
    self.counter.as_ref()
  }

  pub fn reset_counter(&mut self) {
    self.counter = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnBridge {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_encrypted(&mut self, encrypted: bool) {
    self.encrypted = Some(encrypted);
  }

  pub fn with_encrypted(mut self, encrypted: bool) -> MsgVpnBridge {
    self.encrypted = Some(encrypted);
    self
  }

  pub fn encrypted(&self) -> Option<&bool> {
    self.encrypted.as_ref()
  }

  pub fn reset_encrypted(&mut self) {
    self.encrypted = None;
  }

  pub fn set_establisher(&mut self, establisher: String) {
    self.establisher = Some(establisher);
  }

  pub fn with_establisher(mut self, establisher: String) -> MsgVpnBridge {
    self.establisher = Some(establisher);
    self
  }

  pub fn establisher(&self) -> Option<&String> {
    self.establisher.as_ref()
  }

  pub fn reset_establisher(&mut self) {
    self.establisher = None;
  }

  pub fn set_inbound_failure_reason(&mut self, inbound_failure_reason: String) {
    self.inbound_failure_reason = Some(inbound_failure_reason);
  }

  pub fn with_inbound_failure_reason(mut self, inbound_failure_reason: String) -> MsgVpnBridge {
    self.inbound_failure_reason = Some(inbound_failure_reason);
    self
  }

  pub fn inbound_failure_reason(&self) -> Option<&String> {
    self.inbound_failure_reason.as_ref()
  }

  pub fn reset_inbound_failure_reason(&mut self) {
    self.inbound_failure_reason = None;
  }

  pub fn set_inbound_state(&mut self, inbound_state: String) {
    self.inbound_state = Some(inbound_state);
  }

  pub fn with_inbound_state(mut self, inbound_state: String) -> MsgVpnBridge {
    self.inbound_state = Some(inbound_state);
    self
  }

  pub fn inbound_state(&self) -> Option<&String> {
    self.inbound_state.as_ref()
  }

  pub fn reset_inbound_state(&mut self) {
    self.inbound_state = None;
  }

  pub fn set_last_tx_msg_id(&mut self, last_tx_msg_id: i64) {
    self.last_tx_msg_id = Some(last_tx_msg_id);
  }

  pub fn with_last_tx_msg_id(mut self, last_tx_msg_id: i64) -> MsgVpnBridge {
    self.last_tx_msg_id = Some(last_tx_msg_id);
    self
  }

  pub fn last_tx_msg_id(&self) -> Option<&i64> {
    self.last_tx_msg_id.as_ref()
  }

  pub fn reset_last_tx_msg_id(&mut self) {
    self.last_tx_msg_id = None;
  }

  pub fn set_local_interface(&mut self, local_interface: String) {
    self.local_interface = Some(local_interface);
  }

  pub fn with_local_interface(mut self, local_interface: String) -> MsgVpnBridge {
    self.local_interface = Some(local_interface);
    self
  }

  pub fn local_interface(&self) -> Option<&String> {
    self.local_interface.as_ref()
  }

  pub fn reset_local_interface(&mut self) {
    self.local_interface = None;
  }

  pub fn set_local_queue_name(&mut self, local_queue_name: String) {
    self.local_queue_name = Some(local_queue_name);
  }

  pub fn with_local_queue_name(mut self, local_queue_name: String) -> MsgVpnBridge {
    self.local_queue_name = Some(local_queue_name);
    self
  }

  pub fn local_queue_name(&self) -> Option<&String> {
    self.local_queue_name.as_ref()
  }

  pub fn reset_local_queue_name(&mut self) {
    self.local_queue_name = None;
  }

  pub fn set_max_ttl(&mut self, max_ttl: i64) {
    self.max_ttl = Some(max_ttl);
  }

  pub fn with_max_ttl(mut self, max_ttl: i64) -> MsgVpnBridge {
    self.max_ttl = Some(max_ttl);
    self
  }

  pub fn max_ttl(&self) -> Option<&i64> {
    self.max_ttl.as_ref()
  }

  pub fn reset_max_ttl(&mut self) {
    self.max_ttl = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnBridge {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_outbound_state(&mut self, outbound_state: String) {
    self.outbound_state = Some(outbound_state);
  }

  pub fn with_outbound_state(mut self, outbound_state: String) -> MsgVpnBridge {
    self.outbound_state = Some(outbound_state);
    self
  }

  pub fn outbound_state(&self) -> Option<&String> {
    self.outbound_state.as_ref()
  }

  pub fn reset_outbound_state(&mut self) {
    self.outbound_state = None;
  }

  pub fn set_rate(&mut self, rate: ::models::MsgVpnBridgeRate) {
    self.rate = Some(rate);
  }

  pub fn with_rate(mut self, rate: ::models::MsgVpnBridgeRate) -> MsgVpnBridge {
    self.rate = Some(rate);
    self
  }

  pub fn rate(&self) -> Option<&::models::MsgVpnBridgeRate> {
    self.rate.as_ref()
  }

  pub fn reset_rate(&mut self) {
    self.rate = None;
  }

  pub fn set_remote_address(&mut self, remote_address: String) {
    self.remote_address = Some(remote_address);
  }

  pub fn with_remote_address(mut self, remote_address: String) -> MsgVpnBridge {
    self.remote_address = Some(remote_address);
    self
  }

  pub fn remote_address(&self) -> Option<&String> {
    self.remote_address.as_ref()
  }

  pub fn reset_remote_address(&mut self) {
    self.remote_address = None;
  }

  pub fn set_remote_authentication_basic_client_username(&mut self, remote_authentication_basic_client_username: String) {
    self.remote_authentication_basic_client_username = Some(remote_authentication_basic_client_username);
  }

  pub fn with_remote_authentication_basic_client_username(mut self, remote_authentication_basic_client_username: String) -> MsgVpnBridge {
    self.remote_authentication_basic_client_username = Some(remote_authentication_basic_client_username);
    self
  }

  pub fn remote_authentication_basic_client_username(&self) -> Option<&String> {
    self.remote_authentication_basic_client_username.as_ref()
  }

  pub fn reset_remote_authentication_basic_client_username(&mut self) {
    self.remote_authentication_basic_client_username = None;
  }

  pub fn set_remote_authentication_scheme(&mut self, remote_authentication_scheme: String) {
    self.remote_authentication_scheme = Some(remote_authentication_scheme);
  }

  pub fn with_remote_authentication_scheme(mut self, remote_authentication_scheme: String) -> MsgVpnBridge {
    self.remote_authentication_scheme = Some(remote_authentication_scheme);
    self
  }

  pub fn remote_authentication_scheme(&self) -> Option<&String> {
    self.remote_authentication_scheme.as_ref()
  }

  pub fn reset_remote_authentication_scheme(&mut self) {
    self.remote_authentication_scheme = None;
  }

  pub fn set_remote_connection_retry_count(&mut self, remote_connection_retry_count: i64) {
    self.remote_connection_retry_count = Some(remote_connection_retry_count);
  }

  pub fn with_remote_connection_retry_count(mut self, remote_connection_retry_count: i64) -> MsgVpnBridge {
    self.remote_connection_retry_count = Some(remote_connection_retry_count);
    self
  }

  pub fn remote_connection_retry_count(&self) -> Option<&i64> {
    self.remote_connection_retry_count.as_ref()
  }

  pub fn reset_remote_connection_retry_count(&mut self) {
    self.remote_connection_retry_count = None;
  }

  pub fn set_remote_connection_retry_delay(&mut self, remote_connection_retry_delay: i64) {
    self.remote_connection_retry_delay = Some(remote_connection_retry_delay);
  }

  pub fn with_remote_connection_retry_delay(mut self, remote_connection_retry_delay: i64) -> MsgVpnBridge {
    self.remote_connection_retry_delay = Some(remote_connection_retry_delay);
    self
  }

  pub fn remote_connection_retry_delay(&self) -> Option<&i64> {
    self.remote_connection_retry_delay.as_ref()
  }

  pub fn reset_remote_connection_retry_delay(&mut self) {
    self.remote_connection_retry_delay = None;
  }

  pub fn set_remote_deliver_to_one_priority(&mut self, remote_deliver_to_one_priority: String) {
    self.remote_deliver_to_one_priority = Some(remote_deliver_to_one_priority);
  }

  pub fn with_remote_deliver_to_one_priority(mut self, remote_deliver_to_one_priority: String) -> MsgVpnBridge {
    self.remote_deliver_to_one_priority = Some(remote_deliver_to_one_priority);
    self
  }

  pub fn remote_deliver_to_one_priority(&self) -> Option<&String> {
    self.remote_deliver_to_one_priority.as_ref()
  }

  pub fn reset_remote_deliver_to_one_priority(&mut self) {
    self.remote_deliver_to_one_priority = None;
  }

  pub fn set_remote_msg_vpn_name(&mut self, remote_msg_vpn_name: String) {
    self.remote_msg_vpn_name = Some(remote_msg_vpn_name);
  }

  pub fn with_remote_msg_vpn_name(mut self, remote_msg_vpn_name: String) -> MsgVpnBridge {
    self.remote_msg_vpn_name = Some(remote_msg_vpn_name);
    self
  }

  pub fn remote_msg_vpn_name(&self) -> Option<&String> {
    self.remote_msg_vpn_name.as_ref()
  }

  pub fn reset_remote_msg_vpn_name(&mut self) {
    self.remote_msg_vpn_name = None;
  }

  pub fn set_remote_router_name(&mut self, remote_router_name: String) {
    self.remote_router_name = Some(remote_router_name);
  }

  pub fn with_remote_router_name(mut self, remote_router_name: String) -> MsgVpnBridge {
    self.remote_router_name = Some(remote_router_name);
    self
  }

  pub fn remote_router_name(&self) -> Option<&String> {
    self.remote_router_name.as_ref()
  }

  pub fn reset_remote_router_name(&mut self) {
    self.remote_router_name = None;
  }

  pub fn set_remote_tx_flow_id(&mut self, remote_tx_flow_id: i32) {
    self.remote_tx_flow_id = Some(remote_tx_flow_id);
  }

  pub fn with_remote_tx_flow_id(mut self, remote_tx_flow_id: i32) -> MsgVpnBridge {
    self.remote_tx_flow_id = Some(remote_tx_flow_id);
    self
  }

  pub fn remote_tx_flow_id(&self) -> Option<&i32> {
    self.remote_tx_flow_id.as_ref()
  }

  pub fn reset_remote_tx_flow_id(&mut self) {
    self.remote_tx_flow_id = None;
  }

  pub fn set_tls_cipher_suite_list(&mut self, tls_cipher_suite_list: String) {
    self.tls_cipher_suite_list = Some(tls_cipher_suite_list);
  }

  pub fn with_tls_cipher_suite_list(mut self, tls_cipher_suite_list: String) -> MsgVpnBridge {
    self.tls_cipher_suite_list = Some(tls_cipher_suite_list);
    self
  }

  pub fn tls_cipher_suite_list(&self) -> Option<&String> {
    self.tls_cipher_suite_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_list(&mut self) {
    self.tls_cipher_suite_list = None;
  }

  pub fn set_tls_default_cipher_suite_list(&mut self, tls_default_cipher_suite_list: bool) {
    self.tls_default_cipher_suite_list = Some(tls_default_cipher_suite_list);
  }

  pub fn with_tls_default_cipher_suite_list(mut self, tls_default_cipher_suite_list: bool) -> MsgVpnBridge {
    self.tls_default_cipher_suite_list = Some(tls_default_cipher_suite_list);
    self
  }

  pub fn tls_default_cipher_suite_list(&self) -> Option<&bool> {
    self.tls_default_cipher_suite_list.as_ref()
  }

  pub fn reset_tls_default_cipher_suite_list(&mut self) {
    self.tls_default_cipher_suite_list = None;
  }

  pub fn set_ttl_exceeded_event_raised(&mut self, ttl_exceeded_event_raised: bool) {
    self.ttl_exceeded_event_raised = Some(ttl_exceeded_event_raised);
  }

  pub fn with_ttl_exceeded_event_raised(mut self, ttl_exceeded_event_raised: bool) -> MsgVpnBridge {
    self.ttl_exceeded_event_raised = Some(ttl_exceeded_event_raised);
    self
  }

  pub fn ttl_exceeded_event_raised(&self) -> Option<&bool> {
    self.ttl_exceeded_event_raised.as_ref()
  }

  pub fn reset_ttl_exceeded_event_raised(&mut self) {
    self.ttl_exceeded_event_raised = None;
  }

  pub fn set_uptime(&mut self, uptime: i64) {
    self.uptime = Some(uptime);
  }

  pub fn with_uptime(mut self, uptime: i64) -> MsgVpnBridge {
    self.uptime = Some(uptime);
    self
  }

  pub fn uptime(&self) -> Option<&i64> {
    self.uptime.as_ref()
  }

  pub fn reset_uptime(&mut self) {
    self.uptime = None;
  }

}



