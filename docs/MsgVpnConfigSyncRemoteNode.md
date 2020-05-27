# MsgVpnConfigSyncRemoteNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_msg_rx_time** | **i32** | The amount of time in seconds since the last message was received from the config sync table of the remote Message VPN. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**remote_node_name** | **String** | The name of the Config Sync Remote Node. | [optional] [default to null]
**role** | **String** | The role of the config sync table of the remote Message VPN. The allowed values and their meaning are:  &lt;pre&gt; \&quot;unknown\&quot; - The role is unknown. \&quot;primary\&quot; - Acts as the primary source of config data. \&quot;replica\&quot; - Acts as a replica of the primary config data. &lt;/pre&gt;  | [optional] [default to null]
**stale** | **bool** | Indicates whether the config sync table of the remote Message VPN is stale. | [optional] [default to null]
**state** | **String** | The state of the config sync table of the remote Message VPN. The allowed values and their meaning are:  &lt;pre&gt; \&quot;unknown\&quot; - The state is unknown. \&quot;in-sync\&quot; - The config data is synchronized between Message VPNs. \&quot;reconciling\&quot; - The config data is reconciling between Message VPNs. \&quot;blocked\&quot; - The config data is blocked from reconciling due to an error. \&quot;out-of-sync\&quot; - The config data is out of sync between Message VPNs. \&quot;down\&quot; - The state is down due to configuration. &lt;/pre&gt;  | [optional] [default to null]
**time_in_state** | **i32** | The amount of time in seconds the config sync table of the remote Message VPN has been in the current state. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


