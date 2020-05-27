/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/monitor/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/monitor/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/monitor/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/monitor/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/monitor/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/monitor/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/monitor/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/monitor/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/monitor/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.  
 *
 * OpenAPI spec version: 2.16
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnTopicEndpointTxFlow {
  /// The number of guaranteed messages delivered and acknowledged by the consumer.
  #[serde(rename = "ackedMsgCount", skip_serializing_if="Option::is_none")]
  acked_msg_count: Option<i64>,
  /// The activity state of the Flow. The allowed values and their meaning are:  <pre> \"active-browser\" - The Flow is active as a browser. \"active-consumer\" - The Flow is active as a consumer. \"inactive\" - The Flow is inactive. </pre> 
  #[serde(rename = "activityState", skip_serializing_if="Option::is_none")]
  activity_state: Option<String>,
  /// The timestamp of when the Flow bound to the Topic Endpoint. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "bindTime", skip_serializing_if="Option::is_none")]
  bind_time: Option<i32>,
  /// The name of the Client.
  #[serde(rename = "clientName", skip_serializing_if="Option::is_none")]
  client_name: Option<String>,
  /// Indicates whether redelivery requests can be received as negative acknowledgements (NACKs) from the consumer. Applicable only to REST consumers.
  #[serde(rename = "consumerRedeliveryRequestAllowed", skip_serializing_if="Option::is_none")]
  consumer_redelivery_request_allowed: Option<bool>,
  /// The number of guaranteed messages that used cut-through delivery and are acknowledged by the consumer.
  #[serde(rename = "cutThroughAckedMsgCount", skip_serializing_if="Option::is_none")]
  cut_through_acked_msg_count: Option<i64>,
  /// The delivery state of the Flow. The allowed values and their meaning are:  <pre> \"closed\" - The Flow is unbound. \"opened\" - The Flow is bound but inactive. \"unbinding\" - The Flow received an unbind request. \"handshaking\" - The Flow is handshaking to become active. \"deliver-cut-through\" - The Flow is streaming messages using direct+guaranteed delivery. \"deliver-from-input-stream\" - The Flow is streaming messages using guaranteed delivery. \"deliver-from-memory\" - The Flow throttled causing message delivery from memory (RAM). \"deliver-from-spool\" - The Flow stalled causing message delivery from spool (ADB or disk). </pre> 
  #[serde(rename = "deliveryState", skip_serializing_if="Option::is_none")]
  delivery_state: Option<String>,
  /// The identifier (ID) of the Flow.
  #[serde(rename = "flowId", skip_serializing_if="Option::is_none")]
  flow_id: Option<i64>,
  /// The highest identifier (ID) of message transmitted and waiting for acknowledgement.
  #[serde(rename = "highestAckPendingMsgId", skip_serializing_if="Option::is_none")]
  highest_ack_pending_msg_id: Option<i64>,
  /// The identifier (ID) of the last message transmitted and acknowledged by the consumer.
  #[serde(rename = "lastAckedMsgId", skip_serializing_if="Option::is_none")]
  last_acked_msg_id: Option<i64>,
  /// The lowest identifier (ID) of message transmitted and waiting for acknowledgement.
  #[serde(rename = "lowestAckPendingMsgId", skip_serializing_if="Option::is_none")]
  lowest_ack_pending_msg_id: Option<i64>,
  /// The number of guaranteed messages that exceeded the maximum number of delivered unacknowledged messages.
  #[serde(rename = "maxUnackedMsgsExceededMsgCount", skip_serializing_if="Option::is_none")]
  max_unacked_msgs_exceeded_msg_count: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// Indicates whether not to deliver messages to a consumer that published them.
  #[serde(rename = "noLocalDelivery", skip_serializing_if="Option::is_none")]
  no_local_delivery: Option<bool>,
  /// The number of guaranteed messages that were redelivered.
  #[serde(rename = "redeliveredMsgCount", skip_serializing_if="Option::is_none")]
  redelivered_msg_count: Option<i64>,
  /// The number of consumer requests via negative acknowledgements (NACKs) to redeliver guaranteed messages.
  #[serde(rename = "redeliveryRequestCount", skip_serializing_if="Option::is_none")]
  redelivery_request_count: Option<i64>,
  /// The name of the Transacted Session for the Flow.
  #[serde(rename = "sessionName", skip_serializing_if="Option::is_none")]
  session_name: Option<String>,
  /// The number of guaranteed messages that used store and forward delivery and are acknowledged by the consumer.
  #[serde(rename = "storeAndForwardAckedMsgCount", skip_serializing_if="Option::is_none")]
  store_and_forward_acked_msg_count: Option<i64>,
  /// The name of the Topic Endpoint.
  #[serde(rename = "topicEndpointName", skip_serializing_if="Option::is_none")]
  topic_endpoint_name: Option<String>,
  /// The number of guaranteed messages delivered but not yet acknowledged by the consumer.
  #[serde(rename = "unackedMsgCount", skip_serializing_if="Option::is_none")]
  unacked_msg_count: Option<i64>,
  /// The number of guaranteed messages using the available window size.
  #[serde(rename = "usedWindowSize", skip_serializing_if="Option::is_none")]
  used_window_size: Option<i32>,
  /// The number of times the window for guaranteed messages was filled and closed before an acknowledgement was received.
  #[serde(rename = "windowClosedCount", skip_serializing_if="Option::is_none")]
  window_closed_count: Option<i64>,
  /// The number of outstanding guaranteed messages that can be transmitted over the Flow before an acknowledgement is received.
  #[serde(rename = "windowSize", skip_serializing_if="Option::is_none")]
  window_size: Option<i64>
}

impl MsgVpnTopicEndpointTxFlow {
  pub fn new() -> MsgVpnTopicEndpointTxFlow {
    MsgVpnTopicEndpointTxFlow {
      acked_msg_count: None,
      activity_state: None,
      bind_time: None,
      client_name: None,
      consumer_redelivery_request_allowed: None,
      cut_through_acked_msg_count: None,
      delivery_state: None,
      flow_id: None,
      highest_ack_pending_msg_id: None,
      last_acked_msg_id: None,
      lowest_ack_pending_msg_id: None,
      max_unacked_msgs_exceeded_msg_count: None,
      msg_vpn_name: None,
      no_local_delivery: None,
      redelivered_msg_count: None,
      redelivery_request_count: None,
      session_name: None,
      store_and_forward_acked_msg_count: None,
      topic_endpoint_name: None,
      unacked_msg_count: None,
      used_window_size: None,
      window_closed_count: None,
      window_size: None
    }
  }

  pub fn set_acked_msg_count(&mut self, acked_msg_count: i64) {
    self.acked_msg_count = Some(acked_msg_count);
  }

  pub fn with_acked_msg_count(mut self, acked_msg_count: i64) -> MsgVpnTopicEndpointTxFlow {
    self.acked_msg_count = Some(acked_msg_count);
    self
  }

  pub fn acked_msg_count(&self) -> Option<&i64> {
    self.acked_msg_count.as_ref()
  }

  pub fn reset_acked_msg_count(&mut self) {
    self.acked_msg_count = None;
  }

  pub fn set_activity_state(&mut self, activity_state: String) {
    self.activity_state = Some(activity_state);
  }

  pub fn with_activity_state(mut self, activity_state: String) -> MsgVpnTopicEndpointTxFlow {
    self.activity_state = Some(activity_state);
    self
  }

  pub fn activity_state(&self) -> Option<&String> {
    self.activity_state.as_ref()
  }

  pub fn reset_activity_state(&mut self) {
    self.activity_state = None;
  }

  pub fn set_bind_time(&mut self, bind_time: i32) {
    self.bind_time = Some(bind_time);
  }

  pub fn with_bind_time(mut self, bind_time: i32) -> MsgVpnTopicEndpointTxFlow {
    self.bind_time = Some(bind_time);
    self
  }

  pub fn bind_time(&self) -> Option<&i32> {
    self.bind_time.as_ref()
  }

  pub fn reset_bind_time(&mut self) {
    self.bind_time = None;
  }

  pub fn set_client_name(&mut self, client_name: String) {
    self.client_name = Some(client_name);
  }

  pub fn with_client_name(mut self, client_name: String) -> MsgVpnTopicEndpointTxFlow {
    self.client_name = Some(client_name);
    self
  }

  pub fn client_name(&self) -> Option<&String> {
    self.client_name.as_ref()
  }

  pub fn reset_client_name(&mut self) {
    self.client_name = None;
  }

  pub fn set_consumer_redelivery_request_allowed(&mut self, consumer_redelivery_request_allowed: bool) {
    self.consumer_redelivery_request_allowed = Some(consumer_redelivery_request_allowed);
  }

  pub fn with_consumer_redelivery_request_allowed(mut self, consumer_redelivery_request_allowed: bool) -> MsgVpnTopicEndpointTxFlow {
    self.consumer_redelivery_request_allowed = Some(consumer_redelivery_request_allowed);
    self
  }

  pub fn consumer_redelivery_request_allowed(&self) -> Option<&bool> {
    self.consumer_redelivery_request_allowed.as_ref()
  }

  pub fn reset_consumer_redelivery_request_allowed(&mut self) {
    self.consumer_redelivery_request_allowed = None;
  }

  pub fn set_cut_through_acked_msg_count(&mut self, cut_through_acked_msg_count: i64) {
    self.cut_through_acked_msg_count = Some(cut_through_acked_msg_count);
  }

  pub fn with_cut_through_acked_msg_count(mut self, cut_through_acked_msg_count: i64) -> MsgVpnTopicEndpointTxFlow {
    self.cut_through_acked_msg_count = Some(cut_through_acked_msg_count);
    self
  }

  pub fn cut_through_acked_msg_count(&self) -> Option<&i64> {
    self.cut_through_acked_msg_count.as_ref()
  }

  pub fn reset_cut_through_acked_msg_count(&mut self) {
    self.cut_through_acked_msg_count = None;
  }

  pub fn set_delivery_state(&mut self, delivery_state: String) {
    self.delivery_state = Some(delivery_state);
  }

  pub fn with_delivery_state(mut self, delivery_state: String) -> MsgVpnTopicEndpointTxFlow {
    self.delivery_state = Some(delivery_state);
    self
  }

  pub fn delivery_state(&self) -> Option<&String> {
    self.delivery_state.as_ref()
  }

  pub fn reset_delivery_state(&mut self) {
    self.delivery_state = None;
  }

  pub fn set_flow_id(&mut self, flow_id: i64) {
    self.flow_id = Some(flow_id);
  }

  pub fn with_flow_id(mut self, flow_id: i64) -> MsgVpnTopicEndpointTxFlow {
    self.flow_id = Some(flow_id);
    self
  }

  pub fn flow_id(&self) -> Option<&i64> {
    self.flow_id.as_ref()
  }

  pub fn reset_flow_id(&mut self) {
    self.flow_id = None;
  }

  pub fn set_highest_ack_pending_msg_id(&mut self, highest_ack_pending_msg_id: i64) {
    self.highest_ack_pending_msg_id = Some(highest_ack_pending_msg_id);
  }

  pub fn with_highest_ack_pending_msg_id(mut self, highest_ack_pending_msg_id: i64) -> MsgVpnTopicEndpointTxFlow {
    self.highest_ack_pending_msg_id = Some(highest_ack_pending_msg_id);
    self
  }

  pub fn highest_ack_pending_msg_id(&self) -> Option<&i64> {
    self.highest_ack_pending_msg_id.as_ref()
  }

  pub fn reset_highest_ack_pending_msg_id(&mut self) {
    self.highest_ack_pending_msg_id = None;
  }

  pub fn set_last_acked_msg_id(&mut self, last_acked_msg_id: i64) {
    self.last_acked_msg_id = Some(last_acked_msg_id);
  }

  pub fn with_last_acked_msg_id(mut self, last_acked_msg_id: i64) -> MsgVpnTopicEndpointTxFlow {
    self.last_acked_msg_id = Some(last_acked_msg_id);
    self
  }

  pub fn last_acked_msg_id(&self) -> Option<&i64> {
    self.last_acked_msg_id.as_ref()
  }

  pub fn reset_last_acked_msg_id(&mut self) {
    self.last_acked_msg_id = None;
  }

  pub fn set_lowest_ack_pending_msg_id(&mut self, lowest_ack_pending_msg_id: i64) {
    self.lowest_ack_pending_msg_id = Some(lowest_ack_pending_msg_id);
  }

  pub fn with_lowest_ack_pending_msg_id(mut self, lowest_ack_pending_msg_id: i64) -> MsgVpnTopicEndpointTxFlow {
    self.lowest_ack_pending_msg_id = Some(lowest_ack_pending_msg_id);
    self
  }

  pub fn lowest_ack_pending_msg_id(&self) -> Option<&i64> {
    self.lowest_ack_pending_msg_id.as_ref()
  }

  pub fn reset_lowest_ack_pending_msg_id(&mut self) {
    self.lowest_ack_pending_msg_id = None;
  }

  pub fn set_max_unacked_msgs_exceeded_msg_count(&mut self, max_unacked_msgs_exceeded_msg_count: i64) {
    self.max_unacked_msgs_exceeded_msg_count = Some(max_unacked_msgs_exceeded_msg_count);
  }

  pub fn with_max_unacked_msgs_exceeded_msg_count(mut self, max_unacked_msgs_exceeded_msg_count: i64) -> MsgVpnTopicEndpointTxFlow {
    self.max_unacked_msgs_exceeded_msg_count = Some(max_unacked_msgs_exceeded_msg_count);
    self
  }

  pub fn max_unacked_msgs_exceeded_msg_count(&self) -> Option<&i64> {
    self.max_unacked_msgs_exceeded_msg_count.as_ref()
  }

  pub fn reset_max_unacked_msgs_exceeded_msg_count(&mut self) {
    self.max_unacked_msgs_exceeded_msg_count = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnTopicEndpointTxFlow {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_no_local_delivery(&mut self, no_local_delivery: bool) {
    self.no_local_delivery = Some(no_local_delivery);
  }

  pub fn with_no_local_delivery(mut self, no_local_delivery: bool) -> MsgVpnTopicEndpointTxFlow {
    self.no_local_delivery = Some(no_local_delivery);
    self
  }

  pub fn no_local_delivery(&self) -> Option<&bool> {
    self.no_local_delivery.as_ref()
  }

  pub fn reset_no_local_delivery(&mut self) {
    self.no_local_delivery = None;
  }

  pub fn set_redelivered_msg_count(&mut self, redelivered_msg_count: i64) {
    self.redelivered_msg_count = Some(redelivered_msg_count);
  }

  pub fn with_redelivered_msg_count(mut self, redelivered_msg_count: i64) -> MsgVpnTopicEndpointTxFlow {
    self.redelivered_msg_count = Some(redelivered_msg_count);
    self
  }

  pub fn redelivered_msg_count(&self) -> Option<&i64> {
    self.redelivered_msg_count.as_ref()
  }

  pub fn reset_redelivered_msg_count(&mut self) {
    self.redelivered_msg_count = None;
  }

  pub fn set_redelivery_request_count(&mut self, redelivery_request_count: i64) {
    self.redelivery_request_count = Some(redelivery_request_count);
  }

  pub fn with_redelivery_request_count(mut self, redelivery_request_count: i64) -> MsgVpnTopicEndpointTxFlow {
    self.redelivery_request_count = Some(redelivery_request_count);
    self
  }

  pub fn redelivery_request_count(&self) -> Option<&i64> {
    self.redelivery_request_count.as_ref()
  }

  pub fn reset_redelivery_request_count(&mut self) {
    self.redelivery_request_count = None;
  }

  pub fn set_session_name(&mut self, session_name: String) {
    self.session_name = Some(session_name);
  }

  pub fn with_session_name(mut self, session_name: String) -> MsgVpnTopicEndpointTxFlow {
    self.session_name = Some(session_name);
    self
  }

  pub fn session_name(&self) -> Option<&String> {
    self.session_name.as_ref()
  }

  pub fn reset_session_name(&mut self) {
    self.session_name = None;
  }

  pub fn set_store_and_forward_acked_msg_count(&mut self, store_and_forward_acked_msg_count: i64) {
    self.store_and_forward_acked_msg_count = Some(store_and_forward_acked_msg_count);
  }

  pub fn with_store_and_forward_acked_msg_count(mut self, store_and_forward_acked_msg_count: i64) -> MsgVpnTopicEndpointTxFlow {
    self.store_and_forward_acked_msg_count = Some(store_and_forward_acked_msg_count);
    self
  }

  pub fn store_and_forward_acked_msg_count(&self) -> Option<&i64> {
    self.store_and_forward_acked_msg_count.as_ref()
  }

  pub fn reset_store_and_forward_acked_msg_count(&mut self) {
    self.store_and_forward_acked_msg_count = None;
  }

  pub fn set_topic_endpoint_name(&mut self, topic_endpoint_name: String) {
    self.topic_endpoint_name = Some(topic_endpoint_name);
  }

  pub fn with_topic_endpoint_name(mut self, topic_endpoint_name: String) -> MsgVpnTopicEndpointTxFlow {
    self.topic_endpoint_name = Some(topic_endpoint_name);
    self
  }

  pub fn topic_endpoint_name(&self) -> Option<&String> {
    self.topic_endpoint_name.as_ref()
  }

  pub fn reset_topic_endpoint_name(&mut self) {
    self.topic_endpoint_name = None;
  }

  pub fn set_unacked_msg_count(&mut self, unacked_msg_count: i64) {
    self.unacked_msg_count = Some(unacked_msg_count);
  }

  pub fn with_unacked_msg_count(mut self, unacked_msg_count: i64) -> MsgVpnTopicEndpointTxFlow {
    self.unacked_msg_count = Some(unacked_msg_count);
    self
  }

  pub fn unacked_msg_count(&self) -> Option<&i64> {
    self.unacked_msg_count.as_ref()
  }

  pub fn reset_unacked_msg_count(&mut self) {
    self.unacked_msg_count = None;
  }

  pub fn set_used_window_size(&mut self, used_window_size: i32) {
    self.used_window_size = Some(used_window_size);
  }

  pub fn with_used_window_size(mut self, used_window_size: i32) -> MsgVpnTopicEndpointTxFlow {
    self.used_window_size = Some(used_window_size);
    self
  }

  pub fn used_window_size(&self) -> Option<&i32> {
    self.used_window_size.as_ref()
  }

  pub fn reset_used_window_size(&mut self) {
    self.used_window_size = None;
  }

  pub fn set_window_closed_count(&mut self, window_closed_count: i64) {
    self.window_closed_count = Some(window_closed_count);
  }

  pub fn with_window_closed_count(mut self, window_closed_count: i64) -> MsgVpnTopicEndpointTxFlow {
    self.window_closed_count = Some(window_closed_count);
    self
  }

  pub fn window_closed_count(&self) -> Option<&i64> {
    self.window_closed_count.as_ref()
  }

  pub fn reset_window_closed_count(&mut self) {
    self.window_closed_count = None;
  }

  pub fn set_window_size(&mut self, window_size: i64) {
    self.window_size = Some(window_size);
  }

  pub fn with_window_size(mut self, window_size: i64) -> MsgVpnTopicEndpointTxFlow {
    self.window_size = Some(window_size);
    self
  }

  pub fn window_size(&self) -> Option<&i64> {
    self.window_size.as_ref()
  }

  pub fn reset_window_size(&mut self) {
    self.window_size = None;
  }

}



