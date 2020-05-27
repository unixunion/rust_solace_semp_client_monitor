# \AllApi

All URIs are relative to *http://www.solace.com/SEMP/v2/monitor*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_about**](AllApi.md#get_about) | **Get** /about | Get an About object.
[**get_about_api**](AllApi.md#get_about_api) | **Get** /about/api | Get an API Description object.
[**get_broker**](AllApi.md#get_broker) | **Get** / | Get a Broker object.
[**get_cert_authorities**](AllApi.md#get_cert_authorities) | **Get** /certAuthorities | Get a list of Certificate Authority objects.
[**get_dmr_clusters**](AllApi.md#get_dmr_clusters) | **Get** /dmrClusters | Get a list of Cluster objects.
[**get_msg_vpn_acl_profiles**](AllApi.md#get_msg_vpn_acl_profiles) | **Get** /msgVpns/{msgVpnName}/aclProfiles | Get a list of ACL Profile objects.
[**get_msg_vpn_authentication_oauth_providers**](AllApi.md#get_msg_vpn_authentication_oauth_providers) | **Get** /msgVpns/{msgVpnName}/authenticationOauthProviders | Get a list of OAuth Provider objects.
[**get_msg_vpn_authorization_groups**](AllApi.md#get_msg_vpn_authorization_groups) | **Get** /msgVpns/{msgVpnName}/authorizationGroups | Get a list of LDAP Authorization Group objects.
[**get_msg_vpn_bridges**](AllApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Get a list of Bridge objects.
[**get_msg_vpn_client_profiles**](AllApi.md#get_msg_vpn_client_profiles) | **Get** /msgVpns/{msgVpnName}/clientProfiles | Get a list of Client Profile objects.
[**get_msg_vpn_client_usernames**](AllApi.md#get_msg_vpn_client_usernames) | **Get** /msgVpns/{msgVpnName}/clientUsernames | Get a list of Client Username objects.
[**get_msg_vpn_clients**](AllApi.md#get_msg_vpn_clients) | **Get** /msgVpns/{msgVpnName}/clients | Get a list of Client objects.
[**get_msg_vpn_config_sync_remote_nodes**](AllApi.md#get_msg_vpn_config_sync_remote_nodes) | **Get** /msgVpns/{msgVpnName}/configSyncRemoteNodes | Get a list of Config Sync Remote Node objects.
[**get_msg_vpn_distributed_caches**](AllApi.md#get_msg_vpn_distributed_caches) | **Get** /msgVpns/{msgVpnName}/distributedCaches | Get a list of Distributed Cache objects.
[**get_msg_vpn_dmr_bridges**](AllApi.md#get_msg_vpn_dmr_bridges) | **Get** /msgVpns/{msgVpnName}/dmrBridges | Get a list of DMR Bridge objects.
[**get_msg_vpn_jndi_connection_factories**](AllApi.md#get_msg_vpn_jndi_connection_factories) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories | Get a list of JNDI Connection Factory objects.
[**get_msg_vpn_jndi_queues**](AllApi.md#get_msg_vpn_jndi_queues) | **Get** /msgVpns/{msgVpnName}/jndiQueues | Get a list of JNDI Queue objects.
[**get_msg_vpn_jndi_topics**](AllApi.md#get_msg_vpn_jndi_topics) | **Get** /msgVpns/{msgVpnName}/jndiTopics | Get a list of JNDI Topic objects.
[**get_msg_vpn_mqtt_retain_caches**](AllApi.md#get_msg_vpn_mqtt_retain_caches) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches | Get a list of MQTT Retain Cache objects.
[**get_msg_vpn_mqtt_sessions**](AllApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Get a list of MQTT Session objects.
[**get_msg_vpn_queue_templates**](AllApi.md#get_msg_vpn_queue_templates) | **Get** /msgVpns/{msgVpnName}/queueTemplates | Get a list of Queue Template objects.
[**get_msg_vpn_queues**](AllApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Get a list of Queue objects.
[**get_msg_vpn_replay_logs**](AllApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Get a list of Replay Log objects.
[**get_msg_vpn_replicated_topics**](AllApi.md#get_msg_vpn_replicated_topics) | **Get** /msgVpns/{msgVpnName}/replicatedTopics | Get a list of Replicated Topic objects.
[**get_msg_vpn_rest_delivery_points**](AllApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Get a list of REST Delivery Point objects.
[**get_msg_vpn_topic_endpoint_templates**](AllApi.md#get_msg_vpn_topic_endpoint_templates) | **Get** /msgVpns/{msgVpnName}/topicEndpointTemplates | Get a list of Topic Endpoint Template objects.
[**get_msg_vpn_topic_endpoints**](AllApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Get a list of Topic Endpoint objects.
[**get_msg_vpn_transactions**](AllApi.md#get_msg_vpn_transactions) | **Get** /msgVpns/{msgVpnName}/transactions | Get a list of Replicated Local Transaction or XA Transaction objects.
[**get_msg_vpns**](AllApi.md#get_msg_vpns) | **Get** /msgVpns | Get a list of Message VPN objects.


# **get_about**
> ::models::AboutResponse get_about(ctx, optional)
Get an About object.

Get an About object.  This provides metadata about the SEMP API, such as the version of the API supported by the broker.    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.13.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::AboutResponse**](AboutResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_about_api**
> ::models::AboutApiResponse get_about_api(ctx, )
Get an API Description object.

Get an API Description object. The API Description object provides metadata about the SEMP API.  A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AboutApiResponse**](AboutApiResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_broker**
> ::models::BrokerResponse get_broker(ctx, optional)
Get a Broker object.

Get a Broker object.  This object contains global configuration for the message broker.    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation. Requests which include the following attributes require greater access scope/level:   Attribute|Access Scope/Level :---|:---: averageRxByteRate|global/read-only averageRxCompressedByteRate|global/read-only averageRxMsgRate|global/read-only averageRxUncompressedByteRate|global/read-only averageTxByteRate|global/read-only averageTxCompressedByteRate|global/read-only averageTxMsgRate|global/read-only averageTxUncompressedByteRate|global/read-only rxByteCount|global/read-only rxByteRate|global/read-only rxCompressedByteCount|global/read-only rxCompressedByteRate|global/read-only rxCompressionRatio|global/read-only rxMsgCount|global/read-only rxMsgRate|global/read-only rxUncompressedByteCount|global/read-only rxUncompressedByteRate|global/read-only tlsBlockVersion10Enabled|vpn/read-only tlsBlockVersion11Enabled|vpn/read-only tlsCipherSuiteManagementDefaultList|vpn/read-only tlsCipherSuiteManagementList|vpn/read-only tlsCipherSuiteManagementSupportedList|vpn/read-only tlsCipherSuiteMsgBackboneDefaultList|vpn/read-only tlsCipherSuiteMsgBackboneList|vpn/read-only tlsCipherSuiteMsgBackboneSupportedList|vpn/read-only tlsCipherSuiteSecureShellDefaultList|vpn/read-only tlsCipherSuiteSecureShellList|vpn/read-only tlsCipherSuiteSecureShellSupportedList|vpn/read-only tlsCrimeExploitProtectionEnabled|vpn/read-only tlsTicketLifetime|vpn/read-only tlsVersionSupportedList|vpn/read-only txByteCount|global/read-only txByteRate|global/read-only txCompressedByteCount|global/read-only txCompressedByteRate|global/read-only txCompressionRatio|global/read-only txMsgCount|global/read-only txMsgRate|global/read-only txUncompressedByteCount|global/read-only txUncompressedByteRate|global/read-only    This has been available since 2.13.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::BrokerResponse**](BrokerResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cert_authorities**
> ::models::CertAuthoritiesResponse get_cert_authorities(ctx, optional)
Get a list of Certificate Authority objects.

Get a list of Certificate Authority objects.  Clients can authenticate with the message broker over TLS by presenting a valid client certificate. The message broker authenticates the client certificate by constructing a full certificate chain (from the client certificate to intermediate CAs to a configured root CA). The intermediate CAs in this chain can be provided by the client, or configured in the message broker. The root CA must be configured on the message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: certAuthorityName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.13.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::CertAuthoritiesResponse**](CertAuthoritiesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_clusters**
> ::models::DmrClustersResponse get_dmr_clusters(ctx, optional)
Get a list of Cluster objects.

Get a list of Cluster objects.  A Cluster is a provisioned object on a message broker that contains global DMR configuration parameters.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClustersResponse**](DmrClustersResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profiles**
> ::models::MsgVpnAclProfilesResponse get_msg_vpn_acl_profiles(ctx, msg_vpn_name, optional)
Get a list of ACL Profile objects.

Get a list of ACL Profile objects.  An ACL Profile controls whether an authenticated client is permitted to establish a connection with the message broker or permitted to publish and subscribe to specific topics.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfilesResponse**](MsgVpnAclProfilesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_authentication_oauth_providers**
> ::models::MsgVpnAuthenticationOauthProvidersResponse get_msg_vpn_authentication_oauth_providers(ctx, msg_vpn_name, optional)
Get a list of OAuth Provider objects.

Get a list of OAuth Provider objects.  OAuth Providers contain information about the issuer of an OAuth token that is needed to validate the token and derive a client username from it.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| oauthProviderName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.13.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAuthenticationOauthProvidersResponse**](MsgVpnAuthenticationOauthProvidersResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_authorization_groups**
> ::models::MsgVpnAuthorizationGroupsResponse get_msg_vpn_authorization_groups(ctx, msg_vpn_name, optional)
Get a list of LDAP Authorization Group objects.

Get a list of LDAP Authorization Group objects.  To use client authorization groups configured on an external LDAP server to provide client authorizations, LDAP Authorization Group objects must be created on the Message VPN that match the authorization groups provisioned on the LDAP server. These objects must be configured with the client profiles and ACL profiles that will be assigned to the clients that belong to those authorization groups. A newly created group is placed at the end of the group list which is the lowest priority.   Attribute|Identifying|Deprecated :---|:---:|:---: authorizationGroupName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAuthorizationGroupsResponse**](MsgVpnAuthorizationGroupsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridges**
> ::models::MsgVpnBridgesResponse get_msg_vpn_bridges(ctx, msg_vpn_name, optional)
Get a list of Bridge objects.

Get a list of Bridge objects.  Bridges can be used to link two Message VPNs so that messages published to one Message VPN that match the topic subscriptions set for the bridge are also delivered to the linked Message VPN.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| counter.controlRxByteCount||x counter.controlRxMsgCount||x counter.controlTxByteCount||x counter.controlTxMsgCount||x counter.dataRxByteCount||x counter.dataRxMsgCount||x counter.dataTxByteCount||x counter.dataTxMsgCount||x counter.discardedRxMsgCount||x counter.discardedTxMsgCount||x counter.loginRxMsgCount||x counter.loginTxMsgCount||x counter.msgSpoolRxMsgCount||x counter.rxByteCount||x counter.rxMsgCount||x counter.txByteCount||x counter.txMsgCount||x msgVpnName|x| rate.averageRxByteRate||x rate.averageRxMsgRate||x rate.averageTxByteRate||x rate.averageTxMsgRate||x rate.rxByteRate||x rate.rxMsgRate||x rate.txByteRate||x rate.txMsgRate||x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgesResponse**](MsgVpnBridgesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_profiles**
> ::models::MsgVpnClientProfilesResponse get_msg_vpn_client_profiles(ctx, msg_vpn_name, optional)
Get a list of Client Profile objects.

Get a list of Client Profile objects.  Client Profiles are used to assign common configuration properties to clients that have been successfully authorized.   Attribute|Identifying|Deprecated :---|:---:|:---: apiQueueManagementCopyFromOnCreateName||x apiTopicEndpointManagementCopyFromOnCreateName||x clientProfileName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientProfilesResponse**](MsgVpnClientProfilesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_usernames**
> ::models::MsgVpnClientUsernamesResponse get_msg_vpn_client_usernames(ctx, msg_vpn_name, optional)
Get a list of Client Username objects.

Get a list of Client Username objects.  A client is only authorized to connect to a Message VPN that is associated with a Client Username that the client has been assigned.   Attribute|Identifying|Deprecated :---|:---:|:---: clientUsername|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientUsernamesResponse**](MsgVpnClientUsernamesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_clients**
> ::models::MsgVpnClientsResponse get_msg_vpn_clients(ctx, msg_vpn_name, optional)
Get a list of Client objects.

Get a list of Client objects.  Applications or devices that connect to message brokers to send and/or receive messages are represented as Clients.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientsResponse**](MsgVpnClientsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_config_sync_remote_nodes**
> ::models::MsgVpnConfigSyncRemoteNodesResponse get_msg_vpn_config_sync_remote_nodes(ctx, msg_vpn_name, optional)
Get a list of Config Sync Remote Node objects.

Get a list of Config Sync Remote Node objects.  A Config Sync Remote Node object contains information about the status of the table for this Message VPN with respect to a remote node.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnConfigSyncRemoteNodesResponse**](MsgVpnConfigSyncRemoteNodesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_caches**
> ::models::MsgVpnDistributedCachesResponse get_msg_vpn_distributed_caches(ctx, msg_vpn_name, optional)
Get a list of Distributed Cache objects.

Get a list of Distributed Cache objects.  A Distributed Cache is a collection of one or more Cache Clusters that belong to the same Message VPN. Each Cache Cluster in a Distributed Cache is configured to subscribe to a different set of topics. This effectively divides up the configured topic space, to provide scaling to very large topic spaces or very high cached message throughput.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCachesResponse**](MsgVpnDistributedCachesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_dmr_bridges**
> ::models::MsgVpnDmrBridgesResponse get_msg_vpn_dmr_bridges(ctx, msg_vpn_name, optional)
Get a list of DMR Bridge objects.

Get a list of DMR Bridge objects.  A DMR Bridge is required to establish a data channel over a corresponding external link to the remote node for a given Message VPN. Each DMR Bridge identifies which external link the Message VPN should use, and what the name of the equivalent Message VPN at the remote node is.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDmrBridgesResponse**](MsgVpnDmrBridgesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_jndi_connection_factories**
> ::models::MsgVpnJndiConnectionFactoriesResponse get_msg_vpn_jndi_connection_factories(ctx, msg_vpn_name, optional)
Get a list of JNDI Connection Factory objects.

Get a list of JNDI Connection Factory objects.  The message broker provides an internal JNDI store for provisioned Connection Factory objects that clients can access through JNDI lookups.   Attribute|Identifying|Deprecated :---|:---:|:---: connectionFactoryName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnJndiConnectionFactoriesResponse**](MsgVpnJndiConnectionFactoriesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_jndi_queues**
> ::models::MsgVpnJndiQueuesResponse get_msg_vpn_jndi_queues(ctx, msg_vpn_name, optional)
Get a list of JNDI Queue objects.

Get a list of JNDI Queue objects.  The message broker provides an internal JNDI store for provisioned Queue objects that clients can access through JNDI lookups.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnJndiQueuesResponse**](MsgVpnJndiQueuesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_jndi_topics**
> ::models::MsgVpnJndiTopicsResponse get_msg_vpn_jndi_topics(ctx, msg_vpn_name, optional)
Get a list of JNDI Topic objects.

Get a list of JNDI Topic objects.  The message broker provides an internal JNDI store for provisioned Topic objects that clients can access through JNDI lookups.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| topicName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnJndiTopicsResponse**](MsgVpnJndiTopicsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_mqtt_retain_caches**
> ::models::MsgVpnMqttRetainCachesResponse get_msg_vpn_mqtt_retain_caches(ctx, msg_vpn_name, optional)
Get a list of MQTT Retain Cache objects.

Get a list of MQTT Retain Cache objects.  Using MQTT retained messages allows publishing MQTT clients to indicate that a message must be stored for later delivery to subscribing clients when those subscribing clients add subscriptions matching the retained message's topic. An MQTT Retain Cache processes all retained messages for a Message VPN.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnMqttRetainCachesResponse**](MsgVpnMqttRetainCachesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_mqtt_sessions**
> ::models::MsgVpnMqttSessionsResponse get_msg_vpn_mqtt_sessions(ctx, msg_vpn_name, optional)
Get a list of MQTT Session objects.

Get a list of MQTT Session objects.  An MQTT Session object is a virtual representation of an MQTT client connection. An MQTT session holds the state of an MQTT client (that is, it is used to contain a client's QoS 0 and QoS 1 subscription sets and any undelivered QoS 1 messages).   Attribute|Identifying|Deprecated :---|:---:|:---: counter.mqttConnackErrorTxCount||x counter.mqttConnackTxCount||x counter.mqttConnectRxCount||x counter.mqttDisconnectRxCount||x counter.mqttPubcompTxCount||x counter.mqttPublishQos0RxCount||x counter.mqttPublishQos0TxCount||x counter.mqttPublishQos1RxCount||x counter.mqttPublishQos1TxCount||x counter.mqttPublishQos2RxCount||x counter.mqttPubrecTxCount||x counter.mqttPubrelRxCount||x mqttSessionClientId|x| mqttSessionVirtualRouter|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnMqttSessionsResponse**](MsgVpnMqttSessionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_templates**
> ::models::MsgVpnQueueTemplatesResponse get_msg_vpn_queue_templates(ctx, msg_vpn_name, optional)
Get a list of Queue Template objects.

Get a list of Queue Template objects.  A Queue Template provides a mechanism for specifying the initial state for client created queues.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueTemplateName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueTemplatesResponse**](MsgVpnQueueTemplatesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queues**
> ::models::MsgVpnQueuesResponse get_msg_vpn_queues(ctx, msg_vpn_name, optional)
Get a list of Queue objects.

Get a list of Queue objects.  A Queue acts as both a destination that clients can publish messages to, and as an endpoint that clients can bind consumers to and consume messages from.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueuesResponse**](MsgVpnQueuesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_replay_logs**
> ::models::MsgVpnReplayLogsResponse get_msg_vpn_replay_logs(ctx, msg_vpn_name, optional)
Get a list of Replay Log objects.

Get a list of Replay Log objects.  When the Message Replay feature is enabled, message brokers store persistent messages in a Replay Log. These messages are kept until the log is full, after which the oldest messages are removed to free up space for new messages.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| replayLogName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnReplayLogsResponse**](MsgVpnReplayLogsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_replicated_topics**
> ::models::MsgVpnReplicatedTopicsResponse get_msg_vpn_replicated_topics(ctx, msg_vpn_name, optional)
Get a list of Replicated Topic objects.

Get a list of Replicated Topic objects.  To indicate which messages should be replicated between the active and standby site, a Replicated Topic subscription must be configured on a Message VPN. If a published message matches both a replicated topic and an endpoint on the active site, then the message is replicated to the standby site.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| replicatedTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnReplicatedTopicsResponse**](MsgVpnReplicatedTopicsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_points**
> ::models::MsgVpnRestDeliveryPointsResponse get_msg_vpn_rest_delivery_points(ctx, msg_vpn_name, optional)
Get a list of REST Delivery Point objects.

Get a list of REST Delivery Point objects.  A REST Delivery Point manages delivery of messages from queues to a named list of REST Consumers.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointsResponse**](MsgVpnRestDeliveryPointsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_templates**
> ::models::MsgVpnTopicEndpointTemplatesResponse get_msg_vpn_topic_endpoint_templates(ctx, msg_vpn_name, optional)
Get a list of Topic Endpoint Template objects.

Get a list of Topic Endpoint Template objects.  A Topic Endpoint Template provides a mechanism for specifying the initial state for client created topic endpoints.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| topicEndpointTemplateName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointTemplatesResponse**](MsgVpnTopicEndpointTemplatesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoints**
> ::models::MsgVpnTopicEndpointsResponse get_msg_vpn_topic_endpoints(ctx, msg_vpn_name, optional)
Get a list of Topic Endpoint objects.

Get a list of Topic Endpoint objects.  A Topic Endpoint attracts messages published to a topic for which the Topic Endpoint has a matching topic subscription. The topic subscription for the Topic Endpoint is specified in the client request to bind a Flow to that Topic Endpoint. Queues are significantly more flexible than Topic Endpoints and are the recommended approach for most applications. The use of Topic Endpoints should be restricted to JMS applications.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointsResponse**](MsgVpnTopicEndpointsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_transactions**
> ::models::MsgVpnTransactionsResponse get_msg_vpn_transactions(ctx, msg_vpn_name, optional)
Get a list of Replicated Local Transaction or XA Transaction objects.

Get a list of Replicated Local Transaction or XA Transaction objects.  Transactions can be used to group a set of Guaranteed messages to be published or consumed or both as an atomic unit of work.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| xid|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTransactionsResponse**](MsgVpnTransactionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpns**
> ::models::MsgVpnsResponse get_msg_vpns(ctx, optional)
Get a list of Message VPN objects.

Get a list of Message VPN objects.  Message VPNs (Virtual Private Networks) allow for the segregation of topic space and clients. They also group clients connecting to a network of message brokers, such that messages published within a particular group are only visible to that group's clients.   Attribute|Identifying|Deprecated :---|:---:|:---: counter.controlRxByteCount||x counter.controlRxMsgCount||x counter.controlTxByteCount||x counter.controlTxMsgCount||x counter.dataRxByteCount||x counter.dataRxMsgCount||x counter.dataTxByteCount||x counter.dataTxMsgCount||x counter.discardedRxMsgCount||x counter.discardedTxMsgCount||x counter.loginRxMsgCount||x counter.loginTxMsgCount||x counter.msgSpoolRxMsgCount||x counter.msgSpoolTxMsgCount||x counter.tlsRxByteCount||x counter.tlsTxByteCount||x msgVpnName|x| rate.averageRxByteRate||x rate.averageRxMsgRate||x rate.averageTxByteRate||x rate.averageTxMsgRate||x rate.rxByteRate||x rate.rxMsgRate||x rate.tlsAverageRxByteRate||x rate.tlsAverageTxByteRate||x rate.tlsRxByteRate||x rate.tlsTxByteRate||x rate.txByteRate||x rate.txMsgRate||x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnsResponse**](MsgVpnsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

