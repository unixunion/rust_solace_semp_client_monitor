# DmrClusterLink

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_scheme** | **String** | The authentication scheme to be used by the Link which initiates connections to the remote node. The allowed values and their meaning are:  &lt;pre&gt; \&quot;basic\&quot; - Basic Authentication Scheme (via username and password). \&quot;client-certificate\&quot; - Client Certificate Authentication Scheme (via certificate file or content). &lt;/pre&gt;  | [optional] [default to null]
**client_profile_name** | **String** | The name of the Client Profile used by the Link. | [optional] [default to null]
**client_profile_queue_control1_max_depth** | **i32** | The maximum depth of the \&quot;Control 1\&quot; (C-1) priority queue, in work units. Each work unit is 2048 bytes of message data. | [optional] [default to null]
**client_profile_queue_control1_min_msg_burst** | **i32** | The number of messages that are always allowed entry into the \&quot;Control 1\&quot; (C-1) priority queue, regardless of the &#x60;clientProfileQueueControl1MaxDepth&#x60; value. | [optional] [default to null]
**client_profile_queue_direct1_max_depth** | **i32** | The maximum depth of the \&quot;Direct 1\&quot; (D-1) priority queue, in work units. Each work unit is 2048 bytes of message data. | [optional] [default to null]
**client_profile_queue_direct1_min_msg_burst** | **i32** | The number of messages that are always allowed entry into the \&quot;Direct 1\&quot; (D-1) priority queue, regardless of the &#x60;clientProfileQueueDirect1MaxDepth&#x60; value. | [optional] [default to null]
**client_profile_queue_direct2_max_depth** | **i32** | The maximum depth of the \&quot;Direct 2\&quot; (D-2) priority queue, in work units. Each work unit is 2048 bytes of message data. | [optional] [default to null]
**client_profile_queue_direct2_min_msg_burst** | **i32** | The number of messages that are always allowed entry into the \&quot;Direct 2\&quot; (D-2) priority queue, regardless of the &#x60;clientProfileQueueDirect2MaxDepth&#x60; value. | [optional] [default to null]
**client_profile_queue_direct3_max_depth** | **i32** | The maximum depth of the \&quot;Direct 3\&quot; (D-3) priority queue, in work units. Each work unit is 2048 bytes of message data. | [optional] [default to null]
**client_profile_queue_direct3_min_msg_burst** | **i32** | The number of messages that are always allowed entry into the \&quot;Direct 3\&quot; (D-3) priority queue, regardless of the &#x60;clientProfileQueueDirect3MaxDepth&#x60; value. | [optional] [default to null]
**client_profile_queue_guaranteed1_max_depth** | **i32** | The maximum depth of the \&quot;Guaranteed 1\&quot; (G-1) priority queue, in work units. Each work unit is 2048 bytes of message data. | [optional] [default to null]
**client_profile_queue_guaranteed1_min_msg_burst** | **i32** | The number of messages that are always allowed entry into the \&quot;Guaranteed 1\&quot; (G-3) priority queue, regardless of the &#x60;clientProfileQueueGuaranteed1MaxDepth&#x60; value. | [optional] [default to null]
**client_profile_tcp_congestion_window_size** | **i64** | The TCP initial congestion window size, in multiples of the TCP Maximum Segment Size (MSS). Changing the value from its default of 2 results in non-compliance with RFC 2581. Contact Solace Support before changing this value. | [optional] [default to null]
**client_profile_tcp_keepalive_count** | **i64** | The number of TCP keepalive retransmissions to be carried out before declaring that the remote end is not available. | [optional] [default to null]
**client_profile_tcp_keepalive_idle_time** | **i64** | The amount of time a connection must remain idle before TCP begins sending keepalive probes, in seconds. | [optional] [default to null]
**client_profile_tcp_keepalive_interval** | **i64** | The amount of time between TCP keepalive retransmissions when no acknowledgement is received, in seconds. | [optional] [default to null]
**client_profile_tcp_max_segment_size** | **i64** | The TCP maximum segment size, in kilobytes. Changes are applied to all existing connections. | [optional] [default to null]
**client_profile_tcp_max_window_size** | **i64** | The TCP maximum window size, in kilobytes. Changes are applied to all existing connections. | [optional] [default to null]
**dmr_cluster_name** | **String** | The name of the Cluster. | [optional] [default to null]
**egress_flow_window_size** | **i64** | The number of outstanding guaranteed messages that can be sent over the Link before acknowledgement is received by the sender. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Link. When disabled, subscription sets of this and the remote node are not kept up-to-date, and messages are not exchanged with the remote node. Published guaranteed messages will be queued up for future delivery based on current subscription sets. | [optional] [default to null]
**failure_reason** | **String** | The failure reason for the Link being down. | [optional] [default to null]
**initiator** | **String** | The initiator of the Link&#39;s TCP connections. The allowed values and their meaning are:  &lt;pre&gt; \&quot;lexical\&quot; - The \&quot;higher\&quot; node-name initiates. \&quot;local\&quot; - The local node initiates. \&quot;remote\&quot; - The remote node initiates. &lt;/pre&gt;  | [optional] [default to null]
**queue_dead_msg_queue** | **String** | The name of the Dead Message Queue (DMQ) used by the Queue for discarded messages. | [optional] [default to null]
**queue_event_spool_usage_threshold** | [***::models::EventThreshold**](EventThreshold.md) |  | [optional] [default to null]
**queue_max_delivered_unacked_msgs_per_flow** | **i64** | The maximum number of messages delivered but not acknowledged per flow for the Queue. | [optional] [default to null]
**queue_max_msg_spool_usage** | **i64** | The maximum message spool usage by the Queue (quota), in megabytes (MB). | [optional] [default to null]
**queue_max_redelivery_count** | **i64** | The maximum number of times the Queue will attempt redelivery of a message prior to it being discarded or moved to the DMQ. A value of 0 means to retry forever. | [optional] [default to null]
**queue_max_ttl** | **i64** | The maximum time in seconds a message can stay in the Queue when &#x60;queueRespectTtlEnabled&#x60; is &#x60;true&#x60;. A message expires when the lesser of the sender assigned time-to-live (TTL) in the message and the &#x60;queueMaxTtl&#x60; configured for the Queue, is exceeded. A value of 0 disables expiry. | [optional] [default to null]
**queue_reject_msg_to_sender_on_discard_behavior** | **String** | Determines when to return negative acknowledgements (NACKs) to sending clients on message discards. Note that NACKs cause the message to not be delivered to any destination and Transacted Session commits to fail. The allowed values and their meaning are:  &lt;pre&gt; \&quot;always\&quot; - Always return a negative acknowledgment (NACK) to the sending client on message discard. \&quot;when-queue-enabled\&quot; - Only return a negative acknowledgment (NACK) to the sending client on message discard when the Queue is enabled. \&quot;never\&quot; - Never return a negative acknowledgment (NACK) to the sending client on message discard. &lt;/pre&gt;  | [optional] [default to null]
**queue_respect_ttl_enabled** | **bool** | Enable or disable the respecting of the time-to-live (TTL) for messages in the Queue. When enabled, expired messages are discarded or moved to the DMQ. | [optional] [default to null]
**remote_node_name** | **String** | The name of the node at the remote end of the Link. | [optional] [default to null]
**span** | **String** | The span of the Link, either internal or external. Internal Links connect nodes within the same Cluster. External Links connect nodes within different Clusters. The allowed values and their meaning are:  &lt;pre&gt; \&quot;internal\&quot; - Link to same cluster. \&quot;external\&quot; - Link to other cluster. &lt;/pre&gt;  | [optional] [default to null]
**transport_compressed_enabled** | **bool** | Enable or disable compression on the Link. | [optional] [default to null]
**transport_tls_enabled** | **bool** | Enable or disable encryption on the Link. | [optional] [default to null]
**up** | **bool** | Indicates whether the Link is operationally up. | [optional] [default to null]
**uptime** | **i64** | The amount of time in seconds since the Link was up. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


