# MsgVpnBridgeRemoteMsgVpn

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bound_to_queue** | **bool** | Indicates whether the Bridge is bound to the queue in the remote Message VPN. | [optional] [default to null]
**bridge_name** | **String** | The name of the Bridge. | [optional] [default to null]
**bridge_virtual_router** | **String** | The virtual router of the Bridge. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The Bridge is used for the primary virtual router. \&quot;backup\&quot; - The Bridge is used for the backup virtual router. \&quot;auto\&quot; - The Bridge is automatically assigned a virtual router at creation, depending on the broker&#39;s active-standby role. &lt;/pre&gt;  | [optional] [default to null]
**client_username** | **String** | The Client Username the Bridge uses to login to the remote Message VPN. This per remote Message VPN value overrides the value provided for the Bridge overall. | [optional] [default to null]
**compressed_data_enabled** | **bool** | Indicates whether data compression is enabled for the remote Message VPN connection. | [optional] [default to null]
**connect_order** | **i32** | The preference given to incoming connections from remote Message VPN hosts, from 1 (highest priority) to 4 (lowest priority). | [optional] [default to null]
**egress_flow_window_size** | **i64** | The number of outstanding guaranteed messages that can be transmitted over the remote Message VPN connection before an acknowledgement is received. | [optional] [default to null]
**enabled** | **bool** | Indicates whether the remote Message VPN is enabled. | [optional] [default to null]
**last_connection_failure_reason** | **String** | The reason for the last connection failure to the remote Message VPN. | [optional] [default to null]
**last_queue_bind_failure_reason** | **String** | The reason for the last queue bind failure in the remote Message VPN. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**queue_binding** | **String** | The queue binding of the Bridge in the remote Message VPN. | [optional] [default to null]
**queue_bound_uptime** | **i32** | The amount of time in seconds since the queue was bound in the remote Message VPN. | [optional] [default to null]
**remote_msg_vpn_interface** | **String** | The physical interface on the local Message VPN host for connecting to the remote Message VPN. By default, an interface is chosen automatically (recommended), but if specified, &#x60;remoteMsgVpnLocation&#x60; must not be a virtual router name. | [optional] [default to null]
**remote_msg_vpn_location** | **String** | The location of the remote Message VPN as either an FQDN with port, IP address with port, or virtual router name (starting with \&quot;v:\&quot;). | [optional] [default to null]
**remote_msg_vpn_name** | **String** | The name of the remote Message VPN. | [optional] [default to null]
**tls_enabled** | **bool** | Indicates whether encryption (TLS) is enabled for the remote Message VPN connection. | [optional] [default to null]
**unidirectional_client_profile** | **String** | The Client Profile for the unidirectional Bridge of the remote Message VPN. The Client Profile must exist in the local Message VPN, and it is used only for the TCP parameters. Note that the default client profile has a TCP maximum window size of 2MB. | [optional] [default to null]
**up** | **bool** | Indicates whether the connection to the remote Message VPN is up. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


