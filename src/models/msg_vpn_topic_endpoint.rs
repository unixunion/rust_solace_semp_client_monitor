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
pub struct MsgVpnTopicEndpoint {
  /// The access type for delivering messages to consumer flows bound to the Topic Endpoint. The allowed values and their meaning are:  <pre> \"exclusive\" - Exclusive delivery of messages to the first bound consumer flow. \"non-exclusive\" - Non-exclusive delivery of messages to all bound consumer flows in a round-robin fashion. </pre> 
  #[serde(rename = "accessType", skip_serializing_if="Option::is_none")]
  access_type: Option<String>,
  /// The number of Topic Endpoint bind failures due to being already bound.
  #[serde(rename = "alreadyBoundBindFailureCount", skip_serializing_if="Option::is_none")]
  already_bound_bind_failure_count: Option<i64>,
  /// The one minute average of the message rate received by the Topic Endpoint, in bytes per second (B/sec).
  #[serde(rename = "averageRxByteRate", skip_serializing_if="Option::is_none")]
  average_rx_byte_rate: Option<i64>,
  /// The one minute average of the message rate received by the Topic Endpoint, in messages per second (msg/sec).
  #[serde(rename = "averageRxMsgRate", skip_serializing_if="Option::is_none")]
  average_rx_msg_rate: Option<i64>,
  /// The one minute average of the message rate transmitted by the Topic Endpoint, in bytes per second (B/sec).
  #[serde(rename = "averageTxByteRate", skip_serializing_if="Option::is_none")]
  average_tx_byte_rate: Option<i64>,
  /// The one minute average of the message rate transmitted by the Topic Endpoint, in messages per second (msg/sec).
  #[serde(rename = "averageTxMsgRate", skip_serializing_if="Option::is_none")]
  average_tx_msg_rate: Option<i64>,
  /// The number of consumer requests to bind to the Topic Endpoint.
  #[serde(rename = "bindRequestCount", skip_serializing_if="Option::is_none")]
  bind_request_count: Option<i64>,
  /// The number of successful consumer requests to bind to the Topic Endpoint.
  #[serde(rename = "bindSuccessCount", skip_serializing_if="Option::is_none")]
  bind_success_count: Option<i64>,
  /// The forwarding mode of the Topic Endpoint at bind time. The allowed values and their meaning are:  <pre> \"store-and-forward\" - Deliver messages using the guaranteed data path. \"cut-through\" - Deliver messages using the direct and guaranteed data paths for lower latency. </pre> 
  #[serde(rename = "bindTimeForwardingMode", skip_serializing_if="Option::is_none")]
  bind_time_forwarding_mode: Option<String>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to being denied by the Client Profile.
  #[serde(rename = "clientProfileDeniedDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  client_profile_denied_discarded_msg_count: Option<i64>,
  /// Indicates whether the propagation of consumer acknowledgements (ACKs) received on the active replication Message VPN to the standby replication Message VPN is enabled.
  #[serde(rename = "consumerAckPropagationEnabled", skip_serializing_if="Option::is_none")]
  consumer_ack_propagation_enabled: Option<bool>,
  /// Indicates whether the Topic Endpoint was created by a management API (CLI or SEMP).
  #[serde(rename = "createdByManagement", skip_serializing_if="Option::is_none")]
  created_by_management: Option<bool>,
  /// The name of the Dead Message Queue (DMQ) used by the Topic Endpoint.
  #[serde(rename = "deadMsgQueue", skip_serializing_if="Option::is_none")]
  dead_msg_queue: Option<String>,
  /// The number of guaranteed messages deleted from the Topic Endpoint.
  #[serde(rename = "deletedMsgCount", skip_serializing_if="Option::is_none")]
  deleted_msg_count: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to a destination group error.
  #[serde(rename = "destinationGroupErrorDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  destination_group_error_discarded_msg_count: Option<i64>,
  /// The destination topic of the Topic Endpoint.
  #[serde(rename = "destinationTopic", skip_serializing_if="Option::is_none")]
  destination_topic: Option<String>,
  /// The number of Topic Endpoint bind failures due to being disabled.
  #[serde(rename = "disabledBindFailureCount", skip_serializing_if="Option::is_none")]
  disabled_bind_failure_count: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to it being disabled.
  #[serde(rename = "disabledDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  disabled_discarded_msg_count: Option<i64>,
  /// Indicates whether the Topic Endpoint is durable and not temporary.
  #[serde(rename = "durable", skip_serializing_if="Option::is_none")]
  durable: Option<bool>,
  /// Indicates whether the transmission of messages from the Topic Endpoint is enabled.
  #[serde(rename = "egressEnabled", skip_serializing_if="Option::is_none")]
  egress_enabled: Option<bool>,
  #[serde(rename = "eventBindCountThreshold", skip_serializing_if="Option::is_none")]
  event_bind_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventRejectLowPriorityMsgLimitThreshold", skip_serializing_if="Option::is_none")]
  event_reject_low_priority_msg_limit_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventSpoolUsageThreshold", skip_serializing_if="Option::is_none")]
  event_spool_usage_threshold: Option<::models::EventThreshold>,
  /// The highest identifier (ID) of guaranteed messages in the Topic Endpoint that were acknowledged.
  #[serde(rename = "highestAckedMsgId", skip_serializing_if="Option::is_none")]
  highest_acked_msg_id: Option<i64>,
  /// The highest identifier (ID) of guaranteed messages in the Topic Endpoint.
  #[serde(rename = "highestMsgId", skip_serializing_if="Option::is_none")]
  highest_msg_id: Option<i64>,
  /// The number of acknowledgement messages received by the Topic Endpoint that are in the process of updating and deleting associated guaranteed messages.
  #[serde(rename = "inProgressAckMsgCount", skip_serializing_if="Option::is_none")]
  in_progress_ack_msg_count: Option<i64>,
  /// Indicates whether the reception of messages to the Topic Endpoint is enabled.
  #[serde(rename = "ingressEnabled", skip_serializing_if="Option::is_none")]
  ingress_enabled: Option<bool>,
  /// The number of Topic Endpoint bind failures due to an invalid selector.
  #[serde(rename = "invalidSelectorBindFailureCount", skip_serializing_if="Option::is_none")]
  invalid_selector_bind_failure_count: Option<i64>,
  /// The timestamp of the last completed replay for the Topic Endpoint. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastReplayCompleteTime", skip_serializing_if="Option::is_none")]
  last_replay_complete_time: Option<i32>,
  /// The reason for the last replay failure for the Topic Endpoint.
  #[serde(rename = "lastReplayFailureReason", skip_serializing_if="Option::is_none")]
  last_replay_failure_reason: Option<String>,
  /// The timestamp of the last replay failure for the Topic Endpoint. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastReplayFailureTime", skip_serializing_if="Option::is_none")]
  last_replay_failure_time: Option<i32>,
  /// The timestamp of the last replay started for the Topic Endpoint. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastReplayStartTime", skip_serializing_if="Option::is_none")]
  last_replay_start_time: Option<i32>,
  /// The timestamp of the last replayed message transmitted by the Topic Endpoint. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastReplayedMsgTxTime", skip_serializing_if="Option::is_none")]
  last_replayed_msg_tx_time: Option<i32>,
  /// The identifier (ID) of the last message examined by the Topic Endpoint selector.
  #[serde(rename = "lastSelectorExaminedMsgId", skip_serializing_if="Option::is_none")]
  last_selector_examined_msg_id: Option<i64>,
  /// The identifier (ID) of the last guaranteed message spooled in the Topic Endpoint.
  #[serde(rename = "lastSpooledMsgId", skip_serializing_if="Option::is_none")]
  last_spooled_msg_id: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to low priority message congestion control.
  #[serde(rename = "lowPriorityMsgCongestionDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  low_priority_msg_congestion_discarded_msg_count: Option<i64>,
  /// The state of the low priority message congestion in the Topic Endpoint. The allowed values and their meaning are:  <pre> \"disabled\" - Messages are not being checked for priority. \"not-congested\" - Low priority messages are being stored and delivered. \"congested\" - Low priority messages are being discarded. </pre> 
  #[serde(rename = "lowPriorityMsgCongestionState", skip_serializing_if="Option::is_none")]
  low_priority_msg_congestion_state: Option<String>,
  /// The lowest identifier (ID) of guaranteed messages in the Topic Endpoint that were acknowledged.
  #[serde(rename = "lowestAckedMsgId", skip_serializing_if="Option::is_none")]
  lowest_acked_msg_id: Option<i64>,
  /// The lowest identifier (ID) of guaranteed messages in the Topic Endpoint.
  #[serde(rename = "lowestMsgId", skip_serializing_if="Option::is_none")]
  lowest_msg_id: Option<i64>,
  /// The maximum number of consumer flows that can bind to the Topic Endpoint.
  #[serde(rename = "maxBindCount", skip_serializing_if="Option::is_none")]
  max_bind_count: Option<i64>,
  /// The number of Topic Endpoint bind failures due to the maximum bind count being exceeded.
  #[serde(rename = "maxBindCountExceededBindFailureCount", skip_serializing_if="Option::is_none")]
  max_bind_count_exceeded_bind_failure_count: Option<i64>,
  /// The maximum number of messages delivered but not acknowledged per flow for the Topic Endpoint.
  #[serde(rename = "maxDeliveredUnackedMsgsPerFlow", skip_serializing_if="Option::is_none")]
  max_delivered_unacked_msgs_per_flow: Option<i64>,
  /// The effective maximum number of consumer flows that can bind to the Topic Endpoint.
  #[serde(rename = "maxEffectiveBindCount", skip_serializing_if="Option::is_none")]
  max_effective_bind_count: Option<i32>,
  /// The maximum message size allowed in the Topic Endpoint, in bytes (B).
  #[serde(rename = "maxMsgSize", skip_serializing_if="Option::is_none")]
  max_msg_size: Option<i32>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to the maximum message size being exceeded.
  #[serde(rename = "maxMsgSizeExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  max_msg_size_exceeded_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to the maximum message spool usage being exceeded.
  #[serde(rename = "maxMsgSpoolUsageExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  max_msg_spool_usage_exceeded_discarded_msg_count: Option<i64>,
  /// The maximum number of times the Topic Endpoint will attempt redelivery of a message prior to it being discarded or moved to the DMQ. A value of 0 means to retry forever.
  #[serde(rename = "maxRedeliveryCount", skip_serializing_if="Option::is_none")]
  max_redelivery_count: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to the maximum redelivery attempts being exceeded.
  #[serde(rename = "maxRedeliveryExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  max_redelivery_exceeded_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to the maximum redelivery attempts being exceeded and failing to move to the Dead Message Queue (DMQ).
  #[serde(rename = "maxRedeliveryExceededToDmqFailedMsgCount", skip_serializing_if="Option::is_none")]
  max_redelivery_exceeded_to_dmq_failed_msg_count: Option<i64>,
  /// The number of guaranteed messages moved to the Dead Message Queue (DMQ) by the Topic Endpoint due to the maximum redelivery attempts being exceeded.
  #[serde(rename = "maxRedeliveryExceededToDmqMsgCount", skip_serializing_if="Option::is_none")]
  max_redelivery_exceeded_to_dmq_msg_count: Option<i64>,
  /// The maximum message spool usage allowed by the Topic Endpoint, in megabytes (MB). A value of 0 only allows spooling of the last message received and disables quota checking.
  #[serde(rename = "maxSpoolUsage", skip_serializing_if="Option::is_none")]
  max_spool_usage: Option<i64>,
  /// The maximum time in seconds a message can stay in the Topic Endpoint when `respectTtlEnabled` is `\"true\"`. A message expires when the lesser of the sender assigned time-to-live (TTL) in the message and the `maxTtl` configured for the Topic Endpoint, is exceeded. A value of 0 disables expiry.
  #[serde(rename = "maxTtl", skip_serializing_if="Option::is_none")]
  max_ttl: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to the maximum time-to-live (TTL) in hops being exceeded. The TTL hop count is incremented when the message crosses a bridge.
  #[serde(rename = "maxTtlExceededDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  max_ttl_exceeded_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to the maximum time-to-live (TTL) timestamp expiring.
  #[serde(rename = "maxTtlExpiredDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  max_ttl_expired_discarded_msg_count: Option<i64>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to the maximum time-to-live (TTL) timestamp expiring and failing to move to the Dead Message Queue (DMQ).
  #[serde(rename = "maxTtlExpiredToDmqFailedMsgCount", skip_serializing_if="Option::is_none")]
  max_ttl_expired_to_dmq_failed_msg_count: Option<i64>,
  /// The number of guaranteed messages moved to the Dead Message Queue (DMQ) by the Topic Endpoint due to the maximum time-to-live (TTL) timestamp expiring.
  #[serde(rename = "maxTtlExpiredToDmqMsgCount", skip_serializing_if="Option::is_none")]
  max_ttl_expired_to_dmq_msg_count: Option<i64>,
  /// The message spool peak usage by the Topic Endpoint, in bytes (B).
  #[serde(rename = "msgSpoolPeakUsage", skip_serializing_if="Option::is_none")]
  msg_spool_peak_usage: Option<i64>,
  /// The message spool usage by the Topic Endpoint, in bytes (B).
  #[serde(rename = "msgSpoolUsage", skip_serializing_if="Option::is_none")]
  msg_spool_usage: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The name of the network topic for the Topic Endpoint.
  #[serde(rename = "networkTopic", skip_serializing_if="Option::is_none")]
  network_topic: Option<String>,
  /// The number of guaranteed messages discarded by the Topic Endpoint due to no local delivery being requested.
  #[serde(rename = "noLocalDeliveryDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  no_local_delivery_discarded_msg_count: Option<i64>,
  /// The number of Topic Endpoint bind failures due to other reasons.
  #[serde(rename = "otherBindFailureCount", skip_serializing_if="Option::is_none")]
  other_bind_failure_count: Option<i64>,
  /// The Client Username that owns the Topic Endpoint and has permission equivalent to `\"delete\"`.
  #[serde(rename = "owner", skip_serializing_if="Option::is_none")]
  owner: Option<String>,
  /// The permission level for all consumers of the Topic Endpoint, excluding the owner. The allowed values and their meaning are:  <pre> \"no-access\" - Disallows all access. \"read-only\" - Read-only access to the messages. \"consume\" - Consume (read and remove) messages. \"modify-topic\" - Consume messages or modify the topic/selector. \"delete\" - Consume messages, modify the topic/selector or delete the Client created endpoint altogether. </pre> 
  #[serde(rename = "permission", skip_serializing_if="Option::is_none")]
  permission: Option<String>,
  /// The number of guaranteed messages transmitted by the Topic Endpoint for redelivery.
  #[serde(rename = "redeliveredMsgCount", skip_serializing_if="Option::is_none")]
  redelivered_msg_count: Option<i64>,
  /// Indicates whether the checking of low priority messages against the `rejectLowPriorityMsgLimit` is enabled.
  #[serde(rename = "rejectLowPriorityMsgEnabled", skip_serializing_if="Option::is_none")]
  reject_low_priority_msg_enabled: Option<bool>,
  /// The number of messages of any priority in the Topic Endpoint above which low priority messages are not admitted but higher priority messages are allowed.
  #[serde(rename = "rejectLowPriorityMsgLimit", skip_serializing_if="Option::is_none")]
  reject_low_priority_msg_limit: Option<i64>,
  /// Determines when to return negative acknowledgements (NACKs) to sending clients on message discards. Note that NACKs cause the message to not be delivered to any destination and Transacted Session commits to fail. The allowed values and their meaning are:  <pre> \"always\" - Always return a negative acknowledgment (NACK) to the sending client on message discard. \"when-topic-endpoint-enabled\" - Only return a negative acknowledgment (NACK) to the sending client on message discard when the Topic Endpoint is enabled. \"never\" - Never return a negative acknowledgment (NACK) to the sending client on message discard. </pre> 
  #[serde(rename = "rejectMsgToSenderOnDiscardBehavior", skip_serializing_if="Option::is_none")]
  reject_msg_to_sender_on_discard_behavior: Option<String>,
  /// The number of replays that failed for the Topic Endpoint.
  #[serde(rename = "replayFailureCount", skip_serializing_if="Option::is_none")]
  replay_failure_count: Option<i64>,
  /// The number of replays started for the Topic Endpoint.
  #[serde(rename = "replayStartCount", skip_serializing_if="Option::is_none")]
  replay_start_count: Option<i64>,
  /// The state of replay for the Topic Endpoint. The allowed values and their meaning are:  <pre> \"initializing\" - All messages are being deleted from the endpoint before replay starts. \"active\" - Subscription matching logged messages are being replayed to the endpoint. \"pending-complete\" - Replay is complete, but final accounting is in progress. \"complete\" - Replay and all related activities are complete. \"failed\" - Replay has failed and is waiting for an unbind response. </pre> 
  #[serde(rename = "replayState", skip_serializing_if="Option::is_none")]
  replay_state: Option<String>,
  /// The number of replays that succeeded for the Topic Endpoint.
  #[serde(rename = "replaySuccessCount", skip_serializing_if="Option::is_none")]
  replay_success_count: Option<i64>,
  /// The number of replayed messages transmitted by the Topic Endpoint and acked by all consumers.
  #[serde(rename = "replayedAckedMsgCount", skip_serializing_if="Option::is_none")]
  replayed_acked_msg_count: Option<i64>,
  /// The number of replayed messages transmitted by the Topic Endpoint.
  #[serde(rename = "replayedTxMsgCount", skip_serializing_if="Option::is_none")]
  replayed_tx_msg_count: Option<i64>,
  /// The number of acknowledgement messages propagated by the Topic Endpoint to the replication standby remote Message VPN.
  #[serde(rename = "replicationActiveAckPropTxMsgCount", skip_serializing_if="Option::is_none")]
  replication_active_ack_prop_tx_msg_count: Option<i64>,
  /// The number of propagated acknowledgement messages received by the Topic Endpoint from the replication active remote Message VPN.
  #[serde(rename = "replicationStandbyAckPropRxMsgCount", skip_serializing_if="Option::is_none")]
  replication_standby_ack_prop_rx_msg_count: Option<i64>,
  /// The number of messages acknowledged in the Topic Endpoint by acknowledgement propagation from the replication active remote Message VPN.
  #[serde(rename = "replicationStandbyAckedByAckPropMsgCount", skip_serializing_if="Option::is_none")]
  replication_standby_acked_by_ack_prop_msg_count: Option<i64>,
  /// The number of messages received by the Topic Endpoint from the replication active remote Message VPN.
  #[serde(rename = "replicationStandbyRxMsgCount", skip_serializing_if="Option::is_none")]
  replication_standby_rx_msg_count: Option<i64>,
  /// Indicates whether message priorities are respected. When enabled, messages contained in the Topic Endpoint are delivered in priority order, from 9 (highest) to 0 (lowest).
  #[serde(rename = "respectMsgPriorityEnabled", skip_serializing_if="Option::is_none")]
  respect_msg_priority_enabled: Option<bool>,
  /// Indicates whether the time-to-live (TTL) for messages in the Topic Endpoint is respected. When enabled, expired messages are discarded or moved to the DMQ.
  #[serde(rename = "respectTtlEnabled", skip_serializing_if="Option::is_none")]
  respect_ttl_enabled: Option<bool>,
  /// The current message rate received by the Topic Endpoint, in bytes per second (B/sec).
  #[serde(rename = "rxByteRate", skip_serializing_if="Option::is_none")]
  rx_byte_rate: Option<i32>,
  /// The current message rate received by the Topic Endpoint, in messages per second (msg/sec).
  #[serde(rename = "rxMsgRate", skip_serializing_if="Option::is_none")]
  rx_msg_rate: Option<i64>,
  /// Indicates whether the Topic Endpoint has a selector to filter received messages.
  #[serde(rename = "rxSelector", skip_serializing_if="Option::is_none")]
  rx_selector: Option<bool>,
  /// The value of the receive selector for the Topic Endpoint.
  #[serde(rename = "selector", skip_serializing_if="Option::is_none")]
  selector: Option<String>,
  /// The number of guaranteed messages examined by the Topic Endpoint selector.
  #[serde(rename = "selectorExaminedMsgCount", skip_serializing_if="Option::is_none")]
  selector_examined_msg_count: Option<i64>,
  /// The number of guaranteed messages for which the Topic Endpoint selector matched.
  #[serde(rename = "selectorMatchedMsgCount", skip_serializing_if="Option::is_none")]
  selector_matched_msg_count: Option<i64>,
  /// The number of guaranteed messages for which the Topic Endpoint selector did not match.
  #[serde(rename = "selectorNotMatchedMsgCount", skip_serializing_if="Option::is_none")]
  selector_not_matched_msg_count: Option<i64>,
  /// The amount of guaranteed messages that were spooled in the Topic Endpoint, in bytes (B).
  #[serde(rename = "spooledByteCount", skip_serializing_if="Option::is_none")]
  spooled_byte_count: Option<i64>,
  /// The number of guaranteed messages that were spooled in the Topic Endpoint.
  #[serde(rename = "spooledMsgCount", skip_serializing_if="Option::is_none")]
  spooled_msg_count: Option<i64>,
  /// The name of the Topic Endpoint.
  #[serde(rename = "topicEndpointName", skip_serializing_if="Option::is_none")]
  topic_endpoint_name: Option<String>,
  /// The current message rate transmitted by the Topic Endpoint, in bytes per second (B/sec).
  #[serde(rename = "txByteRate", skip_serializing_if="Option::is_none")]
  tx_byte_rate: Option<i64>,
  /// The current message rate transmitted by the Topic Endpoint, in messages per second (msg/sec).
  #[serde(rename = "txMsgRate", skip_serializing_if="Option::is_none")]
  tx_msg_rate: Option<i64>,
  /// The number of guaranteed messages in the Topic Endpoint that have been transmitted but not acknowledged by all consumers.
  #[serde(rename = "txUnackedMsgCount", skip_serializing_if="Option::is_none")]
  tx_unacked_msg_count: Option<i64>,
  /// The virtual router used by the Topic Endpoint. The allowed values and their meaning are:  <pre> \"primary\" - The endpoint belongs to the primary virtual router. \"backup\" - The endpoint belongs to the backup virtual router. </pre> 
  #[serde(rename = "virtualRouter", skip_serializing_if="Option::is_none")]
  virtual_router: Option<String>
}

impl MsgVpnTopicEndpoint {
  pub fn new() -> MsgVpnTopicEndpoint {
    MsgVpnTopicEndpoint {
      access_type: None,
      already_bound_bind_failure_count: None,
      average_rx_byte_rate: None,
      average_rx_msg_rate: None,
      average_tx_byte_rate: None,
      average_tx_msg_rate: None,
      bind_request_count: None,
      bind_success_count: None,
      bind_time_forwarding_mode: None,
      client_profile_denied_discarded_msg_count: None,
      consumer_ack_propagation_enabled: None,
      created_by_management: None,
      dead_msg_queue: None,
      deleted_msg_count: None,
      destination_group_error_discarded_msg_count: None,
      destination_topic: None,
      disabled_bind_failure_count: None,
      disabled_discarded_msg_count: None,
      durable: None,
      egress_enabled: None,
      event_bind_count_threshold: None,
      event_reject_low_priority_msg_limit_threshold: None,
      event_spool_usage_threshold: None,
      highest_acked_msg_id: None,
      highest_msg_id: None,
      in_progress_ack_msg_count: None,
      ingress_enabled: None,
      invalid_selector_bind_failure_count: None,
      last_replay_complete_time: None,
      last_replay_failure_reason: None,
      last_replay_failure_time: None,
      last_replay_start_time: None,
      last_replayed_msg_tx_time: None,
      last_selector_examined_msg_id: None,
      last_spooled_msg_id: None,
      low_priority_msg_congestion_discarded_msg_count: None,
      low_priority_msg_congestion_state: None,
      lowest_acked_msg_id: None,
      lowest_msg_id: None,
      max_bind_count: None,
      max_bind_count_exceeded_bind_failure_count: None,
      max_delivered_unacked_msgs_per_flow: None,
      max_effective_bind_count: None,
      max_msg_size: None,
      max_msg_size_exceeded_discarded_msg_count: None,
      max_msg_spool_usage_exceeded_discarded_msg_count: None,
      max_redelivery_count: None,
      max_redelivery_exceeded_discarded_msg_count: None,
      max_redelivery_exceeded_to_dmq_failed_msg_count: None,
      max_redelivery_exceeded_to_dmq_msg_count: None,
      max_spool_usage: None,
      max_ttl: None,
      max_ttl_exceeded_discarded_msg_count: None,
      max_ttl_expired_discarded_msg_count: None,
      max_ttl_expired_to_dmq_failed_msg_count: None,
      max_ttl_expired_to_dmq_msg_count: None,
      msg_spool_peak_usage: None,
      msg_spool_usage: None,
      msg_vpn_name: None,
      network_topic: None,
      no_local_delivery_discarded_msg_count: None,
      other_bind_failure_count: None,
      owner: None,
      permission: None,
      redelivered_msg_count: None,
      reject_low_priority_msg_enabled: None,
      reject_low_priority_msg_limit: None,
      reject_msg_to_sender_on_discard_behavior: None,
      replay_failure_count: None,
      replay_start_count: None,
      replay_state: None,
      replay_success_count: None,
      replayed_acked_msg_count: None,
      replayed_tx_msg_count: None,
      replication_active_ack_prop_tx_msg_count: None,
      replication_standby_ack_prop_rx_msg_count: None,
      replication_standby_acked_by_ack_prop_msg_count: None,
      replication_standby_rx_msg_count: None,
      respect_msg_priority_enabled: None,
      respect_ttl_enabled: None,
      rx_byte_rate: None,
      rx_msg_rate: None,
      rx_selector: None,
      selector: None,
      selector_examined_msg_count: None,
      selector_matched_msg_count: None,
      selector_not_matched_msg_count: None,
      spooled_byte_count: None,
      spooled_msg_count: None,
      topic_endpoint_name: None,
      tx_byte_rate: None,
      tx_msg_rate: None,
      tx_unacked_msg_count: None,
      virtual_router: None
    }
  }

  pub fn set_access_type(&mut self, access_type: String) {
    self.access_type = Some(access_type);
  }

  pub fn with_access_type(mut self, access_type: String) -> MsgVpnTopicEndpoint {
    self.access_type = Some(access_type);
    self
  }

  pub fn access_type(&self) -> Option<&String> {
    self.access_type.as_ref()
  }

  pub fn reset_access_type(&mut self) {
    self.access_type = None;
  }

  pub fn set_already_bound_bind_failure_count(&mut self, already_bound_bind_failure_count: i64) {
    self.already_bound_bind_failure_count = Some(already_bound_bind_failure_count);
  }

  pub fn with_already_bound_bind_failure_count(mut self, already_bound_bind_failure_count: i64) -> MsgVpnTopicEndpoint {
    self.already_bound_bind_failure_count = Some(already_bound_bind_failure_count);
    self
  }

  pub fn already_bound_bind_failure_count(&self) -> Option<&i64> {
    self.already_bound_bind_failure_count.as_ref()
  }

  pub fn reset_already_bound_bind_failure_count(&mut self) {
    self.already_bound_bind_failure_count = None;
  }

  pub fn set_average_rx_byte_rate(&mut self, average_rx_byte_rate: i64) {
    self.average_rx_byte_rate = Some(average_rx_byte_rate);
  }

  pub fn with_average_rx_byte_rate(mut self, average_rx_byte_rate: i64) -> MsgVpnTopicEndpoint {
    self.average_rx_byte_rate = Some(average_rx_byte_rate);
    self
  }

  pub fn average_rx_byte_rate(&self) -> Option<&i64> {
    self.average_rx_byte_rate.as_ref()
  }

  pub fn reset_average_rx_byte_rate(&mut self) {
    self.average_rx_byte_rate = None;
  }

  pub fn set_average_rx_msg_rate(&mut self, average_rx_msg_rate: i64) {
    self.average_rx_msg_rate = Some(average_rx_msg_rate);
  }

  pub fn with_average_rx_msg_rate(mut self, average_rx_msg_rate: i64) -> MsgVpnTopicEndpoint {
    self.average_rx_msg_rate = Some(average_rx_msg_rate);
    self
  }

  pub fn average_rx_msg_rate(&self) -> Option<&i64> {
    self.average_rx_msg_rate.as_ref()
  }

  pub fn reset_average_rx_msg_rate(&mut self) {
    self.average_rx_msg_rate = None;
  }

  pub fn set_average_tx_byte_rate(&mut self, average_tx_byte_rate: i64) {
    self.average_tx_byte_rate = Some(average_tx_byte_rate);
  }

  pub fn with_average_tx_byte_rate(mut self, average_tx_byte_rate: i64) -> MsgVpnTopicEndpoint {
    self.average_tx_byte_rate = Some(average_tx_byte_rate);
    self
  }

  pub fn average_tx_byte_rate(&self) -> Option<&i64> {
    self.average_tx_byte_rate.as_ref()
  }

  pub fn reset_average_tx_byte_rate(&mut self) {
    self.average_tx_byte_rate = None;
  }

  pub fn set_average_tx_msg_rate(&mut self, average_tx_msg_rate: i64) {
    self.average_tx_msg_rate = Some(average_tx_msg_rate);
  }

  pub fn with_average_tx_msg_rate(mut self, average_tx_msg_rate: i64) -> MsgVpnTopicEndpoint {
    self.average_tx_msg_rate = Some(average_tx_msg_rate);
    self
  }

  pub fn average_tx_msg_rate(&self) -> Option<&i64> {
    self.average_tx_msg_rate.as_ref()
  }

  pub fn reset_average_tx_msg_rate(&mut self) {
    self.average_tx_msg_rate = None;
  }

  pub fn set_bind_request_count(&mut self, bind_request_count: i64) {
    self.bind_request_count = Some(bind_request_count);
  }

  pub fn with_bind_request_count(mut self, bind_request_count: i64) -> MsgVpnTopicEndpoint {
    self.bind_request_count = Some(bind_request_count);
    self
  }

  pub fn bind_request_count(&self) -> Option<&i64> {
    self.bind_request_count.as_ref()
  }

  pub fn reset_bind_request_count(&mut self) {
    self.bind_request_count = None;
  }

  pub fn set_bind_success_count(&mut self, bind_success_count: i64) {
    self.bind_success_count = Some(bind_success_count);
  }

  pub fn with_bind_success_count(mut self, bind_success_count: i64) -> MsgVpnTopicEndpoint {
    self.bind_success_count = Some(bind_success_count);
    self
  }

  pub fn bind_success_count(&self) -> Option<&i64> {
    self.bind_success_count.as_ref()
  }

  pub fn reset_bind_success_count(&mut self) {
    self.bind_success_count = None;
  }

  pub fn set_bind_time_forwarding_mode(&mut self, bind_time_forwarding_mode: String) {
    self.bind_time_forwarding_mode = Some(bind_time_forwarding_mode);
  }

  pub fn with_bind_time_forwarding_mode(mut self, bind_time_forwarding_mode: String) -> MsgVpnTopicEndpoint {
    self.bind_time_forwarding_mode = Some(bind_time_forwarding_mode);
    self
  }

  pub fn bind_time_forwarding_mode(&self) -> Option<&String> {
    self.bind_time_forwarding_mode.as_ref()
  }

  pub fn reset_bind_time_forwarding_mode(&mut self) {
    self.bind_time_forwarding_mode = None;
  }

  pub fn set_client_profile_denied_discarded_msg_count(&mut self, client_profile_denied_discarded_msg_count: i64) {
    self.client_profile_denied_discarded_msg_count = Some(client_profile_denied_discarded_msg_count);
  }

  pub fn with_client_profile_denied_discarded_msg_count(mut self, client_profile_denied_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.client_profile_denied_discarded_msg_count = Some(client_profile_denied_discarded_msg_count);
    self
  }

  pub fn client_profile_denied_discarded_msg_count(&self) -> Option<&i64> {
    self.client_profile_denied_discarded_msg_count.as_ref()
  }

  pub fn reset_client_profile_denied_discarded_msg_count(&mut self) {
    self.client_profile_denied_discarded_msg_count = None;
  }

  pub fn set_consumer_ack_propagation_enabled(&mut self, consumer_ack_propagation_enabled: bool) {
    self.consumer_ack_propagation_enabled = Some(consumer_ack_propagation_enabled);
  }

  pub fn with_consumer_ack_propagation_enabled(mut self, consumer_ack_propagation_enabled: bool) -> MsgVpnTopicEndpoint {
    self.consumer_ack_propagation_enabled = Some(consumer_ack_propagation_enabled);
    self
  }

  pub fn consumer_ack_propagation_enabled(&self) -> Option<&bool> {
    self.consumer_ack_propagation_enabled.as_ref()
  }

  pub fn reset_consumer_ack_propagation_enabled(&mut self) {
    self.consumer_ack_propagation_enabled = None;
  }

  pub fn set_created_by_management(&mut self, created_by_management: bool) {
    self.created_by_management = Some(created_by_management);
  }

  pub fn with_created_by_management(mut self, created_by_management: bool) -> MsgVpnTopicEndpoint {
    self.created_by_management = Some(created_by_management);
    self
  }

  pub fn created_by_management(&self) -> Option<&bool> {
    self.created_by_management.as_ref()
  }

  pub fn reset_created_by_management(&mut self) {
    self.created_by_management = None;
  }

  pub fn set_dead_msg_queue(&mut self, dead_msg_queue: String) {
    self.dead_msg_queue = Some(dead_msg_queue);
  }

  pub fn with_dead_msg_queue(mut self, dead_msg_queue: String) -> MsgVpnTopicEndpoint {
    self.dead_msg_queue = Some(dead_msg_queue);
    self
  }

  pub fn dead_msg_queue(&self) -> Option<&String> {
    self.dead_msg_queue.as_ref()
  }

  pub fn reset_dead_msg_queue(&mut self) {
    self.dead_msg_queue = None;
  }

  pub fn set_deleted_msg_count(&mut self, deleted_msg_count: i64) {
    self.deleted_msg_count = Some(deleted_msg_count);
  }

  pub fn with_deleted_msg_count(mut self, deleted_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.deleted_msg_count = Some(deleted_msg_count);
    self
  }

  pub fn deleted_msg_count(&self) -> Option<&i64> {
    self.deleted_msg_count.as_ref()
  }

  pub fn reset_deleted_msg_count(&mut self) {
    self.deleted_msg_count = None;
  }

  pub fn set_destination_group_error_discarded_msg_count(&mut self, destination_group_error_discarded_msg_count: i64) {
    self.destination_group_error_discarded_msg_count = Some(destination_group_error_discarded_msg_count);
  }

  pub fn with_destination_group_error_discarded_msg_count(mut self, destination_group_error_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.destination_group_error_discarded_msg_count = Some(destination_group_error_discarded_msg_count);
    self
  }

  pub fn destination_group_error_discarded_msg_count(&self) -> Option<&i64> {
    self.destination_group_error_discarded_msg_count.as_ref()
  }

  pub fn reset_destination_group_error_discarded_msg_count(&mut self) {
    self.destination_group_error_discarded_msg_count = None;
  }

  pub fn set_destination_topic(&mut self, destination_topic: String) {
    self.destination_topic = Some(destination_topic);
  }

  pub fn with_destination_topic(mut self, destination_topic: String) -> MsgVpnTopicEndpoint {
    self.destination_topic = Some(destination_topic);
    self
  }

  pub fn destination_topic(&self) -> Option<&String> {
    self.destination_topic.as_ref()
  }

  pub fn reset_destination_topic(&mut self) {
    self.destination_topic = None;
  }

  pub fn set_disabled_bind_failure_count(&mut self, disabled_bind_failure_count: i64) {
    self.disabled_bind_failure_count = Some(disabled_bind_failure_count);
  }

  pub fn with_disabled_bind_failure_count(mut self, disabled_bind_failure_count: i64) -> MsgVpnTopicEndpoint {
    self.disabled_bind_failure_count = Some(disabled_bind_failure_count);
    self
  }

  pub fn disabled_bind_failure_count(&self) -> Option<&i64> {
    self.disabled_bind_failure_count.as_ref()
  }

  pub fn reset_disabled_bind_failure_count(&mut self) {
    self.disabled_bind_failure_count = None;
  }

  pub fn set_disabled_discarded_msg_count(&mut self, disabled_discarded_msg_count: i64) {
    self.disabled_discarded_msg_count = Some(disabled_discarded_msg_count);
  }

  pub fn with_disabled_discarded_msg_count(mut self, disabled_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.disabled_discarded_msg_count = Some(disabled_discarded_msg_count);
    self
  }

  pub fn disabled_discarded_msg_count(&self) -> Option<&i64> {
    self.disabled_discarded_msg_count.as_ref()
  }

  pub fn reset_disabled_discarded_msg_count(&mut self) {
    self.disabled_discarded_msg_count = None;
  }

  pub fn set_durable(&mut self, durable: bool) {
    self.durable = Some(durable);
  }

  pub fn with_durable(mut self, durable: bool) -> MsgVpnTopicEndpoint {
    self.durable = Some(durable);
    self
  }

  pub fn durable(&self) -> Option<&bool> {
    self.durable.as_ref()
  }

  pub fn reset_durable(&mut self) {
    self.durable = None;
  }

  pub fn set_egress_enabled(&mut self, egress_enabled: bool) {
    self.egress_enabled = Some(egress_enabled);
  }

  pub fn with_egress_enabled(mut self, egress_enabled: bool) -> MsgVpnTopicEndpoint {
    self.egress_enabled = Some(egress_enabled);
    self
  }

  pub fn egress_enabled(&self) -> Option<&bool> {
    self.egress_enabled.as_ref()
  }

  pub fn reset_egress_enabled(&mut self) {
    self.egress_enabled = None;
  }

  pub fn set_event_bind_count_threshold(&mut self, event_bind_count_threshold: ::models::EventThreshold) {
    self.event_bind_count_threshold = Some(event_bind_count_threshold);
  }

  pub fn with_event_bind_count_threshold(mut self, event_bind_count_threshold: ::models::EventThreshold) -> MsgVpnTopicEndpoint {
    self.event_bind_count_threshold = Some(event_bind_count_threshold);
    self
  }

  pub fn event_bind_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_bind_count_threshold.as_ref()
  }

  pub fn reset_event_bind_count_threshold(&mut self) {
    self.event_bind_count_threshold = None;
  }

  pub fn set_event_reject_low_priority_msg_limit_threshold(&mut self, event_reject_low_priority_msg_limit_threshold: ::models::EventThreshold) {
    self.event_reject_low_priority_msg_limit_threshold = Some(event_reject_low_priority_msg_limit_threshold);
  }

  pub fn with_event_reject_low_priority_msg_limit_threshold(mut self, event_reject_low_priority_msg_limit_threshold: ::models::EventThreshold) -> MsgVpnTopicEndpoint {
    self.event_reject_low_priority_msg_limit_threshold = Some(event_reject_low_priority_msg_limit_threshold);
    self
  }

  pub fn event_reject_low_priority_msg_limit_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_reject_low_priority_msg_limit_threshold.as_ref()
  }

  pub fn reset_event_reject_low_priority_msg_limit_threshold(&mut self) {
    self.event_reject_low_priority_msg_limit_threshold = None;
  }

  pub fn set_event_spool_usage_threshold(&mut self, event_spool_usage_threshold: ::models::EventThreshold) {
    self.event_spool_usage_threshold = Some(event_spool_usage_threshold);
  }

  pub fn with_event_spool_usage_threshold(mut self, event_spool_usage_threshold: ::models::EventThreshold) -> MsgVpnTopicEndpoint {
    self.event_spool_usage_threshold = Some(event_spool_usage_threshold);
    self
  }

  pub fn event_spool_usage_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_spool_usage_threshold.as_ref()
  }

  pub fn reset_event_spool_usage_threshold(&mut self) {
    self.event_spool_usage_threshold = None;
  }

  pub fn set_highest_acked_msg_id(&mut self, highest_acked_msg_id: i64) {
    self.highest_acked_msg_id = Some(highest_acked_msg_id);
  }

  pub fn with_highest_acked_msg_id(mut self, highest_acked_msg_id: i64) -> MsgVpnTopicEndpoint {
    self.highest_acked_msg_id = Some(highest_acked_msg_id);
    self
  }

  pub fn highest_acked_msg_id(&self) -> Option<&i64> {
    self.highest_acked_msg_id.as_ref()
  }

  pub fn reset_highest_acked_msg_id(&mut self) {
    self.highest_acked_msg_id = None;
  }

  pub fn set_highest_msg_id(&mut self, highest_msg_id: i64) {
    self.highest_msg_id = Some(highest_msg_id);
  }

  pub fn with_highest_msg_id(mut self, highest_msg_id: i64) -> MsgVpnTopicEndpoint {
    self.highest_msg_id = Some(highest_msg_id);
    self
  }

  pub fn highest_msg_id(&self) -> Option<&i64> {
    self.highest_msg_id.as_ref()
  }

  pub fn reset_highest_msg_id(&mut self) {
    self.highest_msg_id = None;
  }

  pub fn set_in_progress_ack_msg_count(&mut self, in_progress_ack_msg_count: i64) {
    self.in_progress_ack_msg_count = Some(in_progress_ack_msg_count);
  }

  pub fn with_in_progress_ack_msg_count(mut self, in_progress_ack_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.in_progress_ack_msg_count = Some(in_progress_ack_msg_count);
    self
  }

  pub fn in_progress_ack_msg_count(&self) -> Option<&i64> {
    self.in_progress_ack_msg_count.as_ref()
  }

  pub fn reset_in_progress_ack_msg_count(&mut self) {
    self.in_progress_ack_msg_count = None;
  }

  pub fn set_ingress_enabled(&mut self, ingress_enabled: bool) {
    self.ingress_enabled = Some(ingress_enabled);
  }

  pub fn with_ingress_enabled(mut self, ingress_enabled: bool) -> MsgVpnTopicEndpoint {
    self.ingress_enabled = Some(ingress_enabled);
    self
  }

  pub fn ingress_enabled(&self) -> Option<&bool> {
    self.ingress_enabled.as_ref()
  }

  pub fn reset_ingress_enabled(&mut self) {
    self.ingress_enabled = None;
  }

  pub fn set_invalid_selector_bind_failure_count(&mut self, invalid_selector_bind_failure_count: i64) {
    self.invalid_selector_bind_failure_count = Some(invalid_selector_bind_failure_count);
  }

  pub fn with_invalid_selector_bind_failure_count(mut self, invalid_selector_bind_failure_count: i64) -> MsgVpnTopicEndpoint {
    self.invalid_selector_bind_failure_count = Some(invalid_selector_bind_failure_count);
    self
  }

  pub fn invalid_selector_bind_failure_count(&self) -> Option<&i64> {
    self.invalid_selector_bind_failure_count.as_ref()
  }

  pub fn reset_invalid_selector_bind_failure_count(&mut self) {
    self.invalid_selector_bind_failure_count = None;
  }

  pub fn set_last_replay_complete_time(&mut self, last_replay_complete_time: i32) {
    self.last_replay_complete_time = Some(last_replay_complete_time);
  }

  pub fn with_last_replay_complete_time(mut self, last_replay_complete_time: i32) -> MsgVpnTopicEndpoint {
    self.last_replay_complete_time = Some(last_replay_complete_time);
    self
  }

  pub fn last_replay_complete_time(&self) -> Option<&i32> {
    self.last_replay_complete_time.as_ref()
  }

  pub fn reset_last_replay_complete_time(&mut self) {
    self.last_replay_complete_time = None;
  }

  pub fn set_last_replay_failure_reason(&mut self, last_replay_failure_reason: String) {
    self.last_replay_failure_reason = Some(last_replay_failure_reason);
  }

  pub fn with_last_replay_failure_reason(mut self, last_replay_failure_reason: String) -> MsgVpnTopicEndpoint {
    self.last_replay_failure_reason = Some(last_replay_failure_reason);
    self
  }

  pub fn last_replay_failure_reason(&self) -> Option<&String> {
    self.last_replay_failure_reason.as_ref()
  }

  pub fn reset_last_replay_failure_reason(&mut self) {
    self.last_replay_failure_reason = None;
  }

  pub fn set_last_replay_failure_time(&mut self, last_replay_failure_time: i32) {
    self.last_replay_failure_time = Some(last_replay_failure_time);
  }

  pub fn with_last_replay_failure_time(mut self, last_replay_failure_time: i32) -> MsgVpnTopicEndpoint {
    self.last_replay_failure_time = Some(last_replay_failure_time);
    self
  }

  pub fn last_replay_failure_time(&self) -> Option<&i32> {
    self.last_replay_failure_time.as_ref()
  }

  pub fn reset_last_replay_failure_time(&mut self) {
    self.last_replay_failure_time = None;
  }

  pub fn set_last_replay_start_time(&mut self, last_replay_start_time: i32) {
    self.last_replay_start_time = Some(last_replay_start_time);
  }

  pub fn with_last_replay_start_time(mut self, last_replay_start_time: i32) -> MsgVpnTopicEndpoint {
    self.last_replay_start_time = Some(last_replay_start_time);
    self
  }

  pub fn last_replay_start_time(&self) -> Option<&i32> {
    self.last_replay_start_time.as_ref()
  }

  pub fn reset_last_replay_start_time(&mut self) {
    self.last_replay_start_time = None;
  }

  pub fn set_last_replayed_msg_tx_time(&mut self, last_replayed_msg_tx_time: i32) {
    self.last_replayed_msg_tx_time = Some(last_replayed_msg_tx_time);
  }

  pub fn with_last_replayed_msg_tx_time(mut self, last_replayed_msg_tx_time: i32) -> MsgVpnTopicEndpoint {
    self.last_replayed_msg_tx_time = Some(last_replayed_msg_tx_time);
    self
  }

  pub fn last_replayed_msg_tx_time(&self) -> Option<&i32> {
    self.last_replayed_msg_tx_time.as_ref()
  }

  pub fn reset_last_replayed_msg_tx_time(&mut self) {
    self.last_replayed_msg_tx_time = None;
  }

  pub fn set_last_selector_examined_msg_id(&mut self, last_selector_examined_msg_id: i64) {
    self.last_selector_examined_msg_id = Some(last_selector_examined_msg_id);
  }

  pub fn with_last_selector_examined_msg_id(mut self, last_selector_examined_msg_id: i64) -> MsgVpnTopicEndpoint {
    self.last_selector_examined_msg_id = Some(last_selector_examined_msg_id);
    self
  }

  pub fn last_selector_examined_msg_id(&self) -> Option<&i64> {
    self.last_selector_examined_msg_id.as_ref()
  }

  pub fn reset_last_selector_examined_msg_id(&mut self) {
    self.last_selector_examined_msg_id = None;
  }

  pub fn set_last_spooled_msg_id(&mut self, last_spooled_msg_id: i64) {
    self.last_spooled_msg_id = Some(last_spooled_msg_id);
  }

  pub fn with_last_spooled_msg_id(mut self, last_spooled_msg_id: i64) -> MsgVpnTopicEndpoint {
    self.last_spooled_msg_id = Some(last_spooled_msg_id);
    self
  }

  pub fn last_spooled_msg_id(&self) -> Option<&i64> {
    self.last_spooled_msg_id.as_ref()
  }

  pub fn reset_last_spooled_msg_id(&mut self) {
    self.last_spooled_msg_id = None;
  }

  pub fn set_low_priority_msg_congestion_discarded_msg_count(&mut self, low_priority_msg_congestion_discarded_msg_count: i64) {
    self.low_priority_msg_congestion_discarded_msg_count = Some(low_priority_msg_congestion_discarded_msg_count);
  }

  pub fn with_low_priority_msg_congestion_discarded_msg_count(mut self, low_priority_msg_congestion_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.low_priority_msg_congestion_discarded_msg_count = Some(low_priority_msg_congestion_discarded_msg_count);
    self
  }

  pub fn low_priority_msg_congestion_discarded_msg_count(&self) -> Option<&i64> {
    self.low_priority_msg_congestion_discarded_msg_count.as_ref()
  }

  pub fn reset_low_priority_msg_congestion_discarded_msg_count(&mut self) {
    self.low_priority_msg_congestion_discarded_msg_count = None;
  }

  pub fn set_low_priority_msg_congestion_state(&mut self, low_priority_msg_congestion_state: String) {
    self.low_priority_msg_congestion_state = Some(low_priority_msg_congestion_state);
  }

  pub fn with_low_priority_msg_congestion_state(mut self, low_priority_msg_congestion_state: String) -> MsgVpnTopicEndpoint {
    self.low_priority_msg_congestion_state = Some(low_priority_msg_congestion_state);
    self
  }

  pub fn low_priority_msg_congestion_state(&self) -> Option<&String> {
    self.low_priority_msg_congestion_state.as_ref()
  }

  pub fn reset_low_priority_msg_congestion_state(&mut self) {
    self.low_priority_msg_congestion_state = None;
  }

  pub fn set_lowest_acked_msg_id(&mut self, lowest_acked_msg_id: i64) {
    self.lowest_acked_msg_id = Some(lowest_acked_msg_id);
  }

  pub fn with_lowest_acked_msg_id(mut self, lowest_acked_msg_id: i64) -> MsgVpnTopicEndpoint {
    self.lowest_acked_msg_id = Some(lowest_acked_msg_id);
    self
  }

  pub fn lowest_acked_msg_id(&self) -> Option<&i64> {
    self.lowest_acked_msg_id.as_ref()
  }

  pub fn reset_lowest_acked_msg_id(&mut self) {
    self.lowest_acked_msg_id = None;
  }

  pub fn set_lowest_msg_id(&mut self, lowest_msg_id: i64) {
    self.lowest_msg_id = Some(lowest_msg_id);
  }

  pub fn with_lowest_msg_id(mut self, lowest_msg_id: i64) -> MsgVpnTopicEndpoint {
    self.lowest_msg_id = Some(lowest_msg_id);
    self
  }

  pub fn lowest_msg_id(&self) -> Option<&i64> {
    self.lowest_msg_id.as_ref()
  }

  pub fn reset_lowest_msg_id(&mut self) {
    self.lowest_msg_id = None;
  }

  pub fn set_max_bind_count(&mut self, max_bind_count: i64) {
    self.max_bind_count = Some(max_bind_count);
  }

  pub fn with_max_bind_count(mut self, max_bind_count: i64) -> MsgVpnTopicEndpoint {
    self.max_bind_count = Some(max_bind_count);
    self
  }

  pub fn max_bind_count(&self) -> Option<&i64> {
    self.max_bind_count.as_ref()
  }

  pub fn reset_max_bind_count(&mut self) {
    self.max_bind_count = None;
  }

  pub fn set_max_bind_count_exceeded_bind_failure_count(&mut self, max_bind_count_exceeded_bind_failure_count: i64) {
    self.max_bind_count_exceeded_bind_failure_count = Some(max_bind_count_exceeded_bind_failure_count);
  }

  pub fn with_max_bind_count_exceeded_bind_failure_count(mut self, max_bind_count_exceeded_bind_failure_count: i64) -> MsgVpnTopicEndpoint {
    self.max_bind_count_exceeded_bind_failure_count = Some(max_bind_count_exceeded_bind_failure_count);
    self
  }

  pub fn max_bind_count_exceeded_bind_failure_count(&self) -> Option<&i64> {
    self.max_bind_count_exceeded_bind_failure_count.as_ref()
  }

  pub fn reset_max_bind_count_exceeded_bind_failure_count(&mut self) {
    self.max_bind_count_exceeded_bind_failure_count = None;
  }

  pub fn set_max_delivered_unacked_msgs_per_flow(&mut self, max_delivered_unacked_msgs_per_flow: i64) {
    self.max_delivered_unacked_msgs_per_flow = Some(max_delivered_unacked_msgs_per_flow);
  }

  pub fn with_max_delivered_unacked_msgs_per_flow(mut self, max_delivered_unacked_msgs_per_flow: i64) -> MsgVpnTopicEndpoint {
    self.max_delivered_unacked_msgs_per_flow = Some(max_delivered_unacked_msgs_per_flow);
    self
  }

  pub fn max_delivered_unacked_msgs_per_flow(&self) -> Option<&i64> {
    self.max_delivered_unacked_msgs_per_flow.as_ref()
  }

  pub fn reset_max_delivered_unacked_msgs_per_flow(&mut self) {
    self.max_delivered_unacked_msgs_per_flow = None;
  }

  pub fn set_max_effective_bind_count(&mut self, max_effective_bind_count: i32) {
    self.max_effective_bind_count = Some(max_effective_bind_count);
  }

  pub fn with_max_effective_bind_count(mut self, max_effective_bind_count: i32) -> MsgVpnTopicEndpoint {
    self.max_effective_bind_count = Some(max_effective_bind_count);
    self
  }

  pub fn max_effective_bind_count(&self) -> Option<&i32> {
    self.max_effective_bind_count.as_ref()
  }

  pub fn reset_max_effective_bind_count(&mut self) {
    self.max_effective_bind_count = None;
  }

  pub fn set_max_msg_size(&mut self, max_msg_size: i32) {
    self.max_msg_size = Some(max_msg_size);
  }

  pub fn with_max_msg_size(mut self, max_msg_size: i32) -> MsgVpnTopicEndpoint {
    self.max_msg_size = Some(max_msg_size);
    self
  }

  pub fn max_msg_size(&self) -> Option<&i32> {
    self.max_msg_size.as_ref()
  }

  pub fn reset_max_msg_size(&mut self) {
    self.max_msg_size = None;
  }

  pub fn set_max_msg_size_exceeded_discarded_msg_count(&mut self, max_msg_size_exceeded_discarded_msg_count: i64) {
    self.max_msg_size_exceeded_discarded_msg_count = Some(max_msg_size_exceeded_discarded_msg_count);
  }

  pub fn with_max_msg_size_exceeded_discarded_msg_count(mut self, max_msg_size_exceeded_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_msg_size_exceeded_discarded_msg_count = Some(max_msg_size_exceeded_discarded_msg_count);
    self
  }

  pub fn max_msg_size_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.max_msg_size_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_max_msg_size_exceeded_discarded_msg_count(&mut self) {
    self.max_msg_size_exceeded_discarded_msg_count = None;
  }

  pub fn set_max_msg_spool_usage_exceeded_discarded_msg_count(&mut self, max_msg_spool_usage_exceeded_discarded_msg_count: i64) {
    self.max_msg_spool_usage_exceeded_discarded_msg_count = Some(max_msg_spool_usage_exceeded_discarded_msg_count);
  }

  pub fn with_max_msg_spool_usage_exceeded_discarded_msg_count(mut self, max_msg_spool_usage_exceeded_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_msg_spool_usage_exceeded_discarded_msg_count = Some(max_msg_spool_usage_exceeded_discarded_msg_count);
    self
  }

  pub fn max_msg_spool_usage_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.max_msg_spool_usage_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_max_msg_spool_usage_exceeded_discarded_msg_count(&mut self) {
    self.max_msg_spool_usage_exceeded_discarded_msg_count = None;
  }

  pub fn set_max_redelivery_count(&mut self, max_redelivery_count: i64) {
    self.max_redelivery_count = Some(max_redelivery_count);
  }

  pub fn with_max_redelivery_count(mut self, max_redelivery_count: i64) -> MsgVpnTopicEndpoint {
    self.max_redelivery_count = Some(max_redelivery_count);
    self
  }

  pub fn max_redelivery_count(&self) -> Option<&i64> {
    self.max_redelivery_count.as_ref()
  }

  pub fn reset_max_redelivery_count(&mut self) {
    self.max_redelivery_count = None;
  }

  pub fn set_max_redelivery_exceeded_discarded_msg_count(&mut self, max_redelivery_exceeded_discarded_msg_count: i64) {
    self.max_redelivery_exceeded_discarded_msg_count = Some(max_redelivery_exceeded_discarded_msg_count);
  }

  pub fn with_max_redelivery_exceeded_discarded_msg_count(mut self, max_redelivery_exceeded_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_redelivery_exceeded_discarded_msg_count = Some(max_redelivery_exceeded_discarded_msg_count);
    self
  }

  pub fn max_redelivery_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.max_redelivery_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_max_redelivery_exceeded_discarded_msg_count(&mut self) {
    self.max_redelivery_exceeded_discarded_msg_count = None;
  }

  pub fn set_max_redelivery_exceeded_to_dmq_failed_msg_count(&mut self, max_redelivery_exceeded_to_dmq_failed_msg_count: i64) {
    self.max_redelivery_exceeded_to_dmq_failed_msg_count = Some(max_redelivery_exceeded_to_dmq_failed_msg_count);
  }

  pub fn with_max_redelivery_exceeded_to_dmq_failed_msg_count(mut self, max_redelivery_exceeded_to_dmq_failed_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_redelivery_exceeded_to_dmq_failed_msg_count = Some(max_redelivery_exceeded_to_dmq_failed_msg_count);
    self
  }

  pub fn max_redelivery_exceeded_to_dmq_failed_msg_count(&self) -> Option<&i64> {
    self.max_redelivery_exceeded_to_dmq_failed_msg_count.as_ref()
  }

  pub fn reset_max_redelivery_exceeded_to_dmq_failed_msg_count(&mut self) {
    self.max_redelivery_exceeded_to_dmq_failed_msg_count = None;
  }

  pub fn set_max_redelivery_exceeded_to_dmq_msg_count(&mut self, max_redelivery_exceeded_to_dmq_msg_count: i64) {
    self.max_redelivery_exceeded_to_dmq_msg_count = Some(max_redelivery_exceeded_to_dmq_msg_count);
  }

  pub fn with_max_redelivery_exceeded_to_dmq_msg_count(mut self, max_redelivery_exceeded_to_dmq_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_redelivery_exceeded_to_dmq_msg_count = Some(max_redelivery_exceeded_to_dmq_msg_count);
    self
  }

  pub fn max_redelivery_exceeded_to_dmq_msg_count(&self) -> Option<&i64> {
    self.max_redelivery_exceeded_to_dmq_msg_count.as_ref()
  }

  pub fn reset_max_redelivery_exceeded_to_dmq_msg_count(&mut self) {
    self.max_redelivery_exceeded_to_dmq_msg_count = None;
  }

  pub fn set_max_spool_usage(&mut self, max_spool_usage: i64) {
    self.max_spool_usage = Some(max_spool_usage);
  }

  pub fn with_max_spool_usage(mut self, max_spool_usage: i64) -> MsgVpnTopicEndpoint {
    self.max_spool_usage = Some(max_spool_usage);
    self
  }

  pub fn max_spool_usage(&self) -> Option<&i64> {
    self.max_spool_usage.as_ref()
  }

  pub fn reset_max_spool_usage(&mut self) {
    self.max_spool_usage = None;
  }

  pub fn set_max_ttl(&mut self, max_ttl: i64) {
    self.max_ttl = Some(max_ttl);
  }

  pub fn with_max_ttl(mut self, max_ttl: i64) -> MsgVpnTopicEndpoint {
    self.max_ttl = Some(max_ttl);
    self
  }

  pub fn max_ttl(&self) -> Option<&i64> {
    self.max_ttl.as_ref()
  }

  pub fn reset_max_ttl(&mut self) {
    self.max_ttl = None;
  }

  pub fn set_max_ttl_exceeded_discarded_msg_count(&mut self, max_ttl_exceeded_discarded_msg_count: i64) {
    self.max_ttl_exceeded_discarded_msg_count = Some(max_ttl_exceeded_discarded_msg_count);
  }

  pub fn with_max_ttl_exceeded_discarded_msg_count(mut self, max_ttl_exceeded_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_ttl_exceeded_discarded_msg_count = Some(max_ttl_exceeded_discarded_msg_count);
    self
  }

  pub fn max_ttl_exceeded_discarded_msg_count(&self) -> Option<&i64> {
    self.max_ttl_exceeded_discarded_msg_count.as_ref()
  }

  pub fn reset_max_ttl_exceeded_discarded_msg_count(&mut self) {
    self.max_ttl_exceeded_discarded_msg_count = None;
  }

  pub fn set_max_ttl_expired_discarded_msg_count(&mut self, max_ttl_expired_discarded_msg_count: i64) {
    self.max_ttl_expired_discarded_msg_count = Some(max_ttl_expired_discarded_msg_count);
  }

  pub fn with_max_ttl_expired_discarded_msg_count(mut self, max_ttl_expired_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_ttl_expired_discarded_msg_count = Some(max_ttl_expired_discarded_msg_count);
    self
  }

  pub fn max_ttl_expired_discarded_msg_count(&self) -> Option<&i64> {
    self.max_ttl_expired_discarded_msg_count.as_ref()
  }

  pub fn reset_max_ttl_expired_discarded_msg_count(&mut self) {
    self.max_ttl_expired_discarded_msg_count = None;
  }

  pub fn set_max_ttl_expired_to_dmq_failed_msg_count(&mut self, max_ttl_expired_to_dmq_failed_msg_count: i64) {
    self.max_ttl_expired_to_dmq_failed_msg_count = Some(max_ttl_expired_to_dmq_failed_msg_count);
  }

  pub fn with_max_ttl_expired_to_dmq_failed_msg_count(mut self, max_ttl_expired_to_dmq_failed_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_ttl_expired_to_dmq_failed_msg_count = Some(max_ttl_expired_to_dmq_failed_msg_count);
    self
  }

  pub fn max_ttl_expired_to_dmq_failed_msg_count(&self) -> Option<&i64> {
    self.max_ttl_expired_to_dmq_failed_msg_count.as_ref()
  }

  pub fn reset_max_ttl_expired_to_dmq_failed_msg_count(&mut self) {
    self.max_ttl_expired_to_dmq_failed_msg_count = None;
  }

  pub fn set_max_ttl_expired_to_dmq_msg_count(&mut self, max_ttl_expired_to_dmq_msg_count: i64) {
    self.max_ttl_expired_to_dmq_msg_count = Some(max_ttl_expired_to_dmq_msg_count);
  }

  pub fn with_max_ttl_expired_to_dmq_msg_count(mut self, max_ttl_expired_to_dmq_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.max_ttl_expired_to_dmq_msg_count = Some(max_ttl_expired_to_dmq_msg_count);
    self
  }

  pub fn max_ttl_expired_to_dmq_msg_count(&self) -> Option<&i64> {
    self.max_ttl_expired_to_dmq_msg_count.as_ref()
  }

  pub fn reset_max_ttl_expired_to_dmq_msg_count(&mut self) {
    self.max_ttl_expired_to_dmq_msg_count = None;
  }

  pub fn set_msg_spool_peak_usage(&mut self, msg_spool_peak_usage: i64) {
    self.msg_spool_peak_usage = Some(msg_spool_peak_usage);
  }

  pub fn with_msg_spool_peak_usage(mut self, msg_spool_peak_usage: i64) -> MsgVpnTopicEndpoint {
    self.msg_spool_peak_usage = Some(msg_spool_peak_usage);
    self
  }

  pub fn msg_spool_peak_usage(&self) -> Option<&i64> {
    self.msg_spool_peak_usage.as_ref()
  }

  pub fn reset_msg_spool_peak_usage(&mut self) {
    self.msg_spool_peak_usage = None;
  }

  pub fn set_msg_spool_usage(&mut self, msg_spool_usage: i64) {
    self.msg_spool_usage = Some(msg_spool_usage);
  }

  pub fn with_msg_spool_usage(mut self, msg_spool_usage: i64) -> MsgVpnTopicEndpoint {
    self.msg_spool_usage = Some(msg_spool_usage);
    self
  }

  pub fn msg_spool_usage(&self) -> Option<&i64> {
    self.msg_spool_usage.as_ref()
  }

  pub fn reset_msg_spool_usage(&mut self) {
    self.msg_spool_usage = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnTopicEndpoint {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_network_topic(&mut self, network_topic: String) {
    self.network_topic = Some(network_topic);
  }

  pub fn with_network_topic(mut self, network_topic: String) -> MsgVpnTopicEndpoint {
    self.network_topic = Some(network_topic);
    self
  }

  pub fn network_topic(&self) -> Option<&String> {
    self.network_topic.as_ref()
  }

  pub fn reset_network_topic(&mut self) {
    self.network_topic = None;
  }

  pub fn set_no_local_delivery_discarded_msg_count(&mut self, no_local_delivery_discarded_msg_count: i64) {
    self.no_local_delivery_discarded_msg_count = Some(no_local_delivery_discarded_msg_count);
  }

  pub fn with_no_local_delivery_discarded_msg_count(mut self, no_local_delivery_discarded_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.no_local_delivery_discarded_msg_count = Some(no_local_delivery_discarded_msg_count);
    self
  }

  pub fn no_local_delivery_discarded_msg_count(&self) -> Option<&i64> {
    self.no_local_delivery_discarded_msg_count.as_ref()
  }

  pub fn reset_no_local_delivery_discarded_msg_count(&mut self) {
    self.no_local_delivery_discarded_msg_count = None;
  }

  pub fn set_other_bind_failure_count(&mut self, other_bind_failure_count: i64) {
    self.other_bind_failure_count = Some(other_bind_failure_count);
  }

  pub fn with_other_bind_failure_count(mut self, other_bind_failure_count: i64) -> MsgVpnTopicEndpoint {
    self.other_bind_failure_count = Some(other_bind_failure_count);
    self
  }

  pub fn other_bind_failure_count(&self) -> Option<&i64> {
    self.other_bind_failure_count.as_ref()
  }

  pub fn reset_other_bind_failure_count(&mut self) {
    self.other_bind_failure_count = None;
  }

  pub fn set_owner(&mut self, owner: String) {
    self.owner = Some(owner);
  }

  pub fn with_owner(mut self, owner: String) -> MsgVpnTopicEndpoint {
    self.owner = Some(owner);
    self
  }

  pub fn owner(&self) -> Option<&String> {
    self.owner.as_ref()
  }

  pub fn reset_owner(&mut self) {
    self.owner = None;
  }

  pub fn set_permission(&mut self, permission: String) {
    self.permission = Some(permission);
  }

  pub fn with_permission(mut self, permission: String) -> MsgVpnTopicEndpoint {
    self.permission = Some(permission);
    self
  }

  pub fn permission(&self) -> Option<&String> {
    self.permission.as_ref()
  }

  pub fn reset_permission(&mut self) {
    self.permission = None;
  }

  pub fn set_redelivered_msg_count(&mut self, redelivered_msg_count: i64) {
    self.redelivered_msg_count = Some(redelivered_msg_count);
  }

  pub fn with_redelivered_msg_count(mut self, redelivered_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.redelivered_msg_count = Some(redelivered_msg_count);
    self
  }

  pub fn redelivered_msg_count(&self) -> Option<&i64> {
    self.redelivered_msg_count.as_ref()
  }

  pub fn reset_redelivered_msg_count(&mut self) {
    self.redelivered_msg_count = None;
  }

  pub fn set_reject_low_priority_msg_enabled(&mut self, reject_low_priority_msg_enabled: bool) {
    self.reject_low_priority_msg_enabled = Some(reject_low_priority_msg_enabled);
  }

  pub fn with_reject_low_priority_msg_enabled(mut self, reject_low_priority_msg_enabled: bool) -> MsgVpnTopicEndpoint {
    self.reject_low_priority_msg_enabled = Some(reject_low_priority_msg_enabled);
    self
  }

  pub fn reject_low_priority_msg_enabled(&self) -> Option<&bool> {
    self.reject_low_priority_msg_enabled.as_ref()
  }

  pub fn reset_reject_low_priority_msg_enabled(&mut self) {
    self.reject_low_priority_msg_enabled = None;
  }

  pub fn set_reject_low_priority_msg_limit(&mut self, reject_low_priority_msg_limit: i64) {
    self.reject_low_priority_msg_limit = Some(reject_low_priority_msg_limit);
  }

  pub fn with_reject_low_priority_msg_limit(mut self, reject_low_priority_msg_limit: i64) -> MsgVpnTopicEndpoint {
    self.reject_low_priority_msg_limit = Some(reject_low_priority_msg_limit);
    self
  }

  pub fn reject_low_priority_msg_limit(&self) -> Option<&i64> {
    self.reject_low_priority_msg_limit.as_ref()
  }

  pub fn reset_reject_low_priority_msg_limit(&mut self) {
    self.reject_low_priority_msg_limit = None;
  }

  pub fn set_reject_msg_to_sender_on_discard_behavior(&mut self, reject_msg_to_sender_on_discard_behavior: String) {
    self.reject_msg_to_sender_on_discard_behavior = Some(reject_msg_to_sender_on_discard_behavior);
  }

  pub fn with_reject_msg_to_sender_on_discard_behavior(mut self, reject_msg_to_sender_on_discard_behavior: String) -> MsgVpnTopicEndpoint {
    self.reject_msg_to_sender_on_discard_behavior = Some(reject_msg_to_sender_on_discard_behavior);
    self
  }

  pub fn reject_msg_to_sender_on_discard_behavior(&self) -> Option<&String> {
    self.reject_msg_to_sender_on_discard_behavior.as_ref()
  }

  pub fn reset_reject_msg_to_sender_on_discard_behavior(&mut self) {
    self.reject_msg_to_sender_on_discard_behavior = None;
  }

  pub fn set_replay_failure_count(&mut self, replay_failure_count: i64) {
    self.replay_failure_count = Some(replay_failure_count);
  }

  pub fn with_replay_failure_count(mut self, replay_failure_count: i64) -> MsgVpnTopicEndpoint {
    self.replay_failure_count = Some(replay_failure_count);
    self
  }

  pub fn replay_failure_count(&self) -> Option<&i64> {
    self.replay_failure_count.as_ref()
  }

  pub fn reset_replay_failure_count(&mut self) {
    self.replay_failure_count = None;
  }

  pub fn set_replay_start_count(&mut self, replay_start_count: i64) {
    self.replay_start_count = Some(replay_start_count);
  }

  pub fn with_replay_start_count(mut self, replay_start_count: i64) -> MsgVpnTopicEndpoint {
    self.replay_start_count = Some(replay_start_count);
    self
  }

  pub fn replay_start_count(&self) -> Option<&i64> {
    self.replay_start_count.as_ref()
  }

  pub fn reset_replay_start_count(&mut self) {
    self.replay_start_count = None;
  }

  pub fn set_replay_state(&mut self, replay_state: String) {
    self.replay_state = Some(replay_state);
  }

  pub fn with_replay_state(mut self, replay_state: String) -> MsgVpnTopicEndpoint {
    self.replay_state = Some(replay_state);
    self
  }

  pub fn replay_state(&self) -> Option<&String> {
    self.replay_state.as_ref()
  }

  pub fn reset_replay_state(&mut self) {
    self.replay_state = None;
  }

  pub fn set_replay_success_count(&mut self, replay_success_count: i64) {
    self.replay_success_count = Some(replay_success_count);
  }

  pub fn with_replay_success_count(mut self, replay_success_count: i64) -> MsgVpnTopicEndpoint {
    self.replay_success_count = Some(replay_success_count);
    self
  }

  pub fn replay_success_count(&self) -> Option<&i64> {
    self.replay_success_count.as_ref()
  }

  pub fn reset_replay_success_count(&mut self) {
    self.replay_success_count = None;
  }

  pub fn set_replayed_acked_msg_count(&mut self, replayed_acked_msg_count: i64) {
    self.replayed_acked_msg_count = Some(replayed_acked_msg_count);
  }

  pub fn with_replayed_acked_msg_count(mut self, replayed_acked_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.replayed_acked_msg_count = Some(replayed_acked_msg_count);
    self
  }

  pub fn replayed_acked_msg_count(&self) -> Option<&i64> {
    self.replayed_acked_msg_count.as_ref()
  }

  pub fn reset_replayed_acked_msg_count(&mut self) {
    self.replayed_acked_msg_count = None;
  }

  pub fn set_replayed_tx_msg_count(&mut self, replayed_tx_msg_count: i64) {
    self.replayed_tx_msg_count = Some(replayed_tx_msg_count);
  }

  pub fn with_replayed_tx_msg_count(mut self, replayed_tx_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.replayed_tx_msg_count = Some(replayed_tx_msg_count);
    self
  }

  pub fn replayed_tx_msg_count(&self) -> Option<&i64> {
    self.replayed_tx_msg_count.as_ref()
  }

  pub fn reset_replayed_tx_msg_count(&mut self) {
    self.replayed_tx_msg_count = None;
  }

  pub fn set_replication_active_ack_prop_tx_msg_count(&mut self, replication_active_ack_prop_tx_msg_count: i64) {
    self.replication_active_ack_prop_tx_msg_count = Some(replication_active_ack_prop_tx_msg_count);
  }

  pub fn with_replication_active_ack_prop_tx_msg_count(mut self, replication_active_ack_prop_tx_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.replication_active_ack_prop_tx_msg_count = Some(replication_active_ack_prop_tx_msg_count);
    self
  }

  pub fn replication_active_ack_prop_tx_msg_count(&self) -> Option<&i64> {
    self.replication_active_ack_prop_tx_msg_count.as_ref()
  }

  pub fn reset_replication_active_ack_prop_tx_msg_count(&mut self) {
    self.replication_active_ack_prop_tx_msg_count = None;
  }

  pub fn set_replication_standby_ack_prop_rx_msg_count(&mut self, replication_standby_ack_prop_rx_msg_count: i64) {
    self.replication_standby_ack_prop_rx_msg_count = Some(replication_standby_ack_prop_rx_msg_count);
  }

  pub fn with_replication_standby_ack_prop_rx_msg_count(mut self, replication_standby_ack_prop_rx_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.replication_standby_ack_prop_rx_msg_count = Some(replication_standby_ack_prop_rx_msg_count);
    self
  }

  pub fn replication_standby_ack_prop_rx_msg_count(&self) -> Option<&i64> {
    self.replication_standby_ack_prop_rx_msg_count.as_ref()
  }

  pub fn reset_replication_standby_ack_prop_rx_msg_count(&mut self) {
    self.replication_standby_ack_prop_rx_msg_count = None;
  }

  pub fn set_replication_standby_acked_by_ack_prop_msg_count(&mut self, replication_standby_acked_by_ack_prop_msg_count: i64) {
    self.replication_standby_acked_by_ack_prop_msg_count = Some(replication_standby_acked_by_ack_prop_msg_count);
  }

  pub fn with_replication_standby_acked_by_ack_prop_msg_count(mut self, replication_standby_acked_by_ack_prop_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.replication_standby_acked_by_ack_prop_msg_count = Some(replication_standby_acked_by_ack_prop_msg_count);
    self
  }

  pub fn replication_standby_acked_by_ack_prop_msg_count(&self) -> Option<&i64> {
    self.replication_standby_acked_by_ack_prop_msg_count.as_ref()
  }

  pub fn reset_replication_standby_acked_by_ack_prop_msg_count(&mut self) {
    self.replication_standby_acked_by_ack_prop_msg_count = None;
  }

  pub fn set_replication_standby_rx_msg_count(&mut self, replication_standby_rx_msg_count: i64) {
    self.replication_standby_rx_msg_count = Some(replication_standby_rx_msg_count);
  }

  pub fn with_replication_standby_rx_msg_count(mut self, replication_standby_rx_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.replication_standby_rx_msg_count = Some(replication_standby_rx_msg_count);
    self
  }

  pub fn replication_standby_rx_msg_count(&self) -> Option<&i64> {
    self.replication_standby_rx_msg_count.as_ref()
  }

  pub fn reset_replication_standby_rx_msg_count(&mut self) {
    self.replication_standby_rx_msg_count = None;
  }

  pub fn set_respect_msg_priority_enabled(&mut self, respect_msg_priority_enabled: bool) {
    self.respect_msg_priority_enabled = Some(respect_msg_priority_enabled);
  }

  pub fn with_respect_msg_priority_enabled(mut self, respect_msg_priority_enabled: bool) -> MsgVpnTopicEndpoint {
    self.respect_msg_priority_enabled = Some(respect_msg_priority_enabled);
    self
  }

  pub fn respect_msg_priority_enabled(&self) -> Option<&bool> {
    self.respect_msg_priority_enabled.as_ref()
  }

  pub fn reset_respect_msg_priority_enabled(&mut self) {
    self.respect_msg_priority_enabled = None;
  }

  pub fn set_respect_ttl_enabled(&mut self, respect_ttl_enabled: bool) {
    self.respect_ttl_enabled = Some(respect_ttl_enabled);
  }

  pub fn with_respect_ttl_enabled(mut self, respect_ttl_enabled: bool) -> MsgVpnTopicEndpoint {
    self.respect_ttl_enabled = Some(respect_ttl_enabled);
    self
  }

  pub fn respect_ttl_enabled(&self) -> Option<&bool> {
    self.respect_ttl_enabled.as_ref()
  }

  pub fn reset_respect_ttl_enabled(&mut self) {
    self.respect_ttl_enabled = None;
  }

  pub fn set_rx_byte_rate(&mut self, rx_byte_rate: i32) {
    self.rx_byte_rate = Some(rx_byte_rate);
  }

  pub fn with_rx_byte_rate(mut self, rx_byte_rate: i32) -> MsgVpnTopicEndpoint {
    self.rx_byte_rate = Some(rx_byte_rate);
    self
  }

  pub fn rx_byte_rate(&self) -> Option<&i32> {
    self.rx_byte_rate.as_ref()
  }

  pub fn reset_rx_byte_rate(&mut self) {
    self.rx_byte_rate = None;
  }

  pub fn set_rx_msg_rate(&mut self, rx_msg_rate: i64) {
    self.rx_msg_rate = Some(rx_msg_rate);
  }

  pub fn with_rx_msg_rate(mut self, rx_msg_rate: i64) -> MsgVpnTopicEndpoint {
    self.rx_msg_rate = Some(rx_msg_rate);
    self
  }

  pub fn rx_msg_rate(&self) -> Option<&i64> {
    self.rx_msg_rate.as_ref()
  }

  pub fn reset_rx_msg_rate(&mut self) {
    self.rx_msg_rate = None;
  }

  pub fn set_rx_selector(&mut self, rx_selector: bool) {
    self.rx_selector = Some(rx_selector);
  }

  pub fn with_rx_selector(mut self, rx_selector: bool) -> MsgVpnTopicEndpoint {
    self.rx_selector = Some(rx_selector);
    self
  }

  pub fn rx_selector(&self) -> Option<&bool> {
    self.rx_selector.as_ref()
  }

  pub fn reset_rx_selector(&mut self) {
    self.rx_selector = None;
  }

  pub fn set_selector(&mut self, selector: String) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: String) -> MsgVpnTopicEndpoint {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&String> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

  pub fn set_selector_examined_msg_count(&mut self, selector_examined_msg_count: i64) {
    self.selector_examined_msg_count = Some(selector_examined_msg_count);
  }

  pub fn with_selector_examined_msg_count(mut self, selector_examined_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.selector_examined_msg_count = Some(selector_examined_msg_count);
    self
  }

  pub fn selector_examined_msg_count(&self) -> Option<&i64> {
    self.selector_examined_msg_count.as_ref()
  }

  pub fn reset_selector_examined_msg_count(&mut self) {
    self.selector_examined_msg_count = None;
  }

  pub fn set_selector_matched_msg_count(&mut self, selector_matched_msg_count: i64) {
    self.selector_matched_msg_count = Some(selector_matched_msg_count);
  }

  pub fn with_selector_matched_msg_count(mut self, selector_matched_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.selector_matched_msg_count = Some(selector_matched_msg_count);
    self
  }

  pub fn selector_matched_msg_count(&self) -> Option<&i64> {
    self.selector_matched_msg_count.as_ref()
  }

  pub fn reset_selector_matched_msg_count(&mut self) {
    self.selector_matched_msg_count = None;
  }

  pub fn set_selector_not_matched_msg_count(&mut self, selector_not_matched_msg_count: i64) {
    self.selector_not_matched_msg_count = Some(selector_not_matched_msg_count);
  }

  pub fn with_selector_not_matched_msg_count(mut self, selector_not_matched_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.selector_not_matched_msg_count = Some(selector_not_matched_msg_count);
    self
  }

  pub fn selector_not_matched_msg_count(&self) -> Option<&i64> {
    self.selector_not_matched_msg_count.as_ref()
  }

  pub fn reset_selector_not_matched_msg_count(&mut self) {
    self.selector_not_matched_msg_count = None;
  }

  pub fn set_spooled_byte_count(&mut self, spooled_byte_count: i64) {
    self.spooled_byte_count = Some(spooled_byte_count);
  }

  pub fn with_spooled_byte_count(mut self, spooled_byte_count: i64) -> MsgVpnTopicEndpoint {
    self.spooled_byte_count = Some(spooled_byte_count);
    self
  }

  pub fn spooled_byte_count(&self) -> Option<&i64> {
    self.spooled_byte_count.as_ref()
  }

  pub fn reset_spooled_byte_count(&mut self) {
    self.spooled_byte_count = None;
  }

  pub fn set_spooled_msg_count(&mut self, spooled_msg_count: i64) {
    self.spooled_msg_count = Some(spooled_msg_count);
  }

  pub fn with_spooled_msg_count(mut self, spooled_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.spooled_msg_count = Some(spooled_msg_count);
    self
  }

  pub fn spooled_msg_count(&self) -> Option<&i64> {
    self.spooled_msg_count.as_ref()
  }

  pub fn reset_spooled_msg_count(&mut self) {
    self.spooled_msg_count = None;
  }

  pub fn set_topic_endpoint_name(&mut self, topic_endpoint_name: String) {
    self.topic_endpoint_name = Some(topic_endpoint_name);
  }

  pub fn with_topic_endpoint_name(mut self, topic_endpoint_name: String) -> MsgVpnTopicEndpoint {
    self.topic_endpoint_name = Some(topic_endpoint_name);
    self
  }

  pub fn topic_endpoint_name(&self) -> Option<&String> {
    self.topic_endpoint_name.as_ref()
  }

  pub fn reset_topic_endpoint_name(&mut self) {
    self.topic_endpoint_name = None;
  }

  pub fn set_tx_byte_rate(&mut self, tx_byte_rate: i64) {
    self.tx_byte_rate = Some(tx_byte_rate);
  }

  pub fn with_tx_byte_rate(mut self, tx_byte_rate: i64) -> MsgVpnTopicEndpoint {
    self.tx_byte_rate = Some(tx_byte_rate);
    self
  }

  pub fn tx_byte_rate(&self) -> Option<&i64> {
    self.tx_byte_rate.as_ref()
  }

  pub fn reset_tx_byte_rate(&mut self) {
    self.tx_byte_rate = None;
  }

  pub fn set_tx_msg_rate(&mut self, tx_msg_rate: i64) {
    self.tx_msg_rate = Some(tx_msg_rate);
  }

  pub fn with_tx_msg_rate(mut self, tx_msg_rate: i64) -> MsgVpnTopicEndpoint {
    self.tx_msg_rate = Some(tx_msg_rate);
    self
  }

  pub fn tx_msg_rate(&self) -> Option<&i64> {
    self.tx_msg_rate.as_ref()
  }

  pub fn reset_tx_msg_rate(&mut self) {
    self.tx_msg_rate = None;
  }

  pub fn set_tx_unacked_msg_count(&mut self, tx_unacked_msg_count: i64) {
    self.tx_unacked_msg_count = Some(tx_unacked_msg_count);
  }

  pub fn with_tx_unacked_msg_count(mut self, tx_unacked_msg_count: i64) -> MsgVpnTopicEndpoint {
    self.tx_unacked_msg_count = Some(tx_unacked_msg_count);
    self
  }

  pub fn tx_unacked_msg_count(&self) -> Option<&i64> {
    self.tx_unacked_msg_count.as_ref()
  }

  pub fn reset_tx_unacked_msg_count(&mut self) {
    self.tx_unacked_msg_count = None;
  }

  pub fn set_virtual_router(&mut self, virtual_router: String) {
    self.virtual_router = Some(virtual_router);
  }

  pub fn with_virtual_router(mut self, virtual_router: String) -> MsgVpnTopicEndpoint {
    self.virtual_router = Some(virtual_router);
    self
  }

  pub fn virtual_router(&self) -> Option<&String> {
    self.virtual_router.as_ref()
  }

  pub fn reset_virtual_router(&mut self) {
    self.virtual_router = None;
  }

}



