# DmrCluster

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_basic_enabled** | **bool** | Enable or disable basic authentication for Cluster Links. | [optional] [default to null]
**authentication_basic_type** | **String** | The type of basic authentication to use for Cluster Links. The allowed values and their meaning are:  &lt;pre&gt; \&quot;internal\&quot; - Use locally configured password. \&quot;none\&quot; - No authentication. &lt;/pre&gt;  | [optional] [default to null]
**authentication_client_cert_enabled** | **bool** | Enable or disable client certificate authentication for Cluster Links. | [optional] [default to null]
**direct_only_enabled** | **bool** | Enable or disable direct messaging only. Guaranteed messages will not be transmitted through the cluster. | [optional] [default to null]
**dmr_cluster_name** | **String** | The name of the Cluster. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Cluster. | [optional] [default to null]
**failure_reason** | **String** | The failure reason for the Cluster being down. | [optional] [default to null]
**node_name** | **String** | The name of this node in the Cluster. This is the name that this broker (or redundant group of brokers) is know by to other nodes in the Cluster. The name is chosen automatically to be either this broker&#39;s Router Name or Mate Router Name, depending on which Active Standby Role (primary or backup) this broker plays in its redundancy group. | [optional] [default to null]
**tls_server_cert_enforce_trusted_common_name_enabled** | **bool** | Enable or disable the enforcing of the common-name provided by the remote broker against the list of trusted common-names configured for the Link. If enabled, the certificate&#39;s common-name must match one of the trusted common-names for the Link to be accepted. | [optional] [default to null]
**tls_server_cert_max_chain_depth** | **i64** | The maximum allowed depth of a certificate chain. The depth of a chain is defined as the number of signing CA certificates that are present in the chain back to a trusted self-signed root CA certificate. | [optional] [default to null]
**tls_server_cert_validate_date_enabled** | **bool** | Enable or disable the validation of the \&quot;Not Before\&quot; and \&quot;Not After\&quot; validity dates in the certificate. When disabled, the certificate is accepted even if the certificate is not valid based on these dates. | [optional] [default to null]
**up** | **bool** | Indicates whether the Cluster is operationally up. | [optional] [default to null]
**uptime** | **i64** | The amount of time in seconds since the Cluster was up. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


