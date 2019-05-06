# \DefaultApi

All URIs are relative to *http://www.solace.com/SEMP/v2/monitor*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_about_user_msg_vpn**](DefaultApi.md#get_about_user_msg_vpn) | **Get** /about/user/msgVpns/{msgVpnName} | Get a User Message VPN object.
[**get_about_user_msg_vpns**](DefaultApi.md#get_about_user_msg_vpns) | **Get** /about/user/msgVpns | Get a list of User Message VPN objects.
[**get_dmr_cluster**](DefaultApi.md#get_dmr_cluster) | **Get** /dmrClusters/{dmrClusterName} | Get a Cluster object.
[**get_dmr_cluster_link**](DefaultApi.md#get_dmr_cluster_link) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Get a Link object.
[**get_dmr_cluster_link_channel**](DefaultApi.md#get_dmr_cluster_link_channel) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/channels/{msgVpnName} | Get a Cluster Link Channels object.
[**get_dmr_cluster_link_channels**](DefaultApi.md#get_dmr_cluster_link_channels) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/channels | Get a list of Cluster Link Channels objects.
[**get_dmr_cluster_link_remote_address**](DefaultApi.md#get_dmr_cluster_link_remote_address) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses/{remoteAddress} | Get a Remote Address object.
[**get_dmr_cluster_link_remote_addresses**](DefaultApi.md#get_dmr_cluster_link_remote_addresses) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses | Get a list of Remote Address objects.
[**get_dmr_cluster_link_tls_trusted_common_name**](DefaultApi.md#get_dmr_cluster_link_tls_trusted_common_name) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
[**get_dmr_cluster_link_tls_trusted_common_names**](DefaultApi.md#get_dmr_cluster_link_tls_trusted_common_names) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
[**get_dmr_cluster_links**](DefaultApi.md#get_dmr_cluster_links) | **Get** /dmrClusters/{dmrClusterName}/links | Get a list of Link objects.
[**get_dmr_cluster_topology_issue**](DefaultApi.md#get_dmr_cluster_topology_issue) | **Get** /dmrClusters/{dmrClusterName}/topologyIssues/{topologyIssue} | Get a Cluster Topology Issue object.
[**get_dmr_cluster_topology_issues**](DefaultApi.md#get_dmr_cluster_topology_issues) | **Get** /dmrClusters/{dmrClusterName}/topologyIssues | Get a list of Cluster Topology Issue objects.
[**get_msg_vpn**](DefaultApi.md#get_msg_vpn) | **Get** /msgVpns/{msgVpnName} | Get a Message VPN object.
[**get_msg_vpn_acl_profile**](DefaultApi.md#get_msg_vpn_acl_profile) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Get an ACL Profile object.
[**get_msg_vpn_acl_profile_client_connect_exception**](DefaultApi.md#get_msg_vpn_acl_profile_client_connect_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Get a Client Connect Exception object.
[**get_msg_vpn_acl_profile_client_connect_exceptions**](DefaultApi.md#get_msg_vpn_acl_profile_client_connect_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Get a list of Client Connect Exception objects.
[**get_msg_vpn_acl_profile_publish_exception**](DefaultApi.md#get_msg_vpn_acl_profile_publish_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Get a Publish Topic Exception object.
[**get_msg_vpn_acl_profile_publish_exceptions**](DefaultApi.md#get_msg_vpn_acl_profile_publish_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Get a list of Publish Topic Exception objects.
[**get_msg_vpn_acl_profile_subscribe_exception**](DefaultApi.md#get_msg_vpn_acl_profile_subscribe_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Get a Subscribe Topic Exception object.
[**get_msg_vpn_acl_profile_subscribe_exceptions**](DefaultApi.md#get_msg_vpn_acl_profile_subscribe_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Get a list of Subscribe Topic Exception objects.
[**get_msg_vpn_authorization_group**](DefaultApi.md#get_msg_vpn_authorization_group) | **Get** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Get an LDAP Authorization Group object.
[**get_msg_vpn_bridge**](DefaultApi.md#get_msg_vpn_bridge) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Get a Bridge object.
[**get_msg_vpn_bridge_local_subscription**](DefaultApi.md#get_msg_vpn_bridge_local_subscription) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/localSubscriptions/{localSubscriptionTopic} | Get a Bridge Local Subscriptions object.
[**get_msg_vpn_bridge_local_subscriptions**](DefaultApi.md#get_msg_vpn_bridge_local_subscriptions) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/localSubscriptions | Get a list of Bridge Local Subscriptions objects.
[**get_msg_vpn_bridge_remote_msg_vpn**](DefaultApi.md#get_msg_vpn_bridge_remote_msg_vpn) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Get a Remote Message VPN object.
[**get_msg_vpn_bridge_remote_msg_vpns**](DefaultApi.md#get_msg_vpn_bridge_remote_msg_vpns) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Get a list of Remote Message VPN objects.
[**get_msg_vpn_bridge_remote_subscription**](DefaultApi.md#get_msg_vpn_bridge_remote_subscription) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Get a Remote Subscription object.
[**get_msg_vpn_bridge_remote_subscriptions**](DefaultApi.md#get_msg_vpn_bridge_remote_subscriptions) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Get a list of Remote Subscription objects.
[**get_msg_vpn_bridge_tls_trusted_common_name**](DefaultApi.md#get_msg_vpn_bridge_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
[**get_msg_vpn_bridge_tls_trusted_common_names**](DefaultApi.md#get_msg_vpn_bridge_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
[**get_msg_vpn_client_profile**](DefaultApi.md#get_msg_vpn_client_profile) | **Get** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Get a Client Profile object.
[**get_msg_vpn_client_username**](DefaultApi.md#get_msg_vpn_client_username) | **Get** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Get a Client Username object.
[**get_msg_vpn_distributed_cache**](DefaultApi.md#get_msg_vpn_distributed_cache) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Get a Distributed Cache object.
[**get_msg_vpn_distributed_cache_cluster**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Get a Cache Cluster object.
[**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Get a Home Cache Cluster object.
[**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Get a Topic Prefix object.
[**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Get a list of Topic Prefix objects.
[**get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Get a list of Home Cache Cluster objects.
[**get_msg_vpn_distributed_cache_cluster_instance**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instance) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Get a Cache Instance object.
[**get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_cluster**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/remoteGlobalCachingHomeClusters/{homeClusterName} | Get a Remote Home Cache Cluster object.
[**get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_clusters**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/remoteGlobalCachingHomeClusters | Get a list of Remote Home Cache Cluster objects.
[**get_msg_vpn_distributed_cache_cluster_instance_remote_topic**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instance_remote_topic) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/remoteTopics/{topic} | Get a Remote Topic object.
[**get_msg_vpn_distributed_cache_cluster_instance_remote_topics**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instance_remote_topics) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/remoteTopics | Get a list of Remote Topic objects.
[**get_msg_vpn_distributed_cache_cluster_instances**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instances) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Get a list of Cache Instance objects.
[**get_msg_vpn_distributed_cache_cluster_topic**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_topic) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Get a Topic object.
[**get_msg_vpn_distributed_cache_cluster_topics**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_topics) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Get a list of Topic objects.
[**get_msg_vpn_distributed_cache_clusters**](DefaultApi.md#get_msg_vpn_distributed_cache_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Get a list of Cache Cluster objects.
[**get_msg_vpn_dmr_bridge**](DefaultApi.md#get_msg_vpn_dmr_bridge) | **Get** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Get a DMR Bridge object.
[**get_msg_vpn_jndi_connection_factory**](DefaultApi.md#get_msg_vpn_jndi_connection_factory) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Get a JNDI Connection Factory object.
[**get_msg_vpn_jndi_queue**](DefaultApi.md#get_msg_vpn_jndi_queue) | **Get** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Get a JNDI Queue object.
[**get_msg_vpn_jndi_topic**](DefaultApi.md#get_msg_vpn_jndi_topic) | **Get** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Get a JNDI Topic object.
[**get_msg_vpn_mqtt_retain_cache**](DefaultApi.md#get_msg_vpn_mqtt_retain_cache) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Get an MQTT Retain Cache object.
[**get_msg_vpn_mqtt_session**](DefaultApi.md#get_msg_vpn_mqtt_session) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Get an MQTT Session object.
[**get_msg_vpn_mqtt_session_subscription**](DefaultApi.md#get_msg_vpn_mqtt_session_subscription) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Get a Subscription object.
[**get_msg_vpn_mqtt_session_subscriptions**](DefaultApi.md#get_msg_vpn_mqtt_session_subscriptions) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Get a list of Subscription objects.
[**get_msg_vpn_replay_log**](DefaultApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Get a Replay Log object.
[**get_msg_vpn_replay_log_msg**](DefaultApi.md#get_msg_vpn_replay_log_msg) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName}/msgs/{msgId} | Get a Message object.
[**get_msg_vpn_replay_log_msgs**](DefaultApi.md#get_msg_vpn_replay_log_msgs) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName}/msgs | Get a list of Message objects.
[**get_msg_vpn_rest_delivery_point**](DefaultApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Get a REST Delivery Point object.
[**get_msg_vpn_rest_delivery_point_queue_binding**](DefaultApi.md#get_msg_vpn_rest_delivery_point_queue_binding) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Get a Queue Binding object.
[**get_msg_vpn_rest_delivery_point_queue_bindings**](DefaultApi.md#get_msg_vpn_rest_delivery_point_queue_bindings) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Get a list of Queue Binding objects.
[**get_msg_vpn_rest_delivery_point_rest_consumer**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Get a REST Consumer object.
[**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
[**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
[**get_msg_vpn_rest_delivery_point_rest_consumers**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Get a list of REST Consumer objects.


# **get_about_user_msg_vpn**
> ::models::AboutUserMsgVpnResponse get_about_user_msg_vpn(ctx, msg_vpn_name, optional)
Get a User Message VPN object.

Get a User Message VPN object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the User Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the User Message VPN. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::AboutUserMsgVpnResponse**](AboutUserMsgVpnResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_about_user_msg_vpns**
> ::models::AboutUserMsgVpnsResponse get_about_user_msg_vpns(ctx, optional)
Get a list of User Message VPN objects.

Get a list of User Message VPN objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.11.

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

[**::models::AboutUserMsgVpnsResponse**](AboutUserMsgVpnsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster**
> ::models::DmrClusterResponse get_dmr_cluster(ctx, dmr_cluster_name, optional)
Get a Cluster object.

Get a Cluster object.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterResponse**](DmrClusterResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_link**
> ::models::DmrClusterLinkResponse get_dmr_cluster_link(ctx, dmr_cluster_name, remote_node_name, optional)
Get a Link object.

Get a Link object.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
  **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterLinkResponse**](DmrClusterLinkResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_link_channel**
> ::models::DmrClusterLinkChannelResponse get_dmr_cluster_link_channel(ctx, dmr_cluster_name, remote_node_name, msg_vpn_name, optional)
Get a Cluster Link Channels object.

Get a Cluster Link Channels object.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
  **remote_node_name** | **String**| The remoteNodeName of the Link. | 
  **msg_vpn_name** | **String**| The msgVpnName of the Cluster Link Channels. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **msg_vpn_name** | **String**| The msgVpnName of the Cluster Link Channels. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterLinkChannelResponse**](DmrClusterLinkChannelResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_link_channels**
> ::models::DmrClusterLinkChannelsResponse get_dmr_cluster_link_channels(ctx, dmr_cluster_name, remote_node_name, optional)
Get a list of Cluster Link Channels objects.

Get a list of Cluster Link Channels objects.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
  **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterLinkChannelsResponse**](DmrClusterLinkChannelsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_link_remote_address**
> ::models::DmrClusterLinkRemoteAddressResponse get_dmr_cluster_link_remote_address(ctx, dmr_cluster_name, remote_node_name, remote_address, optional)
Get a Remote Address object.

Get a Remote Address object.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteAddress|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
  **remote_node_name** | **String**| The remoteNodeName of the Link. | 
  **remote_address** | **String**| The remoteAddress of the Remote Address. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **remote_address** | **String**| The remoteAddress of the Remote Address. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterLinkRemoteAddressResponse**](DmrClusterLinkRemoteAddressResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_link_remote_addresses**
> ::models::DmrClusterLinkRemoteAddressesResponse get_dmr_cluster_link_remote_addresses(ctx, dmr_cluster_name, remote_node_name, optional)
Get a list of Remote Address objects.

Get a list of Remote Address objects.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteAddress|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
  **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterLinkRemoteAddressesResponse**](DmrClusterLinkRemoteAddressesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_link_tls_trusted_common_name**
> ::models::DmrClusterLinkTlsTrustedCommonNameResponse get_dmr_cluster_link_tls_trusted_common_name(ctx, dmr_cluster_name, remote_node_name, tls_trusted_common_name, optional)
Get a Trusted Common Name object.

Get a Trusted Common Name object.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteNodeName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
  **remote_node_name** | **String**| The remoteNodeName of the Link. | 
  **tls_trusted_common_name** | **String**| The tlsTrustedCommonName of the Trusted Common Name. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **tls_trusted_common_name** | **String**| The tlsTrustedCommonName of the Trusted Common Name. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterLinkTlsTrustedCommonNameResponse**](DmrClusterLinkTlsTrustedCommonNameResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_link_tls_trusted_common_names**
> ::models::DmrClusterLinkTlsTrustedCommonNamesResponse get_dmr_cluster_link_tls_trusted_common_names(ctx, dmr_cluster_name, remote_node_name, optional)
Get a list of Trusted Common Name objects.

Get a list of Trusted Common Name objects.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteNodeName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
  **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **remote_node_name** | **String**| The remoteNodeName of the Link. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterLinkTlsTrustedCommonNamesResponse**](DmrClusterLinkTlsTrustedCommonNamesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_links**
> ::models::DmrClusterLinksResponse get_dmr_cluster_links(ctx, dmr_cluster_name, optional)
Get a list of Link objects.

Get a list of Link objects.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterLinksResponse**](DmrClusterLinksResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_topology_issue**
> ::models::DmrClusterTopologyIssueResponse get_dmr_cluster_topology_issue(ctx, dmr_cluster_name, topology_issue, optional)
Get a Cluster Topology Issue object.

Get a Cluster Topology Issue object.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| topologyIssue|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
  **topology_issue** | **String**| The topologyIssue of the Cluster Topology Issue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **topology_issue** | **String**| The topologyIssue of the Cluster Topology Issue. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterTopologyIssueResponse**](DmrClusterTopologyIssueResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster_topology_issues**
> ::models::DmrClusterTopologyIssuesResponse get_dmr_cluster_topology_issues(ctx, dmr_cluster_name, optional)
Get a list of Cluster Topology Issue objects.

Get a list of Cluster Topology Issue objects.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| topologyIssue|x|    A SEMP client authorized with a minimum access scope/level of \"global/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The dmrClusterName of the Cluster. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::DmrClusterTopologyIssuesResponse**](DmrClusterTopologyIssuesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn**
> ::models::MsgVpnResponse get_msg_vpn(ctx, msg_vpn_name, optional)
Get a Message VPN object.

Get a Message VPN object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnResponse**](MsgVpnResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile**
> ::models::MsgVpnAclProfileResponse get_msg_vpn_acl_profile(ctx, msg_vpn_name, acl_profile_name, optional)
Get an ACL Profile object.

Get an ACL Profile object.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileResponse**](MsgVpnAclProfileResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_client_connect_exception**
> ::models::MsgVpnAclProfileClientConnectExceptionResponse get_msg_vpn_acl_profile_client_connect_exception(ctx, msg_vpn_name, acl_profile_name, client_connect_exception_address, optional)
Get a Client Connect Exception object.

Get a Client Connect Exception object.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| clientConnectExceptionAddress|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
  **client_connect_exception_address** | **String**| The clientConnectExceptionAddress of the Client Connect Exception. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **client_connect_exception_address** | **String**| The clientConnectExceptionAddress of the Client Connect Exception. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileClientConnectExceptionResponse**](MsgVpnAclProfileClientConnectExceptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_client_connect_exceptions**
> ::models::MsgVpnAclProfileClientConnectExceptionsResponse get_msg_vpn_acl_profile_client_connect_exceptions(ctx, msg_vpn_name, acl_profile_name, optional)
Get a list of Client Connect Exception objects.

Get a list of Client Connect Exception objects.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| clientConnectExceptionAddress|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileClientConnectExceptionsResponse**](MsgVpnAclProfileClientConnectExceptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_publish_exception**
> ::models::MsgVpnAclProfilePublishExceptionResponse get_msg_vpn_acl_profile_publish_exception(ctx, msg_vpn_name, acl_profile_name, topic_syntax, publish_exception_topic, optional)
Get a Publish Topic Exception object.

Get a Publish Topic Exception object.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| publishExceptionTopic|x| topicSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
  **topic_syntax** | **String**| The topicSyntax of the Publish Topic Exception. | 
  **publish_exception_topic** | **String**| The publishExceptionTopic of the Publish Topic Exception. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **topic_syntax** | **String**| The topicSyntax of the Publish Topic Exception. | 
 **publish_exception_topic** | **String**| The publishExceptionTopic of the Publish Topic Exception. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfilePublishExceptionResponse**](MsgVpnAclProfilePublishExceptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_publish_exceptions**
> ::models::MsgVpnAclProfilePublishExceptionsResponse get_msg_vpn_acl_profile_publish_exceptions(ctx, msg_vpn_name, acl_profile_name, optional)
Get a list of Publish Topic Exception objects.

Get a list of Publish Topic Exception objects.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| publishExceptionTopic|x| topicSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfilePublishExceptionsResponse**](MsgVpnAclProfilePublishExceptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_subscribe_exception**
> ::models::MsgVpnAclProfileSubscribeExceptionResponse get_msg_vpn_acl_profile_subscribe_exception(ctx, msg_vpn_name, acl_profile_name, topic_syntax, subscribe_exception_topic, optional)
Get a Subscribe Topic Exception object.

Get a Subscribe Topic Exception object.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| subscribeExceptionTopic|x| topicSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
  **topic_syntax** | **String**| The topicSyntax of the Subscribe Topic Exception. | 
  **subscribe_exception_topic** | **String**| The subscribeExceptionTopic of the Subscribe Topic Exception. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **topic_syntax** | **String**| The topicSyntax of the Subscribe Topic Exception. | 
 **subscribe_exception_topic** | **String**| The subscribeExceptionTopic of the Subscribe Topic Exception. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileSubscribeExceptionResponse**](MsgVpnAclProfileSubscribeExceptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_subscribe_exceptions**
> ::models::MsgVpnAclProfileSubscribeExceptionsResponse get_msg_vpn_acl_profile_subscribe_exceptions(ctx, msg_vpn_name, acl_profile_name, optional)
Get a list of Subscribe Topic Exception objects.

Get a list of Subscribe Topic Exception objects.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| subscribeExceptionTopic|x| topicSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **acl_profile_name** | **String**| The aclProfileName of the ACL Profile. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileSubscribeExceptionsResponse**](MsgVpnAclProfileSubscribeExceptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_authorization_group**
> ::models::MsgVpnAuthorizationGroupResponse get_msg_vpn_authorization_group(ctx, msg_vpn_name, authorization_group_name, optional)
Get an LDAP Authorization Group object.

Get an LDAP Authorization Group object.   Attribute|Identifying|Deprecated :---|:---:|:---: authorizationGroupName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **authorization_group_name** | **String**| The authorizationGroupName of the LDAP Authorization Group. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **authorization_group_name** | **String**| The authorizationGroupName of the LDAP Authorization Group. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAuthorizationGroupResponse**](MsgVpnAuthorizationGroupResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge**
> ::models::MsgVpnBridgeResponse get_msg_vpn_bridge(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, optional)
Get a Bridge object.

Get a Bridge object.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeResponse**](MsgVpnBridgeResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge_local_subscription**
> ::models::MsgVpnBridgeLocalSubscriptionResponse get_msg_vpn_bridge_local_subscription(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, local_subscription_topic, optional)
Get a Bridge Local Subscriptions object.

Get a Bridge Local Subscriptions object.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| localSubscriptionTopic|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
  **local_subscription_topic** | **String**| The localSubscriptionTopic of the Bridge Local Subscriptions. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **local_subscription_topic** | **String**| The localSubscriptionTopic of the Bridge Local Subscriptions. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeLocalSubscriptionResponse**](MsgVpnBridgeLocalSubscriptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge_local_subscriptions**
> ::models::MsgVpnBridgeLocalSubscriptionsResponse get_msg_vpn_bridge_local_subscriptions(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, optional)
Get a list of Bridge Local Subscriptions objects.

Get a list of Bridge Local Subscriptions objects.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| localSubscriptionTopic|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeLocalSubscriptionsResponse**](MsgVpnBridgeLocalSubscriptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge_remote_msg_vpn**
> ::models::MsgVpnBridgeRemoteMsgVpnResponse get_msg_vpn_bridge_remote_msg_vpn(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, remote_msg_vpn_name, remote_msg_vpn_location, remote_msg_vpn_interface, optional)
Get a Remote Message VPN object.

Get a Remote Message VPN object.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| remoteMsgVpnInterface|x| remoteMsgVpnLocation|x| remoteMsgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
  **remote_msg_vpn_name** | **String**| The remoteMsgVpnName of the Remote Message VPN. | 
  **remote_msg_vpn_location** | **String**| The remoteMsgVpnLocation of the Remote Message VPN. | 
  **remote_msg_vpn_interface** | **String**| The remoteMsgVpnInterface of the Remote Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **remote_msg_vpn_name** | **String**| The remoteMsgVpnName of the Remote Message VPN. | 
 **remote_msg_vpn_location** | **String**| The remoteMsgVpnLocation of the Remote Message VPN. | 
 **remote_msg_vpn_interface** | **String**| The remoteMsgVpnInterface of the Remote Message VPN. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeRemoteMsgVpnResponse**](MsgVpnBridgeRemoteMsgVpnResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge_remote_msg_vpns**
> ::models::MsgVpnBridgeRemoteMsgVpnsResponse get_msg_vpn_bridge_remote_msg_vpns(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, optional)
Get a list of Remote Message VPN objects.

Get a list of Remote Message VPN objects.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| remoteMsgVpnInterface|x| remoteMsgVpnLocation|x| remoteMsgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeRemoteMsgVpnsResponse**](MsgVpnBridgeRemoteMsgVpnsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge_remote_subscription**
> ::models::MsgVpnBridgeRemoteSubscriptionResponse get_msg_vpn_bridge_remote_subscription(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, remote_subscription_topic, optional)
Get a Remote Subscription object.

Get a Remote Subscription object.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| remoteSubscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
  **remote_subscription_topic** | **String**| The remoteSubscriptionTopic of the Remote Subscription. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **remote_subscription_topic** | **String**| The remoteSubscriptionTopic of the Remote Subscription. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeRemoteSubscriptionResponse**](MsgVpnBridgeRemoteSubscriptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge_remote_subscriptions**
> ::models::MsgVpnBridgeRemoteSubscriptionsResponse get_msg_vpn_bridge_remote_subscriptions(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, optional)
Get a list of Remote Subscription objects.

Get a list of Remote Subscription objects.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| remoteSubscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeRemoteSubscriptionsResponse**](MsgVpnBridgeRemoteSubscriptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge_tls_trusted_common_name**
> ::models::MsgVpnBridgeTlsTrustedCommonNameResponse get_msg_vpn_bridge_tls_trusted_common_name(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, tls_trusted_common_name, optional)
Get a Trusted Common Name object.

Get a Trusted Common Name object.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
  **tls_trusted_common_name** | **String**| The tlsTrustedCommonName of the Trusted Common Name. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **tls_trusted_common_name** | **String**| The tlsTrustedCommonName of the Trusted Common Name. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeTlsTrustedCommonNameResponse**](MsgVpnBridgeTlsTrustedCommonNameResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge_tls_trusted_common_names**
> ::models::MsgVpnBridgeTlsTrustedCommonNamesResponse get_msg_vpn_bridge_tls_trusted_common_names(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, optional)
Get a list of Trusted Common Name objects.

Get a list of Trusted Common Name objects.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeTlsTrustedCommonNamesResponse**](MsgVpnBridgeTlsTrustedCommonNamesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_profile**
> ::models::MsgVpnClientProfileResponse get_msg_vpn_client_profile(ctx, msg_vpn_name, client_profile_name, optional)
Get a Client Profile object.

Get a Client Profile object.   Attribute|Identifying|Deprecated :---|:---:|:---: clientProfileName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **client_profile_name** | **String**| The clientProfileName of the Client Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **client_profile_name** | **String**| The clientProfileName of the Client Profile. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientProfileResponse**](MsgVpnClientProfileResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_username**
> ::models::MsgVpnClientUsernameResponse get_msg_vpn_client_username(ctx, msg_vpn_name, client_username, optional)
Get a Client Username object.

Get a Client Username object.   Attribute|Identifying|Deprecated :---|:---:|:---: clientUsername|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **client_username** | **String**| The clientUsername of the Client Username. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **client_username** | **String**| The clientUsername of the Client Username. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientUsernameResponse**](MsgVpnClientUsernameResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache**
> ::models::MsgVpnDistributedCacheResponse get_msg_vpn_distributed_cache(ctx, msg_vpn_name, cache_name, optional)
Get a Distributed Cache object.

Get a Distributed Cache object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheResponse**](MsgVpnDistributedCacheResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster**
> ::models::MsgVpnDistributedCacheClusterResponse get_msg_vpn_distributed_cache_cluster(ctx, msg_vpn_name, cache_name, cluster_name, optional)
Get a Cache Cluster object.

Get a Cache Cluster object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterResponse**](MsgVpnDistributedCacheClusterResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**
> ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(ctx, msg_vpn_name, cache_name, cluster_name, home_cluster_name, optional)
Get a Home Cache Cluster object.

Get a Home Cache Cluster object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **home_cluster_name** | **String**| The homeClusterName of the Home Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **home_cluster_name** | **String**| The homeClusterName of the Home Cache Cluster. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse**](MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**
> ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(ctx, msg_vpn_name, cache_name, cluster_name, home_cluster_name, topic_prefix, optional)
Get a Topic Prefix object.

Get a Topic Prefix object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| msgVpnName|x| topicPrefix|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **home_cluster_name** | **String**| The homeClusterName of the Home Cache Cluster. | 
  **topic_prefix** | **String**| The topicPrefix of the Topic Prefix. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **home_cluster_name** | **String**| The homeClusterName of the Home Cache Cluster. | 
 **topic_prefix** | **String**| The topicPrefix of the Topic Prefix. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse**](MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes**
> ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes(ctx, msg_vpn_name, cache_name, cluster_name, home_cluster_name, optional)
Get a list of Topic Prefix objects.

Get a list of Topic Prefix objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| msgVpnName|x| topicPrefix|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **home_cluster_name** | **String**| The homeClusterName of the Home Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **home_cluster_name** | **String**| The homeClusterName of the Home Cache Cluster. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse**](MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters**
> ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters(ctx, msg_vpn_name, cache_name, cluster_name, optional)
Get a list of Home Cache Cluster objects.

Get a list of Home Cache Cluster objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse**](MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_instance**
> ::models::MsgVpnDistributedCacheClusterInstanceResponse get_msg_vpn_distributed_cache_cluster_instance(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, optional)
Get a Cache Instance object.

Get a Cache Instance object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| instanceName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterInstanceResponse**](MsgVpnDistributedCacheClusterInstanceResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_cluster**
> ::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClusterResponse get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_cluster(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, home_cluster_name, optional)
Get a Remote Home Cache Cluster object.

Get a Remote Home Cache Cluster object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| instanceName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **home_cluster_name** | **String**| The homeClusterName of the Remote Home Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **home_cluster_name** | **String**| The homeClusterName of the Remote Home Cache Cluster. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClusterResponse**](MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClusterResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_clusters**
> ::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClustersResponse get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_clusters(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, optional)
Get a list of Remote Home Cache Cluster objects.

Get a list of Remote Home Cache Cluster objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| instanceName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClustersResponse**](MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClustersResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_instance_remote_topic**
> ::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicResponse get_msg_vpn_distributed_cache_cluster_instance_remote_topic(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, topic, optional)
Get a Remote Topic object.

Get a Remote Topic object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| instanceName|x| msgVpnName|x| topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **topic** | **String**| The topic of the Remote Topic. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **topic** | **String**| The topic of the Remote Topic. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicResponse**](MsgVpnDistributedCacheClusterInstanceRemoteTopicResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_instance_remote_topics**
> ::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicsResponse get_msg_vpn_distributed_cache_cluster_instance_remote_topics(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, optional)
Get a list of Remote Topic objects.

Get a list of Remote Topic objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| instanceName|x| msgVpnName|x| topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicsResponse**](MsgVpnDistributedCacheClusterInstanceRemoteTopicsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_instances**
> ::models::MsgVpnDistributedCacheClusterInstancesResponse get_msg_vpn_distributed_cache_cluster_instances(ctx, msg_vpn_name, cache_name, cluster_name, optional)
Get a list of Cache Instance objects.

Get a list of Cache Instance objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| instanceName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterInstancesResponse**](MsgVpnDistributedCacheClusterInstancesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_topic**
> ::models::MsgVpnDistributedCacheClusterTopicResponse get_msg_vpn_distributed_cache_cluster_topic(ctx, msg_vpn_name, cache_name, cluster_name, topic, optional)
Get a Topic object.

Get a Topic object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x| topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **topic** | **String**| The topic of the Topic. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **topic** | **String**| The topic of the Topic. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterTopicResponse**](MsgVpnDistributedCacheClusterTopicResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_topics**
> ::models::MsgVpnDistributedCacheClusterTopicsResponse get_msg_vpn_distributed_cache_cluster_topics(ctx, msg_vpn_name, cache_name, cluster_name, optional)
Get a list of Topic objects.

Get a list of Topic objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x| topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterTopicsResponse**](MsgVpnDistributedCacheClusterTopicsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_clusters**
> ::models::MsgVpnDistributedCacheClustersResponse get_msg_vpn_distributed_cache_clusters(ctx, msg_vpn_name, cache_name, optional)
Get a list of Cache Cluster objects.

Get a list of Cache Cluster objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClustersResponse**](MsgVpnDistributedCacheClustersResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_dmr_bridge**
> ::models::MsgVpnDmrBridgeResponse get_msg_vpn_dmr_bridge(ctx, msg_vpn_name, remote_node_name, optional)
Get a DMR Bridge object.

Get a DMR Bridge object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **remote_node_name** | **String**| The remoteNodeName of the DMR Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **remote_node_name** | **String**| The remoteNodeName of the DMR Bridge. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDmrBridgeResponse**](MsgVpnDmrBridgeResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_jndi_connection_factory**
> ::models::MsgVpnJndiConnectionFactoryResponse get_msg_vpn_jndi_connection_factory(ctx, msg_vpn_name, connection_factory_name, optional)
Get a JNDI Connection Factory object.

Get a JNDI Connection Factory object.   Attribute|Identifying|Deprecated :---|:---:|:---: connectionFactoryName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **connection_factory_name** | **String**| The connectionFactoryName of the JNDI Connection Factory. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **connection_factory_name** | **String**| The connectionFactoryName of the JNDI Connection Factory. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnJndiConnectionFactoryResponse**](MsgVpnJndiConnectionFactoryResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_jndi_queue**
> ::models::MsgVpnJndiQueueResponse get_msg_vpn_jndi_queue(ctx, msg_vpn_name, queue_name, optional)
Get a JNDI Queue object.

Get a JNDI Queue object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **queue_name** | **String**| The queueName of the JNDI Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **queue_name** | **String**| The queueName of the JNDI Queue. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnJndiQueueResponse**](MsgVpnJndiQueueResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_jndi_topic**
> ::models::MsgVpnJndiTopicResponse get_msg_vpn_jndi_topic(ctx, msg_vpn_name, topic_name, optional)
Get a JNDI Topic object.

Get a JNDI Topic object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| topicName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **topic_name** | **String**| The topicName of the JNDI Topic. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **topic_name** | **String**| The topicName of the JNDI Topic. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnJndiTopicResponse**](MsgVpnJndiTopicResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_mqtt_retain_cache**
> ::models::MsgVpnMqttRetainCacheResponse get_msg_vpn_mqtt_retain_cache(ctx, msg_vpn_name, cache_name, optional)
Get an MQTT Retain Cache object.

Get an MQTT Retain Cache object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the MQTT Retain Cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the MQTT Retain Cache. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnMqttRetainCacheResponse**](MsgVpnMqttRetainCacheResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_mqtt_session**
> ::models::MsgVpnMqttSessionResponse get_msg_vpn_mqtt_session(ctx, msg_vpn_name, mqtt_session_client_id, mqtt_session_virtual_router, optional)
Get an MQTT Session object.

Get an MQTT Session object.   Attribute|Identifying|Deprecated :---|:---:|:---: mqttSessionClientId|x| mqttSessionVirtualRouter|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
  **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
 **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnMqttSessionResponse**](MsgVpnMqttSessionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_mqtt_session_subscription**
> ::models::MsgVpnMqttSessionSubscriptionResponse get_msg_vpn_mqtt_session_subscription(ctx, msg_vpn_name, mqtt_session_client_id, mqtt_session_virtual_router, subscription_topic, optional)
Get a Subscription object.

Get a Subscription object.   Attribute|Identifying|Deprecated :---|:---:|:---: mqttSessionClientId|x| mqttSessionVirtualRouter|x| msgVpnName|x| subscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
  **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
  **subscription_topic** | **String**| The subscriptionTopic of the Subscription. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
 **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
 **subscription_topic** | **String**| The subscriptionTopic of the Subscription. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnMqttSessionSubscriptionResponse**](MsgVpnMqttSessionSubscriptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_mqtt_session_subscriptions**
> ::models::MsgVpnMqttSessionSubscriptionsResponse get_msg_vpn_mqtt_session_subscriptions(ctx, msg_vpn_name, mqtt_session_client_id, mqtt_session_virtual_router, optional)
Get a list of Subscription objects.

Get a list of Subscription objects.   Attribute|Identifying|Deprecated :---|:---:|:---: mqttSessionClientId|x| mqttSessionVirtualRouter|x| msgVpnName|x| subscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
  **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
 **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnMqttSessionSubscriptionsResponse**](MsgVpnMqttSessionSubscriptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_replay_log**
> ::models::MsgVpnReplayLogResponse get_msg_vpn_replay_log(ctx, msg_vpn_name, replay_log_name, optional)
Get a Replay Log object.

Get a Replay Log object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| replayLogName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnReplayLogResponse**](MsgVpnReplayLogResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_replay_log_msg**
> ::models::MsgVpnReplayLogMsgResponse get_msg_vpn_replay_log_msg(ctx, msg_vpn_name, replay_log_name, msg_id, optional)
Get a Message object.

Get a Message object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| replayLogName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
  **msg_id** | **String**| The msgId of the Message. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
 **msg_id** | **String**| The msgId of the Message. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnReplayLogMsgResponse**](MsgVpnReplayLogMsgResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_replay_log_msgs**
> ::models::MsgVpnReplayLogMsgsResponse get_msg_vpn_replay_log_msgs(ctx, msg_vpn_name, replay_log_name, optional)
Get a list of Message objects.

Get a list of Message objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| replayLogName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnReplayLogMsgsResponse**](MsgVpnReplayLogMsgsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point**
> ::models::MsgVpnRestDeliveryPointResponse get_msg_vpn_rest_delivery_point(ctx, msg_vpn_name, rest_delivery_point_name, optional)
Get a REST Delivery Point object.

Get a REST Delivery Point object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointResponse**](MsgVpnRestDeliveryPointResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point_queue_binding**
> ::models::MsgVpnRestDeliveryPointQueueBindingResponse get_msg_vpn_rest_delivery_point_queue_binding(ctx, msg_vpn_name, rest_delivery_point_name, queue_binding_name, optional)
Get a Queue Binding object.

Get a Queue Binding object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueBindingName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
  **queue_binding_name** | **String**| The queueBindingName of the Queue Binding. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **queue_binding_name** | **String**| The queueBindingName of the Queue Binding. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointQueueBindingResponse**](MsgVpnRestDeliveryPointQueueBindingResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point_queue_bindings**
> ::models::MsgVpnRestDeliveryPointQueueBindingsResponse get_msg_vpn_rest_delivery_point_queue_bindings(ctx, msg_vpn_name, rest_delivery_point_name, optional)
Get a list of Queue Binding objects.

Get a list of Queue Binding objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueBindingName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointQueueBindingsResponse**](MsgVpnRestDeliveryPointQueueBindingsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point_rest_consumer**
> ::models::MsgVpnRestDeliveryPointRestConsumerResponse get_msg_vpn_rest_delivery_point_rest_consumer(ctx, msg_vpn_name, rest_delivery_point_name, rest_consumer_name, optional)
Get a REST Consumer object.

Get a REST Consumer object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restConsumerName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
  **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointRestConsumerResponse**](MsgVpnRestDeliveryPointRestConsumerResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**
> ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(ctx, msg_vpn_name, rest_delivery_point_name, rest_consumer_name, tls_trusted_common_name, optional)
Get a Trusted Common Name object.

Get a Trusted Common Name object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restConsumerName|x| restDeliveryPointName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
  **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
  **tls_trusted_common_name** | **String**| The tlsTrustedCommonName of the Trusted Common Name. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
 **tls_trusted_common_name** | **String**| The tlsTrustedCommonName of the Trusted Common Name. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse**](MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names**
> ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names(ctx, msg_vpn_name, rest_delivery_point_name, rest_consumer_name, optional)
Get a list of Trusted Common Name objects.

Get a list of Trusted Common Name objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restConsumerName|x| restDeliveryPointName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
  **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse**](MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point_rest_consumers**
> ::models::MsgVpnRestDeliveryPointRestConsumersResponse get_msg_vpn_rest_delivery_point_rest_consumers(ctx, msg_vpn_name, rest_delivery_point_name, optional)
Get a list of REST Consumer objects.

Get a list of REST Consumer objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restConsumerName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointRestConsumersResponse**](MsgVpnRestDeliveryPointRestConsumersResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

