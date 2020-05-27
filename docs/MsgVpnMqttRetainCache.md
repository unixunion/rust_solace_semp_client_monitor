# MsgVpnMqttRetainCache

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_cache_instance** | **String** | The name of the backup Cache Instance associated with this MQTT Retain Cache. | [optional] [default to null]
**backup_failure_reason** | **String** | The reason why the backup cache associated with this MQTT Retain Cache is operationally down, if any. | [optional] [default to null]
**backup_up** | **bool** | Indicates whether the backup cache associated with this MQTT Retain Cache is operationally up. | [optional] [default to null]
**backup_uptime** | **i32** | The number of seconds that the backup cache associated with this MQTT Retain Cache has been operationally up. | [optional] [default to null]
**cache_cluster** | **String** | The name of the Cache Cluster associated with this MQTT Retain Cache. | [optional] [default to null]
**cache_name** | **String** | The name of the MQTT Retain Cache. | [optional] [default to null]
**distributed_cache** | **String** | The name of the Distributed Cache associated with this MQTT Retain Cache. | [optional] [default to null]
**enabled** | **bool** | Indicates whether this MQTT Retain Cache is enabled. When the cache is disabled, neither retain messages nor retain requests will be delivered by the cache. However, live retain messages will continue to be delivered to currently connected MQTT clients. | [optional] [default to null]
**failure_reason** | **String** | The reason why this MQTT Retain Cache is operationally down, if any. | [optional] [default to null]
**msg_lifetime** | **i64** | The message lifetime, in seconds. If a message remains cached for the duration of its lifetime, the cache will remove the message. A lifetime of 0 results in the message being retained indefinitely. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**primary_cache_instance** | **String** | The name of the primary Cache Instance associated with this MQTT Retain Cache. | [optional] [default to null]
**primary_failure_reason** | **String** | The reason why the primary cache associated with this MQTT Retain Cache is operationally down, if any. | [optional] [default to null]
**primary_up** | **bool** | Indicates whether the primary cache associated with this MQTT Retain Cache is operationally up. | [optional] [default to null]
**primary_uptime** | **i32** | The number of seconds that the primary cache associated with this MQTT Retain Cache has been operationally up. | [optional] [default to null]
**up** | **bool** | Indicates whether this MQTT Retain Cache is operationally up. | [optional] [default to null]
**uptime** | **i32** | The number of seconds that the MQTT Retain Cache has been operationally up. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


