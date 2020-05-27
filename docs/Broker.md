# Broker

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_client_cert_revocation_check_mode** | **String** | The client certificate revocation checking mode used when a client authenticates with a client certificate. The allowed values and their meaning are:  &lt;pre&gt; \&quot;none\&quot; - Do not perform any certificate revocation checking. \&quot;ocsp\&quot; - Use the Open Certificate Status Protcol (OCSP) for certificate revocation checking. \&quot;crl\&quot; - Use Certificate Revocation Lists (CRL) for certificate revocation checking. \&quot;ocsp-crl\&quot; - Use OCSP first, but if OCSP fails to return an unambiguous result, then check via CRL. &lt;/pre&gt;  | [optional] [default to null]
**average_rx_byte_rate** | **i64** | The one minute average of the message rate received by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**average_rx_compressed_byte_rate** | **i64** | The one minute average of the compressed message rate received by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**average_rx_msg_rate** | **i64** | The one minute average of the message rate received by the Broker, in messages per second (msg/sec). Available since 2.14. | [optional] [default to null]
**average_rx_uncompressed_byte_rate** | **i64** | The one minute average of the uncompressed message rate received by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**average_tx_byte_rate** | **i64** | The one minute average of the message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**average_tx_compressed_byte_rate** | **i64** | The one minute average of the compressed message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**average_tx_msg_rate** | **i64** | The one minute average of the message rate transmitted by the Broker, in messages per second (msg/sec). Available since 2.14. | [optional] [default to null]
**average_tx_uncompressed_byte_rate** | **i64** | The one minute average of the uncompressed message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**rx_byte_count** | **i64** | The amount of messages received from clients by the Broker, in bytes (B). Available since 2.14. | [optional] [default to null]
**rx_byte_rate** | **i64** | The current message rate received by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**rx_compressed_byte_count** | **i64** | The amount of compressed messages received by the Broker, in bytes (B). Available since 2.14. | [optional] [default to null]
**rx_compressed_byte_rate** | **i64** | The current compressed message rate received by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**rx_compression_ratio** | **String** | The compression ratio for messages received by the Broker. Available since 2.14. | [optional] [default to null]
**rx_msg_count** | **i64** | The number of messages received from clients by the Broker. Available since 2.14. | [optional] [default to null]
**rx_msg_rate** | **i64** | The current message rate received by the Broker, in messages per second (msg/sec). Available since 2.14. | [optional] [default to null]
**rx_uncompressed_byte_count** | **i64** | The amount of uncompressed messages received by the Broker, in bytes (B). Available since 2.14. | [optional] [default to null]
**rx_uncompressed_byte_rate** | **i64** | The current uncompressed message rate received by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**tls_block_version10_enabled** | **bool** | Indicates whether incoming TLS version 1.0 connections are blocked. When blocked, existing TLS 1.0 connections from Clients and SEMP users remain connected while new connections are blocked. Note that support for TLS 1.0 will eventually be discontinued, at which time TLS 1.0 connections will be blocked regardless of this setting. | [optional] [default to null]
**tls_block_version11_enabled** | **bool** | Indicates whether TLS version 1.1 connections are blocked. When blocked, all existing incoming and outgoing TLS 1.1 connections with Clients, SEMP users, and LDAP servers remain connected while new connections are blocked. Note that support for TLS 1.1 will eventually be discontinued, at which time TLS 1.1 connections will be blocked regardless of this setting. | [optional] [default to null]
**tls_cipher_suite_management_default_list** | **String** | The colon-separated list of default cipher suites for TLS management connections. | [optional] [default to null]
**tls_cipher_suite_management_list** | **String** | The colon-separated list of cipher suites used for TLS management connections (e.g. SEMP, LDAP). The value \&quot;default\&quot; implies all supported suites ordered from most secure to least secure. | [optional] [default to null]
**tls_cipher_suite_management_supported_list** | **String** | The colon-separated list of supported cipher suites for TLS management connections. | [optional] [default to null]
**tls_cipher_suite_msg_backbone_default_list** | **String** | The colon-separated list of default cipher suites for TLS data connections. | [optional] [default to null]
**tls_cipher_suite_msg_backbone_list** | **String** | The colon-separated list of cipher suites used for TLS data connections (e.g. client pub/sub). The value \&quot;default\&quot; implies all supported suites ordered from most secure to least secure. | [optional] [default to null]
**tls_cipher_suite_msg_backbone_supported_list** | **String** | The colon-separated list of supported cipher suites for TLS data connections. | [optional] [default to null]
**tls_cipher_suite_secure_shell_default_list** | **String** | The colon-separated list of default cipher suites for TLS secure shell connections. | [optional] [default to null]
**tls_cipher_suite_secure_shell_list** | **String** | The colon-separated list of cipher suites used for TLS secure shell connections (e.g. SSH, SFTP, SCP). The value \&quot;default\&quot; implies all supported suites ordered from most secure to least secure. | [optional] [default to null]
**tls_cipher_suite_secure_shell_supported_list** | **String** | The colon-separated list of supported cipher suites for TLS secure shell connections. | [optional] [default to null]
**tls_crime_exploit_protection_enabled** | **bool** | Indicates whether protection against the CRIME exploit is enabled. When enabled, TLS+compressed messaging performance is degraded. This protection should only be disabled if sufficient ACL and authentication features are being employed such that a potential attacker does not have sufficient access to trigger the exploit. | [optional] [default to null]
**tls_ticket_lifetime** | **i32** | The TLS ticket lifetime in seconds. When a client connects with TLS, a session with a session ticket is created using the TLS ticket lifetime which determines how long the client has to resume the session. | [optional] [default to null]
**tls_version_supported_list** | **String** | The comma-separated list of supported TLS versions. | [optional] [default to null]
**tx_byte_count** | **i64** | The amount of messages transmitted to clients by the Broker, in bytes (B). Available since 2.14. | [optional] [default to null]
**tx_byte_rate** | **i64** | The current message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**tx_compressed_byte_count** | **i64** | The amount of compressed messages transmitted by the Broker, in bytes (B). Available since 2.14. | [optional] [default to null]
**tx_compressed_byte_rate** | **i64** | The current compressed message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]
**tx_compression_ratio** | **String** | The compression ratio for messages transmitted by the Broker. Available since 2.14. | [optional] [default to null]
**tx_msg_count** | **i64** | The number of messages transmitted to clients by the Broker. Available since 2.14. | [optional] [default to null]
**tx_msg_rate** | **i64** | The current message rate transmitted by the Broker, in messages per second (msg/sec). Available since 2.14. | [optional] [default to null]
**tx_uncompressed_byte_count** | **i64** | The amount of uncompressed messages transmitted by the Broker, in bytes (B). Available since 2.14. | [optional] [default to null]
**tx_uncompressed_byte_rate** | **i64** | The current uncompressed message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

