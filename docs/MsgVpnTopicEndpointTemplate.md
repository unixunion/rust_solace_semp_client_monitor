# MsgVpnTopicEndpointTemplate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_type** | **String** | The access type for delivering messages to consumer flows. The allowed values and their meaning are:  &lt;pre&gt; \&quot;exclusive\&quot; - Exclusive delivery of messages to the first bound consumer flow. \&quot;non-exclusive\&quot; - Non-exclusive delivery of messages to all bound consumer flows in a round-robin fashion. &lt;/pre&gt;  | [optional] [default to null]
**consumer_ack_propagation_enabled** | **bool** | Indicates whether the propagation of consumer acknowledgements (ACKs) received on the active replication Message VPN to the standby replication Message VPN is enabled. | [optional] [default to null]
**dead_msg_queue** | **String** | The name of the Dead Message Queue (DMQ). | [optional] [default to null]
**event_bind_count_threshold** | [***::models::EventThreshold**](EventThreshold.md) |  | [optional] [default to null]
**event_msg_spool_usage_threshold** | [***::models::EventThreshold**](EventThreshold.md) |  | [optional] [default to null]
**event_reject_low_priority_msg_limit_threshold** | [***::models::EventThreshold**](EventThreshold.md) |  | [optional] [default to null]
**max_bind_count** | **i64** | The maximum number of consumer flows that can bind. | [optional] [default to null]
**max_delivered_unacked_msgs_per_flow** | **i64** | The maximum number of messages delivered but not acknowledged per flow. | [optional] [default to null]
**max_msg_size** | **i32** | The maximum message size allowed, in bytes (B). | [optional] [default to null]
**max_msg_spool_usage** | **i64** | The maximum message spool usage allowed, in megabytes (MB). A value of 0 only allows spooling of the last message received and disables quota checking. | [optional] [default to null]
**max_redelivery_count** | **i64** | The maximum number of message redelivery attempts that will occur prior to the message being discarded or moved to the DMQ. A value of 0 means to retry forever. | [optional] [default to null]
**max_ttl** | **i64** | The maximum time in seconds a message can stay in the Topic Endpoint when &#x60;respectTtlEnabled&#x60; is &#x60;\&quot;true\&quot;&#x60;. A message expires when the lesser of the sender assigned time-to-live (TTL) in the message and the &#x60;maxTtl&#x60; configured for the Topic Endpoint, is exceeded. A value of 0 disables expiry. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**permission** | **String** | The permission level for all consumers, excluding the owner. The allowed values and their meaning are:  &lt;pre&gt; \&quot;no-access\&quot; - Disallows all access. \&quot;read-only\&quot; - Read-only access to the messages. \&quot;consume\&quot; - Consume (read and remove) messages. \&quot;modify-topic\&quot; - Consume messages or modify the topic/selector. \&quot;delete\&quot; - Consume messages, modify the topic/selector or delete the Client created endpoint altogether. &lt;/pre&gt;  | [optional] [default to null]
**reject_low_priority_msg_enabled** | **bool** | Indicates whether the checking of low priority messages against the &#x60;rejectLowPriorityMsgLimit&#x60; is enabled. | [optional] [default to null]
**reject_low_priority_msg_limit** | **i64** | The number of messages that are permitted before low priority messages are rejected. | [optional] [default to null]
**reject_msg_to_sender_on_discard_behavior** | **String** | Determines when to return negative acknowledgements (NACKs) to sending clients on message discards. Note that NACKs cause the message to not be delivered to any destination and Transacted Session commits to fail. The allowed values and their meaning are:  &lt;pre&gt; \&quot;always\&quot; - Always return a negative acknowledgment (NACK) to the sending client on message discard. \&quot;when-topic-endpoint-enabled\&quot; - Only return a negative acknowledgment (NACK) to the sending client on message discard when the Topic Endpoint is enabled. \&quot;never\&quot; - Never return a negative acknowledgment (NACK) to the sending client on message discard. &lt;/pre&gt;  | [optional] [default to null]
**respect_msg_priority_enabled** | **bool** | Indicates whether message priorities are respected. When enabled, messages are delivered in priority order, from 9 (highest) to 0 (lowest). | [optional] [default to null]
**respect_ttl_enabled** | **bool** | Indicates whether the time-to-live (TTL) for messages is respected. When enabled, expired messages are discarded or moved to the DMQ. | [optional] [default to null]
**topic_endpoint_name_filter** | **String** | A wildcardable pattern used to determine which Topic Endpoints use settings from this Template. Two different wildcards are supported: * and &gt;. Similar to topic filters or subscription patterns, a &gt; matches anything (but only when used at the end), and a * matches zero or more characters but never a slash (/). A &gt; is only a wildcard when used at the end, after a /. A * is only allowed at the end, after a slash (/). | [optional] [default to null]
**topic_endpoint_template_name** | **String** | The name of the Topic Endpoint Template. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


