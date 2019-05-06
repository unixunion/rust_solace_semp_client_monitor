# MsgVpnRestDeliveryPointRestConsumer

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_http_basic_username** | **String** | The username that the REST Consumer will use to login to the REST host. | [optional] [default to null]
**authentication_scheme** | **String** | The authentication scheme used by the REST Consumer to login to the REST host. The allowed values and their meaning are:  &lt;pre&gt; \&quot;none\&quot; - Login with no authentication. This may be useful for anonymous connections or when a REST Consumer does not require authentication. \&quot;http-basic\&quot; - Login with a username and optional password according to HTTP Basic authentication as per RFC2616. \&quot;client-certificate\&quot; - Login with a client TLS certificate as per RFC5246. Client certificate authentication is only available on TLS connections. &lt;/pre&gt;  | [optional] [default to null]
**counter** | [***::models::MsgVpnRestDeliveryPointRestConsumerCounter**](MsgVpnRestDeliveryPointRestConsumerCounter.md) |  | [optional] [default to null]
**enabled** | **bool** | Indicates whether the REST Consumer is enabled. | [optional] [default to null]
**last_connection_failure_local_endpoint** | **String** | The local endpoint at the time of the last connection failure. | [optional] [default to null]
**last_connection_failure_reason** | **String** | The reason for the last connection failure between local and remote endpoints. | [optional] [default to null]
**last_connection_failure_remote_endpoint** | **String** | The remote endpoint at the time of the last connection failure. | [optional] [default to null]
**last_connection_failure_time** | **i32** | The timestamp of the last connection failure between local and remote endpoints. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**last_failure_reason** | **String** | The reason for the last REST Consumer failure. | [optional] [default to null]
**last_failure_time** | **i32** | The timestamp of the last REST Consumer failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**local_interface** | **String** | The interface that will be used for all outgoing connections associated with the REST Consumer. When unspecified, an interface is automatically chosen. | [optional] [default to null]
**max_post_wait_time** | **i32** | The maximum amount of time (in seconds) to wait for an HTTP POST response from the REST Consumer. Once this time is exceeded, the TCP connection is reset. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**outgoing_connection_count** | **i32** | The number of concurrent TCP connections open to the REST Consumer. | [optional] [default to null]
**remote_host** | **String** | The IP address or DNS name for the REST Consumer. | [optional] [default to null]
**remote_outgoing_connection_up_count** | **i64** | The number of outgoing connections for the REST Consumer that are up. | [optional] [default to null]
**remote_port** | **i64** | The port associated with the host of the REST Consumer. | [optional] [default to null]
**rest_consumer_name** | **String** | The name of the REST Consumer. | [optional] [default to null]
**rest_delivery_point_name** | **String** | The name of the REST Delivery Point. | [optional] [default to null]
**retry_delay** | **i32** | The number of seconds that must pass before retrying the remote REST Consumer connection. | [optional] [default to null]
**tls_cipher_suite_list** | **String** | The colon-separated list of cipher-suites the REST Consumer uses in its encrypted connection. All supported suites are included by default, from most-secure to least-secure. The REST Consumer should choose the first suite from this list that it supports. | [optional] [default to null]
**tls_enabled** | **bool** | Indicates whether TLS for the REST Consumer is enabled. | [optional] [default to null]
**up** | **bool** | Indicates whether the operational state of the REST Consumer is up. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


