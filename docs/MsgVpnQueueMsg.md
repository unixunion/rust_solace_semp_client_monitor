# MsgVpnQueueMsg

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachment_size** | **i64** | The size of the Message attachment, in bytes (B). | [optional] [default to null]
**content_size** | **i64** | The size of the Message content, in bytes (B). | [optional] [default to null]
**dmq_eligible** | **bool** | Indicates whether the Message is eligible for the Dead Message Queue (DMQ). | [optional] [default to null]
**expiry_time** | **i32** | The timestamp of when the Message expires. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**msg_id** | **i64** | The identifier (ID) of the Message. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**priority** | **i32** | The priority level of the Message, from 9 (highest) to 0 (lowest). | [optional] [default to null]
**publisher_id** | **i64** | The identifier (ID) of the Message publisher. | [optional] [default to null]
**queue_name** | **String** | The name of the Queue. | [optional] [default to null]
**redelivery_count** | **i32** | The number of times the Message has been redelivered. | [optional] [default to null]
**replicated_mate_msg_id** | **i64** | The Message identifier (ID) on the replication mate. Applicable only to replicated messages. | [optional] [default to null]
**replication_state** | **String** | The replication state of the Message. The allowed values and their meaning are:  &lt;pre&gt; \&quot;replicated\&quot; - The Message is replicated to the remote Message VPN. \&quot;not-replicated\&quot; - The Message is not being replicated to the remote Message VPN. \&quot;pending-replication\&quot; - The Message is queued for replication to the remote Message VPN. &lt;/pre&gt;  | [optional] [default to null]
**spooled_time** | **i32** | The timestamp of when the Message was spooled in the Queue. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**undelivered** | **bool** | Indicates whether delivery of the Message has never been attempted. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


