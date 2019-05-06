# MsgVpnMqttSession

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clean** | **bool** | Indicates whether the Client requested a clean (newly created) MQTT Session when connecting. If not clean (already existing), then previously stored messages for QoS 1 subscriptions are delivered. | [optional] [default to null]
**client_name** | **String** | The name of the MQTT Session Client. | [optional] [default to null]
**counter** | [***::models::MsgVpnMqttSessionCounter**](MsgVpnMqttSessionCounter.md) |  | [optional] [default to null]
**created_by_management** | **bool** | Indicates whether the MQTT Session was created by a Management API. | [optional] [default to null]
**enabled** | **bool** | Indicates whether the MQTT Session is enabled. | [optional] [default to null]
**mqtt_session_client_id** | **String** | The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | [optional] [default to null]
**mqtt_session_virtual_router** | **String** | The virtual router of the MQTT Session. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The MQTT Session belongs to the primary virtual router. \&quot;backup\&quot; - The MQTT Session belongs to the backup virtual router. &lt;/pre&gt;  | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**owner** | **String** | The Client Username which owns the MQTT Session. | [optional] [default to null]
**queue_name** | **String** | The name of the MQTT Session Queue. | [optional] [default to null]
**will** | **bool** | Indicates whether the MQTT Session has the Will message specified by the Client. The Will message is published if the Client disconnects without sending the MQTT DISCONNECT packet. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


