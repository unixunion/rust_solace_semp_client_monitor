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
pub struct MsgVpnClientProfile {
  /// Indicates whether Bridge clients using the Client Profile are allowed to connect.
  #[serde(rename = "allowBridgeConnectionsEnabled", skip_serializing_if="Option::is_none")]
  allow_bridge_connections_enabled: Option<bool>,
  /// Indicates whether clients using the Client Profile are allowed to bind to endpoints with the cut-through forwarding delivery mode.
  #[serde(rename = "allowCutThroughForwardingEnabled", skip_serializing_if="Option::is_none")]
  allow_cut_through_forwarding_enabled: Option<bool>,
  /// Indicates whether clients using the Client Profile are allowed to create topic endponts or queues.
  #[serde(rename = "allowGuaranteedEndpointCreateEnabled", skip_serializing_if="Option::is_none")]
  allow_guaranteed_endpoint_create_enabled: Option<bool>,
  /// Indicates whether clients using the Client Profile are allowed to receive guaranteed messages.
  #[serde(rename = "allowGuaranteedMsgReceiveEnabled", skip_serializing_if="Option::is_none")]
  allow_guaranteed_msg_receive_enabled: Option<bool>,
  /// Indicates whether clients using the Client Profile are allowed to send guaranteed messages.
  #[serde(rename = "allowGuaranteedMsgSendEnabled", skip_serializing_if="Option::is_none")]
  allow_guaranteed_msg_send_enabled: Option<bool>,
  /// Enable or disable allowing shared subscriptions. Changing this setting does not affect existing subscriptions.
  #[serde(rename = "allowSharedSubscriptionsEnabled", skip_serializing_if="Option::is_none")]
  allow_shared_subscriptions_enabled: Option<bool>,
  /// Indicates whether clients using the Client Profile are allowed to establish transacted sessions.
  #[serde(rename = "allowTransactedSessionsEnabled", skip_serializing_if="Option::is_none")]
  allow_transacted_sessions_enabled: Option<bool>,
  /// The name of a queue to copy settings from when a new queue is created by a client using the Client Profile. The referenced queue must exist in the Message VPN.
  #[serde(rename = "apiQueueManagementCopyFromOnCreateName", skip_serializing_if="Option::is_none")]
  api_queue_management_copy_from_on_create_name: Option<String>,
  /// The name of a topic endpoint to copy settings from when a new topic endpoint is created by a client using the Client Profile. The referenced topic endpoint must exist in the Message VPN.
  #[serde(rename = "apiTopicEndpointManagementCopyFromOnCreateName", skip_serializing_if="Option::is_none")]
  api_topic_endpoint_management_copy_from_on_create_name: Option<String>,
  /// The name of the Client Profile.
  #[serde(rename = "clientProfileName", skip_serializing_if="Option::is_none")]
  client_profile_name: Option<String>,
  /// Indicates whether clients using the Client Profile are allowed to use compression.
  #[serde(rename = "compressionEnabled", skip_serializing_if="Option::is_none")]
  compression_enabled: Option<bool>,
  /// The amount of time to delay the delivery of messages to clients using the Client Profile after the initial message has been delivered (the eliding delay interval), in milliseconds. A value of 0 means there is no delay in delivering messages to clients.
  #[serde(rename = "elidingDelay", skip_serializing_if="Option::is_none")]
  eliding_delay: Option<i64>,
  /// Indicates whether message eliding is enabled for clients using the Client Profile.
  #[serde(rename = "elidingEnabled", skip_serializing_if="Option::is_none")]
  eliding_enabled: Option<bool>,
  /// The maximum number of topics tracked for message eliding per client connection using the Client Profile.
  #[serde(rename = "elidingMaxTopicCount", skip_serializing_if="Option::is_none")]
  eliding_max_topic_count: Option<i64>,
  #[serde(rename = "eventClientProvisionedEndpointSpoolUsageThreshold", skip_serializing_if="Option::is_none")]
  event_client_provisioned_endpoint_spool_usage_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "eventConnectionCountPerClientUsernameThreshold", skip_serializing_if="Option::is_none")]
  event_connection_count_per_client_username_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventEgressFlowCountThreshold", skip_serializing_if="Option::is_none")]
  event_egress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventEndpointCountPerClientUsernameThreshold", skip_serializing_if="Option::is_none")]
  event_endpoint_count_per_client_username_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventIngressFlowCountThreshold", skip_serializing_if="Option::is_none")]
  event_ingress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceSmfConnectionCountPerClientUsernameThreshold", skip_serializing_if="Option::is_none")]
  event_service_smf_connection_count_per_client_username_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceWebConnectionCountPerClientUsernameThreshold", skip_serializing_if="Option::is_none")]
  event_service_web_connection_count_per_client_username_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventSubscriptionCountThreshold", skip_serializing_if="Option::is_none")]
  event_subscription_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventTransactedSessionCountThreshold", skip_serializing_if="Option::is_none")]
  event_transacted_session_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventTransactionCountThreshold", skip_serializing_if="Option::is_none")]
  event_transaction_count_threshold: Option<::models::EventThreshold>,
  /// The maximum number of client connections per Client Username using the Client Profile.
  #[serde(rename = "maxConnectionCountPerClientUsername", skip_serializing_if="Option::is_none")]
  max_connection_count_per_client_username: Option<i64>,
  /// The effective maximum number of queues and topic endpoints per Client Username using the Client Profile.
  #[serde(rename = "maxEffectiveEndpointCount", skip_serializing_if="Option::is_none")]
  max_effective_endpoint_count: Option<i32>,
  /// The effective maximum number of receive flows per client using the Client Profile.
  #[serde(rename = "maxEffectiveRxFlowCount", skip_serializing_if="Option::is_none")]
  max_effective_rx_flow_count: Option<i32>,
  /// The effective maximum number of subscriptions per client using the Client Profile.
  #[serde(rename = "maxEffectiveSubscriptionCount", skip_serializing_if="Option::is_none")]
  max_effective_subscription_count: Option<i32>,
  /// The effective maximum number of transacted sessions per client using the Client Profile.
  #[serde(rename = "maxEffectiveTransactedSessionCount", skip_serializing_if="Option::is_none")]
  max_effective_transacted_session_count: Option<i32>,
  /// The effective maximum number of transactions per client using the Client Profile.
  #[serde(rename = "maxEffectiveTransactionCount", skip_serializing_if="Option::is_none")]
  max_effective_transaction_count: Option<i32>,
  /// The effective maximum number of transmit flows per client using the Client Profile.
  #[serde(rename = "maxEffectiveTxFlowCount", skip_serializing_if="Option::is_none")]
  max_effective_tx_flow_count: Option<i32>,
  /// The maximum number of transmit flows that can be created by one client using the Client Profile.
  #[serde(rename = "maxEgressFlowCount", skip_serializing_if="Option::is_none")]
  max_egress_flow_count: Option<i64>,
  /// The maximum number of queues and topic endpoints that can be created by clients with the same Client Username using the Client Profile.
  #[serde(rename = "maxEndpointCountPerClientUsername", skip_serializing_if="Option::is_none")]
  max_endpoint_count_per_client_username: Option<i64>,
  /// The maximum number of receive flows that can be created by one client using the Client Profile.
  #[serde(rename = "maxIngressFlowCount", skip_serializing_if="Option::is_none")]
  max_ingress_flow_count: Option<i64>,
  /// The maximum number of subscriptions per client using the Client Profile.
  #[serde(rename = "maxSubscriptionCount", skip_serializing_if="Option::is_none")]
  max_subscription_count: Option<i64>,
  /// The maximum number of transacted sessions that can be created by one client using the Client Profile.
  #[serde(rename = "maxTransactedSessionCount", skip_serializing_if="Option::is_none")]
  max_transacted_session_count: Option<i64>,
  /// The maximum number of transactions that can be created by one client using the Client Profile.
  #[serde(rename = "maxTransactionCount", skip_serializing_if="Option::is_none")]
  max_transaction_count: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The maximum depth of the \"Control 1\" (C-1) priority queue, in work units. Each work unit is 2048 bytes of message data.
  #[serde(rename = "queueControl1MaxDepth", skip_serializing_if="Option::is_none")]
  queue_control1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Control 1\" (C-1) priority queue, regardless of the `queueControl1MaxDepth` value.
  #[serde(rename = "queueControl1MinMsgBurst", skip_serializing_if="Option::is_none")]
  queue_control1_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 1\" (D-1) priority queue, in work units. Each work unit is 2048 bytes of message data.
  #[serde(rename = "queueDirect1MaxDepth", skip_serializing_if="Option::is_none")]
  queue_direct1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 1\" (D-1) priority queue, regardless of the `queueDirect1MaxDepth` value.
  #[serde(rename = "queueDirect1MinMsgBurst", skip_serializing_if="Option::is_none")]
  queue_direct1_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 2\" (D-2) priority queue, in work units. Each work unit is 2048 bytes of message data.
  #[serde(rename = "queueDirect2MaxDepth", skip_serializing_if="Option::is_none")]
  queue_direct2_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 2\" (D-2) priority queue, regardless of the `queueDirect2MaxDepth` value.
  #[serde(rename = "queueDirect2MinMsgBurst", skip_serializing_if="Option::is_none")]
  queue_direct2_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 3\" (D-3) priority queue, in work units. Each work unit is 2048 bytes of message data.
  #[serde(rename = "queueDirect3MaxDepth", skip_serializing_if="Option::is_none")]
  queue_direct3_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 3\" (D-3) priority queue, regardless of the `queueDirect3MaxDepth` value.
  #[serde(rename = "queueDirect3MinMsgBurst", skip_serializing_if="Option::is_none")]
  queue_direct3_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Guaranteed 1\" (G-1) priority queue, in work units. Each work unit is 2048 bytes of message data.
  #[serde(rename = "queueGuaranteed1MaxDepth", skip_serializing_if="Option::is_none")]
  queue_guaranteed1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Guaranteed 1\" (G-3) priority queue, regardless of the `queueGuaranteed1MaxDepth` value.
  #[serde(rename = "queueGuaranteed1MinMsgBurst", skip_serializing_if="Option::is_none")]
  queue_guaranteed1_min_msg_burst: Option<i32>,
  /// Indicates whether to send a negative acknowledgement (NACK) to a client using the Client Profile when discarding a guaranteed message due to no matching subscription found.
  #[serde(rename = "rejectMsgToSenderOnNoSubscriptionMatchEnabled", skip_serializing_if="Option::is_none")]
  reject_msg_to_sender_on_no_subscription_match_enabled: Option<bool>,
  /// Indicates whether clients using the Client Profile are allowed to connect to the Message VPN when its replication state is standby.
  #[serde(rename = "replicationAllowClientConnectWhenStandbyEnabled", skip_serializing_if="Option::is_none")]
  replication_allow_client_connect_when_standby_enabled: Option<bool>,
  /// The maximum number of SMF client connections per Client Username using the Client Profile.
  #[serde(rename = "serviceSmfMaxConnectionCountPerClientUsername", skip_serializing_if="Option::is_none")]
  service_smf_max_connection_count_per_client_username: Option<i64>,
  /// The timeout for inactive Web Transport client sessions using the Client Profile, in seconds.
  #[serde(rename = "serviceWebInactiveTimeout", skip_serializing_if="Option::is_none")]
  service_web_inactive_timeout: Option<i64>,
  /// The maximum number of Web Transport client connections per Client Username using the Client Profile.
  #[serde(rename = "serviceWebMaxConnectionCountPerClientUsername", skip_serializing_if="Option::is_none")]
  service_web_max_connection_count_per_client_username: Option<i64>,
  /// The maximum Web Transport payload size before fragmentation occurs for clients using the Client Profile, in bytes. The size of the header is not included.
  #[serde(rename = "serviceWebMaxPayload", skip_serializing_if="Option::is_none")]
  service_web_max_payload: Option<i64>,
  /// The TCP initial congestion window size for clients using the Client Profile, in multiples of the TCP Maximum Segment Size (MSS). Changing the value from its default of 2 results in non-compliance with RFC 2581. Contact Solace Support before changing this value.
  #[serde(rename = "tcpCongestionWindowSize", skip_serializing_if="Option::is_none")]
  tcp_congestion_window_size: Option<i64>,
  /// The number of TCP keepalive retransmissions to a client using the Client Profile before declaring that it is not available.
  #[serde(rename = "tcpKeepaliveCount", skip_serializing_if="Option::is_none")]
  tcp_keepalive_count: Option<i64>,
  /// The amount of time a client connection using the Client Profile must remain idle before TCP begins sending keepalive probes, in seconds.
  #[serde(rename = "tcpKeepaliveIdleTime", skip_serializing_if="Option::is_none")]
  tcp_keepalive_idle_time: Option<i64>,
  /// The amount of time between TCP keepalive retransmissions to a client using the Client Profile when no acknowledgement is received, in seconds.
  #[serde(rename = "tcpKeepaliveInterval", skip_serializing_if="Option::is_none")]
  tcp_keepalive_interval: Option<i64>,
  /// The TCP maximum segment size for clients using the Client Profile, in kilobytes. Changes are applied to all existing connections.
  #[serde(rename = "tcpMaxSegmentSize", skip_serializing_if="Option::is_none")]
  tcp_max_segment_size: Option<i64>,
  /// The TCP maximum window size for clients using the Client Profile, in kilobytes. Changes are applied to all existing connections.
  #[serde(rename = "tcpMaxWindowSize", skip_serializing_if="Option::is_none")]
  tcp_max_window_size: Option<i64>,
  /// Indicates whether clients using the Client Profile are allowed to downgrade an encrypted connection to plain text.
  #[serde(rename = "tlsAllowDowngradeToPlainTextEnabled", skip_serializing_if="Option::is_none")]
  tls_allow_downgrade_to_plain_text_enabled: Option<bool>
}

impl MsgVpnClientProfile {
  pub fn new() -> MsgVpnClientProfile {
    MsgVpnClientProfile {
      allow_bridge_connections_enabled: None,
      allow_cut_through_forwarding_enabled: None,
      allow_guaranteed_endpoint_create_enabled: None,
      allow_guaranteed_msg_receive_enabled: None,
      allow_guaranteed_msg_send_enabled: None,
      allow_shared_subscriptions_enabled: None,
      allow_transacted_sessions_enabled: None,
      api_queue_management_copy_from_on_create_name: None,
      api_topic_endpoint_management_copy_from_on_create_name: None,
      client_profile_name: None,
      compression_enabled: None,
      eliding_delay: None,
      eliding_enabled: None,
      eliding_max_topic_count: None,
      event_client_provisioned_endpoint_spool_usage_threshold: None,
      event_connection_count_per_client_username_threshold: None,
      event_egress_flow_count_threshold: None,
      event_endpoint_count_per_client_username_threshold: None,
      event_ingress_flow_count_threshold: None,
      event_service_smf_connection_count_per_client_username_threshold: None,
      event_service_web_connection_count_per_client_username_threshold: None,
      event_subscription_count_threshold: None,
      event_transacted_session_count_threshold: None,
      event_transaction_count_threshold: None,
      max_connection_count_per_client_username: None,
      max_effective_endpoint_count: None,
      max_effective_rx_flow_count: None,
      max_effective_subscription_count: None,
      max_effective_transacted_session_count: None,
      max_effective_transaction_count: None,
      max_effective_tx_flow_count: None,
      max_egress_flow_count: None,
      max_endpoint_count_per_client_username: None,
      max_ingress_flow_count: None,
      max_subscription_count: None,
      max_transacted_session_count: None,
      max_transaction_count: None,
      msg_vpn_name: None,
      queue_control1_max_depth: None,
      queue_control1_min_msg_burst: None,
      queue_direct1_max_depth: None,
      queue_direct1_min_msg_burst: None,
      queue_direct2_max_depth: None,
      queue_direct2_min_msg_burst: None,
      queue_direct3_max_depth: None,
      queue_direct3_min_msg_burst: None,
      queue_guaranteed1_max_depth: None,
      queue_guaranteed1_min_msg_burst: None,
      reject_msg_to_sender_on_no_subscription_match_enabled: None,
      replication_allow_client_connect_when_standby_enabled: None,
      service_smf_max_connection_count_per_client_username: None,
      service_web_inactive_timeout: None,
      service_web_max_connection_count_per_client_username: None,
      service_web_max_payload: None,
      tcp_congestion_window_size: None,
      tcp_keepalive_count: None,
      tcp_keepalive_idle_time: None,
      tcp_keepalive_interval: None,
      tcp_max_segment_size: None,
      tcp_max_window_size: None,
      tls_allow_downgrade_to_plain_text_enabled: None
    }
  }

  pub fn set_allow_bridge_connections_enabled(&mut self, allow_bridge_connections_enabled: bool) {
    self.allow_bridge_connections_enabled = Some(allow_bridge_connections_enabled);
  }

  pub fn with_allow_bridge_connections_enabled(mut self, allow_bridge_connections_enabled: bool) -> MsgVpnClientProfile {
    self.allow_bridge_connections_enabled = Some(allow_bridge_connections_enabled);
    self
  }

  pub fn allow_bridge_connections_enabled(&self) -> Option<&bool> {
    self.allow_bridge_connections_enabled.as_ref()
  }

  pub fn reset_allow_bridge_connections_enabled(&mut self) {
    self.allow_bridge_connections_enabled = None;
  }

  pub fn set_allow_cut_through_forwarding_enabled(&mut self, allow_cut_through_forwarding_enabled: bool) {
    self.allow_cut_through_forwarding_enabled = Some(allow_cut_through_forwarding_enabled);
  }

  pub fn with_allow_cut_through_forwarding_enabled(mut self, allow_cut_through_forwarding_enabled: bool) -> MsgVpnClientProfile {
    self.allow_cut_through_forwarding_enabled = Some(allow_cut_through_forwarding_enabled);
    self
  }

  pub fn allow_cut_through_forwarding_enabled(&self) -> Option<&bool> {
    self.allow_cut_through_forwarding_enabled.as_ref()
  }

  pub fn reset_allow_cut_through_forwarding_enabled(&mut self) {
    self.allow_cut_through_forwarding_enabled = None;
  }

  pub fn set_allow_guaranteed_endpoint_create_enabled(&mut self, allow_guaranteed_endpoint_create_enabled: bool) {
    self.allow_guaranteed_endpoint_create_enabled = Some(allow_guaranteed_endpoint_create_enabled);
  }

  pub fn with_allow_guaranteed_endpoint_create_enabled(mut self, allow_guaranteed_endpoint_create_enabled: bool) -> MsgVpnClientProfile {
    self.allow_guaranteed_endpoint_create_enabled = Some(allow_guaranteed_endpoint_create_enabled);
    self
  }

  pub fn allow_guaranteed_endpoint_create_enabled(&self) -> Option<&bool> {
    self.allow_guaranteed_endpoint_create_enabled.as_ref()
  }

  pub fn reset_allow_guaranteed_endpoint_create_enabled(&mut self) {
    self.allow_guaranteed_endpoint_create_enabled = None;
  }

  pub fn set_allow_guaranteed_msg_receive_enabled(&mut self, allow_guaranteed_msg_receive_enabled: bool) {
    self.allow_guaranteed_msg_receive_enabled = Some(allow_guaranteed_msg_receive_enabled);
  }

  pub fn with_allow_guaranteed_msg_receive_enabled(mut self, allow_guaranteed_msg_receive_enabled: bool) -> MsgVpnClientProfile {
    self.allow_guaranteed_msg_receive_enabled = Some(allow_guaranteed_msg_receive_enabled);
    self
  }

  pub fn allow_guaranteed_msg_receive_enabled(&self) -> Option<&bool> {
    self.allow_guaranteed_msg_receive_enabled.as_ref()
  }

  pub fn reset_allow_guaranteed_msg_receive_enabled(&mut self) {
    self.allow_guaranteed_msg_receive_enabled = None;
  }

  pub fn set_allow_guaranteed_msg_send_enabled(&mut self, allow_guaranteed_msg_send_enabled: bool) {
    self.allow_guaranteed_msg_send_enabled = Some(allow_guaranteed_msg_send_enabled);
  }

  pub fn with_allow_guaranteed_msg_send_enabled(mut self, allow_guaranteed_msg_send_enabled: bool) -> MsgVpnClientProfile {
    self.allow_guaranteed_msg_send_enabled = Some(allow_guaranteed_msg_send_enabled);
    self
  }

  pub fn allow_guaranteed_msg_send_enabled(&self) -> Option<&bool> {
    self.allow_guaranteed_msg_send_enabled.as_ref()
  }

  pub fn reset_allow_guaranteed_msg_send_enabled(&mut self) {
    self.allow_guaranteed_msg_send_enabled = None;
  }

  pub fn set_allow_shared_subscriptions_enabled(&mut self, allow_shared_subscriptions_enabled: bool) {
    self.allow_shared_subscriptions_enabled = Some(allow_shared_subscriptions_enabled);
  }

  pub fn with_allow_shared_subscriptions_enabled(mut self, allow_shared_subscriptions_enabled: bool) -> MsgVpnClientProfile {
    self.allow_shared_subscriptions_enabled = Some(allow_shared_subscriptions_enabled);
    self
  }

  pub fn allow_shared_subscriptions_enabled(&self) -> Option<&bool> {
    self.allow_shared_subscriptions_enabled.as_ref()
  }

  pub fn reset_allow_shared_subscriptions_enabled(&mut self) {
    self.allow_shared_subscriptions_enabled = None;
  }

  pub fn set_allow_transacted_sessions_enabled(&mut self, allow_transacted_sessions_enabled: bool) {
    self.allow_transacted_sessions_enabled = Some(allow_transacted_sessions_enabled);
  }

  pub fn with_allow_transacted_sessions_enabled(mut self, allow_transacted_sessions_enabled: bool) -> MsgVpnClientProfile {
    self.allow_transacted_sessions_enabled = Some(allow_transacted_sessions_enabled);
    self
  }

  pub fn allow_transacted_sessions_enabled(&self) -> Option<&bool> {
    self.allow_transacted_sessions_enabled.as_ref()
  }

  pub fn reset_allow_transacted_sessions_enabled(&mut self) {
    self.allow_transacted_sessions_enabled = None;
  }

  pub fn set_api_queue_management_copy_from_on_create_name(&mut self, api_queue_management_copy_from_on_create_name: String) {
    self.api_queue_management_copy_from_on_create_name = Some(api_queue_management_copy_from_on_create_name);
  }

  pub fn with_api_queue_management_copy_from_on_create_name(mut self, api_queue_management_copy_from_on_create_name: String) -> MsgVpnClientProfile {
    self.api_queue_management_copy_from_on_create_name = Some(api_queue_management_copy_from_on_create_name);
    self
  }

  pub fn api_queue_management_copy_from_on_create_name(&self) -> Option<&String> {
    self.api_queue_management_copy_from_on_create_name.as_ref()
  }

  pub fn reset_api_queue_management_copy_from_on_create_name(&mut self) {
    self.api_queue_management_copy_from_on_create_name = None;
  }

  pub fn set_api_topic_endpoint_management_copy_from_on_create_name(&mut self, api_topic_endpoint_management_copy_from_on_create_name: String) {
    self.api_topic_endpoint_management_copy_from_on_create_name = Some(api_topic_endpoint_management_copy_from_on_create_name);
  }

  pub fn with_api_topic_endpoint_management_copy_from_on_create_name(mut self, api_topic_endpoint_management_copy_from_on_create_name: String) -> MsgVpnClientProfile {
    self.api_topic_endpoint_management_copy_from_on_create_name = Some(api_topic_endpoint_management_copy_from_on_create_name);
    self
  }

  pub fn api_topic_endpoint_management_copy_from_on_create_name(&self) -> Option<&String> {
    self.api_topic_endpoint_management_copy_from_on_create_name.as_ref()
  }

  pub fn reset_api_topic_endpoint_management_copy_from_on_create_name(&mut self) {
    self.api_topic_endpoint_management_copy_from_on_create_name = None;
  }

  pub fn set_client_profile_name(&mut self, client_profile_name: String) {
    self.client_profile_name = Some(client_profile_name);
  }

  pub fn with_client_profile_name(mut self, client_profile_name: String) -> MsgVpnClientProfile {
    self.client_profile_name = Some(client_profile_name);
    self
  }

  pub fn client_profile_name(&self) -> Option<&String> {
    self.client_profile_name.as_ref()
  }

  pub fn reset_client_profile_name(&mut self) {
    self.client_profile_name = None;
  }

  pub fn set_compression_enabled(&mut self, compression_enabled: bool) {
    self.compression_enabled = Some(compression_enabled);
  }

  pub fn with_compression_enabled(mut self, compression_enabled: bool) -> MsgVpnClientProfile {
    self.compression_enabled = Some(compression_enabled);
    self
  }

  pub fn compression_enabled(&self) -> Option<&bool> {
    self.compression_enabled.as_ref()
  }

  pub fn reset_compression_enabled(&mut self) {
    self.compression_enabled = None;
  }

  pub fn set_eliding_delay(&mut self, eliding_delay: i64) {
    self.eliding_delay = Some(eliding_delay);
  }

  pub fn with_eliding_delay(mut self, eliding_delay: i64) -> MsgVpnClientProfile {
    self.eliding_delay = Some(eliding_delay);
    self
  }

  pub fn eliding_delay(&self) -> Option<&i64> {
    self.eliding_delay.as_ref()
  }

  pub fn reset_eliding_delay(&mut self) {
    self.eliding_delay = None;
  }

  pub fn set_eliding_enabled(&mut self, eliding_enabled: bool) {
    self.eliding_enabled = Some(eliding_enabled);
  }

  pub fn with_eliding_enabled(mut self, eliding_enabled: bool) -> MsgVpnClientProfile {
    self.eliding_enabled = Some(eliding_enabled);
    self
  }

  pub fn eliding_enabled(&self) -> Option<&bool> {
    self.eliding_enabled.as_ref()
  }

  pub fn reset_eliding_enabled(&mut self) {
    self.eliding_enabled = None;
  }

  pub fn set_eliding_max_topic_count(&mut self, eliding_max_topic_count: i64) {
    self.eliding_max_topic_count = Some(eliding_max_topic_count);
  }

  pub fn with_eliding_max_topic_count(mut self, eliding_max_topic_count: i64) -> MsgVpnClientProfile {
    self.eliding_max_topic_count = Some(eliding_max_topic_count);
    self
  }

  pub fn eliding_max_topic_count(&self) -> Option<&i64> {
    self.eliding_max_topic_count.as_ref()
  }

  pub fn reset_eliding_max_topic_count(&mut self) {
    self.eliding_max_topic_count = None;
  }

  pub fn set_event_client_provisioned_endpoint_spool_usage_threshold(&mut self, event_client_provisioned_endpoint_spool_usage_threshold: ::models::EventThresholdByPercent) {
    self.event_client_provisioned_endpoint_spool_usage_threshold = Some(event_client_provisioned_endpoint_spool_usage_threshold);
  }

  pub fn with_event_client_provisioned_endpoint_spool_usage_threshold(mut self, event_client_provisioned_endpoint_spool_usage_threshold: ::models::EventThresholdByPercent) -> MsgVpnClientProfile {
    self.event_client_provisioned_endpoint_spool_usage_threshold = Some(event_client_provisioned_endpoint_spool_usage_threshold);
    self
  }

  pub fn event_client_provisioned_endpoint_spool_usage_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.event_client_provisioned_endpoint_spool_usage_threshold.as_ref()
  }

  pub fn reset_event_client_provisioned_endpoint_spool_usage_threshold(&mut self) {
    self.event_client_provisioned_endpoint_spool_usage_threshold = None;
  }

  pub fn set_event_connection_count_per_client_username_threshold(&mut self, event_connection_count_per_client_username_threshold: ::models::EventThreshold) {
    self.event_connection_count_per_client_username_threshold = Some(event_connection_count_per_client_username_threshold);
  }

  pub fn with_event_connection_count_per_client_username_threshold(mut self, event_connection_count_per_client_username_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_connection_count_per_client_username_threshold = Some(event_connection_count_per_client_username_threshold);
    self
  }

  pub fn event_connection_count_per_client_username_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_connection_count_per_client_username_threshold.as_ref()
  }

  pub fn reset_event_connection_count_per_client_username_threshold(&mut self) {
    self.event_connection_count_per_client_username_threshold = None;
  }

  pub fn set_event_egress_flow_count_threshold(&mut self, event_egress_flow_count_threshold: ::models::EventThreshold) {
    self.event_egress_flow_count_threshold = Some(event_egress_flow_count_threshold);
  }

  pub fn with_event_egress_flow_count_threshold(mut self, event_egress_flow_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_egress_flow_count_threshold = Some(event_egress_flow_count_threshold);
    self
  }

  pub fn event_egress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_egress_flow_count_threshold.as_ref()
  }

  pub fn reset_event_egress_flow_count_threshold(&mut self) {
    self.event_egress_flow_count_threshold = None;
  }

  pub fn set_event_endpoint_count_per_client_username_threshold(&mut self, event_endpoint_count_per_client_username_threshold: ::models::EventThreshold) {
    self.event_endpoint_count_per_client_username_threshold = Some(event_endpoint_count_per_client_username_threshold);
  }

  pub fn with_event_endpoint_count_per_client_username_threshold(mut self, event_endpoint_count_per_client_username_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_endpoint_count_per_client_username_threshold = Some(event_endpoint_count_per_client_username_threshold);
    self
  }

  pub fn event_endpoint_count_per_client_username_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_endpoint_count_per_client_username_threshold.as_ref()
  }

  pub fn reset_event_endpoint_count_per_client_username_threshold(&mut self) {
    self.event_endpoint_count_per_client_username_threshold = None;
  }

  pub fn set_event_ingress_flow_count_threshold(&mut self, event_ingress_flow_count_threshold: ::models::EventThreshold) {
    self.event_ingress_flow_count_threshold = Some(event_ingress_flow_count_threshold);
  }

  pub fn with_event_ingress_flow_count_threshold(mut self, event_ingress_flow_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_ingress_flow_count_threshold = Some(event_ingress_flow_count_threshold);
    self
  }

  pub fn event_ingress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_ingress_flow_count_threshold.as_ref()
  }

  pub fn reset_event_ingress_flow_count_threshold(&mut self) {
    self.event_ingress_flow_count_threshold = None;
  }

  pub fn set_event_service_smf_connection_count_per_client_username_threshold(&mut self, event_service_smf_connection_count_per_client_username_threshold: ::models::EventThreshold) {
    self.event_service_smf_connection_count_per_client_username_threshold = Some(event_service_smf_connection_count_per_client_username_threshold);
  }

  pub fn with_event_service_smf_connection_count_per_client_username_threshold(mut self, event_service_smf_connection_count_per_client_username_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_service_smf_connection_count_per_client_username_threshold = Some(event_service_smf_connection_count_per_client_username_threshold);
    self
  }

  pub fn event_service_smf_connection_count_per_client_username_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_smf_connection_count_per_client_username_threshold.as_ref()
  }

  pub fn reset_event_service_smf_connection_count_per_client_username_threshold(&mut self) {
    self.event_service_smf_connection_count_per_client_username_threshold = None;
  }

  pub fn set_event_service_web_connection_count_per_client_username_threshold(&mut self, event_service_web_connection_count_per_client_username_threshold: ::models::EventThreshold) {
    self.event_service_web_connection_count_per_client_username_threshold = Some(event_service_web_connection_count_per_client_username_threshold);
  }

  pub fn with_event_service_web_connection_count_per_client_username_threshold(mut self, event_service_web_connection_count_per_client_username_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_service_web_connection_count_per_client_username_threshold = Some(event_service_web_connection_count_per_client_username_threshold);
    self
  }

  pub fn event_service_web_connection_count_per_client_username_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_web_connection_count_per_client_username_threshold.as_ref()
  }

  pub fn reset_event_service_web_connection_count_per_client_username_threshold(&mut self) {
    self.event_service_web_connection_count_per_client_username_threshold = None;
  }

  pub fn set_event_subscription_count_threshold(&mut self, event_subscription_count_threshold: ::models::EventThreshold) {
    self.event_subscription_count_threshold = Some(event_subscription_count_threshold);
  }

  pub fn with_event_subscription_count_threshold(mut self, event_subscription_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_subscription_count_threshold = Some(event_subscription_count_threshold);
    self
  }

  pub fn event_subscription_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_subscription_count_threshold.as_ref()
  }

  pub fn reset_event_subscription_count_threshold(&mut self) {
    self.event_subscription_count_threshold = None;
  }

  pub fn set_event_transacted_session_count_threshold(&mut self, event_transacted_session_count_threshold: ::models::EventThreshold) {
    self.event_transacted_session_count_threshold = Some(event_transacted_session_count_threshold);
  }

  pub fn with_event_transacted_session_count_threshold(mut self, event_transacted_session_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_transacted_session_count_threshold = Some(event_transacted_session_count_threshold);
    self
  }

  pub fn event_transacted_session_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_transacted_session_count_threshold.as_ref()
  }

  pub fn reset_event_transacted_session_count_threshold(&mut self) {
    self.event_transacted_session_count_threshold = None;
  }

  pub fn set_event_transaction_count_threshold(&mut self, event_transaction_count_threshold: ::models::EventThreshold) {
    self.event_transaction_count_threshold = Some(event_transaction_count_threshold);
  }

  pub fn with_event_transaction_count_threshold(mut self, event_transaction_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_transaction_count_threshold = Some(event_transaction_count_threshold);
    self
  }

  pub fn event_transaction_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_transaction_count_threshold.as_ref()
  }

  pub fn reset_event_transaction_count_threshold(&mut self) {
    self.event_transaction_count_threshold = None;
  }

  pub fn set_max_connection_count_per_client_username(&mut self, max_connection_count_per_client_username: i64) {
    self.max_connection_count_per_client_username = Some(max_connection_count_per_client_username);
  }

  pub fn with_max_connection_count_per_client_username(mut self, max_connection_count_per_client_username: i64) -> MsgVpnClientProfile {
    self.max_connection_count_per_client_username = Some(max_connection_count_per_client_username);
    self
  }

  pub fn max_connection_count_per_client_username(&self) -> Option<&i64> {
    self.max_connection_count_per_client_username.as_ref()
  }

  pub fn reset_max_connection_count_per_client_username(&mut self) {
    self.max_connection_count_per_client_username = None;
  }

  pub fn set_max_effective_endpoint_count(&mut self, max_effective_endpoint_count: i32) {
    self.max_effective_endpoint_count = Some(max_effective_endpoint_count);
  }

  pub fn with_max_effective_endpoint_count(mut self, max_effective_endpoint_count: i32) -> MsgVpnClientProfile {
    self.max_effective_endpoint_count = Some(max_effective_endpoint_count);
    self
  }

  pub fn max_effective_endpoint_count(&self) -> Option<&i32> {
    self.max_effective_endpoint_count.as_ref()
  }

  pub fn reset_max_effective_endpoint_count(&mut self) {
    self.max_effective_endpoint_count = None;
  }

  pub fn set_max_effective_rx_flow_count(&mut self, max_effective_rx_flow_count: i32) {
    self.max_effective_rx_flow_count = Some(max_effective_rx_flow_count);
  }

  pub fn with_max_effective_rx_flow_count(mut self, max_effective_rx_flow_count: i32) -> MsgVpnClientProfile {
    self.max_effective_rx_flow_count = Some(max_effective_rx_flow_count);
    self
  }

  pub fn max_effective_rx_flow_count(&self) -> Option<&i32> {
    self.max_effective_rx_flow_count.as_ref()
  }

  pub fn reset_max_effective_rx_flow_count(&mut self) {
    self.max_effective_rx_flow_count = None;
  }

  pub fn set_max_effective_subscription_count(&mut self, max_effective_subscription_count: i32) {
    self.max_effective_subscription_count = Some(max_effective_subscription_count);
  }

  pub fn with_max_effective_subscription_count(mut self, max_effective_subscription_count: i32) -> MsgVpnClientProfile {
    self.max_effective_subscription_count = Some(max_effective_subscription_count);
    self
  }

  pub fn max_effective_subscription_count(&self) -> Option<&i32> {
    self.max_effective_subscription_count.as_ref()
  }

  pub fn reset_max_effective_subscription_count(&mut self) {
    self.max_effective_subscription_count = None;
  }

  pub fn set_max_effective_transacted_session_count(&mut self, max_effective_transacted_session_count: i32) {
    self.max_effective_transacted_session_count = Some(max_effective_transacted_session_count);
  }

  pub fn with_max_effective_transacted_session_count(mut self, max_effective_transacted_session_count: i32) -> MsgVpnClientProfile {
    self.max_effective_transacted_session_count = Some(max_effective_transacted_session_count);
    self
  }

  pub fn max_effective_transacted_session_count(&self) -> Option<&i32> {
    self.max_effective_transacted_session_count.as_ref()
  }

  pub fn reset_max_effective_transacted_session_count(&mut self) {
    self.max_effective_transacted_session_count = None;
  }

  pub fn set_max_effective_transaction_count(&mut self, max_effective_transaction_count: i32) {
    self.max_effective_transaction_count = Some(max_effective_transaction_count);
  }

  pub fn with_max_effective_transaction_count(mut self, max_effective_transaction_count: i32) -> MsgVpnClientProfile {
    self.max_effective_transaction_count = Some(max_effective_transaction_count);
    self
  }

  pub fn max_effective_transaction_count(&self) -> Option<&i32> {
    self.max_effective_transaction_count.as_ref()
  }

  pub fn reset_max_effective_transaction_count(&mut self) {
    self.max_effective_transaction_count = None;
  }

  pub fn set_max_effective_tx_flow_count(&mut self, max_effective_tx_flow_count: i32) {
    self.max_effective_tx_flow_count = Some(max_effective_tx_flow_count);
  }

  pub fn with_max_effective_tx_flow_count(mut self, max_effective_tx_flow_count: i32) -> MsgVpnClientProfile {
    self.max_effective_tx_flow_count = Some(max_effective_tx_flow_count);
    self
  }

  pub fn max_effective_tx_flow_count(&self) -> Option<&i32> {
    self.max_effective_tx_flow_count.as_ref()
  }

  pub fn reset_max_effective_tx_flow_count(&mut self) {
    self.max_effective_tx_flow_count = None;
  }

  pub fn set_max_egress_flow_count(&mut self, max_egress_flow_count: i64) {
    self.max_egress_flow_count = Some(max_egress_flow_count);
  }

  pub fn with_max_egress_flow_count(mut self, max_egress_flow_count: i64) -> MsgVpnClientProfile {
    self.max_egress_flow_count = Some(max_egress_flow_count);
    self
  }

  pub fn max_egress_flow_count(&self) -> Option<&i64> {
    self.max_egress_flow_count.as_ref()
  }

  pub fn reset_max_egress_flow_count(&mut self) {
    self.max_egress_flow_count = None;
  }

  pub fn set_max_endpoint_count_per_client_username(&mut self, max_endpoint_count_per_client_username: i64) {
    self.max_endpoint_count_per_client_username = Some(max_endpoint_count_per_client_username);
  }

  pub fn with_max_endpoint_count_per_client_username(mut self, max_endpoint_count_per_client_username: i64) -> MsgVpnClientProfile {
    self.max_endpoint_count_per_client_username = Some(max_endpoint_count_per_client_username);
    self
  }

  pub fn max_endpoint_count_per_client_username(&self) -> Option<&i64> {
    self.max_endpoint_count_per_client_username.as_ref()
  }

  pub fn reset_max_endpoint_count_per_client_username(&mut self) {
    self.max_endpoint_count_per_client_username = None;
  }

  pub fn set_max_ingress_flow_count(&mut self, max_ingress_flow_count: i64) {
    self.max_ingress_flow_count = Some(max_ingress_flow_count);
  }

  pub fn with_max_ingress_flow_count(mut self, max_ingress_flow_count: i64) -> MsgVpnClientProfile {
    self.max_ingress_flow_count = Some(max_ingress_flow_count);
    self
  }

  pub fn max_ingress_flow_count(&self) -> Option<&i64> {
    self.max_ingress_flow_count.as_ref()
  }

  pub fn reset_max_ingress_flow_count(&mut self) {
    self.max_ingress_flow_count = None;
  }

  pub fn set_max_subscription_count(&mut self, max_subscription_count: i64) {
    self.max_subscription_count = Some(max_subscription_count);
  }

  pub fn with_max_subscription_count(mut self, max_subscription_count: i64) -> MsgVpnClientProfile {
    self.max_subscription_count = Some(max_subscription_count);
    self
  }

  pub fn max_subscription_count(&self) -> Option<&i64> {
    self.max_subscription_count.as_ref()
  }

  pub fn reset_max_subscription_count(&mut self) {
    self.max_subscription_count = None;
  }

  pub fn set_max_transacted_session_count(&mut self, max_transacted_session_count: i64) {
    self.max_transacted_session_count = Some(max_transacted_session_count);
  }

  pub fn with_max_transacted_session_count(mut self, max_transacted_session_count: i64) -> MsgVpnClientProfile {
    self.max_transacted_session_count = Some(max_transacted_session_count);
    self
  }

  pub fn max_transacted_session_count(&self) -> Option<&i64> {
    self.max_transacted_session_count.as_ref()
  }

  pub fn reset_max_transacted_session_count(&mut self) {
    self.max_transacted_session_count = None;
  }

  pub fn set_max_transaction_count(&mut self, max_transaction_count: i64) {
    self.max_transaction_count = Some(max_transaction_count);
  }

  pub fn with_max_transaction_count(mut self, max_transaction_count: i64) -> MsgVpnClientProfile {
    self.max_transaction_count = Some(max_transaction_count);
    self
  }

  pub fn max_transaction_count(&self) -> Option<&i64> {
    self.max_transaction_count.as_ref()
  }

  pub fn reset_max_transaction_count(&mut self) {
    self.max_transaction_count = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnClientProfile {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_queue_control1_max_depth(&mut self, queue_control1_max_depth: i32) {
    self.queue_control1_max_depth = Some(queue_control1_max_depth);
  }

  pub fn with_queue_control1_max_depth(mut self, queue_control1_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_control1_max_depth = Some(queue_control1_max_depth);
    self
  }

  pub fn queue_control1_max_depth(&self) -> Option<&i32> {
    self.queue_control1_max_depth.as_ref()
  }

  pub fn reset_queue_control1_max_depth(&mut self) {
    self.queue_control1_max_depth = None;
  }

  pub fn set_queue_control1_min_msg_burst(&mut self, queue_control1_min_msg_burst: i32) {
    self.queue_control1_min_msg_burst = Some(queue_control1_min_msg_burst);
  }

  pub fn with_queue_control1_min_msg_burst(mut self, queue_control1_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_control1_min_msg_burst = Some(queue_control1_min_msg_burst);
    self
  }

  pub fn queue_control1_min_msg_burst(&self) -> Option<&i32> {
    self.queue_control1_min_msg_burst.as_ref()
  }

  pub fn reset_queue_control1_min_msg_burst(&mut self) {
    self.queue_control1_min_msg_burst = None;
  }

  pub fn set_queue_direct1_max_depth(&mut self, queue_direct1_max_depth: i32) {
    self.queue_direct1_max_depth = Some(queue_direct1_max_depth);
  }

  pub fn with_queue_direct1_max_depth(mut self, queue_direct1_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_direct1_max_depth = Some(queue_direct1_max_depth);
    self
  }

  pub fn queue_direct1_max_depth(&self) -> Option<&i32> {
    self.queue_direct1_max_depth.as_ref()
  }

  pub fn reset_queue_direct1_max_depth(&mut self) {
    self.queue_direct1_max_depth = None;
  }

  pub fn set_queue_direct1_min_msg_burst(&mut self, queue_direct1_min_msg_burst: i32) {
    self.queue_direct1_min_msg_burst = Some(queue_direct1_min_msg_burst);
  }

  pub fn with_queue_direct1_min_msg_burst(mut self, queue_direct1_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_direct1_min_msg_burst = Some(queue_direct1_min_msg_burst);
    self
  }

  pub fn queue_direct1_min_msg_burst(&self) -> Option<&i32> {
    self.queue_direct1_min_msg_burst.as_ref()
  }

  pub fn reset_queue_direct1_min_msg_burst(&mut self) {
    self.queue_direct1_min_msg_burst = None;
  }

  pub fn set_queue_direct2_max_depth(&mut self, queue_direct2_max_depth: i32) {
    self.queue_direct2_max_depth = Some(queue_direct2_max_depth);
  }

  pub fn with_queue_direct2_max_depth(mut self, queue_direct2_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_direct2_max_depth = Some(queue_direct2_max_depth);
    self
  }

  pub fn queue_direct2_max_depth(&self) -> Option<&i32> {
    self.queue_direct2_max_depth.as_ref()
  }

  pub fn reset_queue_direct2_max_depth(&mut self) {
    self.queue_direct2_max_depth = None;
  }

  pub fn set_queue_direct2_min_msg_burst(&mut self, queue_direct2_min_msg_burst: i32) {
    self.queue_direct2_min_msg_burst = Some(queue_direct2_min_msg_burst);
  }

  pub fn with_queue_direct2_min_msg_burst(mut self, queue_direct2_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_direct2_min_msg_burst = Some(queue_direct2_min_msg_burst);
    self
  }

  pub fn queue_direct2_min_msg_burst(&self) -> Option<&i32> {
    self.queue_direct2_min_msg_burst.as_ref()
  }

  pub fn reset_queue_direct2_min_msg_burst(&mut self) {
    self.queue_direct2_min_msg_burst = None;
  }

  pub fn set_queue_direct3_max_depth(&mut self, queue_direct3_max_depth: i32) {
    self.queue_direct3_max_depth = Some(queue_direct3_max_depth);
  }

  pub fn with_queue_direct3_max_depth(mut self, queue_direct3_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_direct3_max_depth = Some(queue_direct3_max_depth);
    self
  }

  pub fn queue_direct3_max_depth(&self) -> Option<&i32> {
    self.queue_direct3_max_depth.as_ref()
  }

  pub fn reset_queue_direct3_max_depth(&mut self) {
    self.queue_direct3_max_depth = None;
  }

  pub fn set_queue_direct3_min_msg_burst(&mut self, queue_direct3_min_msg_burst: i32) {
    self.queue_direct3_min_msg_burst = Some(queue_direct3_min_msg_burst);
  }

  pub fn with_queue_direct3_min_msg_burst(mut self, queue_direct3_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_direct3_min_msg_burst = Some(queue_direct3_min_msg_burst);
    self
  }

  pub fn queue_direct3_min_msg_burst(&self) -> Option<&i32> {
    self.queue_direct3_min_msg_burst.as_ref()
  }

  pub fn reset_queue_direct3_min_msg_burst(&mut self) {
    self.queue_direct3_min_msg_burst = None;
  }

  pub fn set_queue_guaranteed1_max_depth(&mut self, queue_guaranteed1_max_depth: i32) {
    self.queue_guaranteed1_max_depth = Some(queue_guaranteed1_max_depth);
  }

  pub fn with_queue_guaranteed1_max_depth(mut self, queue_guaranteed1_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_guaranteed1_max_depth = Some(queue_guaranteed1_max_depth);
    self
  }

  pub fn queue_guaranteed1_max_depth(&self) -> Option<&i32> {
    self.queue_guaranteed1_max_depth.as_ref()
  }

  pub fn reset_queue_guaranteed1_max_depth(&mut self) {
    self.queue_guaranteed1_max_depth = None;
  }

  pub fn set_queue_guaranteed1_min_msg_burst(&mut self, queue_guaranteed1_min_msg_burst: i32) {
    self.queue_guaranteed1_min_msg_burst = Some(queue_guaranteed1_min_msg_burst);
  }

  pub fn with_queue_guaranteed1_min_msg_burst(mut self, queue_guaranteed1_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_guaranteed1_min_msg_burst = Some(queue_guaranteed1_min_msg_burst);
    self
  }

  pub fn queue_guaranteed1_min_msg_burst(&self) -> Option<&i32> {
    self.queue_guaranteed1_min_msg_burst.as_ref()
  }

  pub fn reset_queue_guaranteed1_min_msg_burst(&mut self) {
    self.queue_guaranteed1_min_msg_burst = None;
  }

  pub fn set_reject_msg_to_sender_on_no_subscription_match_enabled(&mut self, reject_msg_to_sender_on_no_subscription_match_enabled: bool) {
    self.reject_msg_to_sender_on_no_subscription_match_enabled = Some(reject_msg_to_sender_on_no_subscription_match_enabled);
  }

  pub fn with_reject_msg_to_sender_on_no_subscription_match_enabled(mut self, reject_msg_to_sender_on_no_subscription_match_enabled: bool) -> MsgVpnClientProfile {
    self.reject_msg_to_sender_on_no_subscription_match_enabled = Some(reject_msg_to_sender_on_no_subscription_match_enabled);
    self
  }

  pub fn reject_msg_to_sender_on_no_subscription_match_enabled(&self) -> Option<&bool> {
    self.reject_msg_to_sender_on_no_subscription_match_enabled.as_ref()
  }

  pub fn reset_reject_msg_to_sender_on_no_subscription_match_enabled(&mut self) {
    self.reject_msg_to_sender_on_no_subscription_match_enabled = None;
  }

  pub fn set_replication_allow_client_connect_when_standby_enabled(&mut self, replication_allow_client_connect_when_standby_enabled: bool) {
    self.replication_allow_client_connect_when_standby_enabled = Some(replication_allow_client_connect_when_standby_enabled);
  }

  pub fn with_replication_allow_client_connect_when_standby_enabled(mut self, replication_allow_client_connect_when_standby_enabled: bool) -> MsgVpnClientProfile {
    self.replication_allow_client_connect_when_standby_enabled = Some(replication_allow_client_connect_when_standby_enabled);
    self
  }

  pub fn replication_allow_client_connect_when_standby_enabled(&self) -> Option<&bool> {
    self.replication_allow_client_connect_when_standby_enabled.as_ref()
  }

  pub fn reset_replication_allow_client_connect_when_standby_enabled(&mut self) {
    self.replication_allow_client_connect_when_standby_enabled = None;
  }

  pub fn set_service_smf_max_connection_count_per_client_username(&mut self, service_smf_max_connection_count_per_client_username: i64) {
    self.service_smf_max_connection_count_per_client_username = Some(service_smf_max_connection_count_per_client_username);
  }

  pub fn with_service_smf_max_connection_count_per_client_username(mut self, service_smf_max_connection_count_per_client_username: i64) -> MsgVpnClientProfile {
    self.service_smf_max_connection_count_per_client_username = Some(service_smf_max_connection_count_per_client_username);
    self
  }

  pub fn service_smf_max_connection_count_per_client_username(&self) -> Option<&i64> {
    self.service_smf_max_connection_count_per_client_username.as_ref()
  }

  pub fn reset_service_smf_max_connection_count_per_client_username(&mut self) {
    self.service_smf_max_connection_count_per_client_username = None;
  }

  pub fn set_service_web_inactive_timeout(&mut self, service_web_inactive_timeout: i64) {
    self.service_web_inactive_timeout = Some(service_web_inactive_timeout);
  }

  pub fn with_service_web_inactive_timeout(mut self, service_web_inactive_timeout: i64) -> MsgVpnClientProfile {
    self.service_web_inactive_timeout = Some(service_web_inactive_timeout);
    self
  }

  pub fn service_web_inactive_timeout(&self) -> Option<&i64> {
    self.service_web_inactive_timeout.as_ref()
  }

  pub fn reset_service_web_inactive_timeout(&mut self) {
    self.service_web_inactive_timeout = None;
  }

  pub fn set_service_web_max_connection_count_per_client_username(&mut self, service_web_max_connection_count_per_client_username: i64) {
    self.service_web_max_connection_count_per_client_username = Some(service_web_max_connection_count_per_client_username);
  }

  pub fn with_service_web_max_connection_count_per_client_username(mut self, service_web_max_connection_count_per_client_username: i64) -> MsgVpnClientProfile {
    self.service_web_max_connection_count_per_client_username = Some(service_web_max_connection_count_per_client_username);
    self
  }

  pub fn service_web_max_connection_count_per_client_username(&self) -> Option<&i64> {
    self.service_web_max_connection_count_per_client_username.as_ref()
  }

  pub fn reset_service_web_max_connection_count_per_client_username(&mut self) {
    self.service_web_max_connection_count_per_client_username = None;
  }

  pub fn set_service_web_max_payload(&mut self, service_web_max_payload: i64) {
    self.service_web_max_payload = Some(service_web_max_payload);
  }

  pub fn with_service_web_max_payload(mut self, service_web_max_payload: i64) -> MsgVpnClientProfile {
    self.service_web_max_payload = Some(service_web_max_payload);
    self
  }

  pub fn service_web_max_payload(&self) -> Option<&i64> {
    self.service_web_max_payload.as_ref()
  }

  pub fn reset_service_web_max_payload(&mut self) {
    self.service_web_max_payload = None;
  }

  pub fn set_tcp_congestion_window_size(&mut self, tcp_congestion_window_size: i64) {
    self.tcp_congestion_window_size = Some(tcp_congestion_window_size);
  }

  pub fn with_tcp_congestion_window_size(mut self, tcp_congestion_window_size: i64) -> MsgVpnClientProfile {
    self.tcp_congestion_window_size = Some(tcp_congestion_window_size);
    self
  }

  pub fn tcp_congestion_window_size(&self) -> Option<&i64> {
    self.tcp_congestion_window_size.as_ref()
  }

  pub fn reset_tcp_congestion_window_size(&mut self) {
    self.tcp_congestion_window_size = None;
  }

  pub fn set_tcp_keepalive_count(&mut self, tcp_keepalive_count: i64) {
    self.tcp_keepalive_count = Some(tcp_keepalive_count);
  }

  pub fn with_tcp_keepalive_count(mut self, tcp_keepalive_count: i64) -> MsgVpnClientProfile {
    self.tcp_keepalive_count = Some(tcp_keepalive_count);
    self
  }

  pub fn tcp_keepalive_count(&self) -> Option<&i64> {
    self.tcp_keepalive_count.as_ref()
  }

  pub fn reset_tcp_keepalive_count(&mut self) {
    self.tcp_keepalive_count = None;
  }

  pub fn set_tcp_keepalive_idle_time(&mut self, tcp_keepalive_idle_time: i64) {
    self.tcp_keepalive_idle_time = Some(tcp_keepalive_idle_time);
  }

  pub fn with_tcp_keepalive_idle_time(mut self, tcp_keepalive_idle_time: i64) -> MsgVpnClientProfile {
    self.tcp_keepalive_idle_time = Some(tcp_keepalive_idle_time);
    self
  }

  pub fn tcp_keepalive_idle_time(&self) -> Option<&i64> {
    self.tcp_keepalive_idle_time.as_ref()
  }

  pub fn reset_tcp_keepalive_idle_time(&mut self) {
    self.tcp_keepalive_idle_time = None;
  }

  pub fn set_tcp_keepalive_interval(&mut self, tcp_keepalive_interval: i64) {
    self.tcp_keepalive_interval = Some(tcp_keepalive_interval);
  }

  pub fn with_tcp_keepalive_interval(mut self, tcp_keepalive_interval: i64) -> MsgVpnClientProfile {
    self.tcp_keepalive_interval = Some(tcp_keepalive_interval);
    self
  }

  pub fn tcp_keepalive_interval(&self) -> Option<&i64> {
    self.tcp_keepalive_interval.as_ref()
  }

  pub fn reset_tcp_keepalive_interval(&mut self) {
    self.tcp_keepalive_interval = None;
  }

  pub fn set_tcp_max_segment_size(&mut self, tcp_max_segment_size: i64) {
    self.tcp_max_segment_size = Some(tcp_max_segment_size);
  }

  pub fn with_tcp_max_segment_size(mut self, tcp_max_segment_size: i64) -> MsgVpnClientProfile {
    self.tcp_max_segment_size = Some(tcp_max_segment_size);
    self
  }

  pub fn tcp_max_segment_size(&self) -> Option<&i64> {
    self.tcp_max_segment_size.as_ref()
  }

  pub fn reset_tcp_max_segment_size(&mut self) {
    self.tcp_max_segment_size = None;
  }

  pub fn set_tcp_max_window_size(&mut self, tcp_max_window_size: i64) {
    self.tcp_max_window_size = Some(tcp_max_window_size);
  }

  pub fn with_tcp_max_window_size(mut self, tcp_max_window_size: i64) -> MsgVpnClientProfile {
    self.tcp_max_window_size = Some(tcp_max_window_size);
    self
  }

  pub fn tcp_max_window_size(&self) -> Option<&i64> {
    self.tcp_max_window_size.as_ref()
  }

  pub fn reset_tcp_max_window_size(&mut self) {
    self.tcp_max_window_size = None;
  }

  pub fn set_tls_allow_downgrade_to_plain_text_enabled(&mut self, tls_allow_downgrade_to_plain_text_enabled: bool) {
    self.tls_allow_downgrade_to_plain_text_enabled = Some(tls_allow_downgrade_to_plain_text_enabled);
  }

  pub fn with_tls_allow_downgrade_to_plain_text_enabled(mut self, tls_allow_downgrade_to_plain_text_enabled: bool) -> MsgVpnClientProfile {
    self.tls_allow_downgrade_to_plain_text_enabled = Some(tls_allow_downgrade_to_plain_text_enabled);
    self
  }

  pub fn tls_allow_downgrade_to_plain_text_enabled(&self) -> Option<&bool> {
    self.tls_allow_downgrade_to_plain_text_enabled.as_ref()
  }

  pub fn reset_tls_allow_downgrade_to_plain_text_enabled(&mut self) {
    self.tls_allow_downgrade_to_plain_text_enabled = None;
  }

}



