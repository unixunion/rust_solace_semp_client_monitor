# MsgVpnReplayLogMsg

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachment_size** | **i64** | The size of the message attachment, in bytes (B). | [optional] [default to null]
**content_size** | **i64** | The size of the message content, in bytes (B). | [optional] [default to null]
**dmq_eligible** | **bool** | Indicates whether the message is eligible for the Dead Message Queue (DMQ). | [optional] [default to null]
**msg_id** | **i64** | The identifier (ID) of the message. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**priority** | **i32** | The priority level of the message. | [optional] [default to null]
**publisher_id** | **i64** | The identifier (ID) of the message publisher. | [optional] [default to null]
**replay_log_name** | **String** | The name of the Replay Log. | [optional] [default to null]
**sequence_number** | **i64** | The sequence number assigned to the message. Applicable only to messages received on sequenced topics. | [optional] [default to null]
**spooled_time** | **i32** | The timestamp of when the message was spooled in the Replay Log. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


