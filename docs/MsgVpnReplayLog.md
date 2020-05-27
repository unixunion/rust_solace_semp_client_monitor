# MsgVpnReplayLog

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**egress_enabled** | **bool** | Indicates whether the transmission of messages from the Replay Log is enabled. | [optional] [default to null]
**ingress_enabled** | **bool** | Indicates whether the reception of messages to the Replay Log is enabled. | [optional] [default to null]
**max_spool_usage** | **i64** | The maximum spool usage allowed by the Replay Log, in megabytes (MB). If this limit is exceeded, old messages will be trimmed. | [optional] [default to null]
**msg_spool_usage** | **i64** | The spool usage of the Replay Log, in bytes (B). | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**replay_log_name** | **String** | The name of the Replay Log. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


