# MsgVpnBridge

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bound_to_queue** | **bool** | Indicates whether the Bridge is bound to the queue in the remote Message VPN. | [optional] [default to null]
**bridge_name** | **String** | The name of the Bridge. | [optional] [default to null]
**bridge_virtual_router** | **String** | The virtual router of the Bridge. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The Bridge is used for the primary virtual router. \&quot;backup\&quot; - The Bridge is used for the backup virtual router. \&quot;auto\&quot; - The Bridge is automatically assigned a router. &lt;/pre&gt;  | [optional] [default to null]
**client_name** | **String** | The name of the Client for the Bridge. | [optional] [default to null]
**compressed** | **bool** | Indicates whether messages transmitted over the Bridge are compressed. | [optional] [default to null]
**counter** | [***::models::MsgVpnBridgeCounter**](MsgVpnBridgeCounter.md) |  | [optional] [default to null]
**enabled** | **bool** | Indicates whether the Bridge is enabled. | [optional] [default to null]
**encrypted** | **bool** | Indicates whether messages transmitted over the Bridge are encrypted with TLS. | [optional] [default to null]
**establisher** | **String** | The establisher of the Bridge connection. The allowed values and their meaning are:  &lt;pre&gt; \&quot;local\&quot; - The Bridge connection was established by the local Message VPN. \&quot;remote\&quot; - The Bridge connection was established by the remote Message VPN. &lt;/pre&gt;  | [optional] [default to null]
**inbound_failure_reason** | **String** | The reason for the inbound connection failure from the Bridge. | [optional] [default to null]
**inbound_state** | **String** | The state of the inbound connection from the Bridge. The allowed values and their meaning are:  &lt;pre&gt; \&quot;init\&quot; - The connection is initializing. \&quot;disabled\&quot; - The connection is disabled by configuration. \&quot;enabled\&quot; - The connection is enabled by configuration. \&quot;not-ready\&quot; - The connection is operationally down. \&quot;not-ready-wait-to-connect\&quot; - The connection is down and waiting to connect. \&quot;not-ready-connecting\&quot; - The connection is trying to connect. \&quot;not-ready-handshaking\&quot; - The connection is handshaking. \&quot;not-ready-wait-next\&quot; - The connection failed to connect and is waiting to retry. \&quot;not-ready-wait-reuse\&quot; - The connection is closing in order to reuse an existing connection. \&quot;not-ready-wait-cleanup\&quot; - The connection is closed and cleaning up. \&quot;ready\&quot; - The connection is operationally up. \&quot;ready-subscribing\&quot; - The connection is up and synchronizing subscriptions. \&quot;ready-in-sync\&quot; - The connection is up and subscriptions are synchronized. &lt;/pre&gt;  | [optional] [default to null]
**last_tx_msg_id** | **i64** | The ID of the last message transmitted to the Bridge. | [optional] [default to null]
**local_interface** | **String** | The physical interface on the local Message VPN host for connecting to the remote Message VPN. | [optional] [default to null]
**local_queue_name** | **String** | The name of the local queue for the Bridge. | [optional] [default to null]
**max_ttl** | **i64** | The maximum time-to-live (TTL) in hops. Messages are discarded if their TTL exceeds this value. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**outbound_state** | **String** | The state of the outbound connection to the Bridge. The allowed values and their meaning are:  &lt;pre&gt; \&quot;init\&quot; - The connection is initializing. \&quot;disabled\&quot; - The connection is disabled by configuration. \&quot;enabled\&quot; - The connection is enabled by configuration. \&quot;not-ready\&quot; - The connection is operationally down. \&quot;not-ready-wait-to-connect\&quot; - The connection is down and waiting to connect. \&quot;not-ready-connecting\&quot; - The connection is trying to connect. \&quot;not-ready-handshaking\&quot; - The connection is handshaking. \&quot;not-ready-wait-next\&quot; - The connection failed to connect and is waiting to retry. \&quot;not-ready-wait-reuse\&quot; - The connection is closing in order to reuse an existing connection. \&quot;not-ready-wait-cleanup\&quot; - The connection is closed and cleaning up. \&quot;ready\&quot; - The connection is operationally up. \&quot;ready-subscribing\&quot; - The connection is up and synchronizing subscriptions. \&quot;ready-in-sync\&quot; - The connection is up and subscriptions are synchronized. &lt;/pre&gt;  | [optional] [default to null]
**rate** | [***::models::MsgVpnBridgeRate**](MsgVpnBridgeRate.md) |  | [optional] [default to null]
**remote_address** | **String** | The FQDN or IP address of the remote Message VPN. | [optional] [default to null]
**remote_authentication_basic_client_username** | **String** | The Client Username the Bridge uses to login to the remote Message VPN. | [optional] [default to null]
**remote_authentication_scheme** | **String** | The authentication scheme for the remote Message VPN. The allowed values and their meaning are:  &lt;pre&gt; \&quot;basic\&quot; - Basic Authentication Scheme (via username and password). \&quot;client-certificate\&quot; - Client Certificate Authentication Scheme (via certificate file or content). &lt;/pre&gt;  | [optional] [default to null]
**remote_connection_retry_count** | **i64** | The maximum number of retry attempts to establish a connection to the remote Message VPN. A value of 0 means to retry forever. | [optional] [default to null]
**remote_connection_retry_delay** | **i64** | The number of seconds to delay before retrying to connect to the remote Message VPN. | [optional] [default to null]
**remote_deliver_to_one_priority** | **String** | The priority for deliver-to-one (DTO) messages transmitted from the remote Message VPN. The allowed values and their meaning are:  &lt;pre&gt; \&quot;p1\&quot; - The 1st or highest priority. \&quot;p2\&quot; - The 2nd highest priority. \&quot;p3\&quot; - The 3rd highest priority. \&quot;p4\&quot; - The 4th highest priority. \&quot;da\&quot; - Ignore priority and deliver always. &lt;/pre&gt;  | [optional] [default to null]
**remote_msg_vpn_name** | **String** | The name of the remote Message VPN. | [optional] [default to null]
**remote_router_name** | **String** | The name of the remote router. | [optional] [default to null]
**remote_tx_flow_id** | **i32** | The ID of the transmit flow for the connected remote Message VPN. | [optional] [default to null]
**tls_cipher_suite_list** | **String** | The colon-separated list of cipher-suites supported for TLS connections to the remote Message VPN. The value \&quot;default\&quot; implies all supported suites ordered from most secure to least secure. | [optional] [default to null]
**tls_default_cipher_suite_list** | **bool** | Indicates whether the Bridge is configured to use the default cipher-suite list. | [optional] [default to null]
**ttl_exceeded_event_raised** | **bool** | Indicates whether the TTL (hops) exceeded event has been raised. | [optional] [default to null]
**uptime** | **i64** | The amount of time in seconds since the Bridge connected to the remote Message VPN. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


