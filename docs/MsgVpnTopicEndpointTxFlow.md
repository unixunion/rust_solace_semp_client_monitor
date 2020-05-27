# MsgVpnTopicEndpointTxFlow

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acked_msg_count** | **i64** | The number of guaranteed messages delivered and acknowledged by the consumer. | [optional] [default to null]
**activity_state** | **String** | The activity state of the Flow. The allowed values and their meaning are:  &lt;pre&gt; \&quot;active-browser\&quot; - The Flow is active as a browser. \&quot;active-consumer\&quot; - The Flow is active as a consumer. \&quot;inactive\&quot; - The Flow is inactive. &lt;/pre&gt;  | [optional] [default to null]
**bind_time** | **i32** | The timestamp of when the Flow bound to the Topic Endpoint. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**client_name** | **String** | The name of the Client. | [optional] [default to null]
**consumer_redelivery_request_allowed** | **bool** | Indicates whether redelivery requests can be received as negative acknowledgements (NACKs) from the consumer. Applicable only to REST consumers. | [optional] [default to null]
**cut_through_acked_msg_count** | **i64** | The number of guaranteed messages that used cut-through delivery and are acknowledged by the consumer. | [optional] [default to null]
**delivery_state** | **String** | The delivery state of the Flow. The allowed values and their meaning are:  &lt;pre&gt; \&quot;closed\&quot; - The Flow is unbound. \&quot;opened\&quot; - The Flow is bound but inactive. \&quot;unbinding\&quot; - The Flow received an unbind request. \&quot;handshaking\&quot; - The Flow is handshaking to become active. \&quot;deliver-cut-through\&quot; - The Flow is streaming messages using direct+guaranteed delivery. \&quot;deliver-from-input-stream\&quot; - The Flow is streaming messages using guaranteed delivery. \&quot;deliver-from-memory\&quot; - The Flow throttled causing message delivery from memory (RAM). \&quot;deliver-from-spool\&quot; - The Flow stalled causing message delivery from spool (ADB or disk). &lt;/pre&gt;  | [optional] [default to null]
**flow_id** | **i64** | The identifier (ID) of the Flow. | [optional] [default to null]
**highest_ack_pending_msg_id** | **i64** | The highest identifier (ID) of message transmitted and waiting for acknowledgement. | [optional] [default to null]
**last_acked_msg_id** | **i64** | The identifier (ID) of the last message transmitted and acknowledged by the consumer. | [optional] [default to null]
**lowest_ack_pending_msg_id** | **i64** | The lowest identifier (ID) of message transmitted and waiting for acknowledgement. | [optional] [default to null]
**max_unacked_msgs_exceeded_msg_count** | **i64** | The number of guaranteed messages that exceeded the maximum number of delivered unacknowledged messages. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**no_local_delivery** | **bool** | Indicates whether not to deliver messages to a consumer that published them. | [optional] [default to null]
**redelivered_msg_count** | **i64** | The number of guaranteed messages that were redelivered. | [optional] [default to null]
**redelivery_request_count** | **i64** | The number of consumer requests via negative acknowledgements (NACKs) to redeliver guaranteed messages. | [optional] [default to null]
**session_name** | **String** | The name of the Transacted Session for the Flow. | [optional] [default to null]
**store_and_forward_acked_msg_count** | **i64** | The number of guaranteed messages that used store and forward delivery and are acknowledged by the consumer. | [optional] [default to null]
**topic_endpoint_name** | **String** | The name of the Topic Endpoint. | [optional] [default to null]
**unacked_msg_count** | **i64** | The number of guaranteed messages delivered but not yet acknowledged by the consumer. | [optional] [default to null]
**used_window_size** | **i32** | The number of guaranteed messages using the available window size. | [optional] [default to null]
**window_closed_count** | **i64** | The number of times the window for guaranteed messages was filled and closed before an acknowledgement was received. | [optional] [default to null]
**window_size** | **i64** | The number of outstanding guaranteed messages that can be transmitted over the Flow before an acknowledgement is received. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


