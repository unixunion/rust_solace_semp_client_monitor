# MsgVpnMqttSession

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clean** | **bool** | Indicates whether the Client requested a clean (newly created) MQTT Session when connecting. If not clean (already existing), then previously stored messages for QoS 1 subscriptions are delivered. | [optional] [default to null]
**client_name** | **String** | The name of the MQTT Session Client. | [optional] [default to null]
**counter** | [***::models::MsgVpnMqttSessionCounter**](MsgVpnMqttSessionCounter.md) |  | [optional] [default to null]
**created_by_management** | **bool** | Indicates whether the MQTT Session was created by a Management API. | [optional] [default to null]
**enabled** | **bool** | Indicates whether the MQTT Session is enabled. | [optional] [default to null]
**mqtt_connack_error_tx_count** | **i64** | The number of MQTT connect acknowledgment (CONNACK) refused response packets transmitted to the Client. Available since 2.13. | [optional] [default to null]
**mqtt_connack_tx_count** | **i64** | The number of MQTT connect acknowledgment (CONNACK) accepted response packets transmitted to the Client. Available since 2.13. | [optional] [default to null]
**mqtt_connect_rx_count** | **i32** | The number of MQTT connect (CONNECT) request packets received from the Client. Available since 2.13. | [optional] [default to null]
**mqtt_disconnect_rx_count** | **i64** | The number of MQTT disconnect (DISCONNECT) request packets received from the Client. Available since 2.13. | [optional] [default to null]
**mqtt_pubcomp_tx_count** | **i64** | The number of MQTT publish complete (PUBCOMP) packets transmitted to the Client in response to a PUBREL packet. These packets are the fourth and final packet of a QoS 2 protocol exchange. Available since 2.13. | [optional] [default to null]
**mqtt_publish_qos0_rx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 0 message delivery. Available since 2.13. | [optional] [default to null]
**mqtt_publish_qos0_tx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets transmitted to the Client for QoS 0 message delivery. Available since 2.13. | [optional] [default to null]
**mqtt_publish_qos1_rx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 1 message delivery. Available since 2.13. | [optional] [default to null]
**mqtt_publish_qos1_tx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets transmitted to the Client for QoS 1 message delivery. Available since 2.13. | [optional] [default to null]
**mqtt_publish_qos2_rx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 2 message delivery. Available since 2.13. | [optional] [default to null]
**mqtt_pubrec_tx_count** | **i64** | The number of MQTT publish received (PUBREC) packets transmitted to the Client in response to a PUBLISH packet with QoS 2. These packets are the second packet of a QoS 2 protocol exchange. Available since 2.13. | [optional] [default to null]
**mqtt_pubrel_rx_count** | **i64** | The number of MQTT publish release (PUBREL) packets received from the Client in response to a PUBREC packet. These packets are the third packet of a QoS 2 protocol exchange. Available since 2.13. | [optional] [default to null]
**mqtt_session_client_id** | **String** | The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | [optional] [default to null]
**mqtt_session_virtual_router** | **String** | The virtual router of the MQTT Session. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The MQTT Session belongs to the primary virtual router. \&quot;backup\&quot; - The MQTT Session belongs to the backup virtual router. &lt;/pre&gt;  | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**owner** | **String** | The Client Username which owns the MQTT Session. | [optional] [default to null]
**queue_consumer_ack_propagation_enabled** | **bool** | Indicates whether consumer acknowledgements (ACKs) received on the active replication Message VPN are propagated to the standby replication Message VPN. Available since 2.14. | [optional] [default to null]
**queue_dead_msg_queue** | **String** | The name of the Dead Message Queue (DMQ) used by the MQTT Session Queue. Available since 2.14. | [optional] [default to null]
**queue_event_bind_count_threshold** | [***::models::EventThreshold**](EventThreshold.md) |  | [optional] [default to null]
**queue_event_msg_spool_usage_threshold** | [***::models::EventThreshold**](EventThreshold.md) |  | [optional] [default to null]
**queue_event_reject_low_priority_msg_limit_threshold** | [***::models::EventThreshold**](EventThreshold.md) |  | [optional] [default to null]
**queue_max_bind_count** | **i64** | The maximum number of consumer flows that can bind to the MQTT Session Queue. Available since 2.14. | [optional] [default to null]
**queue_max_delivered_unacked_msgs_per_flow** | **i64** | The maximum number of messages delivered but not acknowledged per flow for the MQTT Session Queue. Available since 2.14. | [optional] [default to null]
**queue_max_msg_size** | **i32** | The maximum message size allowed in the MQTT Session Queue, in bytes (B). Available since 2.14. | [optional] [default to null]
**queue_max_msg_spool_usage** | **i64** | The maximum message spool usage allowed by the MQTT Session Queue, in megabytes (MB). A value of 0 only allows spooling of the last message received and disables quota checking. Available since 2.14. | [optional] [default to null]
**queue_max_redelivery_count** | **i64** | The maximum number of times the MQTT Session Queue will attempt redelivery of a message prior to it being discarded or moved to the DMQ. A value of 0 means to retry forever. Available since 2.14. | [optional] [default to null]
**queue_max_ttl** | **i64** | The maximum time in seconds a message can stay in the MQTT Session Queue when &#x60;queueRespectTtlEnabled&#x60; is &#x60;\&quot;true\&quot;&#x60;. A message expires when the lesser of the sender assigned time-to-live (TTL) in the message and the &#x60;queueMaxTtl&#x60; configured for the MQTT Session Queue, is exceeded. A value of 0 disables expiry. Available since 2.14. | [optional] [default to null]
**queue_name** | **String** | The name of the MQTT Session Queue. | [optional] [default to null]
**queue_reject_low_priority_msg_enabled** | **bool** | Indicates whether to return negative acknowledgements (NACKs) to sending clients on message discards. Note that NACKs cause the message to not be delivered to any destination and Transacted Session commits to fail. Available since 2.14. | [optional] [default to null]
**queue_reject_low_priority_msg_limit** | **i64** | The number of messages of any priority in the MQTT Session Queue above which low priority messages are not admitted but higher priority messages are allowed. Available since 2.14. | [optional] [default to null]
**queue_reject_msg_to_sender_on_discard_behavior** | **String** | Indicates whether negative acknowledgements (NACKs) are returned to sending clients on message discards. Note that NACKs cause the message to not be delivered to any destination and Transacted Session commits to fail. The allowed values and their meaning are:  &lt;pre&gt; \&quot;always\&quot; - Always return a negative acknowledgment (NACK) to the sending client on message discard. \&quot;when-queue-enabled\&quot; - Only return a negative acknowledgment (NACK) to the sending client on message discard when the Queue is enabled. \&quot;never\&quot; - Never return a negative acknowledgment (NACK) to the sending client on message discard. &lt;/pre&gt;  Available since 2.14. | [optional] [default to null]
**queue_respect_ttl_enabled** | **bool** | Indicates whether the time-to-live (TTL) for messages in the MQTT Session Queue is respected. When enabled, expired messages are discarded or moved to the DMQ. Available since 2.14. | [optional] [default to null]
**will** | **bool** | Indicates whether the MQTT Session has the Will message specified by the Client. The Will message is published if the Client disconnects without sending the MQTT DISCONNECT packet. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


