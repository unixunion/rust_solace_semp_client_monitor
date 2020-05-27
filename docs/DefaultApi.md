# \DefaultApi

All URIs are relative to *http://www.solace.com/SEMP/v2/monitor*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_about_user**](DefaultApi.md#get_about_user) | **Get** /about/user | Get a User object.
[**get_about_user_msg_vpn**](DefaultApi.md#get_about_user_msg_vpn) | **Get** /about/user/msgVpns/{msgVpnName} | Get a User Message VPN object.
[**get_about_user_msg_vpns**](DefaultApi.md#get_about_user_msg_vpns) | **Get** /about/user/msgVpns | Get a list of User Message VPN objects.
[**get_cert_authority**](DefaultApi.md#get_cert_authority) | **Get** /certAuthorities/{certAuthorityName} | Get a Certificate Authority object.
[**get_cert_authority_ocsp_tls_trusted_common_name**](DefaultApi.md#get_cert_authority_ocsp_tls_trusted_common_name) | **Get** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Get an OCSP Responder Trusted Common Name object.
[**get_cert_authority_ocsp_tls_trusted_common_names**](DefaultApi.md#get_cert_authority_ocsp_tls_trusted_common_names) | **Get** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Get a list of OCSP Responder Trusted Common Name objects.
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
[**get_msg_vpn_acl_profile_publish_topic_exception**](DefaultApi.md#get_msg_vpn_acl_profile_publish_topic_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions/{publishTopicExceptionSyntax},{publishTopicException} | Get a Publish Topic Exception object.
[**get_msg_vpn_acl_profile_publish_topic_exceptions**](DefaultApi.md#get_msg_vpn_acl_profile_publish_topic_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions | Get a list of Publish Topic Exception objects.
[**get_msg_vpn_acl_profile_subscribe_exception**](DefaultApi.md#get_msg_vpn_acl_profile_subscribe_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Get a Subscribe Topic Exception object.
[**get_msg_vpn_acl_profile_subscribe_exceptions**](DefaultApi.md#get_msg_vpn_acl_profile_subscribe_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Get a list of Subscribe Topic Exception objects.
[**get_msg_vpn_acl_profile_subscribe_share_name_exception**](DefaultApi.md#get_msg_vpn_acl_profile_subscribe_share_name_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions/{subscribeShareNameExceptionSyntax},{subscribeShareNameException} | Get a Subscribe Share Name Exception object.
[**get_msg_vpn_acl_profile_subscribe_share_name_exceptions**](DefaultApi.md#get_msg_vpn_acl_profile_subscribe_share_name_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions | Get a list of Subscribe Share Name Exception objects.
[**get_msg_vpn_acl_profile_subscribe_topic_exception**](DefaultApi.md#get_msg_vpn_acl_profile_subscribe_topic_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions/{subscribeTopicExceptionSyntax},{subscribeTopicException} | Get a Subscribe Topic Exception object.
[**get_msg_vpn_acl_profile_subscribe_topic_exceptions**](DefaultApi.md#get_msg_vpn_acl_profile_subscribe_topic_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions | Get a list of Subscribe Topic Exception objects.
[**get_msg_vpn_authentication_oauth_provider**](DefaultApi.md#get_msg_vpn_authentication_oauth_provider) | **Get** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Get an OAuth Provider object.
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
[**get_msg_vpn_client**](DefaultApi.md#get_msg_vpn_client) | **Get** /msgVpns/{msgVpnName}/clients/{clientName} | Get a Client object.
[**get_msg_vpn_client_connection**](DefaultApi.md#get_msg_vpn_client_connection) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/connections/{clientAddress} | Get a Client Connection object.
[**get_msg_vpn_client_connections**](DefaultApi.md#get_msg_vpn_client_connections) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/connections | Get a list of Client Connection objects.
[**get_msg_vpn_client_profile**](DefaultApi.md#get_msg_vpn_client_profile) | **Get** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Get a Client Profile object.
[**get_msg_vpn_client_rx_flow**](DefaultApi.md#get_msg_vpn_client_rx_flow) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/rxFlows/{flowId} | Get a Client Receive Flow object.
[**get_msg_vpn_client_rx_flows**](DefaultApi.md#get_msg_vpn_client_rx_flows) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/rxFlows | Get a list of Client Receive Flow objects.
[**get_msg_vpn_client_subscription**](DefaultApi.md#get_msg_vpn_client_subscription) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/subscriptions/{subscriptionTopic} | Get a Client Subscription object.
[**get_msg_vpn_client_subscriptions**](DefaultApi.md#get_msg_vpn_client_subscriptions) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/subscriptions | Get a list of Client Subscription objects.
[**get_msg_vpn_client_transacted_session**](DefaultApi.md#get_msg_vpn_client_transacted_session) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions/{sessionName} | Get a Client Transacted Session object.
[**get_msg_vpn_client_transacted_sessions**](DefaultApi.md#get_msg_vpn_client_transacted_sessions) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions | Get a list of Client Transacted Session objects.
[**get_msg_vpn_client_tx_flow**](DefaultApi.md#get_msg_vpn_client_tx_flow) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/txFlows/{flowId} | Get a Client Transmit Flow object.
[**get_msg_vpn_client_tx_flows**](DefaultApi.md#get_msg_vpn_client_tx_flows) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/txFlows | Get a list of Client Transmit Flow objects.
[**get_msg_vpn_client_username**](DefaultApi.md#get_msg_vpn_client_username) | **Get** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Get a Client Username object.
[**get_msg_vpn_config_sync_remote_node**](DefaultApi.md#get_msg_vpn_config_sync_remote_node) | **Get** /msgVpns/{msgVpnName}/configSyncRemoteNodes/{remoteNodeName} | Get a Config Sync Remote Node object.
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
[**get_msg_vpn_queue**](DefaultApi.md#get_msg_vpn_queue) | **Get** /msgVpns/{msgVpnName}/queues/{queueName} | Get a Queue object.
[**get_msg_vpn_queue_msg**](DefaultApi.md#get_msg_vpn_queue_msg) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/msgs/{msgId} | Get a Queue Message object.
[**get_msg_vpn_queue_msgs**](DefaultApi.md#get_msg_vpn_queue_msgs) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/msgs | Get a list of Queue Message objects.
[**get_msg_vpn_queue_priorities**](DefaultApi.md#get_msg_vpn_queue_priorities) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/priorities | Get a list of Queue Priority objects.
[**get_msg_vpn_queue_priority**](DefaultApi.md#get_msg_vpn_queue_priority) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/priorities/{priority} | Get a Queue Priority object.
[**get_msg_vpn_queue_subscription**](DefaultApi.md#get_msg_vpn_queue_subscription) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Get a Queue Subscription object.
[**get_msg_vpn_queue_subscriptions**](DefaultApi.md#get_msg_vpn_queue_subscriptions) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Get a list of Queue Subscription objects.
[**get_msg_vpn_queue_template**](DefaultApi.md#get_msg_vpn_queue_template) | **Get** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Get a Queue Template object.
[**get_msg_vpn_queue_tx_flow**](DefaultApi.md#get_msg_vpn_queue_tx_flow) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/txFlows/{flowId} | Get a Queue Transmit Flow object.
[**get_msg_vpn_queue_tx_flows**](DefaultApi.md#get_msg_vpn_queue_tx_flows) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/txFlows | Get a list of Queue Transmit Flow objects.
[**get_msg_vpn_replay_log**](DefaultApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Get a Replay Log object.
[**get_msg_vpn_replay_log_msg**](DefaultApi.md#get_msg_vpn_replay_log_msg) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName}/msgs/{msgId} | Get a Message object.
[**get_msg_vpn_replay_log_msgs**](DefaultApi.md#get_msg_vpn_replay_log_msgs) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName}/msgs | Get a list of Message objects.
[**get_msg_vpn_replicated_topic**](DefaultApi.md#get_msg_vpn_replicated_topic) | **Get** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Get a Replicated Topic object.
[**get_msg_vpn_rest_delivery_point**](DefaultApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Get a REST Delivery Point object.
[**get_msg_vpn_rest_delivery_point_queue_binding**](DefaultApi.md#get_msg_vpn_rest_delivery_point_queue_binding) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Get a Queue Binding object.
[**get_msg_vpn_rest_delivery_point_queue_bindings**](DefaultApi.md#get_msg_vpn_rest_delivery_point_queue_bindings) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Get a list of Queue Binding objects.
[**get_msg_vpn_rest_delivery_point_rest_consumer**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Get a REST Consumer object.
[**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
[**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
[**get_msg_vpn_rest_delivery_point_rest_consumers**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Get a list of REST Consumer objects.
[**get_msg_vpn_topic_endpoint**](DefaultApi.md#get_msg_vpn_topic_endpoint) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Get a Topic Endpoint object.
[**get_msg_vpn_topic_endpoint_msg**](DefaultApi.md#get_msg_vpn_topic_endpoint_msg) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs/{msgId} | Get a Topic Endpoint Message object.
[**get_msg_vpn_topic_endpoint_msgs**](DefaultApi.md#get_msg_vpn_topic_endpoint_msgs) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs | Get a list of Topic Endpoint Message objects.
[**get_msg_vpn_topic_endpoint_priorities**](DefaultApi.md#get_msg_vpn_topic_endpoint_priorities) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/priorities | Get a list of Topic Endpoint Priority objects.
[**get_msg_vpn_topic_endpoint_priority**](DefaultApi.md#get_msg_vpn_topic_endpoint_priority) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/priorities/{priority} | Get a Topic Endpoint Priority object.
[**get_msg_vpn_topic_endpoint_template**](DefaultApi.md#get_msg_vpn_topic_endpoint_template) | **Get** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Get a Topic Endpoint Template object.
[**get_msg_vpn_topic_endpoint_tx_flow**](DefaultApi.md#get_msg_vpn_topic_endpoint_tx_flow) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/txFlows/{flowId} | Get a Topic Endpoint Transmit Flow object.
[**get_msg_vpn_topic_endpoint_tx_flows**](DefaultApi.md#get_msg_vpn_topic_endpoint_tx_flows) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/txFlows | Get a list of Topic Endpoint Transmit Flow objects.
[**get_msg_vpn_transaction**](DefaultApi.md#get_msg_vpn_transaction) | **Get** /msgVpns/{msgVpnName}/transactions/{xid} | Get a Replicated Local Transaction or XA Transaction object.
[**get_msg_vpn_transaction_consumer_msg**](DefaultApi.md#get_msg_vpn_transaction_consumer_msg) | **Get** /msgVpns/{msgVpnName}/transactions/{xid}/consumerMsgs/{msgId} | Get a Transaction Consumer Message object.
[**get_msg_vpn_transaction_consumer_msgs**](DefaultApi.md#get_msg_vpn_transaction_consumer_msgs) | **Get** /msgVpns/{msgVpnName}/transactions/{xid}/consumerMsgs | Get a list of Transaction Consumer Message objects.
[**get_msg_vpn_transaction_publisher_msg**](DefaultApi.md#get_msg_vpn_transaction_publisher_msg) | **Get** /msgVpns/{msgVpnName}/transactions/{xid}/publisherMsgs/{msgId} | Get a Transaction Publisher Message object.
[**get_msg_vpn_transaction_publisher_msgs**](DefaultApi.md#get_msg_vpn_transaction_publisher_msgs) | **Get** /msgVpns/{msgVpnName}/transactions/{xid}/publisherMsgs | Get a list of Transaction Publisher Message objects.


# **get_about_user**
> ::models::AboutUserResponse get_about_user(ctx, optional)
Get a User object.

Get a User object.  This provides information about the access level for the username used to access the SEMP API.    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.11.

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

[**::models::AboutUserResponse**](AboutUserResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_about_user_msg_vpn**
> ::models::AboutUserMsgVpnResponse get_about_user_msg_vpn(ctx, msg_vpn_name, optional)
Get a User Message VPN object.

Get a User Message VPN object.  This provides information about the Message VPN access level for the username used to access the SEMP API.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.11.

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

Get a list of User Message VPN objects.  This provides information about the Message VPN access level for the username used to access the SEMP API.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.11.

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

# **get_cert_authority**
> ::models::CertAuthorityResponse get_cert_authority(ctx, cert_authority_name, optional)
Get a Certificate Authority object.

Get a Certificate Authority object.  Clients can authenticate with the message broker over TLS by presenting a valid client certificate. The message broker authenticates the client certificate by constructing a full certificate chain (from the client certificate to intermediate CAs to a configured root CA). The intermediate CAs in this chain can be provided by the client, or configured in the message broker. The root CA must be configured on the message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: certAuthorityName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.13.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cert_authority_name** | **String**| The name of the Certificate Authority. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cert_authority_name** | **String**| The name of the Certificate Authority. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::CertAuthorityResponse**](CertAuthorityResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cert_authority_ocsp_tls_trusted_common_name**
> ::models::CertAuthorityOcspTlsTrustedCommonNameResponse get_cert_authority_ocsp_tls_trusted_common_name(ctx, cert_authority_name, ocsp_tls_trusted_common_name, optional)
Get an OCSP Responder Trusted Common Name object.

Get an OCSP Responder Trusted Common Name object.  When an OCSP override URL is configured, the OCSP responder will be required to sign the OCSP responses with certificates issued to these Trusted Common Names. A maximum of 8 common names can be configured as valid response signers.   Attribute|Identifying|Deprecated :---|:---:|:---: certAuthorityName|x| ocspTlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.13.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cert_authority_name** | **String**| The name of the Certificate Authority. | 
  **ocsp_tls_trusted_common_name** | **String**| The expected Trusted Common Name of the OCSP responder remote certificate. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cert_authority_name** | **String**| The name of the Certificate Authority. | 
 **ocsp_tls_trusted_common_name** | **String**| The expected Trusted Common Name of the OCSP responder remote certificate. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::CertAuthorityOcspTlsTrustedCommonNameResponse**](CertAuthorityOcspTlsTrustedCommonNameResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cert_authority_ocsp_tls_trusted_common_names**
> ::models::CertAuthorityOcspTlsTrustedCommonNamesResponse get_cert_authority_ocsp_tls_trusted_common_names(ctx, cert_authority_name, optional)
Get a list of OCSP Responder Trusted Common Name objects.

Get a list of OCSP Responder Trusted Common Name objects.  When an OCSP override URL is configured, the OCSP responder will be required to sign the OCSP responses with certificates issued to these Trusted Common Names. A maximum of 8 common names can be configured as valid response signers.   Attribute|Identifying|Deprecated :---|:---:|:---: certAuthorityName|x| ocspTlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.13.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cert_authority_name** | **String**| The name of the Certificate Authority. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cert_authority_name** | **String**| The name of the Certificate Authority. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::CertAuthorityOcspTlsTrustedCommonNamesResponse**](CertAuthorityOcspTlsTrustedCommonNamesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dmr_cluster**
> ::models::DmrClusterResponse get_dmr_cluster(ctx, dmr_cluster_name, optional)
Get a Cluster object.

Get a Cluster object.  A Cluster is a provisioned object on a message broker that contains global DMR configuration parameters.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
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

Get a Link object.  A Link connects nodes (either within a Cluster or between two different Clusters) and allows them to exchange topology information, subscriptions and data.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
  **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
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

Get a Cluster Link Channels object.  A Channel is a connection between this broker and a remote node in the Cluster.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
  **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
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

Get a list of Cluster Link Channels objects.  A Channel is a connection between this broker and a remote node in the Cluster.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
  **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
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

Get a Remote Address object.  Each Remote Address, consisting of a FQDN or IP address and optional port, is used to connect to the remote node for this Link. Up to 4 addresses may be provided for each Link, and will be tried on a round-robin basis.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteAddress|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
  **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
  **remote_address** | **String**| The FQDN or IP address (and optional port) of the remote node. If a port is not provided, it will vary based on the transport encoding: 55555 (plain-text), 55443 (encrypted), or 55003 (compressed). | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
 **remote_address** | **String**| The FQDN or IP address (and optional port) of the remote node. If a port is not provided, it will vary based on the transport encoding: 55555 (plain-text), 55443 (encrypted), or 55003 (compressed). | 
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

Get a list of Remote Address objects.  Each Remote Address, consisting of a FQDN or IP address and optional port, is used to connect to the remote node for this Link. Up to 4 addresses may be provided for each Link, and will be tried on a round-robin basis.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteAddress|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
  **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
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

Get a Trusted Common Name object.  The Trusted Common Names for the Link are used by encrypted transports to verify the name in the certificate presented by the remote node. They must include the common name of the remote node's server certificate or client certificate, depending upon the initiator of the connection.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteNodeName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
  **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
  **tls_trusted_common_name** | **String**| The expected trusted common name of the remote certificate. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
 **tls_trusted_common_name** | **String**| The expected trusted common name of the remote certificate. | 
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

Get a list of Trusted Common Name objects.  The Trusted Common Names for the Link are used by encrypted transports to verify the name in the certificate presented by the remote node. They must include the common name of the remote node's server certificate or client certificate, depending upon the initiator of the connection.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteNodeName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
  **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **remote_node_name** | **String**| The name of the node at the remote end of the Link. | 
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

Get a list of Link objects.  A Link connects nodes (either within a Cluster or between two different Clusters) and allows them to exchange topology information, subscriptions and data.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
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

Get a Cluster Topology Issue object.  A Cluster Topology Issue indicates incorrect or inconsistent configuration within the DMR network. Such issues will cause messages to be misdelivered or lost.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| topologyIssue|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
  **topology_issue** | **String**| The topology issue discovered in the Cluster. A topology issue indicates incorrect or inconsistent configuration within the DMR network. Such issues will cause messages to be misdelivered or lost. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **topology_issue** | **String**| The topology issue discovered in the Cluster. A topology issue indicates incorrect or inconsistent configuration within the DMR network. Such issues will cause messages to be misdelivered or lost. | 
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

Get a list of Cluster Topology Issue objects.  A Cluster Topology Issue indicates incorrect or inconsistent configuration within the DMR network. Such issues will cause messages to be misdelivered or lost.   Attribute|Identifying|Deprecated :---|:---:|:---: dmrClusterName|x| topologyIssue|x|    A SEMP client authorized with a minimum access scope/level of \"global/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dmr_cluster_name** | **String**| The name of the Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dmr_cluster_name** | **String**| The name of the Cluster. | 
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

Get a Message VPN object.  Message VPNs (Virtual Private Networks) allow for the segregation of topic space and clients. They also group clients connecting to a network of message brokers, such that messages published within a particular group are only visible to that group's clients.   Attribute|Identifying|Deprecated :---|:---:|:---: counter.controlRxByteCount||x counter.controlRxMsgCount||x counter.controlTxByteCount||x counter.controlTxMsgCount||x counter.dataRxByteCount||x counter.dataRxMsgCount||x counter.dataTxByteCount||x counter.dataTxMsgCount||x counter.discardedRxMsgCount||x counter.discardedTxMsgCount||x counter.loginRxMsgCount||x counter.loginTxMsgCount||x counter.msgSpoolRxMsgCount||x counter.msgSpoolTxMsgCount||x counter.tlsRxByteCount||x counter.tlsTxByteCount||x msgVpnName|x| rate.averageRxByteRate||x rate.averageRxMsgRate||x rate.averageTxByteRate||x rate.averageTxMsgRate||x rate.rxByteRate||x rate.rxMsgRate||x rate.tlsAverageRxByteRate||x rate.tlsAverageTxByteRate||x rate.tlsRxByteRate||x rate.tlsTxByteRate||x rate.txByteRate||x rate.txMsgRate||x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

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

Get an ACL Profile object.  An ACL Profile controls whether an authenticated client is permitted to establish a connection with the message broker or permitted to publish and subscribe to specific topics.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
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

Get a Client Connect Exception object.  A Client Connect Exception is an exception to the default action to take when a client using the ACL Profile connects to the Message VPN. Exceptions must be expressed as an IP address/netmask in CIDR form.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| clientConnectExceptionAddress|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
  **client_connect_exception_address** | **String**| The IP address/netmask of the client connect exception in CIDR form. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **client_connect_exception_address** | **String**| The IP address/netmask of the client connect exception in CIDR form. | 
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

Get a list of Client Connect Exception objects.  A Client Connect Exception is an exception to the default action to take when a client using the ACL Profile connects to the Message VPN. Exceptions must be expressed as an IP address/netmask in CIDR form.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| clientConnectExceptionAddress|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
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

Get a Publish Topic Exception object.  A Publish Topic Exception is an exception to the default action to take when a client using the ACL Profile publishes to a topic in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x|x msgVpnName|x|x publishExceptionTopic|x|x topicSyntax|x|x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been deprecated since 2.16. Replaced by publishTopicExceptions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
  **topic_syntax** | **String**| The syntax of the topic for the exception to the default action taken. | 
  **publish_exception_topic** | **String**| The topic for the exception to the default action taken. May include wildcard characters. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **topic_syntax** | **String**| The syntax of the topic for the exception to the default action taken. | 
 **publish_exception_topic** | **String**| The topic for the exception to the default action taken. May include wildcard characters. | 
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

Get a list of Publish Topic Exception objects.  A Publish Topic Exception is an exception to the default action to take when a client using the ACL Profile publishes to a topic in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x|x msgVpnName|x|x publishExceptionTopic|x|x topicSyntax|x|x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been deprecated since 2.16. Replaced by publishTopicExceptions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
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

# **get_msg_vpn_acl_profile_publish_topic_exception**
> ::models::MsgVpnAclProfilePublishTopicExceptionResponse get_msg_vpn_acl_profile_publish_topic_exception(ctx, msg_vpn_name, acl_profile_name, publish_topic_exception_syntax, publish_topic_exception, optional)
Get a Publish Topic Exception object.

Get a Publish Topic Exception object.  A Publish Topic Exception is an exception to the default action to take when a client using the ACL Profile publishes to a topic in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| publishTopicException|x| publishTopicExceptionSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
  **publish_topic_exception_syntax** | **String**| The syntax of the topic for the exception to the default action taken. | 
  **publish_topic_exception** | **String**| The topic for the exception to the default action taken. May include wildcard characters. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **publish_topic_exception_syntax** | **String**| The syntax of the topic for the exception to the default action taken. | 
 **publish_topic_exception** | **String**| The topic for the exception to the default action taken. May include wildcard characters. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfilePublishTopicExceptionResponse**](MsgVpnAclProfilePublishTopicExceptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_publish_topic_exceptions**
> ::models::MsgVpnAclProfilePublishTopicExceptionsResponse get_msg_vpn_acl_profile_publish_topic_exceptions(ctx, msg_vpn_name, acl_profile_name, optional)
Get a list of Publish Topic Exception objects.

Get a list of Publish Topic Exception objects.  A Publish Topic Exception is an exception to the default action to take when a client using the ACL Profile publishes to a topic in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| publishTopicException|x| publishTopicExceptionSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfilePublishTopicExceptionsResponse**](MsgVpnAclProfilePublishTopicExceptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_subscribe_exception**
> ::models::MsgVpnAclProfileSubscribeExceptionResponse get_msg_vpn_acl_profile_subscribe_exception(ctx, msg_vpn_name, acl_profile_name, topic_syntax, subscribe_exception_topic, optional)
Get a Subscribe Topic Exception object.

Get a Subscribe Topic Exception object.  A Subscribe Topic Exception is an exception to the default action to take when a client using the ACL Profile subscribes to a topic in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x|x msgVpnName|x|x subscribeExceptionTopic|x|x topicSyntax|x|x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been deprecated since 2.16. Replaced by subscribeTopicExceptions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
  **topic_syntax** | **String**| The syntax of the topic for the exception to the default action taken. | 
  **subscribe_exception_topic** | **String**| The topic for the exception to the default action taken. May include wildcard characters. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **topic_syntax** | **String**| The syntax of the topic for the exception to the default action taken. | 
 **subscribe_exception_topic** | **String**| The topic for the exception to the default action taken. May include wildcard characters. | 
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

Get a list of Subscribe Topic Exception objects.  A Subscribe Topic Exception is an exception to the default action to take when a client using the ACL Profile subscribes to a topic in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x|x msgVpnName|x|x subscribeExceptionTopic|x|x topicSyntax|x|x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been deprecated since 2.16. Replaced by subscribeTopicExceptions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
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

# **get_msg_vpn_acl_profile_subscribe_share_name_exception**
> ::models::MsgVpnAclProfileSubscribeShareNameExceptionResponse get_msg_vpn_acl_profile_subscribe_share_name_exception(ctx, msg_vpn_name, acl_profile_name, subscribe_share_name_exception_syntax, subscribe_share_name_exception, optional)
Get a Subscribe Share Name Exception object.

Get a Subscribe Share Name Exception object.  A Subscribe Share Name Exception is an exception to the default action to take when a client using the ACL Profile subscribes to a share-name subscription in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| subscribeShareNameException|x| subscribeShareNameExceptionSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
  **subscribe_share_name_exception_syntax** | **String**| The syntax of the subscribe share name for the exception to the default action taken. | 
  **subscribe_share_name_exception** | **String**| The subscribe share name exception to the default action taken. May include wildcard characters. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **subscribe_share_name_exception_syntax** | **String**| The syntax of the subscribe share name for the exception to the default action taken. | 
 **subscribe_share_name_exception** | **String**| The subscribe share name exception to the default action taken. May include wildcard characters. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileSubscribeShareNameExceptionResponse**](MsgVpnAclProfileSubscribeShareNameExceptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_subscribe_share_name_exceptions**
> ::models::MsgVpnAclProfileSubscribeShareNameExceptionsResponse get_msg_vpn_acl_profile_subscribe_share_name_exceptions(ctx, msg_vpn_name, acl_profile_name, optional)
Get a list of Subscribe Share Name Exception objects.

Get a list of Subscribe Share Name Exception objects.  A Subscribe Share Name Exception is an exception to the default action to take when a client using the ACL Profile subscribes to a share-name subscription in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| subscribeShareNameException|x| subscribeShareNameExceptionSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileSubscribeShareNameExceptionsResponse**](MsgVpnAclProfileSubscribeShareNameExceptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_subscribe_topic_exception**
> ::models::MsgVpnAclProfileSubscribeTopicExceptionResponse get_msg_vpn_acl_profile_subscribe_topic_exception(ctx, msg_vpn_name, acl_profile_name, subscribe_topic_exception_syntax, subscribe_topic_exception, optional)
Get a Subscribe Topic Exception object.

Get a Subscribe Topic Exception object.  A Subscribe Topic Exception is an exception to the default action to take when a client using the ACL Profile subscribes to a topic in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| subscribeTopicException|x| subscribeTopicExceptionSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
  **subscribe_topic_exception_syntax** | **String**| The syntax of the topic for the exception to the default action taken. | 
  **subscribe_topic_exception** | **String**| The topic for the exception to the default action taken. May include wildcard characters. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **subscribe_topic_exception_syntax** | **String**| The syntax of the topic for the exception to the default action taken. | 
 **subscribe_topic_exception** | **String**| The topic for the exception to the default action taken. May include wildcard characters. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileSubscribeTopicExceptionResponse**](MsgVpnAclProfileSubscribeTopicExceptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_acl_profile_subscribe_topic_exceptions**
> ::models::MsgVpnAclProfileSubscribeTopicExceptionsResponse get_msg_vpn_acl_profile_subscribe_topic_exceptions(ctx, msg_vpn_name, acl_profile_name, optional)
Get a list of Subscribe Topic Exception objects.

Get a list of Subscribe Topic Exception objects.  A Subscribe Topic Exception is an exception to the default action to take when a client using the ACL Profile subscribes to a topic in the Message VPN. Exceptions must be expressed as a topic.   Attribute|Identifying|Deprecated :---|:---:|:---: aclProfileName|x| msgVpnName|x| subscribeTopicException|x| subscribeTopicExceptionSyntax|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **acl_profile_name** | **String**| The name of the ACL Profile. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAclProfileSubscribeTopicExceptionsResponse**](MsgVpnAclProfileSubscribeTopicExceptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_authentication_oauth_provider**
> ::models::MsgVpnAuthenticationOauthProviderResponse get_msg_vpn_authentication_oauth_provider(ctx, msg_vpn_name, oauth_provider_name, optional)
Get an OAuth Provider object.

Get an OAuth Provider object.  OAuth Providers contain information about the issuer of an OAuth token that is needed to validate the token and derive a client username from it.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| oauthProviderName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.13.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **oauth_provider_name** | **String**| The name of the OAuth Provider. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **oauth_provider_name** | **String**| The name of the OAuth Provider. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnAuthenticationOauthProviderResponse**](MsgVpnAuthenticationOauthProviderResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_authorization_group**
> ::models::MsgVpnAuthorizationGroupResponse get_msg_vpn_authorization_group(ctx, msg_vpn_name, authorization_group_name, optional)
Get an LDAP Authorization Group object.

Get an LDAP Authorization Group object.  To use client authorization groups configured on an external LDAP server to provide client authorizations, LDAP Authorization Group objects must be created on the Message VPN that match the authorization groups provisioned on the LDAP server. These objects must be configured with the client profiles and ACL profiles that will be assigned to the clients that belong to those authorization groups. A newly created group is placed at the end of the group list which is the lowest priority.   Attribute|Identifying|Deprecated :---|:---:|:---: authorizationGroupName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **authorization_group_name** | **String**| The name of the LDAP Authorization Group. Special care is needed if the group name contains special characters such as &#39;#&#39;, &#39;+&#39;, &#39;;&#39;, &#39;&#x3D;&#39; as the value of the group name returned from the LDAP server might prepend those characters with &#39;\\&#39;. For example a group name called &#39;test#,lab,com&#39; will be returned from the LDAP server as &#39;test\\#,lab,com&#39;. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **authorization_group_name** | **String**| The name of the LDAP Authorization Group. Special care is needed if the group name contains special characters such as &#39;#&#39;, &#39;+&#39;, &#39;;&#39;, &#39;&#x3D;&#39; as the value of the group name returned from the LDAP server might prepend those characters with &#39;\\&#39;. For example a group name called &#39;test#,lab,com&#39; will be returned from the LDAP server as &#39;test\\#,lab,com&#39;. | 
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

Get a Bridge object.  Bridges can be used to link two Message VPNs so that messages published to one Message VPN that match the topic subscriptions set for the bridge are also delivered to the linked Message VPN.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| counter.controlRxByteCount||x counter.controlRxMsgCount||x counter.controlTxByteCount||x counter.controlTxMsgCount||x counter.dataRxByteCount||x counter.dataRxMsgCount||x counter.dataTxByteCount||x counter.dataTxMsgCount||x counter.discardedRxMsgCount||x counter.discardedTxMsgCount||x counter.loginRxMsgCount||x counter.loginTxMsgCount||x counter.msgSpoolRxMsgCount||x counter.rxByteCount||x counter.rxMsgCount||x counter.txByteCount||x counter.txMsgCount||x msgVpnName|x| rate.averageRxByteRate||x rate.averageRxMsgRate||x rate.averageTxByteRate||x rate.averageTxMsgRate||x rate.rxByteRate||x rate.rxMsgRate||x rate.txByteRate||x rate.txMsgRate||x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
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

Get a Bridge Local Subscriptions object.  A Local Subscription is a topic subscription used by a remote Message VPN Bridge to attract messages from this broker.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| localSubscriptionTopic|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
  **local_subscription_topic** | **String**| The topic of the Bridge local subscription. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **local_subscription_topic** | **String**| The topic of the Bridge local subscription. | 
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

Get a list of Bridge Local Subscriptions objects.  A Local Subscription is a topic subscription used by a remote Message VPN Bridge to attract messages from this broker.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| localSubscriptionTopic|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
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

Get a Remote Message VPN object.  The Remote Message VPN is the Message VPN that the Bridge connects to.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| remoteMsgVpnInterface|x| remoteMsgVpnLocation|x| remoteMsgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
  **remote_msg_vpn_name** | **String**| The name of the remote Message VPN. | 
  **remote_msg_vpn_location** | **String**| The location of the remote Message VPN as either an FQDN with port, IP address with port, or virtual router name (starting with \&quot;v:\&quot;). | 
  **remote_msg_vpn_interface** | **String**| The physical interface on the local Message VPN host for connecting to the remote Message VPN. By default, an interface is chosen automatically (recommended), but if specified, &#x60;remoteMsgVpnLocation&#x60; must not be a virtual router name. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **remote_msg_vpn_name** | **String**| The name of the remote Message VPN. | 
 **remote_msg_vpn_location** | **String**| The location of the remote Message VPN as either an FQDN with port, IP address with port, or virtual router name (starting with \&quot;v:\&quot;). | 
 **remote_msg_vpn_interface** | **String**| The physical interface on the local Message VPN host for connecting to the remote Message VPN. By default, an interface is chosen automatically (recommended), but if specified, &#x60;remoteMsgVpnLocation&#x60; must not be a virtual router name. | 
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

Get a list of Remote Message VPN objects.  The Remote Message VPN is the Message VPN that the Bridge connects to.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| remoteMsgVpnInterface|x| remoteMsgVpnLocation|x| remoteMsgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
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

Get a Remote Subscription object.  A Remote Subscription is a topic subscription used by the Message VPN Bridge to attract messages from the remote message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| remoteSubscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
  **remote_subscription_topic** | **String**| The topic of the Bridge remote subscription. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **remote_subscription_topic** | **String**| The topic of the Bridge remote subscription. | 
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

Get a list of Remote Subscription objects.  A Remote Subscription is a topic subscription used by the Message VPN Bridge to attract messages from the remote message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| remoteSubscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
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

Get a Trusted Common Name object.  The Trusted Common Names for the Bridge are used by encrypted transports to verify the name in the certificate presented by the remote node. They must include the common name of the remote node's server certificate or client certificate, depending upon the initiator of the connection.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
  **tls_trusted_common_name** | **String**| The expected trusted common name of the remote certificate. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **tls_trusted_common_name** | **String**| The expected trusted common name of the remote certificate. | 
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

Get a list of Trusted Common Name objects.  The Trusted Common Names for the Bridge are used by encrypted transports to verify the name in the certificate presented by the remote node. They must include the common name of the remote node's server certificate or client certificate, depending upon the initiator of the connection.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **bridge_name** | **String**| The name of the Bridge. | 
  **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **bridge_name** | **String**| The name of the Bridge. | 
 **bridge_virtual_router** | **String**| The virtual router of the Bridge. | 
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

# **get_msg_vpn_client**
> ::models::MsgVpnClientResponse get_msg_vpn_client(ctx, msg_vpn_name, client_name, optional)
Get a Client object.

Get a Client object.  Applications or devices that connect to message brokers to send and/or receive messages are represented as Clients.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientResponse**](MsgVpnClientResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_connection**
> ::models::MsgVpnClientConnectionResponse get_msg_vpn_client_connection(ctx, msg_vpn_name, client_name, client_address, optional)
Get a Client Connection object.

Get a Client Connection object.  A Client Connection represents the Transmission Control Protocol (TCP) connection the Client uses to communicate with the message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: clientAddress|x| clientName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
  **client_address** | **String**| The IP address and TCP port on the Client side of the Client Connection. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **client_address** | **String**| The IP address and TCP port on the Client side of the Client Connection. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientConnectionResponse**](MsgVpnClientConnectionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_connections**
> ::models::MsgVpnClientConnectionsResponse get_msg_vpn_client_connections(ctx, msg_vpn_name, client_name, optional)
Get a list of Client Connection objects.

Get a list of Client Connection objects.  A Client Connection represents the Transmission Control Protocol (TCP) connection the Client uses to communicate with the message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: clientAddress|x| clientName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientConnectionsResponse**](MsgVpnClientConnectionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_profile**
> ::models::MsgVpnClientProfileResponse get_msg_vpn_client_profile(ctx, msg_vpn_name, client_profile_name, optional)
Get a Client Profile object.

Get a Client Profile object.  Client Profiles are used to assign common configuration properties to clients that have been successfully authorized.   Attribute|Identifying|Deprecated :---|:---:|:---: apiQueueManagementCopyFromOnCreateName||x apiTopicEndpointManagementCopyFromOnCreateName||x clientProfileName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_profile_name** | **String**| The name of the Client Profile. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_profile_name** | **String**| The name of the Client Profile. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientProfileResponse**](MsgVpnClientProfileResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_rx_flow**
> ::models::MsgVpnClientRxFlowResponse get_msg_vpn_client_rx_flow(ctx, msg_vpn_name, client_name, flow_id, optional)
Get a Client Receive Flow object.

Get a Client Receive Flow object.  Client Receive Flows are used by clients to publish Guaranteed messages to a message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| flowId|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
  **flow_id** | **String**| The identifier (ID) of the flow. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **flow_id** | **String**| The identifier (ID) of the flow. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientRxFlowResponse**](MsgVpnClientRxFlowResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_rx_flows**
> ::models::MsgVpnClientRxFlowsResponse get_msg_vpn_client_rx_flows(ctx, msg_vpn_name, client_name, optional)
Get a list of Client Receive Flow objects.

Get a list of Client Receive Flow objects.  Client Receive Flows are used by clients to publish Guaranteed messages to a message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| flowId|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientRxFlowsResponse**](MsgVpnClientRxFlowsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_subscription**
> ::models::MsgVpnClientSubscriptionResponse get_msg_vpn_client_subscription(ctx, msg_vpn_name, client_name, subscription_topic, optional)
Get a Client Subscription object.

Get a Client Subscription object.  Once clients are authenticated on the message broker they can add and remove Client Subscriptions for Direct messages published to the Message VPN to which they have connected.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x| subscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
  **subscription_topic** | **String**| The topic of the Subscription. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **subscription_topic** | **String**| The topic of the Subscription. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientSubscriptionResponse**](MsgVpnClientSubscriptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_subscriptions**
> ::models::MsgVpnClientSubscriptionsResponse get_msg_vpn_client_subscriptions(ctx, msg_vpn_name, client_name, optional)
Get a list of Client Subscription objects.

Get a list of Client Subscription objects.  Once clients are authenticated on the message broker they can add and remove Client Subscriptions for Direct messages published to the Message VPN to which they have connected.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x| subscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientSubscriptionsResponse**](MsgVpnClientSubscriptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_transacted_session**
> ::models::MsgVpnClientTransactedSessionResponse get_msg_vpn_client_transacted_session(ctx, msg_vpn_name, client_name, session_name, optional)
Get a Client Transacted Session object.

Get a Client Transacted Session object.  Transacted Sessions enable clients to group multiple message send and/or receive operations together in single, atomic units known as local transactions.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x| sessionName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
  **session_name** | **String**| The name of the Transacted Session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **session_name** | **String**| The name of the Transacted Session. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientTransactedSessionResponse**](MsgVpnClientTransactedSessionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_transacted_sessions**
> ::models::MsgVpnClientTransactedSessionsResponse get_msg_vpn_client_transacted_sessions(ctx, msg_vpn_name, client_name, optional)
Get a list of Client Transacted Session objects.

Get a list of Client Transacted Session objects.  Transacted Sessions enable clients to group multiple message send and/or receive operations together in single, atomic units known as local transactions.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x| sessionName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientTransactedSessionsResponse**](MsgVpnClientTransactedSessionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_tx_flow**
> ::models::MsgVpnClientTxFlowResponse get_msg_vpn_client_tx_flow(ctx, msg_vpn_name, client_name, flow_id, optional)
Get a Client Transmit Flow object.

Get a Client Transmit Flow object.  Client Transmit Flows are used by clients to consume Guaranteed messages from a message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| flowId|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
  **flow_id** | **String**| The identifier (ID) of the flow. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **flow_id** | **String**| The identifier (ID) of the flow. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientTxFlowResponse**](MsgVpnClientTxFlowResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_tx_flows**
> ::models::MsgVpnClientTxFlowsResponse get_msg_vpn_client_tx_flows(ctx, msg_vpn_name, client_name, optional)
Get a list of Client Transmit Flow objects.

Get a list of Client Transmit Flow objects.  Client Transmit Flows are used by clients to consume Guaranteed messages from a message broker.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| flowId|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_name** | **String**| The name of the Client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_name** | **String**| The name of the Client. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientTxFlowsResponse**](MsgVpnClientTxFlowsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_username**
> ::models::MsgVpnClientUsernameResponse get_msg_vpn_client_username(ctx, msg_vpn_name, client_username, optional)
Get a Client Username object.

Get a Client Username object.  A client is only authorized to connect to a Message VPN that is associated with a Client Username that the client has been assigned.   Attribute|Identifying|Deprecated :---|:---:|:---: clientUsername|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **client_username** | **String**| The name of the Client Username. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **client_username** | **String**| The name of the Client Username. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientUsernameResponse**](MsgVpnClientUsernameResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_config_sync_remote_node**
> ::models::MsgVpnConfigSyncRemoteNodeResponse get_msg_vpn_config_sync_remote_node(ctx, msg_vpn_name, remote_node_name, optional)
Get a Config Sync Remote Node object.

Get a Config Sync Remote Node object.  A Config Sync Remote Node object contains information about the status of the table for this Message VPN with respect to a remote node.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **remote_node_name** | **String**| The name of the Config Sync Remote Node. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **remote_node_name** | **String**| The name of the Config Sync Remote Node. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnConfigSyncRemoteNodeResponse**](MsgVpnConfigSyncRemoteNodeResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache**
> ::models::MsgVpnDistributedCacheResponse get_msg_vpn_distributed_cache(ctx, msg_vpn_name, cache_name, optional)
Get a Distributed Cache object.

Get a Distributed Cache object.  A Distributed Cache is a collection of one or more Cache Clusters that belong to the same Message VPN. Each Cache Cluster in a Distributed Cache is configured to subscribe to a different set of topics. This effectively divides up the configured topic space, to provide scaling to very large topic spaces or very high cached message throughput.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
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

Get a Cache Cluster object.  A Cache Cluster is a collection of one or more Cache Instances that subscribe to exactly the same topics. Cache Instances are grouped together in a Cache Cluster for the purpose of fault tolerance and load balancing. As published messages are received, the message broker message bus sends these live data messages to the Cache Instances in the Cache Cluster. This enables client cache requests to be served by any of Cache Instances in the Cache Cluster.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
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

Get a Home Cache Cluster object.  A Home Cache Cluster is a Cache Cluster that is the \"definitive\" Cache Cluster for a given topic in the context of the Global Caching feature.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **home_cluster_name** | **String**| The name of the remote Home Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **home_cluster_name** | **String**| The name of the remote Home Cache Cluster. | 
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

Get a Topic Prefix object.  A Topic Prefix is a prefix for a global topic that is available from the containing Home Cache Cluster.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| msgVpnName|x| topicPrefix|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **home_cluster_name** | **String**| The name of the remote Home Cache Cluster. | 
  **topic_prefix** | **String**| A topic prefix for global topics available from the remote Home Cache Cluster. A wildcard (/&gt;) is implied at the end of the prefix. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **home_cluster_name** | **String**| The name of the remote Home Cache Cluster. | 
 **topic_prefix** | **String**| A topic prefix for global topics available from the remote Home Cache Cluster. A wildcard (/&gt;) is implied at the end of the prefix. | 
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

Get a list of Topic Prefix objects.  A Topic Prefix is a prefix for a global topic that is available from the containing Home Cache Cluster.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| msgVpnName|x| topicPrefix|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **home_cluster_name** | **String**| The name of the remote Home Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **home_cluster_name** | **String**| The name of the remote Home Cache Cluster. | 
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

Get a list of Home Cache Cluster objects.  A Home Cache Cluster is a Cache Cluster that is the \"definitive\" Cache Cluster for a given topic in the context of the Global Caching feature.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
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

Get a Cache Instance object.  A Cache Instance is a single Cache process that belongs to a single Cache Cluster. A Cache Instance object provisioned on the broker is used to disseminate configuration information to the Cache process. Cache Instances listen for and cache live data messages that match the topic subscriptions configured for their parent Cache Cluster.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| counter.msgCount||x counter.msgPeakCount||x counter.requestQueueDepthCount||x counter.requestQueueDepthPeakCount||x counter.topicCount||x counter.topicPeakCount||x instanceName|x| msgVpnName|x| rate.averageDataRxBytePeakRate||x rate.averageDataRxByteRate||x rate.averageDataRxMsgPeakRate||x rate.averageDataRxMsgRate||x rate.averageDataTxMsgPeakRate||x rate.averageDataTxMsgRate||x rate.averageRequestRxPeakRate||x rate.averageRequestRxRate||x rate.dataRxBytePeakRate||x rate.dataRxByteRate||x rate.dataRxMsgPeakRate||x rate.dataRxMsgRate||x rate.dataTxMsgPeakRate||x rate.dataTxMsgRate||x rate.requestRxPeakRate||x rate.requestRxRate||x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **instance_name** | **String**| The name of the Cache Instance. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **instance_name** | **String**| The name of the Cache Instance. | 
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

Get a Remote Home Cache Cluster object.  A Remote Home Cache Cluster is a Home Cache Cluster that the Cache Instance is communicating with in the context of the Global Caching feature.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| instanceName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **instance_name** | **String**| The name of the Cache Instance. | 
  **home_cluster_name** | **String**| The name of the remote Home Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **instance_name** | **String**| The name of the Cache Instance. | 
 **home_cluster_name** | **String**| The name of the remote Home Cache Cluster. | 
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

Get a list of Remote Home Cache Cluster objects.  A Remote Home Cache Cluster is a Home Cache Cluster that the Cache Instance is communicating with in the context of the Global Caching feature.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| homeClusterName|x| instanceName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **instance_name** | **String**| The name of the Cache Instance. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **instance_name** | **String**| The name of the Cache Instance. | 
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

Get a Remote Topic object.  A Remote Topic is a topic for which the Cache Instance has cached messages.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| instanceName|x| msgVpnName|x| topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **instance_name** | **String**| The name of the Cache Instance. | 
  **topic** | **String**| The value of the remote Topic. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **instance_name** | **String**| The name of the Cache Instance. | 
 **topic** | **String**| The value of the remote Topic. | 
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

Get a list of Remote Topic objects.  A Remote Topic is a topic for which the Cache Instance has cached messages.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| instanceName|x| msgVpnName|x| topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **instance_name** | **String**| The name of the Cache Instance. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **instance_name** | **String**| The name of the Cache Instance. | 
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

Get a list of Cache Instance objects.  A Cache Instance is a single Cache process that belongs to a single Cache Cluster. A Cache Instance object provisioned on the broker is used to disseminate configuration information to the Cache process. Cache Instances listen for and cache live data messages that match the topic subscriptions configured for their parent Cache Cluster.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| counter.msgCount||x counter.msgPeakCount||x counter.requestQueueDepthCount||x counter.requestQueueDepthPeakCount||x counter.topicCount||x counter.topicPeakCount||x instanceName|x| msgVpnName|x| rate.averageDataRxBytePeakRate||x rate.averageDataRxByteRate||x rate.averageDataRxMsgPeakRate||x rate.averageDataRxMsgRate||x rate.averageDataTxMsgPeakRate||x rate.averageDataTxMsgRate||x rate.averageRequestRxPeakRate||x rate.averageRequestRxRate||x rate.dataRxBytePeakRate||x rate.dataRxByteRate||x rate.dataRxMsgPeakRate||x rate.dataRxMsgRate||x rate.dataTxMsgPeakRate||x rate.dataTxMsgRate||x rate.requestRxPeakRate||x rate.requestRxRate||x    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
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

Get a Topic object.  The Cache Instances that belong to the containing Cache Cluster will cache any messages published to topics that match a Topic Subscription.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x| topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
  **topic** | **String**| The value of the Topic in the form a/b/c. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
 **topic** | **String**| The value of the Topic in the form a/b/c. | 
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

Get a list of Topic objects.  The Cache Instances that belong to the containing Cache Cluster will cache any messages published to topics that match a Topic Subscription.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x| topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
  **cluster_name** | **String**| The name of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
 **cluster_name** | **String**| The name of the Cache Cluster. | 
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

Get a list of Cache Cluster objects.  A Cache Cluster is a collection of one or more Cache Instances that subscribe to exactly the same topics. Cache Instances are grouped together in a Cache Cluster for the purpose of fault tolerance and load balancing. As published messages are received, the message broker message bus sends these live data messages to the Cache Instances in the Cache Cluster. This enables client cache requests to be served by any of Cache Instances in the Cache Cluster.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the Distributed Cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the Distributed Cache. | 
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

Get a DMR Bridge object.  A DMR Bridge is required to establish a data channel over a corresponding external link to the remote node for a given Message VPN. Each DMR Bridge identifies which external link the Message VPN should use, and what the name of the equivalent Message VPN at the remote node is.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| remoteNodeName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **remote_node_name** | **String**| The name of the node at the remote end of the DMR Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **remote_node_name** | **String**| The name of the node at the remote end of the DMR Bridge. | 
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

Get a JNDI Connection Factory object.  The message broker provides an internal JNDI store for provisioned Connection Factory objects that clients can access through JNDI lookups.   Attribute|Identifying|Deprecated :---|:---:|:---: connectionFactoryName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **connection_factory_name** | **String**| The name of the JMS Connection Factory. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **connection_factory_name** | **String**| The name of the JMS Connection Factory. | 
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

Get a JNDI Queue object.  The message broker provides an internal JNDI store for provisioned Queue objects that clients can access through JNDI lookups.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The JNDI name of the JMS Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The JNDI name of the JMS Queue. | 
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

Get a JNDI Topic object.  The message broker provides an internal JNDI store for provisioned Topic objects that clients can access through JNDI lookups.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| topicName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_name** | **String**| The JNDI name of the JMS Topic. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_name** | **String**| The JNDI name of the JMS Topic. | 
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

Get an MQTT Retain Cache object.  Using MQTT retained messages allows publishing MQTT clients to indicate that a message must be stored for later delivery to subscribing clients when those subscribing clients add subscriptions matching the retained message's topic. An MQTT Retain Cache processes all retained messages for a Message VPN.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **cache_name** | **String**| The name of the MQTT Retain Cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **cache_name** | **String**| The name of the MQTT Retain Cache. | 
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

Get an MQTT Session object.  An MQTT Session object is a virtual representation of an MQTT client connection. An MQTT session holds the state of an MQTT client (that is, it is used to contain a client's QoS 0 and QoS 1 subscription sets and any undelivered QoS 1 messages).   Attribute|Identifying|Deprecated :---|:---:|:---: counter.mqttConnackErrorTxCount||x counter.mqttConnackTxCount||x counter.mqttConnectRxCount||x counter.mqttDisconnectRxCount||x counter.mqttPubcompTxCount||x counter.mqttPublishQos0RxCount||x counter.mqttPublishQos0TxCount||x counter.mqttPublishQos1RxCount||x counter.mqttPublishQos1TxCount||x counter.mqttPublishQos2RxCount||x counter.mqttPubrecTxCount||x counter.mqttPubrelRxCount||x mqttSessionClientId|x| mqttSessionVirtualRouter|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **mqtt_session_client_id** | **String**| The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | 
  **mqtt_session_virtual_router** | **String**| The virtual router of the MQTT Session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **mqtt_session_client_id** | **String**| The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | 
 **mqtt_session_virtual_router** | **String**| The virtual router of the MQTT Session. | 
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

Get a Subscription object.  An MQTT session contains a client's QoS 0 and QoS 1 subscription sets. On creation, a subscription defaults to QoS 0.   Attribute|Identifying|Deprecated :---|:---:|:---: mqttSessionClientId|x| mqttSessionVirtualRouter|x| msgVpnName|x| subscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **mqtt_session_client_id** | **String**| The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | 
  **mqtt_session_virtual_router** | **String**| The virtual router of the MQTT Session. | 
  **subscription_topic** | **String**| The MQTT subscription topic. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **mqtt_session_client_id** | **String**| The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | 
 **mqtt_session_virtual_router** | **String**| The virtual router of the MQTT Session. | 
 **subscription_topic** | **String**| The MQTT subscription topic. | 
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

Get a list of Subscription objects.  An MQTT session contains a client's QoS 0 and QoS 1 subscription sets. On creation, a subscription defaults to QoS 0.   Attribute|Identifying|Deprecated :---|:---:|:---: mqttSessionClientId|x| mqttSessionVirtualRouter|x| msgVpnName|x| subscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **mqtt_session_client_id** | **String**| The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | 
  **mqtt_session_virtual_router** | **String**| The virtual router of the MQTT Session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **mqtt_session_client_id** | **String**| The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | 
 **mqtt_session_virtual_router** | **String**| The virtual router of the MQTT Session. | 
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

# **get_msg_vpn_queue**
> ::models::MsgVpnQueueResponse get_msg_vpn_queue(ctx, msg_vpn_name, queue_name, optional)
Get a Queue object.

Get a Queue object.  A Queue acts as both a destination that clients can publish messages to, and as an endpoint that clients can bind consumers to and consume messages from.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueResponse**](MsgVpnQueueResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_msg**
> ::models::MsgVpnQueueMsgResponse get_msg_vpn_queue_msg(ctx, msg_vpn_name, queue_name, msg_id, optional)
Get a Queue Message object.

Get a Queue Message object.  A Queue Message is a packet of information sent from producers to consumers using the Queue.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
  **msg_id** | **String**| The identifier (ID) of the Message. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **msg_id** | **String**| The identifier (ID) of the Message. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueMsgResponse**](MsgVpnQueueMsgResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_msgs**
> ::models::MsgVpnQueueMsgsResponse get_msg_vpn_queue_msgs(ctx, msg_vpn_name, queue_name, optional)
Get a list of Queue Message objects.

Get a list of Queue Message objects.  A Queue Message is a packet of information sent from producers to consumers using the Queue.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueMsgsResponse**](MsgVpnQueueMsgsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_priorities**
> ::models::MsgVpnQueuePrioritiesResponse get_msg_vpn_queue_priorities(ctx, msg_vpn_name, queue_name, optional)
Get a list of Queue Priority objects.

Get a list of Queue Priority objects.  Queues can optionally support priority message delivery; all messages of a higher priority are delivered before any messages of a lower priority. A Priority object contains information about the number and size of the messages with a particular priority in the Queue.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| priority|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueuePrioritiesResponse**](MsgVpnQueuePrioritiesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_priority**
> ::models::MsgVpnQueuePriorityResponse get_msg_vpn_queue_priority(ctx, msg_vpn_name, queue_name, priority, optional)
Get a Queue Priority object.

Get a Queue Priority object.  Queues can optionally support priority message delivery; all messages of a higher priority are delivered before any messages of a lower priority. A Priority object contains information about the number and size of the messages with a particular priority in the Queue.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| priority|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
  **priority** | **String**| The level of the Priority, from 9 (highest) to 0 (lowest). | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **priority** | **String**| The level of the Priority, from 9 (highest) to 0 (lowest). | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueuePriorityResponse**](MsgVpnQueuePriorityResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_subscription**
> ::models::MsgVpnQueueSubscriptionResponse get_msg_vpn_queue_subscription(ctx, msg_vpn_name, queue_name, subscription_topic, optional)
Get a Queue Subscription object.

Get a Queue Subscription object.  One or more Queue Subscriptions can be added to a durable queue so that Guaranteed messages published to matching topics are also delivered to and spooled by the queue.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueName|x| subscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
  **subscription_topic** | **String**| The topic of the Subscription. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **subscription_topic** | **String**| The topic of the Subscription. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueSubscriptionResponse**](MsgVpnQueueSubscriptionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_subscriptions**
> ::models::MsgVpnQueueSubscriptionsResponse get_msg_vpn_queue_subscriptions(ctx, msg_vpn_name, queue_name, optional)
Get a list of Queue Subscription objects.

Get a list of Queue Subscription objects.  One or more Queue Subscriptions can be added to a durable queue so that Guaranteed messages published to matching topics are also delivered to and spooled by the queue.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueName|x| subscriptionTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueSubscriptionsResponse**](MsgVpnQueueSubscriptionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_template**
> ::models::MsgVpnQueueTemplateResponse get_msg_vpn_queue_template(ctx, msg_vpn_name, queue_template_name, optional)
Get a Queue Template object.

Get a Queue Template object.  A Queue Template provides a mechanism for specifying the initial state for client created queues.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueTemplateName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_template_name** | **String**| The name of the Queue Template. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_template_name** | **String**| The name of the Queue Template. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueTemplateResponse**](MsgVpnQueueTemplateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_tx_flow**
> ::models::MsgVpnQueueTxFlowResponse get_msg_vpn_queue_tx_flow(ctx, msg_vpn_name, queue_name, flow_id, optional)
Get a Queue Transmit Flow object.

Get a Queue Transmit Flow object.  Queue Transmit Flows are used by clients to consume Guaranteed messages from a Queue.   Attribute|Identifying|Deprecated :---|:---:|:---: flowId|x| msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
  **flow_id** | **String**| The identifier (ID) of the Flow. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **flow_id** | **String**| The identifier (ID) of the Flow. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueTxFlowResponse**](MsgVpnQueueTxFlowResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_tx_flows**
> ::models::MsgVpnQueueTxFlowsResponse get_msg_vpn_queue_tx_flows(ctx, msg_vpn_name, queue_name, optional)
Get a list of Queue Transmit Flow objects.

Get a list of Queue Transmit Flow objects.  Queue Transmit Flows are used by clients to consume Guaranteed messages from a Queue.   Attribute|Identifying|Deprecated :---|:---:|:---: flowId|x| msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **queue_name** | **String**| The name of the Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **queue_name** | **String**| The name of the Queue. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueTxFlowsResponse**](MsgVpnQueueTxFlowsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_replay_log**
> ::models::MsgVpnReplayLogResponse get_msg_vpn_replay_log(ctx, msg_vpn_name, replay_log_name, optional)
Get a Replay Log object.

Get a Replay Log object.  When the Message Replay feature is enabled, message brokers store persistent messages in a Replay Log. These messages are kept until the log is full, after which the oldest messages are removed to free up space for new messages.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| replayLogName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **replay_log_name** | **String**| The name of the Replay Log. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **replay_log_name** | **String**| The name of the Replay Log. | 
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

Get a Message object.  A Message is a packet of information sent from producers to consumers. Messages are the central units of information that clients exchange using the message broker and which are cached in the Replay Log.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| replayLogName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **replay_log_name** | **String**| The name of the Replay Log. | 
  **msg_id** | **String**| The identifier (ID) of the message. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **replay_log_name** | **String**| The name of the Replay Log. | 
 **msg_id** | **String**| The identifier (ID) of the message. | 
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

Get a list of Message objects.  A Message is a packet of information sent from producers to consumers. Messages are the central units of information that clients exchange using the message broker and which are cached in the Replay Log.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| replayLogName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **replay_log_name** | **String**| The name of the Replay Log. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **replay_log_name** | **String**| The name of the Replay Log. | 
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

# **get_msg_vpn_replicated_topic**
> ::models::MsgVpnReplicatedTopicResponse get_msg_vpn_replicated_topic(ctx, msg_vpn_name, replicated_topic, optional)
Get a Replicated Topic object.

Get a Replicated Topic object.  To indicate which messages should be replicated between the active and standby site, a Replicated Topic subscription must be configured on a Message VPN. If a published message matches both a replicated topic and an endpoint on the active site, then the message is replicated to the standby site.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| replicatedTopic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **replicated_topic** | **String**| The topic for applying replication. Published messages matching this topic will be replicated to the standby site. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **replicated_topic** | **String**| The topic for applying replication. Published messages matching this topic will be replicated to the standby site. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnReplicatedTopicResponse**](MsgVpnReplicatedTopicResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point**
> ::models::MsgVpnRestDeliveryPointResponse get_msg_vpn_rest_delivery_point(ctx, msg_vpn_name, rest_delivery_point_name, optional)
Get a REST Delivery Point object.

Get a REST Delivery Point object.  A REST Delivery Point manages delivery of messages from queues to a named list of REST Consumers.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
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

Get a Queue Binding object.  A Queue Binding for a REST Delivery Point attracts messages to be delivered to REST consumers. If the queue does not exist it can be created subsequently, and once the queue is operational the broker performs the queue binding. Removing the queue binding does not delete the queue itself. Similarly, removing the queue does not remove the queue binding, which fails until the queue is recreated or the queue binding is deleted.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueBindingName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
  **queue_binding_name** | **String**| The name of a queue in the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
 **queue_binding_name** | **String**| The name of a queue in the Message VPN. | 
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

Get a list of Queue Binding objects.  A Queue Binding for a REST Delivery Point attracts messages to be delivered to REST consumers. If the queue does not exist it can be created subsequently, and once the queue is operational the broker performs the queue binding. Removing the queue binding does not delete the queue itself. Similarly, removing the queue does not remove the queue binding, which fails until the queue is recreated or the queue binding is deleted.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueBindingName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
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

Get a REST Consumer object.  REST Consumer objects establish HTTP connectivity to REST consumer applications who wish to receive messages from a broker.   Attribute|Identifying|Deprecated :---|:---:|:---: counter.httpRequestConnectionCloseTxMsgCount||x counter.httpRequestOutstandingTxMsgCount||x counter.httpRequestTimedOutTxMsgCount||x counter.httpRequestTxByteCount||x counter.httpRequestTxMsgCount||x counter.httpResponseErrorRxMsgCount||x counter.httpResponseRxByteCount||x counter.httpResponseRxMsgCount||x counter.httpResponseSuccessRxMsgCount||x msgVpnName|x| restConsumerName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
  **rest_consumer_name** | **String**| The name of the REST Consumer. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
 **rest_consumer_name** | **String**| The name of the REST Consumer. | 
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

Get a Trusted Common Name object.  The Trusted Common Names for the REST Consumer are used by encrypted transports to verify the name in the certificate presented by the remote REST consumer. They must include the common name of the remote REST consumer's server certificate.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restConsumerName|x| restDeliveryPointName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
  **rest_consumer_name** | **String**| The name of the REST Consumer. | 
  **tls_trusted_common_name** | **String**| The expected trusted common name of the remote certificate. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
 **rest_consumer_name** | **String**| The name of the REST Consumer. | 
 **tls_trusted_common_name** | **String**| The expected trusted common name of the remote certificate. | 
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

Get a list of Trusted Common Name objects.  The Trusted Common Names for the REST Consumer are used by encrypted transports to verify the name in the certificate presented by the remote REST consumer. They must include the common name of the remote REST consumer's server certificate.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restConsumerName|x| restDeliveryPointName|x| tlsTrustedCommonName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
  **rest_consumer_name** | **String**| The name of the REST Consumer. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
 **rest_consumer_name** | **String**| The name of the REST Consumer. | 
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

Get a list of REST Consumer objects.  REST Consumer objects establish HTTP connectivity to REST consumer applications who wish to receive messages from a broker.   Attribute|Identifying|Deprecated :---|:---:|:---: counter.httpRequestConnectionCloseTxMsgCount||x counter.httpRequestOutstandingTxMsgCount||x counter.httpRequestTimedOutTxMsgCount||x counter.httpRequestTxByteCount||x counter.httpRequestTxMsgCount||x counter.httpResponseErrorRxMsgCount||x counter.httpResponseRxByteCount||x counter.httpResponseRxMsgCount||x counter.httpResponseSuccessRxMsgCount||x msgVpnName|x| restConsumerName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The name of the REST Delivery Point. | 
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

# **get_msg_vpn_topic_endpoint**
> ::models::MsgVpnTopicEndpointResponse get_msg_vpn_topic_endpoint(ctx, msg_vpn_name, topic_endpoint_name, optional)
Get a Topic Endpoint object.

Get a Topic Endpoint object.  A Topic Endpoint attracts messages published to a topic for which the Topic Endpoint has a matching topic subscription. The topic subscription for the Topic Endpoint is specified in the client request to bind a Flow to that Topic Endpoint. Queues are significantly more flexible than Topic Endpoints and are the recommended approach for most applications. The use of Topic Endpoints should be restricted to JMS applications.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointResponse**](MsgVpnTopicEndpointResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_msg**
> ::models::MsgVpnTopicEndpointMsgResponse get_msg_vpn_topic_endpoint_msg(ctx, msg_vpn_name, topic_endpoint_name, msg_id, optional)
Get a Topic Endpoint Message object.

Get a Topic Endpoint Message object.  A Topic Endpoint Message is a packet of information sent from producers to consumers using the Topic Endpoint.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
  **msg_id** | **String**| The identifier (ID) of the Message. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **msg_id** | **String**| The identifier (ID) of the Message. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointMsgResponse**](MsgVpnTopicEndpointMsgResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_msgs**
> ::models::MsgVpnTopicEndpointMsgsResponse get_msg_vpn_topic_endpoint_msgs(ctx, msg_vpn_name, topic_endpoint_name, optional)
Get a list of Topic Endpoint Message objects.

Get a list of Topic Endpoint Message objects.  A Topic Endpoint Message is a packet of information sent from producers to consumers using the Topic Endpoint.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointMsgsResponse**](MsgVpnTopicEndpointMsgsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_priorities**
> ::models::MsgVpnTopicEndpointPrioritiesResponse get_msg_vpn_topic_endpoint_priorities(ctx, msg_vpn_name, topic_endpoint_name, optional)
Get a list of Topic Endpoint Priority objects.

Get a list of Topic Endpoint Priority objects.  Topic Endpoints can optionally support priority message delivery; all messages of a higher priority are delivered before any messages of a lower priority. A Priority object contains information about the number and size of the messages with a particular priority in the Topic Endpoint.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| priority|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointPrioritiesResponse**](MsgVpnTopicEndpointPrioritiesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_priority**
> ::models::MsgVpnTopicEndpointPriorityResponse get_msg_vpn_topic_endpoint_priority(ctx, msg_vpn_name, topic_endpoint_name, priority, optional)
Get a Topic Endpoint Priority object.

Get a Topic Endpoint Priority object.  Topic Endpoints can optionally support priority message delivery; all messages of a higher priority are delivered before any messages of a lower priority. A Priority object contains information about the number and size of the messages with a particular priority in the Topic Endpoint.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| priority|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
  **priority** | **String**| The level of the Priority, from 9 (highest) to 0 (lowest). | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **priority** | **String**| The level of the Priority, from 9 (highest) to 0 (lowest). | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointPriorityResponse**](MsgVpnTopicEndpointPriorityResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_template**
> ::models::MsgVpnTopicEndpointTemplateResponse get_msg_vpn_topic_endpoint_template(ctx, msg_vpn_name, topic_endpoint_template_name, optional)
Get a Topic Endpoint Template object.

Get a Topic Endpoint Template object.  A Topic Endpoint Template provides a mechanism for specifying the initial state for client created topic endpoints.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| topicEndpointTemplateName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.14.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_endpoint_template_name** | **String**| The name of the Topic Endpoint Template. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_endpoint_template_name** | **String**| The name of the Topic Endpoint Template. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointTemplateResponse**](MsgVpnTopicEndpointTemplateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_tx_flow**
> ::models::MsgVpnTopicEndpointTxFlowResponse get_msg_vpn_topic_endpoint_tx_flow(ctx, msg_vpn_name, topic_endpoint_name, flow_id, optional)
Get a Topic Endpoint Transmit Flow object.

Get a Topic Endpoint Transmit Flow object.  Topic Endpoint Transmit Flows are used by clients to consume Guaranteed messages from a Topic Endpoint.   Attribute|Identifying|Deprecated :---|:---:|:---: flowId|x| msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
  **flow_id** | **String**| The identifier (ID) of the Flow. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **flow_id** | **String**| The identifier (ID) of the Flow. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointTxFlowResponse**](MsgVpnTopicEndpointTxFlowResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_tx_flows**
> ::models::MsgVpnTopicEndpointTxFlowsResponse get_msg_vpn_topic_endpoint_tx_flows(ctx, msg_vpn_name, topic_endpoint_name, optional)
Get a list of Topic Endpoint Transmit Flow objects.

Get a list of Topic Endpoint Transmit Flow objects.  Topic Endpoint Transmit Flows are used by clients to consume Guaranteed messages from a Topic Endpoint.   Attribute|Identifying|Deprecated :---|:---:|:---: flowId|x| msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **topic_endpoint_name** | **String**| The name of the Topic Endpoint. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointTxFlowsResponse**](MsgVpnTopicEndpointTxFlowsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_transaction**
> ::models::MsgVpnTransactionResponse get_msg_vpn_transaction(ctx, msg_vpn_name, xid, optional)
Get a Replicated Local Transaction or XA Transaction object.

Get a Replicated Local Transaction or XA Transaction object.  Transactions can be used to group a set of Guaranteed messages to be published or consumed or both as an atomic unit of work.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| xid|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **xid** | **String**| The identifier (ID) of the Transaction. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **xid** | **String**| The identifier (ID) of the Transaction. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTransactionResponse**](MsgVpnTransactionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_transaction_consumer_msg**
> ::models::MsgVpnTransactionConsumerMsgResponse get_msg_vpn_transaction_consumer_msg(ctx, msg_vpn_name, xid, msg_id, optional)
Get a Transaction Consumer Message object.

Get a Transaction Consumer Message object.  A Transaction Consumer Message is a message that will be consumed as part of this Transaction once the Transaction is committed.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| xid|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **xid** | **String**| The identifier (ID) of the Transaction. | 
  **msg_id** | **String**| The identifier (ID) of the Message. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **xid** | **String**| The identifier (ID) of the Transaction. | 
 **msg_id** | **String**| The identifier (ID) of the Message. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTransactionConsumerMsgResponse**](MsgVpnTransactionConsumerMsgResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_transaction_consumer_msgs**
> ::models::MsgVpnTransactionConsumerMsgsResponse get_msg_vpn_transaction_consumer_msgs(ctx, msg_vpn_name, xid, optional)
Get a list of Transaction Consumer Message objects.

Get a list of Transaction Consumer Message objects.  A Transaction Consumer Message is a message that will be consumed as part of this Transaction once the Transaction is committed.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| xid|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **xid** | **String**| The identifier (ID) of the Transaction. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **xid** | **String**| The identifier (ID) of the Transaction. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTransactionConsumerMsgsResponse**](MsgVpnTransactionConsumerMsgsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_transaction_publisher_msg**
> ::models::MsgVpnTransactionPublisherMsgResponse get_msg_vpn_transaction_publisher_msg(ctx, msg_vpn_name, xid, msg_id, optional)
Get a Transaction Publisher Message object.

Get a Transaction Publisher Message object.  A Transaction Publisher Message is a message that will be published as part of this Transaction once the Transaction is committed.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| xid|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **xid** | **String**| The identifier (ID) of the Transaction. | 
  **msg_id** | **String**| The identifier (ID) of the Message. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **xid** | **String**| The identifier (ID) of the Transaction. | 
 **msg_id** | **String**| The identifier (ID) of the Message. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTransactionPublisherMsgResponse**](MsgVpnTransactionPublisherMsgResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_transaction_publisher_msgs**
> ::models::MsgVpnTransactionPublisherMsgsResponse get_msg_vpn_transaction_publisher_msgs(ctx, msg_vpn_name, xid, optional)
Get a list of Transaction Publisher Message objects.

Get a list of Transaction Publisher Message objects.  A Transaction Publisher Message is a message that will be published as part of this Transaction once the Transaction is committed.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| xid|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/read-only\" is required to perform this operation.  This has been available since 2.12.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The name of the Message VPN. | 
  **xid** | **String**| The identifier (ID) of the Transaction. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The name of the Message VPN. | 
 **xid** | **String**| The identifier (ID) of the Transaction. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTransactionPublisherMsgsResponse**](MsgVpnTransactionPublisherMsgsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

