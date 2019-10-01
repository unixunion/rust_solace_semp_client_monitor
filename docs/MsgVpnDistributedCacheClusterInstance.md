# MsgVpnDistributedCacheClusterInstance

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_start_enabled** | **bool** | Indicates whether auto-start for the Cache Instance is enabled, and the Cache Instance will automatically attempt to transition from the Stopped operational state to Up whenever it restarts or reconnects to the message broker. | [optional] [default to null]
**cache_name** | **String** | The name of the Distributed Cache. | [optional] [default to null]
**cluster_name** | **String** | The name of the Cache Cluster. | [optional] [default to null]
**counter** | [***::models::MsgVpnDistributedCacheClusterInstanceCounter**](MsgVpnDistributedCacheClusterInstanceCounter.md) |  | [optional] [default to null]
**enabled** | **bool** | Indicates whether the Cache Instance is enabled. | [optional] [default to null]
**instance_name** | **String** | The name of the Cache Instance. | [optional] [default to null]
**last_registered_time** | **i32** | The timestamp of when the Cache Instance last registered with the message broker. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**last_rx_heartbeat_time** | **i32** | The timestamp of the last heartbeat message received from the Cache Instance. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**last_rx_set_lost_msg_time** | **i32** | The timestamp of the last request for setting the lost message indication received from the Cache Instance. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**last_stopped_reason** | **String** | The reason why the Cache Instance was last stopped. | [optional] [default to null]
**last_stopped_time** | **i32** | The timestamp of when the Cache Instance was last stopped. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**last_tx_clear_lost_msg_time** | **i32** | The timestamp of the last request for clearing the lost message indication transmitted to the Cache Instance. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**memory_usage** | **i32** | The memory usage of the Cache Instance, in megabytes (MB). | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**msgs_lost** | **bool** | Indicates whether one or more messages were lost by the Cache Instance. | [optional] [default to null]
**rate** | [***::models::MsgVpnDistributedCacheClusterInstanceRate**](MsgVpnDistributedCacheClusterInstanceRate.md) |  | [optional] [default to null]
**state** | **String** | The operational state of the Cache Instance. The allowed values and their meaning are:  &lt;pre&gt; \&quot;invalid\&quot; - The Cache Instance state is invalid. \&quot;down\&quot; - The Cache Instance is operationally down. \&quot;stopped\&quot; - The Cache Instance has stopped processing cache requests. \&quot;stopped-lost-msg\&quot; - The Cache Instance has stopped due to a lost message. \&quot;register\&quot; - The Cache Instance is registering with the broker. \&quot;config-sync\&quot; - The Cache Instance is synchronizing its configuration with the broker. \&quot;cluster-sync\&quot; - The Cache Instance is synchronizing its messages with the Cache Cluster. \&quot;up\&quot; - The Cache Instance is operationally up. \&quot;backup\&quot; - The Cache Instance is backing up its messages to disk. \&quot;restore\&quot; - The Cache Instance is restoring its messages from disk. \&quot;not-available\&quot; - The Cache Instance state is not available. &lt;/pre&gt;  | [optional] [default to null]
**stop_on_lost_msg_enabled** | **bool** | Indicates whether stop-on-lost-message is enabled, and the Cache Instance will transition to the Stopped operational state upon losing a message. When Stopped, it cannot accept or respond to cache requests, but continues to cache messages. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


