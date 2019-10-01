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
pub struct MsgVpnClientRxFlow {
  /// The name of the Client.
  #[serde(rename = "clientName", skip_serializing_if="Option::is_none")]
  client_name: Option<String>,
  /// The timestamp of when the Flow from the Client connected.
  #[serde(rename = "connectTime", skip_serializing_if="Option::is_none")]
  connect_time: Option<i32>,
  /// The number of guaranteed messages from the Flow discarded due to a destination group error.
  #[serde(rename = "destinationGroupErrorDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  destination_group_error_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to being a duplicate.
  #[serde(rename = "duplicateDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  duplicate_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to an eligible endpoint destination being disabled.
  #[serde(rename = "endpointDisabledDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  endpoint_disabled_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to an eligible endpoint destination having its maximum message spool usage exceeded.
  #[serde(rename = "endpointUsageExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  endpoint_usage_exceeded_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to errors being detected.
  #[serde(rename = "erroredDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  errored_discarded_msg_count: Option<i64>,
  /// The identifier (ID) of the flow.
  #[serde(rename = "flowId", skip_serializing_if="Option::is_none")]
  flow_id: Option<i64>,
  /// The name of the Flow.
  #[serde(rename = "flowName", skip_serializing_if="Option::is_none")]
  flow_name: Option<String>,
  /// The number of guaranteed messages from the Flow.
  #[serde(rename = "guaranteedMsgCount", skip_serializing_if="Option::is_none")]
  guaranteed_msg_count: Option<i64>,
  /// The identifier (ID) of the last message received on the Flow.
  #[serde(rename = "lastRxMsgId", skip_serializing_if="Option::is_none")]
  last_rx_msg_id: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to the maximum number of messages allowed on the broker being exceeded.
  #[serde(rename = "localMsgCountExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  local_msg_count_exceeded_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to congestion of low priority messages.
  #[serde(rename = "lowPriorityMsgCongestionDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  low_priority_msg_congestion_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to the maximum allowed message size being exceeded.
  #[serde(rename = "maxMsgSizeExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  max_msg_size_exceeded_discarded_msg_count: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The number of guaranteed messages from the Flow discarded due to there being no eligible endpoint destination.
  #[serde(rename = "noEligibleDestinationsDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  no_eligible_destinations_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to no local delivery being requested.
  #[serde(rename = "noLocalDeliveryDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  no_local_delivery_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to being incompatible with the forwarding mode of an eligible endpoint destination.
  #[serde(rename = "notCompatibleWithForwardingModeDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  not_compatible_with_forwarding_mode_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to being received out of order.
  #[serde(rename = "outOfOrderDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  out_of_order_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to being denied by the access control list (ACL) profile for the published topic.
  #[serde(rename = "publishAclDeniedDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  publish_acl_denied_discarded_msg_count: Option<i64>,
  /// The identifier (ID) of the publisher for the Flow.
  #[serde(rename = "publisherId", skip_serializing_if="Option::is_none")]
  publisher_id: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to the destination queue not being found.
  #[serde(rename = "queueNotFoundDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  queue_not_found_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to the Message VPN being in the replication standby state.
  #[serde(rename = "replicationStandbyDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  replication_standby_discarded_msg_count: Option<i64>,
  /// The name of the transacted session on the Flow.
  #[serde(rename = "sessionName", skip_serializing_if="Option::is_none")]
  session_name: Option<String>,
  /// The number of guaranteed messages from the Flow discarded due to the message time-to-live (TTL) count being exceeded. The message TTL count is the maximum number of times the message can cross a bridge between Message VPNs.
  #[serde(rename = "smfTtlExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  smf_ttl_exceeded_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to all available message spool file resources being used.
  #[serde(rename = "spoolFileLimitExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  spool_file_limit_exceeded_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to the message spool being not ready.
  #[serde(rename = "spoolNotReadyDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  spool_not_ready_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to a failure while spooling to the Assured Delivery Blade (ADB).
  #[serde(rename = "spoolToAdbFailDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  spool_to_adb_fail_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to a failure while spooling to the disk.
  #[serde(rename = "spoolToDiskFailDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  spool_to_disk_fail_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to the maximum message spool usage being exceeded.
  #[serde(rename = "spoolUsageExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  spool_usage_exceeded_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to synchronous replication being ineligible.
  #[serde(rename = "syncReplicationIneligibleDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  sync_replication_ineligible_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages from the Flow discarded due to being denied by the client profile.
  #[serde(rename = "userProfileDeniedGuaranteedDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  user_profile_denied_guaranteed_discarded_msg_count: Option<i64>,
  /// The size of the window used for guaranteed messages sent on the Flow, in messages.
  #[serde(rename = "windowSize", skip_serializing_if="Option::is_none")]
  window_size: Option<i32>
}

impl MsgVpnClientRxFlow {
  pub fn new() -> MsgVpnClientRxFlow {
    MsgVpnClientRxFlow {
      client_name: None,
      connect_time: None,
      destination_group_error_discarded_msg_count: None,
      duplicate_discarded_msg_count: None,
      endpoint_disabled_discarded_msg_count: None,
      endpoint_usage_exceeded_discarded_msg_count: None,
      errored_discarded_msg_count: None,
      flow_id: None,
      flow_name: None,
      guaranteed_msg_count: None,
      last_rx_msg_id: None,
      local_msg_count_exceeded_discarded_msg_count: None,
      low_priority_msg_congestion_discarded_msg_count: None,
      max_msg_size_exceeded_discarded_msg_count: None,
      msg_vpn_name: None,
      no_eligible_destinations_discarded_msg_count: None,
      no_local_delivery_discarded_msg_count: None,
      not_compatible_with_forwarding_mode_discarded_msg_count: None,
      out_of_order_discarded_msg_count: None,
      publish_acl_denied_discarded_msg_count: None,
      publisher_id: None,
      queue_not_found_discarded_msg_count: None,
      replication_standby_discarded_msg_count: None,
      session_name: None,
      smf_ttl_exceeded_discarded_msg_count: None,
      spool_file_limit_exceeded_discarded_msg_count: None,
      spool_not_ready_discarded_msg_count: None,
      spool_to_adb_fail_discarded_msg_count: None,
      spool_to_disk_fail_discarded_msg_count: None,
      spool_usage_exceeded_discarded_msg_count: None,
      sync_replication_ineligible_discarded_msg_count: None,
      user_profile_denied_guaranteed_discarded_msg_count: None,
      window_size: None
    }
  }

  pub fn set_client_name(&mut self, client_name: String) {
    self.client_name = Some(client_name);
  }

  pub fn with_client_name(mut self, client_name: String) -> MsgVpnClientRxFlow {
    self.client_name = Some(client_name);
    self
  }

  pub fn client_name(&self) -> Option<&String> {
    self.client_name.as_ref()
  }

  pub fn reset_client_name(&mut self) {
    self.client_name = None;
  }

  pub fn set_connect_time(&mut self, connect_time: i32) {
    self.connect_time = Some(connect_time);
  }

  pub fn with_connect_time(mut self, connect_time: i32) -> MsgVpnClientRxFlow {
    self.connect_time = Some(connect_time);
    self
  }

  pub fn connect_time(&self) -> Option<&i32> {
    self.connect_time.as_ref()
  }

  pub fn reset_connect_time(&mut self) {
    self.connect_time = None;
  }

  pub fn set_destination_group_error_discarded_msg_count(&mut self, destination_group_error_discarded_msg_count: i64) {
    self.destination_group_error_discarded_msg_count = Some(destination_group_error_discarded_msg_count);
  }

  pub fn with_destination_group_error_discarded_msg_count(mut self, destination_group_error_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.destination_group_error_discarded_msg_count = Some(destination_group_error_discarded_msg_count);
    self
  }

  pub fn destination_group_error_discarded_msg_count(&self) -> Option<&i64> {
    self.destination_group_error_discarded_msg_count.as_ref()
  }

  pub fn reset_destination_group_error_discarded_msg_count(&mut self) {
    self.destination_group_error_discarded_msg_count = None;
  }

  pub fn set_duplicate_discarded_msg_count(&mut self, duplicate_discarded_msg_count: i64) {
    self.duplicate_discarded_msg_count = Some(duplicate_discarded_msg_count);
  }

  pub fn with_duplicate_discarded_msg_count(mut self, duplicate_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.duplicate_discarded_msg_count = Some(duplicate_discarded_msg_count);
    self
  }

  pub fn duplicate_discarded_msg_count(&self) -> Option<&i64> {
    self.duplicate_discarded_msg_count.as_ref()
  }

  pub fn reset_duplicate_discarded_msg_count(&mut self) {
    self.duplicate_discarded_msg_count = None;
  }

  pub fn set_endpoint_disabled_discarded_msg_count(&mut self, endpoint_disabled_discarded_msg_count: i64) {
    self.endpoint_disabled_discarded_msg_count = Some(endpoint_disabled_discarded_msg_count);
  }

  pub fn with_endpoint_disabled_discarded_msg_count(mut self, endpoint_disabled_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.endpoint_disabled_discarded_msg_count = Some(endpoint_disabled_discarded_msg_count);
    self
  }

  pub fn endpoint_disabled_discarded_msg_count(&self) -> Option<&i64> {
    self.endpoint_disabled_discarded_msg_count.as_ref()
  }

  pub fn reset_endpoint_disabled_discarded_msg_count(&mut self) {
    self.endpoint_disabled_discarded_msg_count = None;
  }

  pub fn set_endpoint_usage_exceeded_discarded_msg_count(&mut self, endpoint_usage_exceeded_discarded_msg_count: i64) {
    self.endpoint_usage_exceeded_discarded_msg_count = Some(endpoint_usage_exceeded_discarded_msg_count);
  }

  pub fn with_endpoint_usage_exceeded_discarded_msg_count(mut self, endpoint_usage_exceeded_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.endpoint_usage_exceeded_discarded_msg_count = Some(endpoint_usage_exceeded_discarded_msg_count);
    self
  }

  pub fn endpoint_usage_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.endpoint_usage_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_endpoint_usage_exceeded_discarded_msg_count(&mut self) {
    self.endpoint_usage_exceeded_discarded_msg_count = None;
  }

  pub fn set_errored_discarded_msg_count(&mut self, errored_discarded_msg_count: i64) {
    self.errored_discarded_msg_count = Some(errored_discarded_msg_count);
  }

  pub fn with_errored_discarded_msg_count(mut self, errored_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.errored_discarded_msg_count = Some(errored_discarded_msg_count);
    self
  }

  pub fn errored_discarded_msg_count(&self) -> Option<&i64> {
    self.errored_discarded_msg_count.as_ref()
  }

  pub fn reset_errored_discarded_msg_count(&mut self) {
    self.errored_discarded_msg_count = None;
  }

  pub fn set_flow_id(&mut self, flow_id: i64) {
    self.flow_id = Some(flow_id);
  }

  pub fn with_flow_id(mut self, flow_id: i64) -> MsgVpnClientRxFlow {
    self.flow_id = Some(flow_id);
    self
  }

  pub fn flow_id(&self) -> Option<&i64> {
    self.flow_id.as_ref()
  }

  pub fn reset_flow_id(&mut self) {
    self.flow_id = None;
  }

  pub fn set_flow_name(&mut self, flow_name: String) {
    self.flow_name = Some(flow_name);
  }

  pub fn with_flow_name(mut self, flow_name: String) -> MsgVpnClientRxFlow {
    self.flow_name = Some(flow_name);
    self
  }

  pub fn flow_name(&self) -> Option<&String> {
    self.flow_name.as_ref()
  }

  pub fn reset_flow_name(&mut self) {
    self.flow_name = None;
  }

  pub fn set_guaranteed_msg_count(&mut self, guaranteed_msg_count: i64) {
    self.guaranteed_msg_count = Some(guaranteed_msg_count);
  }

  pub fn with_guaranteed_msg_count(mut self, guaranteed_msg_count: i64) -> MsgVpnClientRxFlow {
    self.guaranteed_msg_count = Some(guaranteed_msg_count);
    self
  }

  pub fn guaranteed_msg_count(&self) -> Option<&i64> {
    self.guaranteed_msg_count.as_ref()
  }

  pub fn reset_guaranteed_msg_count(&mut self) {
    self.guaranteed_msg_count = None;
  }

  pub fn set_last_rx_msg_id(&mut self, last_rx_msg_id: i64) {
    self.last_rx_msg_id = Some(last_rx_msg_id);
  }

  pub fn with_last_rx_msg_id(mut self, last_rx_msg_id: i64) -> MsgVpnClientRxFlow {
    self.last_rx_msg_id = Some(last_rx_msg_id);
    self
  }

  pub fn last_rx_msg_id(&self) -> Option<&i64> {
    self.last_rx_msg_id.as_ref()
  }

  pub fn reset_last_rx_msg_id(&mut self) {
    self.last_rx_msg_id = None;
  }

  pub fn set_local_msg_count_exceeded_discarded_msg_count(&mut self, local_msg_count_exceeded_discarded_msg_count: i64) {
    self.local_msg_count_exceeded_discarded_msg_count = Some(local_msg_count_exceeded_discarded_msg_count);
  }

  pub fn with_local_msg_count_exceeded_discarded_msg_count(mut self, local_msg_count_exceeded_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.local_msg_count_exceeded_discarded_msg_count = Some(local_msg_count_exceeded_discarded_msg_count);
    self
  }

  pub fn local_msg_count_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.local_msg_count_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_local_msg_count_exceeded_discarded_msg_count(&mut self) {
    self.local_msg_count_exceeded_discarded_msg_count = None;
  }

  pub fn set_low_priority_msg_congestion_discarded_msg_count(&mut self, low_priority_msg_congestion_discarded_msg_count: i64) {
    self.low_priority_msg_congestion_discarded_msg_count = Some(low_priority_msg_congestion_discarded_msg_count);
  }

  pub fn with_low_priority_msg_congestion_discarded_msg_count(mut self, low_priority_msg_congestion_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.low_priority_msg_congestion_discarded_msg_count = Some(low_priority_msg_congestion_discarded_msg_count);
    self
  }

  pub fn low_priority_msg_congestion_discarded_msg_count(&self) -> Option<&i64> {
    self.low_priority_msg_congestion_discarded_msg_count.as_ref()
  }

  pub fn reset_low_priority_msg_congestion_discarded_msg_count(&mut self) {
    self.low_priority_msg_congestion_discarded_msg_count = None;
  }

  pub fn set_max_msg_size_exceeded_discarded_msg_count(&mut self, max_msg_size_exceeded_discarded_msg_count: i64) {
    self.max_msg_size_exceeded_discarded_msg_count = Some(max_msg_size_exceeded_discarded_msg_count);
  }

  pub fn with_max_msg_size_exceeded_discarded_msg_count(mut self, max_msg_size_exceeded_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.max_msg_size_exceeded_discarded_msg_count = Some(max_msg_size_exceeded_discarded_msg_count);
    self
  }

  pub fn max_msg_size_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.max_msg_size_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_max_msg_size_exceeded_discarded_msg_count(&mut self) {
    self.max_msg_size_exceeded_discarded_msg_count = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnClientRxFlow {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_no_eligible_destinations_discarded_msg_count(&mut self, no_eligible_destinations_discarded_msg_count: i64) {
    self.no_eligible_destinations_discarded_msg_count = Some(no_eligible_destinations_discarded_msg_count);
  }

  pub fn with_no_eligible_destinations_discarded_msg_count(mut self, no_eligible_destinations_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.no_eligible_destinations_discarded_msg_count = Some(no_eligible_destinations_discarded_msg_count);
    self
  }

  pub fn no_eligible_destinations_discarded_msg_count(&self) -> Option<&i64> {
    self.no_eligible_destinations_discarded_msg_count.as_ref()
  }

  pub fn reset_no_eligible_destinations_discarded_msg_count(&mut self) {
    self.no_eligible_destinations_discarded_msg_count = None;
  }

  pub fn set_no_local_delivery_discarded_msg_count(&mut self, no_local_delivery_discarded_msg_count: i64) {
    self.no_local_delivery_discarded_msg_count = Some(no_local_delivery_discarded_msg_count);
  }

  pub fn with_no_local_delivery_discarded_msg_count(mut self, no_local_delivery_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.no_local_delivery_discarded_msg_count = Some(no_local_delivery_discarded_msg_count);
    self
  }

  pub fn no_local_delivery_discarded_msg_count(&self) -> Option<&i64> {
    self.no_local_delivery_discarded_msg_count.as_ref()
  }

  pub fn reset_no_local_delivery_discarded_msg_count(&mut self) {
    self.no_local_delivery_discarded_msg_count = None;
  }

  pub fn set_not_compatible_with_forwarding_mode_discarded_msg_count(&mut self, not_compatible_with_forwarding_mode_discarded_msg_count: i64) {
    self.not_compatible_with_forwarding_mode_discarded_msg_count = Some(not_compatible_with_forwarding_mode_discarded_msg_count);
  }

  pub fn with_not_compatible_with_forwarding_mode_discarded_msg_count(mut self, not_compatible_with_forwarding_mode_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.not_compatible_with_forwarding_mode_discarded_msg_count = Some(not_compatible_with_forwarding_mode_discarded_msg_count);
    self
  }

  pub fn not_compatible_with_forwarding_mode_discarded_msg_count(&self) -> Option<&i64> {
    self.not_compatible_with_forwarding_mode_discarded_msg_count.as_ref()
  }

  pub fn reset_not_compatible_with_forwarding_mode_discarded_msg_count(&mut self) {
    self.not_compatible_with_forwarding_mode_discarded_msg_count = None;
  }

  pub fn set_out_of_order_discarded_msg_count(&mut self, out_of_order_discarded_msg_count: i64) {
    self.out_of_order_discarded_msg_count = Some(out_of_order_discarded_msg_count);
  }

  pub fn with_out_of_order_discarded_msg_count(mut self, out_of_order_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.out_of_order_discarded_msg_count = Some(out_of_order_discarded_msg_count);
    self
  }

  pub fn out_of_order_discarded_msg_count(&self) -> Option<&i64> {
    self.out_of_order_discarded_msg_count.as_ref()
  }

  pub fn reset_out_of_order_discarded_msg_count(&mut self) {
    self.out_of_order_discarded_msg_count = None;
  }

  pub fn set_publish_acl_denied_discarded_msg_count(&mut self, publish_acl_denied_discarded_msg_count: i64) {
    self.publish_acl_denied_discarded_msg_count = Some(publish_acl_denied_discarded_msg_count);
  }

  pub fn with_publish_acl_denied_discarded_msg_count(mut self, publish_acl_denied_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.publish_acl_denied_discarded_msg_count = Some(publish_acl_denied_discarded_msg_count);
    self
  }

  pub fn publish_acl_denied_discarded_msg_count(&self) -> Option<&i64> {
    self.publish_acl_denied_discarded_msg_count.as_ref()
  }

  pub fn reset_publish_acl_denied_discarded_msg_count(&mut self) {
    self.publish_acl_denied_discarded_msg_count = None;
  }

  pub fn set_publisher_id(&mut self, publisher_id: i64) {
    self.publisher_id = Some(publisher_id);
  }

  pub fn with_publisher_id(mut self, publisher_id: i64) -> MsgVpnClientRxFlow {
    self.publisher_id = Some(publisher_id);
    self
  }

  pub fn publisher_id(&self) -> Option<&i64> {
    self.publisher_id.as_ref()
  }

  pub fn reset_publisher_id(&mut self) {
    self.publisher_id = None;
  }

  pub fn set_queue_not_found_discarded_msg_count(&mut self, queue_not_found_discarded_msg_count: i64) {
    self.queue_not_found_discarded_msg_count = Some(queue_not_found_discarded_msg_count);
  }

  pub fn with_queue_not_found_discarded_msg_count(mut self, queue_not_found_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.queue_not_found_discarded_msg_count = Some(queue_not_found_discarded_msg_count);
    self
  }

  pub fn queue_not_found_discarded_msg_count(&self) -> Option<&i64> {
    self.queue_not_found_discarded_msg_count.as_ref()
  }

  pub fn reset_queue_not_found_discarded_msg_count(&mut self) {
    self.queue_not_found_discarded_msg_count = None;
  }

  pub fn set_replication_standby_discarded_msg_count(&mut self, replication_standby_discarded_msg_count: i64) {
    self.replication_standby_discarded_msg_count = Some(replication_standby_discarded_msg_count);
  }

  pub fn with_replication_standby_discarded_msg_count(mut self, replication_standby_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.replication_standby_discarded_msg_count = Some(replication_standby_discarded_msg_count);
    self
  }

  pub fn replication_standby_discarded_msg_count(&self) -> Option<&i64> {
    self.replication_standby_discarded_msg_count.as_ref()
  }

  pub fn reset_replication_standby_discarded_msg_count(&mut self) {
    self.replication_standby_discarded_msg_count = None;
  }

  pub fn set_session_name(&mut self, session_name: String) {
    self.session_name = Some(session_name);
  }

  pub fn with_session_name(mut self, session_name: String) -> MsgVpnClientRxFlow {
    self.session_name = Some(session_name);
    self
  }

  pub fn session_name(&self) -> Option<&String> {
    self.session_name.as_ref()
  }

  pub fn reset_session_name(&mut self) {
    self.session_name = None;
  }

  pub fn set_smf_ttl_exceeded_discarded_msg_count(&mut self, smf_ttl_exceeded_discarded_msg_count: i64) {
    self.smf_ttl_exceeded_discarded_msg_count = Some(smf_ttl_exceeded_discarded_msg_count);
  }

  pub fn with_smf_ttl_exceeded_discarded_msg_count(mut self, smf_ttl_exceeded_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.smf_ttl_exceeded_discarded_msg_count = Some(smf_ttl_exceeded_discarded_msg_count);
    self
  }

  pub fn smf_ttl_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.smf_ttl_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_smf_ttl_exceeded_discarded_msg_count(&mut self) {
    self.smf_ttl_exceeded_discarded_msg_count = None;
  }

  pub fn set_spool_file_limit_exceeded_discarded_msg_count(&mut self, spool_file_limit_exceeded_discarded_msg_count: i64) {
    self.spool_file_limit_exceeded_discarded_msg_count = Some(spool_file_limit_exceeded_discarded_msg_count);
  }

  pub fn with_spool_file_limit_exceeded_discarded_msg_count(mut self, spool_file_limit_exceeded_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.spool_file_limit_exceeded_discarded_msg_count = Some(spool_file_limit_exceeded_discarded_msg_count);
    self
  }

  pub fn spool_file_limit_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.spool_file_limit_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_spool_file_limit_exceeded_discarded_msg_count(&mut self) {
    self.spool_file_limit_exceeded_discarded_msg_count = None;
  }

  pub fn set_spool_not_ready_discarded_msg_count(&mut self, spool_not_ready_discarded_msg_count: i64) {
    self.spool_not_ready_discarded_msg_count = Some(spool_not_ready_discarded_msg_count);
  }

  pub fn with_spool_not_ready_discarded_msg_count(mut self, spool_not_ready_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.spool_not_ready_discarded_msg_count = Some(spool_not_ready_discarded_msg_count);
    self
  }

  pub fn spool_not_ready_discarded_msg_count(&self) -> Option<&i64> {
    self.spool_not_ready_discarded_msg_count.as_ref()
  }

  pub fn reset_spool_not_ready_discarded_msg_count(&mut self) {
    self.spool_not_ready_discarded_msg_count = None;
  }

  pub fn set_spool_to_adb_fail_discarded_msg_count(&mut self, spool_to_adb_fail_discarded_msg_count: i64) {
    self.spool_to_adb_fail_discarded_msg_count = Some(spool_to_adb_fail_discarded_msg_count);
  }

  pub fn with_spool_to_adb_fail_discarded_msg_count(mut self, spool_to_adb_fail_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.spool_to_adb_fail_discarded_msg_count = Some(spool_to_adb_fail_discarded_msg_count);
    self
  }

  pub fn spool_to_adb_fail_discarded_msg_count(&self) -> Option<&i64> {
    self.spool_to_adb_fail_discarded_msg_count.as_ref()
  }

  pub fn reset_spool_to_adb_fail_discarded_msg_count(&mut self) {
    self.spool_to_adb_fail_discarded_msg_count = None;
  }

  pub fn set_spool_to_disk_fail_discarded_msg_count(&mut self, spool_to_disk_fail_discarded_msg_count: i64) {
    self.spool_to_disk_fail_discarded_msg_count = Some(spool_to_disk_fail_discarded_msg_count);
  }

  pub fn with_spool_to_disk_fail_discarded_msg_count(mut self, spool_to_disk_fail_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.spool_to_disk_fail_discarded_msg_count = Some(spool_to_disk_fail_discarded_msg_count);
    self
  }

  pub fn spool_to_disk_fail_discarded_msg_count(&self) -> Option<&i64> {
    self.spool_to_disk_fail_discarded_msg_count.as_ref()
  }

  pub fn reset_spool_to_disk_fail_discarded_msg_count(&mut self) {
    self.spool_to_disk_fail_discarded_msg_count = None;
  }

  pub fn set_spool_usage_exceeded_discarded_msg_count(&mut self, spool_usage_exceeded_discarded_msg_count: i64) {
    self.spool_usage_exceeded_discarded_msg_count = Some(spool_usage_exceeded_discarded_msg_count);
  }

  pub fn with_spool_usage_exceeded_discarded_msg_count(mut self, spool_usage_exceeded_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.spool_usage_exceeded_discarded_msg_count = Some(spool_usage_exceeded_discarded_msg_count);
    self
  }

  pub fn spool_usage_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.spool_usage_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_spool_usage_exceeded_discarded_msg_count(&mut self) {
    self.spool_usage_exceeded_discarded_msg_count = None;
  }

  pub fn set_sync_replication_ineligible_discarded_msg_count(&mut self, sync_replication_ineligible_discarded_msg_count: i64) {
    self.sync_replication_ineligible_discarded_msg_count = Some(sync_replication_ineligible_discarded_msg_count);
  }

  pub fn with_sync_replication_ineligible_discarded_msg_count(mut self, sync_replication_ineligible_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.sync_replication_ineligible_discarded_msg_count = Some(sync_replication_ineligible_discarded_msg_count);
    self
  }

  pub fn sync_replication_ineligible_discarded_msg_count(&self) -> Option<&i64> {
    self.sync_replication_ineligible_discarded_msg_count.as_ref()
  }

  pub fn reset_sync_replication_ineligible_discarded_msg_count(&mut self) {
    self.sync_replication_ineligible_discarded_msg_count = None;
  }

  pub fn set_user_profile_denied_guaranteed_discarded_msg_count(&mut self, user_profile_denied_guaranteed_discarded_msg_count: i64) {
    self.user_profile_denied_guaranteed_discarded_msg_count = Some(user_profile_denied_guaranteed_discarded_msg_count);
  }

  pub fn with_user_profile_denied_guaranteed_discarded_msg_count(mut self, user_profile_denied_guaranteed_discarded_msg_count: i64) -> MsgVpnClientRxFlow {
    self.user_profile_denied_guaranteed_discarded_msg_count = Some(user_profile_denied_guaranteed_discarded_msg_count);
    self
  }

  pub fn user_profile_denied_guaranteed_discarded_msg_count(&self) -> Option<&i64> {
    self.user_profile_denied_guaranteed_discarded_msg_count.as_ref()
  }

  pub fn reset_user_profile_denied_guaranteed_discarded_msg_count(&mut self) {
    self.user_profile_denied_guaranteed_discarded_msg_count = None;
  }

  pub fn set_window_size(&mut self, window_size: i32) {
    self.window_size = Some(window_size);
  }

  pub fn with_window_size(mut self, window_size: i32) -> MsgVpnClientRxFlow {
    self.window_size = Some(window_size);
    self
  }

  pub fn window_size(&self) -> Option<&i32> {
    self.window_size.as_ref()
  }

  pub fn reset_window_size(&mut self) {
    self.window_size = None;
  }

}



