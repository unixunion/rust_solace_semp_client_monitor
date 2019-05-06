# MsgVpnRestDeliveryPointQueueBinding

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gateway_replace_target_authority_enabled** | **bool** | Indicates whether the authority for the request-target is replaced with that configured for the REST Consumer remote. | [optional] [default to null]
**last_failure_reason** | **String** | The reason for the last REST Delivery Point queue binding failure. | [optional] [default to null]
**last_failure_time** | **i32** | The timestamp of the last REST Delivery Point queue binding failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**post_request_target** | **String** | The POST request-target string being used when sending POST requests to a REST Consumer. | [optional] [default to null]
**queue_binding_name** | **String** | The name of a queue in the Message VPN. | [optional] [default to null]
**rest_delivery_point_name** | **String** | The name of the REST Delivery Point. | [optional] [default to null]
**up** | **bool** | Indicates whether the operational state of the REST Delivery Point queue binding is up. | [optional] [default to null]
**uptime** | **i64** | The amount of time in seconds since the REST Delivery Point queue binding was up. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


