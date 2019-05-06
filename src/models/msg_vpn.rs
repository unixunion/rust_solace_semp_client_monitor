/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the  action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters \"q1\" and \"q2\" ; with values \"val1\" and \"val2\" respectively /SEMP/v2/monitor/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/monitor/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/monitor/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/monitor/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/monitor/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/monitor/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/monitor/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/monitor/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is a hidden maximum as to prevent overloading the system. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/monitor/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.    
 *
 * OpenAPI spec version: 2.11.00901000077
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpn {
  /// Indicates whether basic authentication is enabled for clients connecting to the Message VPN.
  #[serde(rename = "authenticationBasicEnabled", skip_serializing_if="Option::is_none")]
  authentication_basic_enabled: Option<bool>,
  /// The name of the RADIUS or LDAP Profile to use for basic authentication.
  #[serde(rename = "authenticationBasicProfileName", skip_serializing_if="Option::is_none")]
  authentication_basic_profile_name: Option<String>,
  /// The RADIUS domain to use for basic authentication.
  #[serde(rename = "authenticationBasicRadiusDomain", skip_serializing_if="Option::is_none")]
  authentication_basic_radius_domain: Option<String>,
  /// The type of basic authentication to use for clients connecting to the Message VPN. The allowed values and their meaning are:  <pre> \"internal\" - Internal database. Authentication is against Client Usernames. \"ldap\" - LDAP authentication. An LDAP profile name must be provided. \"radius\" - RADIUS authentication. A RADIUS profile name must be provided. \"none\" - No authentication. Anonymous login allowed. </pre> 
  #[serde(rename = "authenticationBasicType", skip_serializing_if="Option::is_none")]
  authentication_basic_type: Option<String>,
  /// Indicates whether a client is allowed to specify a Client Username via the API connect method. When disabled, the certificate CN (Common Name) is always used.
  #[serde(rename = "authenticationClientCertAllowApiProvidedUsernameEnabled", skip_serializing_if="Option::is_none")]
  authentication_client_cert_allow_api_provided_username_enabled: Option<bool>,
  /// Indicates whether client certificate authentication is enabled in the Message VPN.
  #[serde(rename = "authenticationClientCertEnabled", skip_serializing_if="Option::is_none")]
  authentication_client_cert_enabled: Option<bool>,
  /// The maximum depth for a client certificate chain. The depth of a chain is defined as the number of signing CA certificates that are present in the chain back to a trusted self-signed root CA certificate.
  #[serde(rename = "authenticationClientCertMaxChainDepth", skip_serializing_if="Option::is_none")]
  authentication_client_cert_max_chain_depth: Option<i64>,
  /// The desired behavior for client certificate revocation checking. The allowed values and their meaning are:  <pre> \"allow-all\" - Allow the client to authenticate, the result of client certificate revocation check is ignored. \"allow-unknown\" - Allow the client to authenticate even if the revocation status of his certificate cannot be determined. \"allow-valid\" - Allow the client to authenticate only when the revocation check returned an explicit positive response. </pre> 
  #[serde(rename = "authenticationClientCertRevocationCheckMode", skip_serializing_if="Option::is_none")]
  authentication_client_cert_revocation_check_mode: Option<String>,
  /// The field from the client certificate to use as the client username. The allowed values and their meaning are:  <pre> \"common-name\" - the username is extracted from the certificate's Common Name. \"subject-alternate-name-msupn\" - the username is extracted from the certificate's Other Name type of the Subject Alternative Name and must have the msUPN signature. </pre> 
  #[serde(rename = "authenticationClientCertUsernameSource", skip_serializing_if="Option::is_none")]
  authentication_client_cert_username_source: Option<String>,
  /// Indicates whether the \"Not Before\" and \"Not After\" validity dates in the client certificate are checked.
  #[serde(rename = "authenticationClientCertValidateDateEnabled", skip_serializing_if="Option::is_none")]
  authentication_client_cert_validate_date_enabled: Option<bool>,
  /// Indicates whether a client is allowed to specify a Client Username via the API connect method. When disabled, the Kerberos Principal name is always used.
  #[serde(rename = "authenticationKerberosAllowApiProvidedUsernameEnabled", skip_serializing_if="Option::is_none")]
  authentication_kerberos_allow_api_provided_username_enabled: Option<bool>,
  /// Indicates whether Kerberos authentication is enabled in the Message VPN.
  #[serde(rename = "authenticationKerberosEnabled", skip_serializing_if="Option::is_none")]
  authentication_kerberos_enabled: Option<bool>,
  /// The name of the attribute that is retrieved from the LDAP server as part of the LDAP search when authorizing a client connecting to the Message VPN.
  #[serde(rename = "authorizationLdapGroupMembershipAttributeName", skip_serializing_if="Option::is_none")]
  authorization_ldap_group_membership_attribute_name: Option<String>,
  /// The name of the LDAP Profile to use for client authorization.
  #[serde(rename = "authorizationProfileName", skip_serializing_if="Option::is_none")]
  authorization_profile_name: Option<String>,
  /// The type of authorization to use for clients connecting to the Message VPN. The allowed values and their meaning are:  <pre> \"ldap\" - LDAP authorization. \"internal\" - Internal authorization. </pre> 
  #[serde(rename = "authorizationType", skip_serializing_if="Option::is_none")]
  authorization_type: Option<String>,
  /// Indicates whether the Common Name (CN) in the server certificate from the remote broker is validated for the Bridge.
  #[serde(rename = "bridgingTlsServerCertEnforceTrustedCommonNameEnabled", skip_serializing_if="Option::is_none")]
  bridging_tls_server_cert_enforce_trusted_common_name_enabled: Option<bool>,
  /// The maximum depth for a server certificate chain. The depth of a chain is defined as the number of signing CA certificates that are present in the chain back to a trusted self-signed root CA certificate.
  #[serde(rename = "bridgingTlsServerCertMaxChainDepth", skip_serializing_if="Option::is_none")]
  bridging_tls_server_cert_max_chain_depth: Option<i64>,
  /// Indicates whether the \"Not Before\" and \"Not After\" validity dates in the server certificate are checked.
  #[serde(rename = "bridgingTlsServerCertValidateDateEnabled", skip_serializing_if="Option::is_none")]
  bridging_tls_server_cert_validate_date_enabled: Option<bool>,
  #[serde(rename = "counter", skip_serializing_if="Option::is_none")]
  counter: Option<::models::MsgVpnCounter>,
  /// Indicates whether managing of cache instances over the message bus is enabled in the Message VPN.
  #[serde(rename = "distributedCacheManagementEnabled", skip_serializing_if="Option::is_none")]
  distributed_cache_management_enabled: Option<bool>,
  /// Enable or disable Dynamic Message Routing (DMR) for the Message VPN.
  #[serde(rename = "dmrEnabled", skip_serializing_if="Option::is_none")]
  dmr_enabled: Option<bool>,
  /// Indicates whether the Message VPN is enabled.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  #[serde(rename = "eventConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  event_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventEgressFlowCountThreshold", skip_serializing_if="Option::is_none")]
  event_egress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventEgressMsgRateThreshold", skip_serializing_if="Option::is_none")]
  event_egress_msg_rate_threshold: Option<::models::EventThresholdByValue>,
  #[serde(rename = "eventEndpointCountThreshold", skip_serializing_if="Option::is_none")]
  event_endpoint_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventIngressFlowCountThreshold", skip_serializing_if="Option::is_none")]
  event_ingress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventIngressMsgRateThreshold", skip_serializing_if="Option::is_none")]
  event_ingress_msg_rate_threshold: Option<::models::EventThresholdByValue>,
  /// Exceeding this message size in kilobytes (KB) triggers a corresponding Event in the Message VPN.
  #[serde(rename = "eventLargeMsgThreshold", skip_serializing_if="Option::is_none")]
  event_large_msg_threshold: Option<i64>,
  /// The value of the prefix applied to all published Events in the Message VPN.
  #[serde(rename = "eventLogTag", skip_serializing_if="Option::is_none")]
  event_log_tag: Option<String>,
  #[serde(rename = "eventMsgSpoolUsageThreshold", skip_serializing_if="Option::is_none")]
  event_msg_spool_usage_threshold: Option<::models::EventThreshold>,
  /// Indicates whether client Events are published in the Message VPN.
  #[serde(rename = "eventPublishClientEnabled", skip_serializing_if="Option::is_none")]
  event_publish_client_enabled: Option<bool>,
  /// Indicates whether Message VPN Events are published in the Message VPN.
  #[serde(rename = "eventPublishMsgVpnEnabled", skip_serializing_if="Option::is_none")]
  event_publish_msg_vpn_enabled: Option<bool>,
  /// The mode of subscription Events published in the Message VPN. The allowed values and their meaning are:  <pre> \"off\" - Disable client level event message publishing. \"on-with-format-v1\" - Enable client level event message publishing with format v1. \"on-with-no-unsubscribe-events-on-disconnect-format-v1\" - As \"on-with-format-v1\", but unsubscribe events are not generated when a client disconnects. Unsubscribe events are still raised when a client explicitly unsubscribes from its subscriptions. \"on-with-format-v2\" - Enable client level event message publishing with format v2. \"on-with-no-unsubscribe-events-on-disconnect-format-v2\" - As \"on-with-format-v2\", but unsubscribe events are not generated when a client disconnects. Unsubscribe events are still raised when a client explicitly unsubscribes from its subscriptions. </pre> 
  #[serde(rename = "eventPublishSubscriptionMode", skip_serializing_if="Option::is_none")]
  event_publish_subscription_mode: Option<String>,
  /// Indicates whether Message VPN Events are published in the MQTT format.
  #[serde(rename = "eventPublishTopicFormatMqttEnabled", skip_serializing_if="Option::is_none")]
  event_publish_topic_format_mqtt_enabled: Option<bool>,
  /// Indicates whether Message VPN Events are published in the SMF format.
  #[serde(rename = "eventPublishTopicFormatSmfEnabled", skip_serializing_if="Option::is_none")]
  event_publish_topic_format_smf_enabled: Option<bool>,
  #[serde(rename = "eventServiceAmqpConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  event_service_amqp_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceMqttConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  event_service_mqtt_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceRestIncomingConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  event_service_rest_incoming_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceSmfConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  event_service_smf_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceWebConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  event_service_web_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventSubscriptionCountThreshold", skip_serializing_if="Option::is_none")]
  event_subscription_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventTransactedSessionCountThreshold", skip_serializing_if="Option::is_none")]
  event_transacted_session_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventTransactionCountThreshold", skip_serializing_if="Option::is_none")]
  event_transaction_count_threshold: Option<::models::EventThreshold>,
  /// Indicates whether exports of subscriptions to other routers in the network over neighbour links is enabled in the Message VPN.
  #[serde(rename = "exportSubscriptionsEnabled", skip_serializing_if="Option::is_none")]
  export_subscriptions_enabled: Option<bool>,
  /// The reason for the Message VPN failure as: Message VPN Disabled, Internal Client Not Ready, or Cluster Subscription Sync Pending.
  #[serde(rename = "failureReason", skip_serializing_if="Option::is_none")]
  failure_reason: Option<String>,
  /// Indicates whether the JNDI access for clients is enabled in the Message VPN.
  #[serde(rename = "jndiEnabled", skip_serializing_if="Option::is_none")]
  jndi_enabled: Option<bool>,
  /// The maximum number of client connections to the Message VPN.
  #[serde(rename = "maxConnectionCount", skip_serializing_if="Option::is_none")]
  max_connection_count: Option<i64>,
  /// The effective maximum number of Queues and Topic Endpoints allowed in the Message VPN.
  #[serde(rename = "maxEffectiveEndpointCount", skip_serializing_if="Option::is_none")]
  max_effective_endpoint_count: Option<i32>,
  /// The effective maximum number of receive flows allowed in the Message VPN.
  #[serde(rename = "maxEffectiveRxFlowCount", skip_serializing_if="Option::is_none")]
  max_effective_rx_flow_count: Option<i32>,
  /// The effective maximum number of subscriptions allowed in the Message VPN.
  #[serde(rename = "maxEffectiveSubscriptionCount", skip_serializing_if="Option::is_none")]
  max_effective_subscription_count: Option<i64>,
  /// The effective maximum number of transacted sessions allowed in the Message VPN.
  #[serde(rename = "maxEffectiveTransactedSessionCount", skip_serializing_if="Option::is_none")]
  max_effective_transacted_session_count: Option<i32>,
  /// The effective maximum number of transactions allowed in the Message VPN.
  #[serde(rename = "maxEffectiveTransactionCount", skip_serializing_if="Option::is_none")]
  max_effective_transaction_count: Option<i32>,
  /// The effective maximum number of transmit flows allowed in the Message VPN.
  #[serde(rename = "maxEffectiveTxFlowCount", skip_serializing_if="Option::is_none")]
  max_effective_tx_flow_count: Option<i32>,
  /// The maximum number of transmit flows that can be created in the Message VPN.
  #[serde(rename = "maxEgressFlowCount", skip_serializing_if="Option::is_none")]
  max_egress_flow_count: Option<i64>,
  /// The maximum number of Queues and Topic Endpoints that can be created in the Message VPN.
  #[serde(rename = "maxEndpointCount", skip_serializing_if="Option::is_none")]
  max_endpoint_count: Option<i64>,
  /// The maximum number of receive flows that can be created in the Message VPN.
  #[serde(rename = "maxIngressFlowCount", skip_serializing_if="Option::is_none")]
  max_ingress_flow_count: Option<i64>,
  /// The maximum message spool usage by the Message VPN, in megabytes.
  #[serde(rename = "maxMsgSpoolUsage", skip_serializing_if="Option::is_none")]
  max_msg_spool_usage: Option<i64>,
  /// The maximum number of local client subscriptions (both primary and backup) that can be added to the Message VPN.
  #[serde(rename = "maxSubscriptionCount", skip_serializing_if="Option::is_none")]
  max_subscription_count: Option<i64>,
  /// The maximum number of transacted sessions that can be created in the Message VPN.
  #[serde(rename = "maxTransactedSessionCount", skip_serializing_if="Option::is_none")]
  max_transacted_session_count: Option<i64>,
  /// The maximum number of transactions that can be created in the Message VPN.
  #[serde(rename = "maxTransactionCount", skip_serializing_if="Option::is_none")]
  max_transaction_count: Option<i64>,
  /// The maximum total memory usage of the MQTT Retain feature for this Message VPN, in MB. If the maximum memory is reached, any arriving retain messages that require more memory are discarded.  A value of -1 indicates that the memory is bounded only by the global max memory limit. A value of 0 prevents MQTT Retain from becoming operational.
  #[serde(rename = "mqttRetainMaxMemory", skip_serializing_if="Option::is_none")]
  mqtt_retain_max_memory: Option<i32>,
  /// The number of message replays that are currently active in the Message VPN.
  #[serde(rename = "msgReplayActiveCount", skip_serializing_if="Option::is_none")]
  msg_replay_active_count: Option<i32>,
  /// The number of message replays that are currently failed in the Message VPN.
  #[serde(rename = "msgReplayFailedCount", skip_serializing_if="Option::is_none")]
  msg_replay_failed_count: Option<i32>,
  /// The number of message replays that are currently initializing in the Message VPN.
  #[serde(rename = "msgReplayInitializingCount", skip_serializing_if="Option::is_none")]
  msg_replay_initializing_count: Option<i32>,
  /// The number of message replays that are pending complete in the Message VPN.
  #[serde(rename = "msgReplayPendingCompleteCount", skip_serializing_if="Option::is_none")]
  msg_replay_pending_complete_count: Option<i32>,
  /// The current message spool usage by the Message VPN, in bytes (B).
  #[serde(rename = "msgSpoolUsage", skip_serializing_if="Option::is_none")]
  msg_spool_usage: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  #[serde(rename = "rate", skip_serializing_if="Option::is_none")]
  rate: Option<::models::MsgVpnRate>,
  /// Indicates whether the Common Name (CN) in the server certificate from the remote REST Consumer is validated.
  #[serde(rename = "restTlsServerCertEnforceTrustedCommonNameEnabled", skip_serializing_if="Option::is_none")]
  rest_tls_server_cert_enforce_trusted_common_name_enabled: Option<bool>,
  /// The maximum depth for a REST Consumer server certificate chain. The depth of a chain is defined as the number of signing CA certificates that are present in the chain back to a trusted self-signed root CA certificate.
  #[serde(rename = "restTlsServerCertMaxChainDepth", skip_serializing_if="Option::is_none")]
  rest_tls_server_cert_max_chain_depth: Option<i64>,
  /// Indicates whether the \"Not Before\" and \"Not After\" validity dates in the REST Consumer server certificate are checked.
  #[serde(rename = "restTlsServerCertValidateDateEnabled", skip_serializing_if="Option::is_none")]
  rest_tls_server_cert_validate_date_enabled: Option<bool>,
  /// Indicates whether the \"admin\" level \"client\" commands are enabled for SEMP over the message bus in the Message VPN.
  #[serde(rename = "sempOverMsgBusAdminClientEnabled", skip_serializing_if="Option::is_none")]
  semp_over_msg_bus_admin_client_enabled: Option<bool>,
  /// Indicates whether the \"admin\" level \"Distributed Cache\" commands are enabled for SEMP over the message bus in the Message VPN.
  #[serde(rename = "sempOverMsgBusAdminDistributedCacheEnabled", skip_serializing_if="Option::is_none")]
  semp_over_msg_bus_admin_distributed_cache_enabled: Option<bool>,
  /// Indicates whether the \"admin\" level commands are enabled for SEMP over the message bus in the Message VPN.
  #[serde(rename = "sempOverMsgBusAdminEnabled", skip_serializing_if="Option::is_none")]
  semp_over_msg_bus_admin_enabled: Option<bool>,
  /// Indicates whether SEMP over the message bus is enabled in the Message VPN.
  #[serde(rename = "sempOverMsgBusEnabled", skip_serializing_if="Option::is_none")]
  semp_over_msg_bus_enabled: Option<bool>,
  /// Indicates whether the \"show\" level commands are enabled for SEMP over the message bus in the Message VPN.
  #[serde(rename = "sempOverMsgBusShowEnabled", skip_serializing_if="Option::is_none")]
  semp_over_msg_bus_show_enabled: Option<bool>,
  /// The maximum number of AMQP client connections that can be simultaneously connected to the Message VPN. This value may be higher than supported by the platform.
  #[serde(rename = "serviceAmqpMaxConnectionCount", skip_serializing_if="Option::is_none")]
  service_amqp_max_connection_count: Option<i64>,
  /// Indicates whether the AMQP Service is compressed in the Message VPN.
  #[serde(rename = "serviceAmqpPlainTextCompressed", skip_serializing_if="Option::is_none")]
  service_amqp_plain_text_compressed: Option<bool>,
  /// Indicates whether the AMQP Service is enabled in the Message VPN.
  #[serde(rename = "serviceAmqpPlainTextEnabled", skip_serializing_if="Option::is_none")]
  service_amqp_plain_text_enabled: Option<bool>,
  /// The reason for the AMQP Service failure in the Message VPN.
  #[serde(rename = "serviceAmqpPlainTextFailureReason", skip_serializing_if="Option::is_none")]
  service_amqp_plain_text_failure_reason: Option<String>,
  /// The port number for plain-text AMQP clients that connect to the Message VPN.
  #[serde(rename = "serviceAmqpPlainTextListenPort", skip_serializing_if="Option::is_none")]
  service_amqp_plain_text_listen_port: Option<i64>,
  /// Indicates whether the AMQP Service is operationally up in the Message VPN.
  #[serde(rename = "serviceAmqpPlainTextUp", skip_serializing_if="Option::is_none")]
  service_amqp_plain_text_up: Option<bool>,
  /// Indicates whether the TLS related AMQP Service is compressed in the Message VPN.
  #[serde(rename = "serviceAmqpTlsCompressed", skip_serializing_if="Option::is_none")]
  service_amqp_tls_compressed: Option<bool>,
  /// Indicates whether TLS is enabled for AMQP clients in the Message VPN.
  #[serde(rename = "serviceAmqpTlsEnabled", skip_serializing_if="Option::is_none")]
  service_amqp_tls_enabled: Option<bool>,
  /// The reason for the TLS related AMQP Service failure in the Message VPN.
  #[serde(rename = "serviceAmqpTlsFailureReason", skip_serializing_if="Option::is_none")]
  service_amqp_tls_failure_reason: Option<String>,
  /// The port number for AMQP clients that connect to the Message VPN over TLS.
  #[serde(rename = "serviceAmqpTlsListenPort", skip_serializing_if="Option::is_none")]
  service_amqp_tls_listen_port: Option<i64>,
  /// Indicates whether the TLS related AMQP Service is operationally up in the Message VPN.
  #[serde(rename = "serviceAmqpTlsUp", skip_serializing_if="Option::is_none")]
  service_amqp_tls_up: Option<bool>,
  /// The maximum number of MQTT client connections that can be simultaneously connected to the Message VPN.
  #[serde(rename = "serviceMqttMaxConnectionCount", skip_serializing_if="Option::is_none")]
  service_mqtt_max_connection_count: Option<i64>,
  /// Indicates whether the MQTT Service is compressed in the Message VPN.
  #[serde(rename = "serviceMqttPlainTextCompressed", skip_serializing_if="Option::is_none")]
  service_mqtt_plain_text_compressed: Option<bool>,
  /// Indicates whether the MQTT Service is enabled in the Message VPN.
  #[serde(rename = "serviceMqttPlainTextEnabled", skip_serializing_if="Option::is_none")]
  service_mqtt_plain_text_enabled: Option<bool>,
  /// The reason for the MQTT Service failure in the Message VPN.
  #[serde(rename = "serviceMqttPlainTextFailureReason", skip_serializing_if="Option::is_none")]
  service_mqtt_plain_text_failure_reason: Option<String>,
  /// The port number for plain-text MQTT clients that connect to the Message VPN.
  #[serde(rename = "serviceMqttPlainTextListenPort", skip_serializing_if="Option::is_none")]
  service_mqtt_plain_text_listen_port: Option<i64>,
  /// Indicates whether the MQTT Service is operationally up in the Message VPN.
  #[serde(rename = "serviceMqttPlainTextUp", skip_serializing_if="Option::is_none")]
  service_mqtt_plain_text_up: Option<bool>,
  /// Indicates whether the TLS related MQTT Service is compressed in the Message VPN.
  #[serde(rename = "serviceMqttTlsCompressed", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_compressed: Option<bool>,
  /// Indicates whether TLS is enabled for MQTT clients in the Message VPN.
  #[serde(rename = "serviceMqttTlsEnabled", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_enabled: Option<bool>,
  /// The reason for the TLS related MQTT Service failure in the Message VPN.
  #[serde(rename = "serviceMqttTlsFailureReason", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_failure_reason: Option<String>,
  /// The port number for MQTT clients that connect to the Message VPN over TLS.
  #[serde(rename = "serviceMqttTlsListenPort", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_listen_port: Option<i64>,
  /// Indicates whether the TLS related MQTT Service is operationally up in the Message VPN.
  #[serde(rename = "serviceMqttTlsUp", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_up: Option<bool>,
  /// Indicates whether the TLS related Web transport MQTT Service is compressed in the Message VPN.
  #[serde(rename = "serviceMqttTlsWebSocketCompressed", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_web_socket_compressed: Option<bool>,
  /// Indicates whether TLS is enabled for MQTT Web clients in the Message VPN.
  #[serde(rename = "serviceMqttTlsWebSocketEnabled", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_web_socket_enabled: Option<bool>,
  /// The reason for the TLS related Web transport MQTT Service failure in the Message VPN.
  #[serde(rename = "serviceMqttTlsWebSocketFailureReason", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_web_socket_failure_reason: Option<String>,
  /// The port number for MQTT clients that connect to the Message VPN using WebSocket over TLS.
  #[serde(rename = "serviceMqttTlsWebSocketListenPort", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_web_socket_listen_port: Option<i64>,
  /// Indicates whether the TLS related Web transport MQTT Service is operationally up in the Message VPN.
  #[serde(rename = "serviceMqttTlsWebSocketUp", skip_serializing_if="Option::is_none")]
  service_mqtt_tls_web_socket_up: Option<bool>,
  /// Indicates whether the Web transport related MQTT Service is compressed in the Message VPN.
  #[serde(rename = "serviceMqttWebSocketCompressed", skip_serializing_if="Option::is_none")]
  service_mqtt_web_socket_compressed: Option<bool>,
  /// Indicates whether the Web transport for the SMF Service is enabled in the Message VPN.
  #[serde(rename = "serviceMqttWebSocketEnabled", skip_serializing_if="Option::is_none")]
  service_mqtt_web_socket_enabled: Option<bool>,
  /// The reason for the Web transport related MQTT Service failure in the Message VPN.
  #[serde(rename = "serviceMqttWebSocketFailureReason", skip_serializing_if="Option::is_none")]
  service_mqtt_web_socket_failure_reason: Option<String>,
  /// The port number for plain-text MQTT clients that connect to the Message VPN using WebSocket.
  #[serde(rename = "serviceMqttWebSocketListenPort", skip_serializing_if="Option::is_none")]
  service_mqtt_web_socket_listen_port: Option<i64>,
  /// Indicates whether the Web transport related MQTT Service is operationally up in the Message VPN.
  #[serde(rename = "serviceMqttWebSocketUp", skip_serializing_if="Option::is_none")]
  service_mqtt_web_socket_up: Option<bool>,
  /// The maximum number of REST incoming client connections that can be simultaneously connected to the Message VPN. This value may be higher than supported by the platform.
  #[serde(rename = "serviceRestIncomingMaxConnectionCount", skip_serializing_if="Option::is_none")]
  service_rest_incoming_max_connection_count: Option<i64>,
  /// Indicates whether the incoming REST Service is compressed in the Message VPN.
  #[serde(rename = "serviceRestIncomingPlainTextCompressed", skip_serializing_if="Option::is_none")]
  service_rest_incoming_plain_text_compressed: Option<bool>,
  /// Indicates whether the REST Service is enabled in the Message VPN for incoming clients.
  #[serde(rename = "serviceRestIncomingPlainTextEnabled", skip_serializing_if="Option::is_none")]
  service_rest_incoming_plain_text_enabled: Option<bool>,
  /// The reason for the incoming REST Service failure in the Message VPN.
  #[serde(rename = "serviceRestIncomingPlainTextFailureReason", skip_serializing_if="Option::is_none")]
  service_rest_incoming_plain_text_failure_reason: Option<String>,
  /// The port number for incoming plain-text REST clients that connect to the Message VPN.
  #[serde(rename = "serviceRestIncomingPlainTextListenPort", skip_serializing_if="Option::is_none")]
  service_rest_incoming_plain_text_listen_port: Option<i64>,
  /// Indicates whether the incoming REST Service is operationally up in the Message VPN.
  #[serde(rename = "serviceRestIncomingPlainTextUp", skip_serializing_if="Option::is_none")]
  service_rest_incoming_plain_text_up: Option<bool>,
  /// Indicates whether the TLS related incoming REST Service is compressed in the Message VPN.
  #[serde(rename = "serviceRestIncomingTlsCompressed", skip_serializing_if="Option::is_none")]
  service_rest_incoming_tls_compressed: Option<bool>,
  /// Indicates whether TLS is enabled for incoming REST clients in the Message VPN.
  #[serde(rename = "serviceRestIncomingTlsEnabled", skip_serializing_if="Option::is_none")]
  service_rest_incoming_tls_enabled: Option<bool>,
  /// The reason for the TLS related incoming REST Service failure in the Message VPN.
  #[serde(rename = "serviceRestIncomingTlsFailureReason", skip_serializing_if="Option::is_none")]
  service_rest_incoming_tls_failure_reason: Option<String>,
  /// The port number for incoming REST clients that connect to the Message VPN over TLS.
  #[serde(rename = "serviceRestIncomingTlsListenPort", skip_serializing_if="Option::is_none")]
  service_rest_incoming_tls_listen_port: Option<i64>,
  /// Indicates whether the TLS related incoming REST Service is operationally up in the Message VPN.
  #[serde(rename = "serviceRestIncomingTlsUp", skip_serializing_if="Option::is_none")]
  service_rest_incoming_tls_up: Option<bool>,
  /// The REST service mode for incoming REST clients that connect to the Message VPN. The allowed values and their meaning are:  <pre> \"gateway\" - Act as a message gateway through which REST messages are propagated. \"messaging\" - Act as a message router on which REST messages are queued. </pre> 
  #[serde(rename = "serviceRestMode", skip_serializing_if="Option::is_none")]
  service_rest_mode: Option<String>,
  /// The maximum number of REST Consumer (outgoing) client connections that can be simultaneously connected to the Message VPN.
  #[serde(rename = "serviceRestOutgoingMaxConnectionCount", skip_serializing_if="Option::is_none")]
  service_rest_outgoing_max_connection_count: Option<i64>,
  /// The maximum number of SMF client connections that can be simultaneously connected to the Message VPN. This value may be higher than supported by the platform.
  #[serde(rename = "serviceSmfMaxConnectionCount", skip_serializing_if="Option::is_none")]
  service_smf_max_connection_count: Option<i64>,
  /// Indicates whether the SMF Service is enabled in the Message VPN.
  #[serde(rename = "serviceSmfPlainTextEnabled", skip_serializing_if="Option::is_none")]
  service_smf_plain_text_enabled: Option<bool>,
  /// The reason for the SMF Service failure in the Message VPN.
  #[serde(rename = "serviceSmfPlainTextFailureReason", skip_serializing_if="Option::is_none")]
  service_smf_plain_text_failure_reason: Option<String>,
  /// Indicates whether the SMF Service is operationally up in the Message VPN.
  #[serde(rename = "serviceSmfPlainTextUp", skip_serializing_if="Option::is_none")]
  service_smf_plain_text_up: Option<bool>,
  /// Indicates whether TLS is enabled for SMF clients in the Message VPN.
  #[serde(rename = "serviceSmfTlsEnabled", skip_serializing_if="Option::is_none")]
  service_smf_tls_enabled: Option<bool>,
  /// The reason for the TLS related SMF Service failure in the Message VPN.
  #[serde(rename = "serviceSmfTlsFailureReason", skip_serializing_if="Option::is_none")]
  service_smf_tls_failure_reason: Option<String>,
  /// Indicates whether the TLS related SMF Service is operationally up in the Message VPN.
  #[serde(rename = "serviceSmfTlsUp", skip_serializing_if="Option::is_none")]
  service_smf_tls_up: Option<bool>,
  /// The maximum number of Web Transport client connections that can be simultaneously connected to the Message VPN. This value may be higher than supported by the platform.
  #[serde(rename = "serviceWebMaxConnectionCount", skip_serializing_if="Option::is_none")]
  service_web_max_connection_count: Option<i64>,
  /// Indicates whether the Web transport for the SMF Service is enabled in the Message VPN.
  #[serde(rename = "serviceWebPlainTextEnabled", skip_serializing_if="Option::is_none")]
  service_web_plain_text_enabled: Option<bool>,
  /// The reason for the Web transport related SMF Service failure in the Message VPN.
  #[serde(rename = "serviceWebPlainTextFailureReason", skip_serializing_if="Option::is_none")]
  service_web_plain_text_failure_reason: Option<String>,
  /// Indicates whether the Web transport for the SMF Service is operationally up in the Message VPN.
  #[serde(rename = "serviceWebPlainTextUp", skip_serializing_if="Option::is_none")]
  service_web_plain_text_up: Option<bool>,
  /// Indicates whether TLS is enabled for SMF clients in the Message VPN that use the Web transport.
  #[serde(rename = "serviceWebTlsEnabled", skip_serializing_if="Option::is_none")]
  service_web_tls_enabled: Option<bool>,
  /// The reason for the TLS related Web transport SMF Service failure in the Message VPN.
  #[serde(rename = "serviceWebTlsFailureReason", skip_serializing_if="Option::is_none")]
  service_web_tls_failure_reason: Option<String>,
  /// Indicates whether the TLS related Web transport SMF Service is operationally up in the Message VPN.
  #[serde(rename = "serviceWebTlsUp", skip_serializing_if="Option::is_none")]
  service_web_tls_up: Option<bool>,
  /// The operational state of the local Message VPN as: up, down or standby.
  #[serde(rename = "state", skip_serializing_if="Option::is_none")]
  state: Option<String>,
  /// The progress of the subscription export task, in percent complete.
  #[serde(rename = "subscriptionExportProgress", skip_serializing_if="Option::is_none")]
  subscription_export_progress: Option<i64>,
  /// Indicates whether the Message VPN is the system manager for handling system level SEMP get requests and system level event publishing.
  #[serde(rename = "systemManager", skip_serializing_if="Option::is_none")]
  system_manager: Option<bool>,
  /// Indicates whether SMF clients connected to the Message VPN are allowed to downgrade their connections from TLS to plain text.
  #[serde(rename = "tlsAllowDowngradeToPlainTextEnabled", skip_serializing_if="Option::is_none")]
  tls_allow_downgrade_to_plain_text_enabled: Option<bool>
}

impl MsgVpn {
  pub fn new() -> MsgVpn {
    MsgVpn {
      authentication_basic_enabled: None,
      authentication_basic_profile_name: None,
      authentication_basic_radius_domain: None,
      authentication_basic_type: None,
      authentication_client_cert_allow_api_provided_username_enabled: None,
      authentication_client_cert_enabled: None,
      authentication_client_cert_max_chain_depth: None,
      authentication_client_cert_revocation_check_mode: None,
      authentication_client_cert_username_source: None,
      authentication_client_cert_validate_date_enabled: None,
      authentication_kerberos_allow_api_provided_username_enabled: None,
      authentication_kerberos_enabled: None,
      authorization_ldap_group_membership_attribute_name: None,
      authorization_profile_name: None,
      authorization_type: None,
      bridging_tls_server_cert_enforce_trusted_common_name_enabled: None,
      bridging_tls_server_cert_max_chain_depth: None,
      bridging_tls_server_cert_validate_date_enabled: None,
      counter: None,
      distributed_cache_management_enabled: None,
      dmr_enabled: None,
      enabled: None,
      event_connection_count_threshold: None,
      event_egress_flow_count_threshold: None,
      event_egress_msg_rate_threshold: None,
      event_endpoint_count_threshold: None,
      event_ingress_flow_count_threshold: None,
      event_ingress_msg_rate_threshold: None,
      event_large_msg_threshold: None,
      event_log_tag: None,
      event_msg_spool_usage_threshold: None,
      event_publish_client_enabled: None,
      event_publish_msg_vpn_enabled: None,
      event_publish_subscription_mode: None,
      event_publish_topic_format_mqtt_enabled: None,
      event_publish_topic_format_smf_enabled: None,
      event_service_amqp_connection_count_threshold: None,
      event_service_mqtt_connection_count_threshold: None,
      event_service_rest_incoming_connection_count_threshold: None,
      event_service_smf_connection_count_threshold: None,
      event_service_web_connection_count_threshold: None,
      event_subscription_count_threshold: None,
      event_transacted_session_count_threshold: None,
      event_transaction_count_threshold: None,
      export_subscriptions_enabled: None,
      failure_reason: None,
      jndi_enabled: None,
      max_connection_count: None,
      max_effective_endpoint_count: None,
      max_effective_rx_flow_count: None,
      max_effective_subscription_count: None,
      max_effective_transacted_session_count: None,
      max_effective_transaction_count: None,
      max_effective_tx_flow_count: None,
      max_egress_flow_count: None,
      max_endpoint_count: None,
      max_ingress_flow_count: None,
      max_msg_spool_usage: None,
      max_subscription_count: None,
      max_transacted_session_count: None,
      max_transaction_count: None,
      mqtt_retain_max_memory: None,
      msg_replay_active_count: None,
      msg_replay_failed_count: None,
      msg_replay_initializing_count: None,
      msg_replay_pending_complete_count: None,
      msg_spool_usage: None,
      msg_vpn_name: None,
      rate: None,
      rest_tls_server_cert_enforce_trusted_common_name_enabled: None,
      rest_tls_server_cert_max_chain_depth: None,
      rest_tls_server_cert_validate_date_enabled: None,
      semp_over_msg_bus_admin_client_enabled: None,
      semp_over_msg_bus_admin_distributed_cache_enabled: None,
      semp_over_msg_bus_admin_enabled: None,
      semp_over_msg_bus_enabled: None,
      semp_over_msg_bus_show_enabled: None,
      service_amqp_max_connection_count: None,
      service_amqp_plain_text_compressed: None,
      service_amqp_plain_text_enabled: None,
      service_amqp_plain_text_failure_reason: None,
      service_amqp_plain_text_listen_port: None,
      service_amqp_plain_text_up: None,
      service_amqp_tls_compressed: None,
      service_amqp_tls_enabled: None,
      service_amqp_tls_failure_reason: None,
      service_amqp_tls_listen_port: None,
      service_amqp_tls_up: None,
      service_mqtt_max_connection_count: None,
      service_mqtt_plain_text_compressed: None,
      service_mqtt_plain_text_enabled: None,
      service_mqtt_plain_text_failure_reason: None,
      service_mqtt_plain_text_listen_port: None,
      service_mqtt_plain_text_up: None,
      service_mqtt_tls_compressed: None,
      service_mqtt_tls_enabled: None,
      service_mqtt_tls_failure_reason: None,
      service_mqtt_tls_listen_port: None,
      service_mqtt_tls_up: None,
      service_mqtt_tls_web_socket_compressed: None,
      service_mqtt_tls_web_socket_enabled: None,
      service_mqtt_tls_web_socket_failure_reason: None,
      service_mqtt_tls_web_socket_listen_port: None,
      service_mqtt_tls_web_socket_up: None,
      service_mqtt_web_socket_compressed: None,
      service_mqtt_web_socket_enabled: None,
      service_mqtt_web_socket_failure_reason: None,
      service_mqtt_web_socket_listen_port: None,
      service_mqtt_web_socket_up: None,
      service_rest_incoming_max_connection_count: None,
      service_rest_incoming_plain_text_compressed: None,
      service_rest_incoming_plain_text_enabled: None,
      service_rest_incoming_plain_text_failure_reason: None,
      service_rest_incoming_plain_text_listen_port: None,
      service_rest_incoming_plain_text_up: None,
      service_rest_incoming_tls_compressed: None,
      service_rest_incoming_tls_enabled: None,
      service_rest_incoming_tls_failure_reason: None,
      service_rest_incoming_tls_listen_port: None,
      service_rest_incoming_tls_up: None,
      service_rest_mode: None,
      service_rest_outgoing_max_connection_count: None,
      service_smf_max_connection_count: None,
      service_smf_plain_text_enabled: None,
      service_smf_plain_text_failure_reason: None,
      service_smf_plain_text_up: None,
      service_smf_tls_enabled: None,
      service_smf_tls_failure_reason: None,
      service_smf_tls_up: None,
      service_web_max_connection_count: None,
      service_web_plain_text_enabled: None,
      service_web_plain_text_failure_reason: None,
      service_web_plain_text_up: None,
      service_web_tls_enabled: None,
      service_web_tls_failure_reason: None,
      service_web_tls_up: None,
      state: None,
      subscription_export_progress: None,
      system_manager: None,
      tls_allow_downgrade_to_plain_text_enabled: None
    }
  }

  pub fn set_authentication_basic_enabled(&mut self, authentication_basic_enabled: bool) {
    self.authentication_basic_enabled = Some(authentication_basic_enabled);
  }

  pub fn with_authentication_basic_enabled(mut self, authentication_basic_enabled: bool) -> MsgVpn {
    self.authentication_basic_enabled = Some(authentication_basic_enabled);
    self
  }

  pub fn authentication_basic_enabled(&self) -> Option<&bool> {
    self.authentication_basic_enabled.as_ref()
  }

  pub fn reset_authentication_basic_enabled(&mut self) {
    self.authentication_basic_enabled = None;
  }

  pub fn set_authentication_basic_profile_name(&mut self, authentication_basic_profile_name: String) {
    self.authentication_basic_profile_name = Some(authentication_basic_profile_name);
  }

  pub fn with_authentication_basic_profile_name(mut self, authentication_basic_profile_name: String) -> MsgVpn {
    self.authentication_basic_profile_name = Some(authentication_basic_profile_name);
    self
  }

  pub fn authentication_basic_profile_name(&self) -> Option<&String> {
    self.authentication_basic_profile_name.as_ref()
  }

  pub fn reset_authentication_basic_profile_name(&mut self) {
    self.authentication_basic_profile_name = None;
  }

  pub fn set_authentication_basic_radius_domain(&mut self, authentication_basic_radius_domain: String) {
    self.authentication_basic_radius_domain = Some(authentication_basic_radius_domain);
  }

  pub fn with_authentication_basic_radius_domain(mut self, authentication_basic_radius_domain: String) -> MsgVpn {
    self.authentication_basic_radius_domain = Some(authentication_basic_radius_domain);
    self
  }

  pub fn authentication_basic_radius_domain(&self) -> Option<&String> {
    self.authentication_basic_radius_domain.as_ref()
  }

  pub fn reset_authentication_basic_radius_domain(&mut self) {
    self.authentication_basic_radius_domain = None;
  }

  pub fn set_authentication_basic_type(&mut self, authentication_basic_type: String) {
    self.authentication_basic_type = Some(authentication_basic_type);
  }

  pub fn with_authentication_basic_type(mut self, authentication_basic_type: String) -> MsgVpn {
    self.authentication_basic_type = Some(authentication_basic_type);
    self
  }

  pub fn authentication_basic_type(&self) -> Option<&String> {
    self.authentication_basic_type.as_ref()
  }

  pub fn reset_authentication_basic_type(&mut self) {
    self.authentication_basic_type = None;
  }

  pub fn set_authentication_client_cert_allow_api_provided_username_enabled(&mut self, authentication_client_cert_allow_api_provided_username_enabled: bool) {
    self.authentication_client_cert_allow_api_provided_username_enabled = Some(authentication_client_cert_allow_api_provided_username_enabled);
  }

  pub fn with_authentication_client_cert_allow_api_provided_username_enabled(mut self, authentication_client_cert_allow_api_provided_username_enabled: bool) -> MsgVpn {
    self.authentication_client_cert_allow_api_provided_username_enabled = Some(authentication_client_cert_allow_api_provided_username_enabled);
    self
  }

  pub fn authentication_client_cert_allow_api_provided_username_enabled(&self) -> Option<&bool> {
    self.authentication_client_cert_allow_api_provided_username_enabled.as_ref()
  }

  pub fn reset_authentication_client_cert_allow_api_provided_username_enabled(&mut self) {
    self.authentication_client_cert_allow_api_provided_username_enabled = None;
  }

  pub fn set_authentication_client_cert_enabled(&mut self, authentication_client_cert_enabled: bool) {
    self.authentication_client_cert_enabled = Some(authentication_client_cert_enabled);
  }

  pub fn with_authentication_client_cert_enabled(mut self, authentication_client_cert_enabled: bool) -> MsgVpn {
    self.authentication_client_cert_enabled = Some(authentication_client_cert_enabled);
    self
  }

  pub fn authentication_client_cert_enabled(&self) -> Option<&bool> {
    self.authentication_client_cert_enabled.as_ref()
  }

  pub fn reset_authentication_client_cert_enabled(&mut self) {
    self.authentication_client_cert_enabled = None;
  }

  pub fn set_authentication_client_cert_max_chain_depth(&mut self, authentication_client_cert_max_chain_depth: i64) {
    self.authentication_client_cert_max_chain_depth = Some(authentication_client_cert_max_chain_depth);
  }

  pub fn with_authentication_client_cert_max_chain_depth(mut self, authentication_client_cert_max_chain_depth: i64) -> MsgVpn {
    self.authentication_client_cert_max_chain_depth = Some(authentication_client_cert_max_chain_depth);
    self
  }

  pub fn authentication_client_cert_max_chain_depth(&self) -> Option<&i64> {
    self.authentication_client_cert_max_chain_depth.as_ref()
  }

  pub fn reset_authentication_client_cert_max_chain_depth(&mut self) {
    self.authentication_client_cert_max_chain_depth = None;
  }

  pub fn set_authentication_client_cert_revocation_check_mode(&mut self, authentication_client_cert_revocation_check_mode: String) {
    self.authentication_client_cert_revocation_check_mode = Some(authentication_client_cert_revocation_check_mode);
  }

  pub fn with_authentication_client_cert_revocation_check_mode(mut self, authentication_client_cert_revocation_check_mode: String) -> MsgVpn {
    self.authentication_client_cert_revocation_check_mode = Some(authentication_client_cert_revocation_check_mode);
    self
  }

  pub fn authentication_client_cert_revocation_check_mode(&self) -> Option<&String> {
    self.authentication_client_cert_revocation_check_mode.as_ref()
  }

  pub fn reset_authentication_client_cert_revocation_check_mode(&mut self) {
    self.authentication_client_cert_revocation_check_mode = None;
  }

  pub fn set_authentication_client_cert_username_source(&mut self, authentication_client_cert_username_source: String) {
    self.authentication_client_cert_username_source = Some(authentication_client_cert_username_source);
  }

  pub fn with_authentication_client_cert_username_source(mut self, authentication_client_cert_username_source: String) -> MsgVpn {
    self.authentication_client_cert_username_source = Some(authentication_client_cert_username_source);
    self
  }

  pub fn authentication_client_cert_username_source(&self) -> Option<&String> {
    self.authentication_client_cert_username_source.as_ref()
  }

  pub fn reset_authentication_client_cert_username_source(&mut self) {
    self.authentication_client_cert_username_source = None;
  }

  pub fn set_authentication_client_cert_validate_date_enabled(&mut self, authentication_client_cert_validate_date_enabled: bool) {
    self.authentication_client_cert_validate_date_enabled = Some(authentication_client_cert_validate_date_enabled);
  }

  pub fn with_authentication_client_cert_validate_date_enabled(mut self, authentication_client_cert_validate_date_enabled: bool) -> MsgVpn {
    self.authentication_client_cert_validate_date_enabled = Some(authentication_client_cert_validate_date_enabled);
    self
  }

  pub fn authentication_client_cert_validate_date_enabled(&self) -> Option<&bool> {
    self.authentication_client_cert_validate_date_enabled.as_ref()
  }

  pub fn reset_authentication_client_cert_validate_date_enabled(&mut self) {
    self.authentication_client_cert_validate_date_enabled = None;
  }

  pub fn set_authentication_kerberos_allow_api_provided_username_enabled(&mut self, authentication_kerberos_allow_api_provided_username_enabled: bool) {
    self.authentication_kerberos_allow_api_provided_username_enabled = Some(authentication_kerberos_allow_api_provided_username_enabled);
  }

  pub fn with_authentication_kerberos_allow_api_provided_username_enabled(mut self, authentication_kerberos_allow_api_provided_username_enabled: bool) -> MsgVpn {
    self.authentication_kerberos_allow_api_provided_username_enabled = Some(authentication_kerberos_allow_api_provided_username_enabled);
    self
  }

  pub fn authentication_kerberos_allow_api_provided_username_enabled(&self) -> Option<&bool> {
    self.authentication_kerberos_allow_api_provided_username_enabled.as_ref()
  }

  pub fn reset_authentication_kerberos_allow_api_provided_username_enabled(&mut self) {
    self.authentication_kerberos_allow_api_provided_username_enabled = None;
  }

  pub fn set_authentication_kerberos_enabled(&mut self, authentication_kerberos_enabled: bool) {
    self.authentication_kerberos_enabled = Some(authentication_kerberos_enabled);
  }

  pub fn with_authentication_kerberos_enabled(mut self, authentication_kerberos_enabled: bool) -> MsgVpn {
    self.authentication_kerberos_enabled = Some(authentication_kerberos_enabled);
    self
  }

  pub fn authentication_kerberos_enabled(&self) -> Option<&bool> {
    self.authentication_kerberos_enabled.as_ref()
  }

  pub fn reset_authentication_kerberos_enabled(&mut self) {
    self.authentication_kerberos_enabled = None;
  }

  pub fn set_authorization_ldap_group_membership_attribute_name(&mut self, authorization_ldap_group_membership_attribute_name: String) {
    self.authorization_ldap_group_membership_attribute_name = Some(authorization_ldap_group_membership_attribute_name);
  }

  pub fn with_authorization_ldap_group_membership_attribute_name(mut self, authorization_ldap_group_membership_attribute_name: String) -> MsgVpn {
    self.authorization_ldap_group_membership_attribute_name = Some(authorization_ldap_group_membership_attribute_name);
    self
  }

  pub fn authorization_ldap_group_membership_attribute_name(&self) -> Option<&String> {
    self.authorization_ldap_group_membership_attribute_name.as_ref()
  }

  pub fn reset_authorization_ldap_group_membership_attribute_name(&mut self) {
    self.authorization_ldap_group_membership_attribute_name = None;
  }

  pub fn set_authorization_profile_name(&mut self, authorization_profile_name: String) {
    self.authorization_profile_name = Some(authorization_profile_name);
  }

  pub fn with_authorization_profile_name(mut self, authorization_profile_name: String) -> MsgVpn {
    self.authorization_profile_name = Some(authorization_profile_name);
    self
  }

  pub fn authorization_profile_name(&self) -> Option<&String> {
    self.authorization_profile_name.as_ref()
  }

  pub fn reset_authorization_profile_name(&mut self) {
    self.authorization_profile_name = None;
  }

  pub fn set_authorization_type(&mut self, authorization_type: String) {
    self.authorization_type = Some(authorization_type);
  }

  pub fn with_authorization_type(mut self, authorization_type: String) -> MsgVpn {
    self.authorization_type = Some(authorization_type);
    self
  }

  pub fn authorization_type(&self) -> Option<&String> {
    self.authorization_type.as_ref()
  }

  pub fn reset_authorization_type(&mut self) {
    self.authorization_type = None;
  }

  pub fn set_bridging_tls_server_cert_enforce_trusted_common_name_enabled(&mut self, bridging_tls_server_cert_enforce_trusted_common_name_enabled: bool) {
    self.bridging_tls_server_cert_enforce_trusted_common_name_enabled = Some(bridging_tls_server_cert_enforce_trusted_common_name_enabled);
  }

  pub fn with_bridging_tls_server_cert_enforce_trusted_common_name_enabled(mut self, bridging_tls_server_cert_enforce_trusted_common_name_enabled: bool) -> MsgVpn {
    self.bridging_tls_server_cert_enforce_trusted_common_name_enabled = Some(bridging_tls_server_cert_enforce_trusted_common_name_enabled);
    self
  }

  pub fn bridging_tls_server_cert_enforce_trusted_common_name_enabled(&self) -> Option<&bool> {
    self.bridging_tls_server_cert_enforce_trusted_common_name_enabled.as_ref()
  }

  pub fn reset_bridging_tls_server_cert_enforce_trusted_common_name_enabled(&mut self) {
    self.bridging_tls_server_cert_enforce_trusted_common_name_enabled = None;
  }

  pub fn set_bridging_tls_server_cert_max_chain_depth(&mut self, bridging_tls_server_cert_max_chain_depth: i64) {
    self.bridging_tls_server_cert_max_chain_depth = Some(bridging_tls_server_cert_max_chain_depth);
  }

  pub fn with_bridging_tls_server_cert_max_chain_depth(mut self, bridging_tls_server_cert_max_chain_depth: i64) -> MsgVpn {
    self.bridging_tls_server_cert_max_chain_depth = Some(bridging_tls_server_cert_max_chain_depth);
    self
  }

  pub fn bridging_tls_server_cert_max_chain_depth(&self) -> Option<&i64> {
    self.bridging_tls_server_cert_max_chain_depth.as_ref()
  }

  pub fn reset_bridging_tls_server_cert_max_chain_depth(&mut self) {
    self.bridging_tls_server_cert_max_chain_depth = None;
  }

  pub fn set_bridging_tls_server_cert_validate_date_enabled(&mut self, bridging_tls_server_cert_validate_date_enabled: bool) {
    self.bridging_tls_server_cert_validate_date_enabled = Some(bridging_tls_server_cert_validate_date_enabled);
  }

  pub fn with_bridging_tls_server_cert_validate_date_enabled(mut self, bridging_tls_server_cert_validate_date_enabled: bool) -> MsgVpn {
    self.bridging_tls_server_cert_validate_date_enabled = Some(bridging_tls_server_cert_validate_date_enabled);
    self
  }

  pub fn bridging_tls_server_cert_validate_date_enabled(&self) -> Option<&bool> {
    self.bridging_tls_server_cert_validate_date_enabled.as_ref()
  }

  pub fn reset_bridging_tls_server_cert_validate_date_enabled(&mut self) {
    self.bridging_tls_server_cert_validate_date_enabled = None;
  }

  pub fn set_counter(&mut self, counter: ::models::MsgVpnCounter) {
    self.counter = Some(counter);
  }

  pub fn with_counter(mut self, counter: ::models::MsgVpnCounter) -> MsgVpn {
    self.counter = Some(counter);
    self
  }

  pub fn counter(&self) -> Option<&::models::MsgVpnCounter> {
    self.counter.as_ref()
  }

  pub fn reset_counter(&mut self) {
    self.counter = None;
  }

  pub fn set_distributed_cache_management_enabled(&mut self, distributed_cache_management_enabled: bool) {
    self.distributed_cache_management_enabled = Some(distributed_cache_management_enabled);
  }

  pub fn with_distributed_cache_management_enabled(mut self, distributed_cache_management_enabled: bool) -> MsgVpn {
    self.distributed_cache_management_enabled = Some(distributed_cache_management_enabled);
    self
  }

  pub fn distributed_cache_management_enabled(&self) -> Option<&bool> {
    self.distributed_cache_management_enabled.as_ref()
  }

  pub fn reset_distributed_cache_management_enabled(&mut self) {
    self.distributed_cache_management_enabled = None;
  }

  pub fn set_dmr_enabled(&mut self, dmr_enabled: bool) {
    self.dmr_enabled = Some(dmr_enabled);
  }

  pub fn with_dmr_enabled(mut self, dmr_enabled: bool) -> MsgVpn {
    self.dmr_enabled = Some(dmr_enabled);
    self
  }

  pub fn dmr_enabled(&self) -> Option<&bool> {
    self.dmr_enabled.as_ref()
  }

  pub fn reset_dmr_enabled(&mut self) {
    self.dmr_enabled = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpn {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_event_connection_count_threshold(&mut self, event_connection_count_threshold: ::models::EventThreshold) {
    self.event_connection_count_threshold = Some(event_connection_count_threshold);
  }

  pub fn with_event_connection_count_threshold(mut self, event_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_connection_count_threshold = Some(event_connection_count_threshold);
    self
  }

  pub fn event_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_connection_count_threshold.as_ref()
  }

  pub fn reset_event_connection_count_threshold(&mut self) {
    self.event_connection_count_threshold = None;
  }

  pub fn set_event_egress_flow_count_threshold(&mut self, event_egress_flow_count_threshold: ::models::EventThreshold) {
    self.event_egress_flow_count_threshold = Some(event_egress_flow_count_threshold);
  }

  pub fn with_event_egress_flow_count_threshold(mut self, event_egress_flow_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_egress_flow_count_threshold = Some(event_egress_flow_count_threshold);
    self
  }

  pub fn event_egress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_egress_flow_count_threshold.as_ref()
  }

  pub fn reset_event_egress_flow_count_threshold(&mut self) {
    self.event_egress_flow_count_threshold = None;
  }

  pub fn set_event_egress_msg_rate_threshold(&mut self, event_egress_msg_rate_threshold: ::models::EventThresholdByValue) {
    self.event_egress_msg_rate_threshold = Some(event_egress_msg_rate_threshold);
  }

  pub fn with_event_egress_msg_rate_threshold(mut self, event_egress_msg_rate_threshold: ::models::EventThresholdByValue) -> MsgVpn {
    self.event_egress_msg_rate_threshold = Some(event_egress_msg_rate_threshold);
    self
  }

  pub fn event_egress_msg_rate_threshold(&self) -> Option<&::models::EventThresholdByValue> {
    self.event_egress_msg_rate_threshold.as_ref()
  }

  pub fn reset_event_egress_msg_rate_threshold(&mut self) {
    self.event_egress_msg_rate_threshold = None;
  }

  pub fn set_event_endpoint_count_threshold(&mut self, event_endpoint_count_threshold: ::models::EventThreshold) {
    self.event_endpoint_count_threshold = Some(event_endpoint_count_threshold);
  }

  pub fn with_event_endpoint_count_threshold(mut self, event_endpoint_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_endpoint_count_threshold = Some(event_endpoint_count_threshold);
    self
  }

  pub fn event_endpoint_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_endpoint_count_threshold.as_ref()
  }

  pub fn reset_event_endpoint_count_threshold(&mut self) {
    self.event_endpoint_count_threshold = None;
  }

  pub fn set_event_ingress_flow_count_threshold(&mut self, event_ingress_flow_count_threshold: ::models::EventThreshold) {
    self.event_ingress_flow_count_threshold = Some(event_ingress_flow_count_threshold);
  }

  pub fn with_event_ingress_flow_count_threshold(mut self, event_ingress_flow_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_ingress_flow_count_threshold = Some(event_ingress_flow_count_threshold);
    self
  }

  pub fn event_ingress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_ingress_flow_count_threshold.as_ref()
  }

  pub fn reset_event_ingress_flow_count_threshold(&mut self) {
    self.event_ingress_flow_count_threshold = None;
  }

  pub fn set_event_ingress_msg_rate_threshold(&mut self, event_ingress_msg_rate_threshold: ::models::EventThresholdByValue) {
    self.event_ingress_msg_rate_threshold = Some(event_ingress_msg_rate_threshold);
  }

  pub fn with_event_ingress_msg_rate_threshold(mut self, event_ingress_msg_rate_threshold: ::models::EventThresholdByValue) -> MsgVpn {
    self.event_ingress_msg_rate_threshold = Some(event_ingress_msg_rate_threshold);
    self
  }

  pub fn event_ingress_msg_rate_threshold(&self) -> Option<&::models::EventThresholdByValue> {
    self.event_ingress_msg_rate_threshold.as_ref()
  }

  pub fn reset_event_ingress_msg_rate_threshold(&mut self) {
    self.event_ingress_msg_rate_threshold = None;
  }

  pub fn set_event_large_msg_threshold(&mut self, event_large_msg_threshold: i64) {
    self.event_large_msg_threshold = Some(event_large_msg_threshold);
  }

  pub fn with_event_large_msg_threshold(mut self, event_large_msg_threshold: i64) -> MsgVpn {
    self.event_large_msg_threshold = Some(event_large_msg_threshold);
    self
  }

  pub fn event_large_msg_threshold(&self) -> Option<&i64> {
    self.event_large_msg_threshold.as_ref()
  }

  pub fn reset_event_large_msg_threshold(&mut self) {
    self.event_large_msg_threshold = None;
  }

  pub fn set_event_log_tag(&mut self, event_log_tag: String) {
    self.event_log_tag = Some(event_log_tag);
  }

  pub fn with_event_log_tag(mut self, event_log_tag: String) -> MsgVpn {
    self.event_log_tag = Some(event_log_tag);
    self
  }

  pub fn event_log_tag(&self) -> Option<&String> {
    self.event_log_tag.as_ref()
  }

  pub fn reset_event_log_tag(&mut self) {
    self.event_log_tag = None;
  }

  pub fn set_event_msg_spool_usage_threshold(&mut self, event_msg_spool_usage_threshold: ::models::EventThreshold) {
    self.event_msg_spool_usage_threshold = Some(event_msg_spool_usage_threshold);
  }

  pub fn with_event_msg_spool_usage_threshold(mut self, event_msg_spool_usage_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_msg_spool_usage_threshold = Some(event_msg_spool_usage_threshold);
    self
  }

  pub fn event_msg_spool_usage_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_msg_spool_usage_threshold.as_ref()
  }

  pub fn reset_event_msg_spool_usage_threshold(&mut self) {
    self.event_msg_spool_usage_threshold = None;
  }

  pub fn set_event_publish_client_enabled(&mut self, event_publish_client_enabled: bool) {
    self.event_publish_client_enabled = Some(event_publish_client_enabled);
  }

  pub fn with_event_publish_client_enabled(mut self, event_publish_client_enabled: bool) -> MsgVpn {
    self.event_publish_client_enabled = Some(event_publish_client_enabled);
    self
  }

  pub fn event_publish_client_enabled(&self) -> Option<&bool> {
    self.event_publish_client_enabled.as_ref()
  }

  pub fn reset_event_publish_client_enabled(&mut self) {
    self.event_publish_client_enabled = None;
  }

  pub fn set_event_publish_msg_vpn_enabled(&mut self, event_publish_msg_vpn_enabled: bool) {
    self.event_publish_msg_vpn_enabled = Some(event_publish_msg_vpn_enabled);
  }

  pub fn with_event_publish_msg_vpn_enabled(mut self, event_publish_msg_vpn_enabled: bool) -> MsgVpn {
    self.event_publish_msg_vpn_enabled = Some(event_publish_msg_vpn_enabled);
    self
  }

  pub fn event_publish_msg_vpn_enabled(&self) -> Option<&bool> {
    self.event_publish_msg_vpn_enabled.as_ref()
  }

  pub fn reset_event_publish_msg_vpn_enabled(&mut self) {
    self.event_publish_msg_vpn_enabled = None;
  }

  pub fn set_event_publish_subscription_mode(&mut self, event_publish_subscription_mode: String) {
    self.event_publish_subscription_mode = Some(event_publish_subscription_mode);
  }

  pub fn with_event_publish_subscription_mode(mut self, event_publish_subscription_mode: String) -> MsgVpn {
    self.event_publish_subscription_mode = Some(event_publish_subscription_mode);
    self
  }

  pub fn event_publish_subscription_mode(&self) -> Option<&String> {
    self.event_publish_subscription_mode.as_ref()
  }

  pub fn reset_event_publish_subscription_mode(&mut self) {
    self.event_publish_subscription_mode = None;
  }

  pub fn set_event_publish_topic_format_mqtt_enabled(&mut self, event_publish_topic_format_mqtt_enabled: bool) {
    self.event_publish_topic_format_mqtt_enabled = Some(event_publish_topic_format_mqtt_enabled);
  }

  pub fn with_event_publish_topic_format_mqtt_enabled(mut self, event_publish_topic_format_mqtt_enabled: bool) -> MsgVpn {
    self.event_publish_topic_format_mqtt_enabled = Some(event_publish_topic_format_mqtt_enabled);
    self
  }

  pub fn event_publish_topic_format_mqtt_enabled(&self) -> Option<&bool> {
    self.event_publish_topic_format_mqtt_enabled.as_ref()
  }

  pub fn reset_event_publish_topic_format_mqtt_enabled(&mut self) {
    self.event_publish_topic_format_mqtt_enabled = None;
  }

  pub fn set_event_publish_topic_format_smf_enabled(&mut self, event_publish_topic_format_smf_enabled: bool) {
    self.event_publish_topic_format_smf_enabled = Some(event_publish_topic_format_smf_enabled);
  }

  pub fn with_event_publish_topic_format_smf_enabled(mut self, event_publish_topic_format_smf_enabled: bool) -> MsgVpn {
    self.event_publish_topic_format_smf_enabled = Some(event_publish_topic_format_smf_enabled);
    self
  }

  pub fn event_publish_topic_format_smf_enabled(&self) -> Option<&bool> {
    self.event_publish_topic_format_smf_enabled.as_ref()
  }

  pub fn reset_event_publish_topic_format_smf_enabled(&mut self) {
    self.event_publish_topic_format_smf_enabled = None;
  }

  pub fn set_event_service_amqp_connection_count_threshold(&mut self, event_service_amqp_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_amqp_connection_count_threshold = Some(event_service_amqp_connection_count_threshold);
  }

  pub fn with_event_service_amqp_connection_count_threshold(mut self, event_service_amqp_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_amqp_connection_count_threshold = Some(event_service_amqp_connection_count_threshold);
    self
  }

  pub fn event_service_amqp_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_amqp_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_amqp_connection_count_threshold(&mut self) {
    self.event_service_amqp_connection_count_threshold = None;
  }

  pub fn set_event_service_mqtt_connection_count_threshold(&mut self, event_service_mqtt_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_mqtt_connection_count_threshold = Some(event_service_mqtt_connection_count_threshold);
  }

  pub fn with_event_service_mqtt_connection_count_threshold(mut self, event_service_mqtt_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_mqtt_connection_count_threshold = Some(event_service_mqtt_connection_count_threshold);
    self
  }

  pub fn event_service_mqtt_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_mqtt_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_mqtt_connection_count_threshold(&mut self) {
    self.event_service_mqtt_connection_count_threshold = None;
  }

  pub fn set_event_service_rest_incoming_connection_count_threshold(&mut self, event_service_rest_incoming_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_rest_incoming_connection_count_threshold = Some(event_service_rest_incoming_connection_count_threshold);
  }

  pub fn with_event_service_rest_incoming_connection_count_threshold(mut self, event_service_rest_incoming_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_rest_incoming_connection_count_threshold = Some(event_service_rest_incoming_connection_count_threshold);
    self
  }

  pub fn event_service_rest_incoming_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_rest_incoming_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_rest_incoming_connection_count_threshold(&mut self) {
    self.event_service_rest_incoming_connection_count_threshold = None;
  }

  pub fn set_event_service_smf_connection_count_threshold(&mut self, event_service_smf_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_smf_connection_count_threshold = Some(event_service_smf_connection_count_threshold);
  }

  pub fn with_event_service_smf_connection_count_threshold(mut self, event_service_smf_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_smf_connection_count_threshold = Some(event_service_smf_connection_count_threshold);
    self
  }

  pub fn event_service_smf_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_smf_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_smf_connection_count_threshold(&mut self) {
    self.event_service_smf_connection_count_threshold = None;
  }

  pub fn set_event_service_web_connection_count_threshold(&mut self, event_service_web_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_web_connection_count_threshold = Some(event_service_web_connection_count_threshold);
  }

  pub fn with_event_service_web_connection_count_threshold(mut self, event_service_web_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_web_connection_count_threshold = Some(event_service_web_connection_count_threshold);
    self
  }

  pub fn event_service_web_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_web_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_web_connection_count_threshold(&mut self) {
    self.event_service_web_connection_count_threshold = None;
  }

  pub fn set_event_subscription_count_threshold(&mut self, event_subscription_count_threshold: ::models::EventThreshold) {
    self.event_subscription_count_threshold = Some(event_subscription_count_threshold);
  }

  pub fn with_event_subscription_count_threshold(mut self, event_subscription_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_subscription_count_threshold = Some(event_subscription_count_threshold);
    self
  }

  pub fn event_subscription_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_subscription_count_threshold.as_ref()
  }

  pub fn reset_event_subscription_count_threshold(&mut self) {
    self.event_subscription_count_threshold = None;
  }

  pub fn set_event_transacted_session_count_threshold(&mut self, event_transacted_session_count_threshold: ::models::EventThreshold) {
    self.event_transacted_session_count_threshold = Some(event_transacted_session_count_threshold);
  }

  pub fn with_event_transacted_session_count_threshold(mut self, event_transacted_session_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_transacted_session_count_threshold = Some(event_transacted_session_count_threshold);
    self
  }

  pub fn event_transacted_session_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_transacted_session_count_threshold.as_ref()
  }

  pub fn reset_event_transacted_session_count_threshold(&mut self) {
    self.event_transacted_session_count_threshold = None;
  }

  pub fn set_event_transaction_count_threshold(&mut self, event_transaction_count_threshold: ::models::EventThreshold) {
    self.event_transaction_count_threshold = Some(event_transaction_count_threshold);
  }

  pub fn with_event_transaction_count_threshold(mut self, event_transaction_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_transaction_count_threshold = Some(event_transaction_count_threshold);
    self
  }

  pub fn event_transaction_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_transaction_count_threshold.as_ref()
  }

  pub fn reset_event_transaction_count_threshold(&mut self) {
    self.event_transaction_count_threshold = None;
  }

  pub fn set_export_subscriptions_enabled(&mut self, export_subscriptions_enabled: bool) {
    self.export_subscriptions_enabled = Some(export_subscriptions_enabled);
  }

  pub fn with_export_subscriptions_enabled(mut self, export_subscriptions_enabled: bool) -> MsgVpn {
    self.export_subscriptions_enabled = Some(export_subscriptions_enabled);
    self
  }

  pub fn export_subscriptions_enabled(&self) -> Option<&bool> {
    self.export_subscriptions_enabled.as_ref()
  }

  pub fn reset_export_subscriptions_enabled(&mut self) {
    self.export_subscriptions_enabled = None;
  }

  pub fn set_failure_reason(&mut self, failure_reason: String) {
    self.failure_reason = Some(failure_reason);
  }

  pub fn with_failure_reason(mut self, failure_reason: String) -> MsgVpn {
    self.failure_reason = Some(failure_reason);
    self
  }

  pub fn failure_reason(&self) -> Option<&String> {
    self.failure_reason.as_ref()
  }

  pub fn reset_failure_reason(&mut self) {
    self.failure_reason = None;
  }

  pub fn set_jndi_enabled(&mut self, jndi_enabled: bool) {
    self.jndi_enabled = Some(jndi_enabled);
  }

  pub fn with_jndi_enabled(mut self, jndi_enabled: bool) -> MsgVpn {
    self.jndi_enabled = Some(jndi_enabled);
    self
  }

  pub fn jndi_enabled(&self) -> Option<&bool> {
    self.jndi_enabled.as_ref()
  }

  pub fn reset_jndi_enabled(&mut self) {
    self.jndi_enabled = None;
  }

  pub fn set_max_connection_count(&mut self, max_connection_count: i64) {
    self.max_connection_count = Some(max_connection_count);
  }

  pub fn with_max_connection_count(mut self, max_connection_count: i64) -> MsgVpn {
    self.max_connection_count = Some(max_connection_count);
    self
  }

  pub fn max_connection_count(&self) -> Option<&i64> {
    self.max_connection_count.as_ref()
  }

  pub fn reset_max_connection_count(&mut self) {
    self.max_connection_count = None;
  }

  pub fn set_max_effective_endpoint_count(&mut self, max_effective_endpoint_count: i32) {
    self.max_effective_endpoint_count = Some(max_effective_endpoint_count);
  }

  pub fn with_max_effective_endpoint_count(mut self, max_effective_endpoint_count: i32) -> MsgVpn {
    self.max_effective_endpoint_count = Some(max_effective_endpoint_count);
    self
  }

  pub fn max_effective_endpoint_count(&self) -> Option<&i32> {
    self.max_effective_endpoint_count.as_ref()
  }

  pub fn reset_max_effective_endpoint_count(&mut self) {
    self.max_effective_endpoint_count = None;
  }

  pub fn set_max_effective_rx_flow_count(&mut self, max_effective_rx_flow_count: i32) {
    self.max_effective_rx_flow_count = Some(max_effective_rx_flow_count);
  }

  pub fn with_max_effective_rx_flow_count(mut self, max_effective_rx_flow_count: i32) -> MsgVpn {
    self.max_effective_rx_flow_count = Some(max_effective_rx_flow_count);
    self
  }

  pub fn max_effective_rx_flow_count(&self) -> Option<&i32> {
    self.max_effective_rx_flow_count.as_ref()
  }

  pub fn reset_max_effective_rx_flow_count(&mut self) {
    self.max_effective_rx_flow_count = None;
  }

  pub fn set_max_effective_subscription_count(&mut self, max_effective_subscription_count: i64) {
    self.max_effective_subscription_count = Some(max_effective_subscription_count);
  }

  pub fn with_max_effective_subscription_count(mut self, max_effective_subscription_count: i64) -> MsgVpn {
    self.max_effective_subscription_count = Some(max_effective_subscription_count);
    self
  }

  pub fn max_effective_subscription_count(&self) -> Option<&i64> {
    self.max_effective_subscription_count.as_ref()
  }

  pub fn reset_max_effective_subscription_count(&mut self) {
    self.max_effective_subscription_count = None;
  }

  pub fn set_max_effective_transacted_session_count(&mut self, max_effective_transacted_session_count: i32) {
    self.max_effective_transacted_session_count = Some(max_effective_transacted_session_count);
  }

  pub fn with_max_effective_transacted_session_count(mut self, max_effective_transacted_session_count: i32) -> MsgVpn {
    self.max_effective_transacted_session_count = Some(max_effective_transacted_session_count);
    self
  }

  pub fn max_effective_transacted_session_count(&self) -> Option<&i32> {
    self.max_effective_transacted_session_count.as_ref()
  }

  pub fn reset_max_effective_transacted_session_count(&mut self) {
    self.max_effective_transacted_session_count = None;
  }

  pub fn set_max_effective_transaction_count(&mut self, max_effective_transaction_count: i32) {
    self.max_effective_transaction_count = Some(max_effective_transaction_count);
  }

  pub fn with_max_effective_transaction_count(mut self, max_effective_transaction_count: i32) -> MsgVpn {
    self.max_effective_transaction_count = Some(max_effective_transaction_count);
    self
  }

  pub fn max_effective_transaction_count(&self) -> Option<&i32> {
    self.max_effective_transaction_count.as_ref()
  }

  pub fn reset_max_effective_transaction_count(&mut self) {
    self.max_effective_transaction_count = None;
  }

  pub fn set_max_effective_tx_flow_count(&mut self, max_effective_tx_flow_count: i32) {
    self.max_effective_tx_flow_count = Some(max_effective_tx_flow_count);
  }

  pub fn with_max_effective_tx_flow_count(mut self, max_effective_tx_flow_count: i32) -> MsgVpn {
    self.max_effective_tx_flow_count = Some(max_effective_tx_flow_count);
    self
  }

  pub fn max_effective_tx_flow_count(&self) -> Option<&i32> {
    self.max_effective_tx_flow_count.as_ref()
  }

  pub fn reset_max_effective_tx_flow_count(&mut self) {
    self.max_effective_tx_flow_count = None;
  }

  pub fn set_max_egress_flow_count(&mut self, max_egress_flow_count: i64) {
    self.max_egress_flow_count = Some(max_egress_flow_count);
  }

  pub fn with_max_egress_flow_count(mut self, max_egress_flow_count: i64) -> MsgVpn {
    self.max_egress_flow_count = Some(max_egress_flow_count);
    self
  }

  pub fn max_egress_flow_count(&self) -> Option<&i64> {
    self.max_egress_flow_count.as_ref()
  }

  pub fn reset_max_egress_flow_count(&mut self) {
    self.max_egress_flow_count = None;
  }

  pub fn set_max_endpoint_count(&mut self, max_endpoint_count: i64) {
    self.max_endpoint_count = Some(max_endpoint_count);
  }

  pub fn with_max_endpoint_count(mut self, max_endpoint_count: i64) -> MsgVpn {
    self.max_endpoint_count = Some(max_endpoint_count);
    self
  }

  pub fn max_endpoint_count(&self) -> Option<&i64> {
    self.max_endpoint_count.as_ref()
  }

  pub fn reset_max_endpoint_count(&mut self) {
    self.max_endpoint_count = None;
  }

  pub fn set_max_ingress_flow_count(&mut self, max_ingress_flow_count: i64) {
    self.max_ingress_flow_count = Some(max_ingress_flow_count);
  }

  pub fn with_max_ingress_flow_count(mut self, max_ingress_flow_count: i64) -> MsgVpn {
    self.max_ingress_flow_count = Some(max_ingress_flow_count);
    self
  }

  pub fn max_ingress_flow_count(&self) -> Option<&i64> {
    self.max_ingress_flow_count.as_ref()
  }

  pub fn reset_max_ingress_flow_count(&mut self) {
    self.max_ingress_flow_count = None;
  }

  pub fn set_max_msg_spool_usage(&mut self, max_msg_spool_usage: i64) {
    self.max_msg_spool_usage = Some(max_msg_spool_usage);
  }

  pub fn with_max_msg_spool_usage(mut self, max_msg_spool_usage: i64) -> MsgVpn {
    self.max_msg_spool_usage = Some(max_msg_spool_usage);
    self
  }

  pub fn max_msg_spool_usage(&self) -> Option<&i64> {
    self.max_msg_spool_usage.as_ref()
  }

  pub fn reset_max_msg_spool_usage(&mut self) {
    self.max_msg_spool_usage = None;
  }

  pub fn set_max_subscription_count(&mut self, max_subscription_count: i64) {
    self.max_subscription_count = Some(max_subscription_count);
  }

  pub fn with_max_subscription_count(mut self, max_subscription_count: i64) -> MsgVpn {
    self.max_subscription_count = Some(max_subscription_count);
    self
  }

  pub fn max_subscription_count(&self) -> Option<&i64> {
    self.max_subscription_count.as_ref()
  }

  pub fn reset_max_subscription_count(&mut self) {
    self.max_subscription_count = None;
  }

  pub fn set_max_transacted_session_count(&mut self, max_transacted_session_count: i64) {
    self.max_transacted_session_count = Some(max_transacted_session_count);
  }

  pub fn with_max_transacted_session_count(mut self, max_transacted_session_count: i64) -> MsgVpn {
    self.max_transacted_session_count = Some(max_transacted_session_count);
    self
  }

  pub fn max_transacted_session_count(&self) -> Option<&i64> {
    self.max_transacted_session_count.as_ref()
  }

  pub fn reset_max_transacted_session_count(&mut self) {
    self.max_transacted_session_count = None;
  }

  pub fn set_max_transaction_count(&mut self, max_transaction_count: i64) {
    self.max_transaction_count = Some(max_transaction_count);
  }

  pub fn with_max_transaction_count(mut self, max_transaction_count: i64) -> MsgVpn {
    self.max_transaction_count = Some(max_transaction_count);
    self
  }

  pub fn max_transaction_count(&self) -> Option<&i64> {
    self.max_transaction_count.as_ref()
  }

  pub fn reset_max_transaction_count(&mut self) {
    self.max_transaction_count = None;
  }

  pub fn set_mqtt_retain_max_memory(&mut self, mqtt_retain_max_memory: i32) {
    self.mqtt_retain_max_memory = Some(mqtt_retain_max_memory);
  }

  pub fn with_mqtt_retain_max_memory(mut self, mqtt_retain_max_memory: i32) -> MsgVpn {
    self.mqtt_retain_max_memory = Some(mqtt_retain_max_memory);
    self
  }

  pub fn mqtt_retain_max_memory(&self) -> Option<&i32> {
    self.mqtt_retain_max_memory.as_ref()
  }

  pub fn reset_mqtt_retain_max_memory(&mut self) {
    self.mqtt_retain_max_memory = None;
  }

  pub fn set_msg_replay_active_count(&mut self, msg_replay_active_count: i32) {
    self.msg_replay_active_count = Some(msg_replay_active_count);
  }

  pub fn with_msg_replay_active_count(mut self, msg_replay_active_count: i32) -> MsgVpn {
    self.msg_replay_active_count = Some(msg_replay_active_count);
    self
  }

  pub fn msg_replay_active_count(&self) -> Option<&i32> {
    self.msg_replay_active_count.as_ref()
  }

  pub fn reset_msg_replay_active_count(&mut self) {
    self.msg_replay_active_count = None;
  }

  pub fn set_msg_replay_failed_count(&mut self, msg_replay_failed_count: i32) {
    self.msg_replay_failed_count = Some(msg_replay_failed_count);
  }

  pub fn with_msg_replay_failed_count(mut self, msg_replay_failed_count: i32) -> MsgVpn {
    self.msg_replay_failed_count = Some(msg_replay_failed_count);
    self
  }

  pub fn msg_replay_failed_count(&self) -> Option<&i32> {
    self.msg_replay_failed_count.as_ref()
  }

  pub fn reset_msg_replay_failed_count(&mut self) {
    self.msg_replay_failed_count = None;
  }

  pub fn set_msg_replay_initializing_count(&mut self, msg_replay_initializing_count: i32) {
    self.msg_replay_initializing_count = Some(msg_replay_initializing_count);
  }

  pub fn with_msg_replay_initializing_count(mut self, msg_replay_initializing_count: i32) -> MsgVpn {
    self.msg_replay_initializing_count = Some(msg_replay_initializing_count);
    self
  }

  pub fn msg_replay_initializing_count(&self) -> Option<&i32> {
    self.msg_replay_initializing_count.as_ref()
  }

  pub fn reset_msg_replay_initializing_count(&mut self) {
    self.msg_replay_initializing_count = None;
  }

  pub fn set_msg_replay_pending_complete_count(&mut self, msg_replay_pending_complete_count: i32) {
    self.msg_replay_pending_complete_count = Some(msg_replay_pending_complete_count);
  }

  pub fn with_msg_replay_pending_complete_count(mut self, msg_replay_pending_complete_count: i32) -> MsgVpn {
    self.msg_replay_pending_complete_count = Some(msg_replay_pending_complete_count);
    self
  }

  pub fn msg_replay_pending_complete_count(&self) -> Option<&i32> {
    self.msg_replay_pending_complete_count.as_ref()
  }

  pub fn reset_msg_replay_pending_complete_count(&mut self) {
    self.msg_replay_pending_complete_count = None;
  }

  pub fn set_msg_spool_usage(&mut self, msg_spool_usage: i64) {
    self.msg_spool_usage = Some(msg_spool_usage);
  }

  pub fn with_msg_spool_usage(mut self, msg_spool_usage: i64) -> MsgVpn {
    self.msg_spool_usage = Some(msg_spool_usage);
    self
  }

  pub fn msg_spool_usage(&self) -> Option<&i64> {
    self.msg_spool_usage.as_ref()
  }

  pub fn reset_msg_spool_usage(&mut self) {
    self.msg_spool_usage = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpn {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_rate(&mut self, rate: ::models::MsgVpnRate) {
    self.rate = Some(rate);
  }

  pub fn with_rate(mut self, rate: ::models::MsgVpnRate) -> MsgVpn {
    self.rate = Some(rate);
    self
  }

  pub fn rate(&self) -> Option<&::models::MsgVpnRate> {
    self.rate.as_ref()
  }

  pub fn reset_rate(&mut self) {
    self.rate = None;
  }

  pub fn set_rest_tls_server_cert_enforce_trusted_common_name_enabled(&mut self, rest_tls_server_cert_enforce_trusted_common_name_enabled: bool) {
    self.rest_tls_server_cert_enforce_trusted_common_name_enabled = Some(rest_tls_server_cert_enforce_trusted_common_name_enabled);
  }

  pub fn with_rest_tls_server_cert_enforce_trusted_common_name_enabled(mut self, rest_tls_server_cert_enforce_trusted_common_name_enabled: bool) -> MsgVpn {
    self.rest_tls_server_cert_enforce_trusted_common_name_enabled = Some(rest_tls_server_cert_enforce_trusted_common_name_enabled);
    self
  }

  pub fn rest_tls_server_cert_enforce_trusted_common_name_enabled(&self) -> Option<&bool> {
    self.rest_tls_server_cert_enforce_trusted_common_name_enabled.as_ref()
  }

  pub fn reset_rest_tls_server_cert_enforce_trusted_common_name_enabled(&mut self) {
    self.rest_tls_server_cert_enforce_trusted_common_name_enabled = None;
  }

  pub fn set_rest_tls_server_cert_max_chain_depth(&mut self, rest_tls_server_cert_max_chain_depth: i64) {
    self.rest_tls_server_cert_max_chain_depth = Some(rest_tls_server_cert_max_chain_depth);
  }

  pub fn with_rest_tls_server_cert_max_chain_depth(mut self, rest_tls_server_cert_max_chain_depth: i64) -> MsgVpn {
    self.rest_tls_server_cert_max_chain_depth = Some(rest_tls_server_cert_max_chain_depth);
    self
  }

  pub fn rest_tls_server_cert_max_chain_depth(&self) -> Option<&i64> {
    self.rest_tls_server_cert_max_chain_depth.as_ref()
  }

  pub fn reset_rest_tls_server_cert_max_chain_depth(&mut self) {
    self.rest_tls_server_cert_max_chain_depth = None;
  }

  pub fn set_rest_tls_server_cert_validate_date_enabled(&mut self, rest_tls_server_cert_validate_date_enabled: bool) {
    self.rest_tls_server_cert_validate_date_enabled = Some(rest_tls_server_cert_validate_date_enabled);
  }

  pub fn with_rest_tls_server_cert_validate_date_enabled(mut self, rest_tls_server_cert_validate_date_enabled: bool) -> MsgVpn {
    self.rest_tls_server_cert_validate_date_enabled = Some(rest_tls_server_cert_validate_date_enabled);
    self
  }

  pub fn rest_tls_server_cert_validate_date_enabled(&self) -> Option<&bool> {
    self.rest_tls_server_cert_validate_date_enabled.as_ref()
  }

  pub fn reset_rest_tls_server_cert_validate_date_enabled(&mut self) {
    self.rest_tls_server_cert_validate_date_enabled = None;
  }

  pub fn set_semp_over_msg_bus_admin_client_enabled(&mut self, semp_over_msg_bus_admin_client_enabled: bool) {
    self.semp_over_msg_bus_admin_client_enabled = Some(semp_over_msg_bus_admin_client_enabled);
  }

  pub fn with_semp_over_msg_bus_admin_client_enabled(mut self, semp_over_msg_bus_admin_client_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_admin_client_enabled = Some(semp_over_msg_bus_admin_client_enabled);
    self
  }

  pub fn semp_over_msg_bus_admin_client_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_admin_client_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_admin_client_enabled(&mut self) {
    self.semp_over_msg_bus_admin_client_enabled = None;
  }

  pub fn set_semp_over_msg_bus_admin_distributed_cache_enabled(&mut self, semp_over_msg_bus_admin_distributed_cache_enabled: bool) {
    self.semp_over_msg_bus_admin_distributed_cache_enabled = Some(semp_over_msg_bus_admin_distributed_cache_enabled);
  }

  pub fn with_semp_over_msg_bus_admin_distributed_cache_enabled(mut self, semp_over_msg_bus_admin_distributed_cache_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_admin_distributed_cache_enabled = Some(semp_over_msg_bus_admin_distributed_cache_enabled);
    self
  }

  pub fn semp_over_msg_bus_admin_distributed_cache_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_admin_distributed_cache_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_admin_distributed_cache_enabled(&mut self) {
    self.semp_over_msg_bus_admin_distributed_cache_enabled = None;
  }

  pub fn set_semp_over_msg_bus_admin_enabled(&mut self, semp_over_msg_bus_admin_enabled: bool) {
    self.semp_over_msg_bus_admin_enabled = Some(semp_over_msg_bus_admin_enabled);
  }

  pub fn with_semp_over_msg_bus_admin_enabled(mut self, semp_over_msg_bus_admin_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_admin_enabled = Some(semp_over_msg_bus_admin_enabled);
    self
  }

  pub fn semp_over_msg_bus_admin_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_admin_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_admin_enabled(&mut self) {
    self.semp_over_msg_bus_admin_enabled = None;
  }

  pub fn set_semp_over_msg_bus_enabled(&mut self, semp_over_msg_bus_enabled: bool) {
    self.semp_over_msg_bus_enabled = Some(semp_over_msg_bus_enabled);
  }

  pub fn with_semp_over_msg_bus_enabled(mut self, semp_over_msg_bus_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_enabled = Some(semp_over_msg_bus_enabled);
    self
  }

  pub fn semp_over_msg_bus_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_enabled(&mut self) {
    self.semp_over_msg_bus_enabled = None;
  }

  pub fn set_semp_over_msg_bus_show_enabled(&mut self, semp_over_msg_bus_show_enabled: bool) {
    self.semp_over_msg_bus_show_enabled = Some(semp_over_msg_bus_show_enabled);
  }

  pub fn with_semp_over_msg_bus_show_enabled(mut self, semp_over_msg_bus_show_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_show_enabled = Some(semp_over_msg_bus_show_enabled);
    self
  }

  pub fn semp_over_msg_bus_show_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_show_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_show_enabled(&mut self) {
    self.semp_over_msg_bus_show_enabled = None;
  }

  pub fn set_service_amqp_max_connection_count(&mut self, service_amqp_max_connection_count: i64) {
    self.service_amqp_max_connection_count = Some(service_amqp_max_connection_count);
  }

  pub fn with_service_amqp_max_connection_count(mut self, service_amqp_max_connection_count: i64) -> MsgVpn {
    self.service_amqp_max_connection_count = Some(service_amqp_max_connection_count);
    self
  }

  pub fn service_amqp_max_connection_count(&self) -> Option<&i64> {
    self.service_amqp_max_connection_count.as_ref()
  }

  pub fn reset_service_amqp_max_connection_count(&mut self) {
    self.service_amqp_max_connection_count = None;
  }

  pub fn set_service_amqp_plain_text_compressed(&mut self, service_amqp_plain_text_compressed: bool) {
    self.service_amqp_plain_text_compressed = Some(service_amqp_plain_text_compressed);
  }

  pub fn with_service_amqp_plain_text_compressed(mut self, service_amqp_plain_text_compressed: bool) -> MsgVpn {
    self.service_amqp_plain_text_compressed = Some(service_amqp_plain_text_compressed);
    self
  }

  pub fn service_amqp_plain_text_compressed(&self) -> Option<&bool> {
    self.service_amqp_plain_text_compressed.as_ref()
  }

  pub fn reset_service_amqp_plain_text_compressed(&mut self) {
    self.service_amqp_plain_text_compressed = None;
  }

  pub fn set_service_amqp_plain_text_enabled(&mut self, service_amqp_plain_text_enabled: bool) {
    self.service_amqp_plain_text_enabled = Some(service_amqp_plain_text_enabled);
  }

  pub fn with_service_amqp_plain_text_enabled(mut self, service_amqp_plain_text_enabled: bool) -> MsgVpn {
    self.service_amqp_plain_text_enabled = Some(service_amqp_plain_text_enabled);
    self
  }

  pub fn service_amqp_plain_text_enabled(&self) -> Option<&bool> {
    self.service_amqp_plain_text_enabled.as_ref()
  }

  pub fn reset_service_amqp_plain_text_enabled(&mut self) {
    self.service_amqp_plain_text_enabled = None;
  }

  pub fn set_service_amqp_plain_text_failure_reason(&mut self, service_amqp_plain_text_failure_reason: String) {
    self.service_amqp_plain_text_failure_reason = Some(service_amqp_plain_text_failure_reason);
  }

  pub fn with_service_amqp_plain_text_failure_reason(mut self, service_amqp_plain_text_failure_reason: String) -> MsgVpn {
    self.service_amqp_plain_text_failure_reason = Some(service_amqp_plain_text_failure_reason);
    self
  }

  pub fn service_amqp_plain_text_failure_reason(&self) -> Option<&String> {
    self.service_amqp_plain_text_failure_reason.as_ref()
  }

  pub fn reset_service_amqp_plain_text_failure_reason(&mut self) {
    self.service_amqp_plain_text_failure_reason = None;
  }

  pub fn set_service_amqp_plain_text_listen_port(&mut self, service_amqp_plain_text_listen_port: i64) {
    self.service_amqp_plain_text_listen_port = Some(service_amqp_plain_text_listen_port);
  }

  pub fn with_service_amqp_plain_text_listen_port(mut self, service_amqp_plain_text_listen_port: i64) -> MsgVpn {
    self.service_amqp_plain_text_listen_port = Some(service_amqp_plain_text_listen_port);
    self
  }

  pub fn service_amqp_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_amqp_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_amqp_plain_text_listen_port(&mut self) {
    self.service_amqp_plain_text_listen_port = None;
  }

  pub fn set_service_amqp_plain_text_up(&mut self, service_amqp_plain_text_up: bool) {
    self.service_amqp_plain_text_up = Some(service_amqp_plain_text_up);
  }

  pub fn with_service_amqp_plain_text_up(mut self, service_amqp_plain_text_up: bool) -> MsgVpn {
    self.service_amqp_plain_text_up = Some(service_amqp_plain_text_up);
    self
  }

  pub fn service_amqp_plain_text_up(&self) -> Option<&bool> {
    self.service_amqp_plain_text_up.as_ref()
  }

  pub fn reset_service_amqp_plain_text_up(&mut self) {
    self.service_amqp_plain_text_up = None;
  }

  pub fn set_service_amqp_tls_compressed(&mut self, service_amqp_tls_compressed: bool) {
    self.service_amqp_tls_compressed = Some(service_amqp_tls_compressed);
  }

  pub fn with_service_amqp_tls_compressed(mut self, service_amqp_tls_compressed: bool) -> MsgVpn {
    self.service_amqp_tls_compressed = Some(service_amqp_tls_compressed);
    self
  }

  pub fn service_amqp_tls_compressed(&self) -> Option<&bool> {
    self.service_amqp_tls_compressed.as_ref()
  }

  pub fn reset_service_amqp_tls_compressed(&mut self) {
    self.service_amqp_tls_compressed = None;
  }

  pub fn set_service_amqp_tls_enabled(&mut self, service_amqp_tls_enabled: bool) {
    self.service_amqp_tls_enabled = Some(service_amqp_tls_enabled);
  }

  pub fn with_service_amqp_tls_enabled(mut self, service_amqp_tls_enabled: bool) -> MsgVpn {
    self.service_amqp_tls_enabled = Some(service_amqp_tls_enabled);
    self
  }

  pub fn service_amqp_tls_enabled(&self) -> Option<&bool> {
    self.service_amqp_tls_enabled.as_ref()
  }

  pub fn reset_service_amqp_tls_enabled(&mut self) {
    self.service_amqp_tls_enabled = None;
  }

  pub fn set_service_amqp_tls_failure_reason(&mut self, service_amqp_tls_failure_reason: String) {
    self.service_amqp_tls_failure_reason = Some(service_amqp_tls_failure_reason);
  }

  pub fn with_service_amqp_tls_failure_reason(mut self, service_amqp_tls_failure_reason: String) -> MsgVpn {
    self.service_amqp_tls_failure_reason = Some(service_amqp_tls_failure_reason);
    self
  }

  pub fn service_amqp_tls_failure_reason(&self) -> Option<&String> {
    self.service_amqp_tls_failure_reason.as_ref()
  }

  pub fn reset_service_amqp_tls_failure_reason(&mut self) {
    self.service_amqp_tls_failure_reason = None;
  }

  pub fn set_service_amqp_tls_listen_port(&mut self, service_amqp_tls_listen_port: i64) {
    self.service_amqp_tls_listen_port = Some(service_amqp_tls_listen_port);
  }

  pub fn with_service_amqp_tls_listen_port(mut self, service_amqp_tls_listen_port: i64) -> MsgVpn {
    self.service_amqp_tls_listen_port = Some(service_amqp_tls_listen_port);
    self
  }

  pub fn service_amqp_tls_listen_port(&self) -> Option<&i64> {
    self.service_amqp_tls_listen_port.as_ref()
  }

  pub fn reset_service_amqp_tls_listen_port(&mut self) {
    self.service_amqp_tls_listen_port = None;
  }

  pub fn set_service_amqp_tls_up(&mut self, service_amqp_tls_up: bool) {
    self.service_amqp_tls_up = Some(service_amqp_tls_up);
  }

  pub fn with_service_amqp_tls_up(mut self, service_amqp_tls_up: bool) -> MsgVpn {
    self.service_amqp_tls_up = Some(service_amqp_tls_up);
    self
  }

  pub fn service_amqp_tls_up(&self) -> Option<&bool> {
    self.service_amqp_tls_up.as_ref()
  }

  pub fn reset_service_amqp_tls_up(&mut self) {
    self.service_amqp_tls_up = None;
  }

  pub fn set_service_mqtt_max_connection_count(&mut self, service_mqtt_max_connection_count: i64) {
    self.service_mqtt_max_connection_count = Some(service_mqtt_max_connection_count);
  }

  pub fn with_service_mqtt_max_connection_count(mut self, service_mqtt_max_connection_count: i64) -> MsgVpn {
    self.service_mqtt_max_connection_count = Some(service_mqtt_max_connection_count);
    self
  }

  pub fn service_mqtt_max_connection_count(&self) -> Option<&i64> {
    self.service_mqtt_max_connection_count.as_ref()
  }

  pub fn reset_service_mqtt_max_connection_count(&mut self) {
    self.service_mqtt_max_connection_count = None;
  }

  pub fn set_service_mqtt_plain_text_compressed(&mut self, service_mqtt_plain_text_compressed: bool) {
    self.service_mqtt_plain_text_compressed = Some(service_mqtt_plain_text_compressed);
  }

  pub fn with_service_mqtt_plain_text_compressed(mut self, service_mqtt_plain_text_compressed: bool) -> MsgVpn {
    self.service_mqtt_plain_text_compressed = Some(service_mqtt_plain_text_compressed);
    self
  }

  pub fn service_mqtt_plain_text_compressed(&self) -> Option<&bool> {
    self.service_mqtt_plain_text_compressed.as_ref()
  }

  pub fn reset_service_mqtt_plain_text_compressed(&mut self) {
    self.service_mqtt_plain_text_compressed = None;
  }

  pub fn set_service_mqtt_plain_text_enabled(&mut self, service_mqtt_plain_text_enabled: bool) {
    self.service_mqtt_plain_text_enabled = Some(service_mqtt_plain_text_enabled);
  }

  pub fn with_service_mqtt_plain_text_enabled(mut self, service_mqtt_plain_text_enabled: bool) -> MsgVpn {
    self.service_mqtt_plain_text_enabled = Some(service_mqtt_plain_text_enabled);
    self
  }

  pub fn service_mqtt_plain_text_enabled(&self) -> Option<&bool> {
    self.service_mqtt_plain_text_enabled.as_ref()
  }

  pub fn reset_service_mqtt_plain_text_enabled(&mut self) {
    self.service_mqtt_plain_text_enabled = None;
  }

  pub fn set_service_mqtt_plain_text_failure_reason(&mut self, service_mqtt_plain_text_failure_reason: String) {
    self.service_mqtt_plain_text_failure_reason = Some(service_mqtt_plain_text_failure_reason);
  }

  pub fn with_service_mqtt_plain_text_failure_reason(mut self, service_mqtt_plain_text_failure_reason: String) -> MsgVpn {
    self.service_mqtt_plain_text_failure_reason = Some(service_mqtt_plain_text_failure_reason);
    self
  }

  pub fn service_mqtt_plain_text_failure_reason(&self) -> Option<&String> {
    self.service_mqtt_plain_text_failure_reason.as_ref()
  }

  pub fn reset_service_mqtt_plain_text_failure_reason(&mut self) {
    self.service_mqtt_plain_text_failure_reason = None;
  }

  pub fn set_service_mqtt_plain_text_listen_port(&mut self, service_mqtt_plain_text_listen_port: i64) {
    self.service_mqtt_plain_text_listen_port = Some(service_mqtt_plain_text_listen_port);
  }

  pub fn with_service_mqtt_plain_text_listen_port(mut self, service_mqtt_plain_text_listen_port: i64) -> MsgVpn {
    self.service_mqtt_plain_text_listen_port = Some(service_mqtt_plain_text_listen_port);
    self
  }

  pub fn service_mqtt_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_mqtt_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_mqtt_plain_text_listen_port(&mut self) {
    self.service_mqtt_plain_text_listen_port = None;
  }

  pub fn set_service_mqtt_plain_text_up(&mut self, service_mqtt_plain_text_up: bool) {
    self.service_mqtt_plain_text_up = Some(service_mqtt_plain_text_up);
  }

  pub fn with_service_mqtt_plain_text_up(mut self, service_mqtt_plain_text_up: bool) -> MsgVpn {
    self.service_mqtt_plain_text_up = Some(service_mqtt_plain_text_up);
    self
  }

  pub fn service_mqtt_plain_text_up(&self) -> Option<&bool> {
    self.service_mqtt_plain_text_up.as_ref()
  }

  pub fn reset_service_mqtt_plain_text_up(&mut self) {
    self.service_mqtt_plain_text_up = None;
  }

  pub fn set_service_mqtt_tls_compressed(&mut self, service_mqtt_tls_compressed: bool) {
    self.service_mqtt_tls_compressed = Some(service_mqtt_tls_compressed);
  }

  pub fn with_service_mqtt_tls_compressed(mut self, service_mqtt_tls_compressed: bool) -> MsgVpn {
    self.service_mqtt_tls_compressed = Some(service_mqtt_tls_compressed);
    self
  }

  pub fn service_mqtt_tls_compressed(&self) -> Option<&bool> {
    self.service_mqtt_tls_compressed.as_ref()
  }

  pub fn reset_service_mqtt_tls_compressed(&mut self) {
    self.service_mqtt_tls_compressed = None;
  }

  pub fn set_service_mqtt_tls_enabled(&mut self, service_mqtt_tls_enabled: bool) {
    self.service_mqtt_tls_enabled = Some(service_mqtt_tls_enabled);
  }

  pub fn with_service_mqtt_tls_enabled(mut self, service_mqtt_tls_enabled: bool) -> MsgVpn {
    self.service_mqtt_tls_enabled = Some(service_mqtt_tls_enabled);
    self
  }

  pub fn service_mqtt_tls_enabled(&self) -> Option<&bool> {
    self.service_mqtt_tls_enabled.as_ref()
  }

  pub fn reset_service_mqtt_tls_enabled(&mut self) {
    self.service_mqtt_tls_enabled = None;
  }

  pub fn set_service_mqtt_tls_failure_reason(&mut self, service_mqtt_tls_failure_reason: String) {
    self.service_mqtt_tls_failure_reason = Some(service_mqtt_tls_failure_reason);
  }

  pub fn with_service_mqtt_tls_failure_reason(mut self, service_mqtt_tls_failure_reason: String) -> MsgVpn {
    self.service_mqtt_tls_failure_reason = Some(service_mqtt_tls_failure_reason);
    self
  }

  pub fn service_mqtt_tls_failure_reason(&self) -> Option<&String> {
    self.service_mqtt_tls_failure_reason.as_ref()
  }

  pub fn reset_service_mqtt_tls_failure_reason(&mut self) {
    self.service_mqtt_tls_failure_reason = None;
  }

  pub fn set_service_mqtt_tls_listen_port(&mut self, service_mqtt_tls_listen_port: i64) {
    self.service_mqtt_tls_listen_port = Some(service_mqtt_tls_listen_port);
  }

  pub fn with_service_mqtt_tls_listen_port(mut self, service_mqtt_tls_listen_port: i64) -> MsgVpn {
    self.service_mqtt_tls_listen_port = Some(service_mqtt_tls_listen_port);
    self
  }

  pub fn service_mqtt_tls_listen_port(&self) -> Option<&i64> {
    self.service_mqtt_tls_listen_port.as_ref()
  }

  pub fn reset_service_mqtt_tls_listen_port(&mut self) {
    self.service_mqtt_tls_listen_port = None;
  }

  pub fn set_service_mqtt_tls_up(&mut self, service_mqtt_tls_up: bool) {
    self.service_mqtt_tls_up = Some(service_mqtt_tls_up);
  }

  pub fn with_service_mqtt_tls_up(mut self, service_mqtt_tls_up: bool) -> MsgVpn {
    self.service_mqtt_tls_up = Some(service_mqtt_tls_up);
    self
  }

  pub fn service_mqtt_tls_up(&self) -> Option<&bool> {
    self.service_mqtt_tls_up.as_ref()
  }

  pub fn reset_service_mqtt_tls_up(&mut self) {
    self.service_mqtt_tls_up = None;
  }

  pub fn set_service_mqtt_tls_web_socket_compressed(&mut self, service_mqtt_tls_web_socket_compressed: bool) {
    self.service_mqtt_tls_web_socket_compressed = Some(service_mqtt_tls_web_socket_compressed);
  }

  pub fn with_service_mqtt_tls_web_socket_compressed(mut self, service_mqtt_tls_web_socket_compressed: bool) -> MsgVpn {
    self.service_mqtt_tls_web_socket_compressed = Some(service_mqtt_tls_web_socket_compressed);
    self
  }

  pub fn service_mqtt_tls_web_socket_compressed(&self) -> Option<&bool> {
    self.service_mqtt_tls_web_socket_compressed.as_ref()
  }

  pub fn reset_service_mqtt_tls_web_socket_compressed(&mut self) {
    self.service_mqtt_tls_web_socket_compressed = None;
  }

  pub fn set_service_mqtt_tls_web_socket_enabled(&mut self, service_mqtt_tls_web_socket_enabled: bool) {
    self.service_mqtt_tls_web_socket_enabled = Some(service_mqtt_tls_web_socket_enabled);
  }

  pub fn with_service_mqtt_tls_web_socket_enabled(mut self, service_mqtt_tls_web_socket_enabled: bool) -> MsgVpn {
    self.service_mqtt_tls_web_socket_enabled = Some(service_mqtt_tls_web_socket_enabled);
    self
  }

  pub fn service_mqtt_tls_web_socket_enabled(&self) -> Option<&bool> {
    self.service_mqtt_tls_web_socket_enabled.as_ref()
  }

  pub fn reset_service_mqtt_tls_web_socket_enabled(&mut self) {
    self.service_mqtt_tls_web_socket_enabled = None;
  }

  pub fn set_service_mqtt_tls_web_socket_failure_reason(&mut self, service_mqtt_tls_web_socket_failure_reason: String) {
    self.service_mqtt_tls_web_socket_failure_reason = Some(service_mqtt_tls_web_socket_failure_reason);
  }

  pub fn with_service_mqtt_tls_web_socket_failure_reason(mut self, service_mqtt_tls_web_socket_failure_reason: String) -> MsgVpn {
    self.service_mqtt_tls_web_socket_failure_reason = Some(service_mqtt_tls_web_socket_failure_reason);
    self
  }

  pub fn service_mqtt_tls_web_socket_failure_reason(&self) -> Option<&String> {
    self.service_mqtt_tls_web_socket_failure_reason.as_ref()
  }

  pub fn reset_service_mqtt_tls_web_socket_failure_reason(&mut self) {
    self.service_mqtt_tls_web_socket_failure_reason = None;
  }

  pub fn set_service_mqtt_tls_web_socket_listen_port(&mut self, service_mqtt_tls_web_socket_listen_port: i64) {
    self.service_mqtt_tls_web_socket_listen_port = Some(service_mqtt_tls_web_socket_listen_port);
  }

  pub fn with_service_mqtt_tls_web_socket_listen_port(mut self, service_mqtt_tls_web_socket_listen_port: i64) -> MsgVpn {
    self.service_mqtt_tls_web_socket_listen_port = Some(service_mqtt_tls_web_socket_listen_port);
    self
  }

  pub fn service_mqtt_tls_web_socket_listen_port(&self) -> Option<&i64> {
    self.service_mqtt_tls_web_socket_listen_port.as_ref()
  }

  pub fn reset_service_mqtt_tls_web_socket_listen_port(&mut self) {
    self.service_mqtt_tls_web_socket_listen_port = None;
  }

  pub fn set_service_mqtt_tls_web_socket_up(&mut self, service_mqtt_tls_web_socket_up: bool) {
    self.service_mqtt_tls_web_socket_up = Some(service_mqtt_tls_web_socket_up);
  }

  pub fn with_service_mqtt_tls_web_socket_up(mut self, service_mqtt_tls_web_socket_up: bool) -> MsgVpn {
    self.service_mqtt_tls_web_socket_up = Some(service_mqtt_tls_web_socket_up);
    self
  }

  pub fn service_mqtt_tls_web_socket_up(&self) -> Option<&bool> {
    self.service_mqtt_tls_web_socket_up.as_ref()
  }

  pub fn reset_service_mqtt_tls_web_socket_up(&mut self) {
    self.service_mqtt_tls_web_socket_up = None;
  }

  pub fn set_service_mqtt_web_socket_compressed(&mut self, service_mqtt_web_socket_compressed: bool) {
    self.service_mqtt_web_socket_compressed = Some(service_mqtt_web_socket_compressed);
  }

  pub fn with_service_mqtt_web_socket_compressed(mut self, service_mqtt_web_socket_compressed: bool) -> MsgVpn {
    self.service_mqtt_web_socket_compressed = Some(service_mqtt_web_socket_compressed);
    self
  }

  pub fn service_mqtt_web_socket_compressed(&self) -> Option<&bool> {
    self.service_mqtt_web_socket_compressed.as_ref()
  }

  pub fn reset_service_mqtt_web_socket_compressed(&mut self) {
    self.service_mqtt_web_socket_compressed = None;
  }

  pub fn set_service_mqtt_web_socket_enabled(&mut self, service_mqtt_web_socket_enabled: bool) {
    self.service_mqtt_web_socket_enabled = Some(service_mqtt_web_socket_enabled);
  }

  pub fn with_service_mqtt_web_socket_enabled(mut self, service_mqtt_web_socket_enabled: bool) -> MsgVpn {
    self.service_mqtt_web_socket_enabled = Some(service_mqtt_web_socket_enabled);
    self
  }

  pub fn service_mqtt_web_socket_enabled(&self) -> Option<&bool> {
    self.service_mqtt_web_socket_enabled.as_ref()
  }

  pub fn reset_service_mqtt_web_socket_enabled(&mut self) {
    self.service_mqtt_web_socket_enabled = None;
  }

  pub fn set_service_mqtt_web_socket_failure_reason(&mut self, service_mqtt_web_socket_failure_reason: String) {
    self.service_mqtt_web_socket_failure_reason = Some(service_mqtt_web_socket_failure_reason);
  }

  pub fn with_service_mqtt_web_socket_failure_reason(mut self, service_mqtt_web_socket_failure_reason: String) -> MsgVpn {
    self.service_mqtt_web_socket_failure_reason = Some(service_mqtt_web_socket_failure_reason);
    self
  }

  pub fn service_mqtt_web_socket_failure_reason(&self) -> Option<&String> {
    self.service_mqtt_web_socket_failure_reason.as_ref()
  }

  pub fn reset_service_mqtt_web_socket_failure_reason(&mut self) {
    self.service_mqtt_web_socket_failure_reason = None;
  }

  pub fn set_service_mqtt_web_socket_listen_port(&mut self, service_mqtt_web_socket_listen_port: i64) {
    self.service_mqtt_web_socket_listen_port = Some(service_mqtt_web_socket_listen_port);
  }

  pub fn with_service_mqtt_web_socket_listen_port(mut self, service_mqtt_web_socket_listen_port: i64) -> MsgVpn {
    self.service_mqtt_web_socket_listen_port = Some(service_mqtt_web_socket_listen_port);
    self
  }

  pub fn service_mqtt_web_socket_listen_port(&self) -> Option<&i64> {
    self.service_mqtt_web_socket_listen_port.as_ref()
  }

  pub fn reset_service_mqtt_web_socket_listen_port(&mut self) {
    self.service_mqtt_web_socket_listen_port = None;
  }

  pub fn set_service_mqtt_web_socket_up(&mut self, service_mqtt_web_socket_up: bool) {
    self.service_mqtt_web_socket_up = Some(service_mqtt_web_socket_up);
  }

  pub fn with_service_mqtt_web_socket_up(mut self, service_mqtt_web_socket_up: bool) -> MsgVpn {
    self.service_mqtt_web_socket_up = Some(service_mqtt_web_socket_up);
    self
  }

  pub fn service_mqtt_web_socket_up(&self) -> Option<&bool> {
    self.service_mqtt_web_socket_up.as_ref()
  }

  pub fn reset_service_mqtt_web_socket_up(&mut self) {
    self.service_mqtt_web_socket_up = None;
  }

  pub fn set_service_rest_incoming_max_connection_count(&mut self, service_rest_incoming_max_connection_count: i64) {
    self.service_rest_incoming_max_connection_count = Some(service_rest_incoming_max_connection_count);
  }

  pub fn with_service_rest_incoming_max_connection_count(mut self, service_rest_incoming_max_connection_count: i64) -> MsgVpn {
    self.service_rest_incoming_max_connection_count = Some(service_rest_incoming_max_connection_count);
    self
  }

  pub fn service_rest_incoming_max_connection_count(&self) -> Option<&i64> {
    self.service_rest_incoming_max_connection_count.as_ref()
  }

  pub fn reset_service_rest_incoming_max_connection_count(&mut self) {
    self.service_rest_incoming_max_connection_count = None;
  }

  pub fn set_service_rest_incoming_plain_text_compressed(&mut self, service_rest_incoming_plain_text_compressed: bool) {
    self.service_rest_incoming_plain_text_compressed = Some(service_rest_incoming_plain_text_compressed);
  }

  pub fn with_service_rest_incoming_plain_text_compressed(mut self, service_rest_incoming_plain_text_compressed: bool) -> MsgVpn {
    self.service_rest_incoming_plain_text_compressed = Some(service_rest_incoming_plain_text_compressed);
    self
  }

  pub fn service_rest_incoming_plain_text_compressed(&self) -> Option<&bool> {
    self.service_rest_incoming_plain_text_compressed.as_ref()
  }

  pub fn reset_service_rest_incoming_plain_text_compressed(&mut self) {
    self.service_rest_incoming_plain_text_compressed = None;
  }

  pub fn set_service_rest_incoming_plain_text_enabled(&mut self, service_rest_incoming_plain_text_enabled: bool) {
    self.service_rest_incoming_plain_text_enabled = Some(service_rest_incoming_plain_text_enabled);
  }

  pub fn with_service_rest_incoming_plain_text_enabled(mut self, service_rest_incoming_plain_text_enabled: bool) -> MsgVpn {
    self.service_rest_incoming_plain_text_enabled = Some(service_rest_incoming_plain_text_enabled);
    self
  }

  pub fn service_rest_incoming_plain_text_enabled(&self) -> Option<&bool> {
    self.service_rest_incoming_plain_text_enabled.as_ref()
  }

  pub fn reset_service_rest_incoming_plain_text_enabled(&mut self) {
    self.service_rest_incoming_plain_text_enabled = None;
  }

  pub fn set_service_rest_incoming_plain_text_failure_reason(&mut self, service_rest_incoming_plain_text_failure_reason: String) {
    self.service_rest_incoming_plain_text_failure_reason = Some(service_rest_incoming_plain_text_failure_reason);
  }

  pub fn with_service_rest_incoming_plain_text_failure_reason(mut self, service_rest_incoming_plain_text_failure_reason: String) -> MsgVpn {
    self.service_rest_incoming_plain_text_failure_reason = Some(service_rest_incoming_plain_text_failure_reason);
    self
  }

  pub fn service_rest_incoming_plain_text_failure_reason(&self) -> Option<&String> {
    self.service_rest_incoming_plain_text_failure_reason.as_ref()
  }

  pub fn reset_service_rest_incoming_plain_text_failure_reason(&mut self) {
    self.service_rest_incoming_plain_text_failure_reason = None;
  }

  pub fn set_service_rest_incoming_plain_text_listen_port(&mut self, service_rest_incoming_plain_text_listen_port: i64) {
    self.service_rest_incoming_plain_text_listen_port = Some(service_rest_incoming_plain_text_listen_port);
  }

  pub fn with_service_rest_incoming_plain_text_listen_port(mut self, service_rest_incoming_plain_text_listen_port: i64) -> MsgVpn {
    self.service_rest_incoming_plain_text_listen_port = Some(service_rest_incoming_plain_text_listen_port);
    self
  }

  pub fn service_rest_incoming_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_rest_incoming_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_rest_incoming_plain_text_listen_port(&mut self) {
    self.service_rest_incoming_plain_text_listen_port = None;
  }

  pub fn set_service_rest_incoming_plain_text_up(&mut self, service_rest_incoming_plain_text_up: bool) {
    self.service_rest_incoming_plain_text_up = Some(service_rest_incoming_plain_text_up);
  }

  pub fn with_service_rest_incoming_plain_text_up(mut self, service_rest_incoming_plain_text_up: bool) -> MsgVpn {
    self.service_rest_incoming_plain_text_up = Some(service_rest_incoming_plain_text_up);
    self
  }

  pub fn service_rest_incoming_plain_text_up(&self) -> Option<&bool> {
    self.service_rest_incoming_plain_text_up.as_ref()
  }

  pub fn reset_service_rest_incoming_plain_text_up(&mut self) {
    self.service_rest_incoming_plain_text_up = None;
  }

  pub fn set_service_rest_incoming_tls_compressed(&mut self, service_rest_incoming_tls_compressed: bool) {
    self.service_rest_incoming_tls_compressed = Some(service_rest_incoming_tls_compressed);
  }

  pub fn with_service_rest_incoming_tls_compressed(mut self, service_rest_incoming_tls_compressed: bool) -> MsgVpn {
    self.service_rest_incoming_tls_compressed = Some(service_rest_incoming_tls_compressed);
    self
  }

  pub fn service_rest_incoming_tls_compressed(&self) -> Option<&bool> {
    self.service_rest_incoming_tls_compressed.as_ref()
  }

  pub fn reset_service_rest_incoming_tls_compressed(&mut self) {
    self.service_rest_incoming_tls_compressed = None;
  }

  pub fn set_service_rest_incoming_tls_enabled(&mut self, service_rest_incoming_tls_enabled: bool) {
    self.service_rest_incoming_tls_enabled = Some(service_rest_incoming_tls_enabled);
  }

  pub fn with_service_rest_incoming_tls_enabled(mut self, service_rest_incoming_tls_enabled: bool) -> MsgVpn {
    self.service_rest_incoming_tls_enabled = Some(service_rest_incoming_tls_enabled);
    self
  }

  pub fn service_rest_incoming_tls_enabled(&self) -> Option<&bool> {
    self.service_rest_incoming_tls_enabled.as_ref()
  }

  pub fn reset_service_rest_incoming_tls_enabled(&mut self) {
    self.service_rest_incoming_tls_enabled = None;
  }

  pub fn set_service_rest_incoming_tls_failure_reason(&mut self, service_rest_incoming_tls_failure_reason: String) {
    self.service_rest_incoming_tls_failure_reason = Some(service_rest_incoming_tls_failure_reason);
  }

  pub fn with_service_rest_incoming_tls_failure_reason(mut self, service_rest_incoming_tls_failure_reason: String) -> MsgVpn {
    self.service_rest_incoming_tls_failure_reason = Some(service_rest_incoming_tls_failure_reason);
    self
  }

  pub fn service_rest_incoming_tls_failure_reason(&self) -> Option<&String> {
    self.service_rest_incoming_tls_failure_reason.as_ref()
  }

  pub fn reset_service_rest_incoming_tls_failure_reason(&mut self) {
    self.service_rest_incoming_tls_failure_reason = None;
  }

  pub fn set_service_rest_incoming_tls_listen_port(&mut self, service_rest_incoming_tls_listen_port: i64) {
    self.service_rest_incoming_tls_listen_port = Some(service_rest_incoming_tls_listen_port);
  }

  pub fn with_service_rest_incoming_tls_listen_port(mut self, service_rest_incoming_tls_listen_port: i64) -> MsgVpn {
    self.service_rest_incoming_tls_listen_port = Some(service_rest_incoming_tls_listen_port);
    self
  }

  pub fn service_rest_incoming_tls_listen_port(&self) -> Option<&i64> {
    self.service_rest_incoming_tls_listen_port.as_ref()
  }

  pub fn reset_service_rest_incoming_tls_listen_port(&mut self) {
    self.service_rest_incoming_tls_listen_port = None;
  }

  pub fn set_service_rest_incoming_tls_up(&mut self, service_rest_incoming_tls_up: bool) {
    self.service_rest_incoming_tls_up = Some(service_rest_incoming_tls_up);
  }

  pub fn with_service_rest_incoming_tls_up(mut self, service_rest_incoming_tls_up: bool) -> MsgVpn {
    self.service_rest_incoming_tls_up = Some(service_rest_incoming_tls_up);
    self
  }

  pub fn service_rest_incoming_tls_up(&self) -> Option<&bool> {
    self.service_rest_incoming_tls_up.as_ref()
  }

  pub fn reset_service_rest_incoming_tls_up(&mut self) {
    self.service_rest_incoming_tls_up = None;
  }

  pub fn set_service_rest_mode(&mut self, service_rest_mode: String) {
    self.service_rest_mode = Some(service_rest_mode);
  }

  pub fn with_service_rest_mode(mut self, service_rest_mode: String) -> MsgVpn {
    self.service_rest_mode = Some(service_rest_mode);
    self
  }

  pub fn service_rest_mode(&self) -> Option<&String> {
    self.service_rest_mode.as_ref()
  }

  pub fn reset_service_rest_mode(&mut self) {
    self.service_rest_mode = None;
  }

  pub fn set_service_rest_outgoing_max_connection_count(&mut self, service_rest_outgoing_max_connection_count: i64) {
    self.service_rest_outgoing_max_connection_count = Some(service_rest_outgoing_max_connection_count);
  }

  pub fn with_service_rest_outgoing_max_connection_count(mut self, service_rest_outgoing_max_connection_count: i64) -> MsgVpn {
    self.service_rest_outgoing_max_connection_count = Some(service_rest_outgoing_max_connection_count);
    self
  }

  pub fn service_rest_outgoing_max_connection_count(&self) -> Option<&i64> {
    self.service_rest_outgoing_max_connection_count.as_ref()
  }

  pub fn reset_service_rest_outgoing_max_connection_count(&mut self) {
    self.service_rest_outgoing_max_connection_count = None;
  }

  pub fn set_service_smf_max_connection_count(&mut self, service_smf_max_connection_count: i64) {
    self.service_smf_max_connection_count = Some(service_smf_max_connection_count);
  }

  pub fn with_service_smf_max_connection_count(mut self, service_smf_max_connection_count: i64) -> MsgVpn {
    self.service_smf_max_connection_count = Some(service_smf_max_connection_count);
    self
  }

  pub fn service_smf_max_connection_count(&self) -> Option<&i64> {
    self.service_smf_max_connection_count.as_ref()
  }

  pub fn reset_service_smf_max_connection_count(&mut self) {
    self.service_smf_max_connection_count = None;
  }

  pub fn set_service_smf_plain_text_enabled(&mut self, service_smf_plain_text_enabled: bool) {
    self.service_smf_plain_text_enabled = Some(service_smf_plain_text_enabled);
  }

  pub fn with_service_smf_plain_text_enabled(mut self, service_smf_plain_text_enabled: bool) -> MsgVpn {
    self.service_smf_plain_text_enabled = Some(service_smf_plain_text_enabled);
    self
  }

  pub fn service_smf_plain_text_enabled(&self) -> Option<&bool> {
    self.service_smf_plain_text_enabled.as_ref()
  }

  pub fn reset_service_smf_plain_text_enabled(&mut self) {
    self.service_smf_plain_text_enabled = None;
  }

  pub fn set_service_smf_plain_text_failure_reason(&mut self, service_smf_plain_text_failure_reason: String) {
    self.service_smf_plain_text_failure_reason = Some(service_smf_plain_text_failure_reason);
  }

  pub fn with_service_smf_plain_text_failure_reason(mut self, service_smf_plain_text_failure_reason: String) -> MsgVpn {
    self.service_smf_plain_text_failure_reason = Some(service_smf_plain_text_failure_reason);
    self
  }

  pub fn service_smf_plain_text_failure_reason(&self) -> Option<&String> {
    self.service_smf_plain_text_failure_reason.as_ref()
  }

  pub fn reset_service_smf_plain_text_failure_reason(&mut self) {
    self.service_smf_plain_text_failure_reason = None;
  }

  pub fn set_service_smf_plain_text_up(&mut self, service_smf_plain_text_up: bool) {
    self.service_smf_plain_text_up = Some(service_smf_plain_text_up);
  }

  pub fn with_service_smf_plain_text_up(mut self, service_smf_plain_text_up: bool) -> MsgVpn {
    self.service_smf_plain_text_up = Some(service_smf_plain_text_up);
    self
  }

  pub fn service_smf_plain_text_up(&self) -> Option<&bool> {
    self.service_smf_plain_text_up.as_ref()
  }

  pub fn reset_service_smf_plain_text_up(&mut self) {
    self.service_smf_plain_text_up = None;
  }

  pub fn set_service_smf_tls_enabled(&mut self, service_smf_tls_enabled: bool) {
    self.service_smf_tls_enabled = Some(service_smf_tls_enabled);
  }

  pub fn with_service_smf_tls_enabled(mut self, service_smf_tls_enabled: bool) -> MsgVpn {
    self.service_smf_tls_enabled = Some(service_smf_tls_enabled);
    self
  }

  pub fn service_smf_tls_enabled(&self) -> Option<&bool> {
    self.service_smf_tls_enabled.as_ref()
  }

  pub fn reset_service_smf_tls_enabled(&mut self) {
    self.service_smf_tls_enabled = None;
  }

  pub fn set_service_smf_tls_failure_reason(&mut self, service_smf_tls_failure_reason: String) {
    self.service_smf_tls_failure_reason = Some(service_smf_tls_failure_reason);
  }

  pub fn with_service_smf_tls_failure_reason(mut self, service_smf_tls_failure_reason: String) -> MsgVpn {
    self.service_smf_tls_failure_reason = Some(service_smf_tls_failure_reason);
    self
  }

  pub fn service_smf_tls_failure_reason(&self) -> Option<&String> {
    self.service_smf_tls_failure_reason.as_ref()
  }

  pub fn reset_service_smf_tls_failure_reason(&mut self) {
    self.service_smf_tls_failure_reason = None;
  }

  pub fn set_service_smf_tls_up(&mut self, service_smf_tls_up: bool) {
    self.service_smf_tls_up = Some(service_smf_tls_up);
  }

  pub fn with_service_smf_tls_up(mut self, service_smf_tls_up: bool) -> MsgVpn {
    self.service_smf_tls_up = Some(service_smf_tls_up);
    self
  }

  pub fn service_smf_tls_up(&self) -> Option<&bool> {
    self.service_smf_tls_up.as_ref()
  }

  pub fn reset_service_smf_tls_up(&mut self) {
    self.service_smf_tls_up = None;
  }

  pub fn set_service_web_max_connection_count(&mut self, service_web_max_connection_count: i64) {
    self.service_web_max_connection_count = Some(service_web_max_connection_count);
  }

  pub fn with_service_web_max_connection_count(mut self, service_web_max_connection_count: i64) -> MsgVpn {
    self.service_web_max_connection_count = Some(service_web_max_connection_count);
    self
  }

  pub fn service_web_max_connection_count(&self) -> Option<&i64> {
    self.service_web_max_connection_count.as_ref()
  }

  pub fn reset_service_web_max_connection_count(&mut self) {
    self.service_web_max_connection_count = None;
  }

  pub fn set_service_web_plain_text_enabled(&mut self, service_web_plain_text_enabled: bool) {
    self.service_web_plain_text_enabled = Some(service_web_plain_text_enabled);
  }

  pub fn with_service_web_plain_text_enabled(mut self, service_web_plain_text_enabled: bool) -> MsgVpn {
    self.service_web_plain_text_enabled = Some(service_web_plain_text_enabled);
    self
  }

  pub fn service_web_plain_text_enabled(&self) -> Option<&bool> {
    self.service_web_plain_text_enabled.as_ref()
  }

  pub fn reset_service_web_plain_text_enabled(&mut self) {
    self.service_web_plain_text_enabled = None;
  }

  pub fn set_service_web_plain_text_failure_reason(&mut self, service_web_plain_text_failure_reason: String) {
    self.service_web_plain_text_failure_reason = Some(service_web_plain_text_failure_reason);
  }

  pub fn with_service_web_plain_text_failure_reason(mut self, service_web_plain_text_failure_reason: String) -> MsgVpn {
    self.service_web_plain_text_failure_reason = Some(service_web_plain_text_failure_reason);
    self
  }

  pub fn service_web_plain_text_failure_reason(&self) -> Option<&String> {
    self.service_web_plain_text_failure_reason.as_ref()
  }

  pub fn reset_service_web_plain_text_failure_reason(&mut self) {
    self.service_web_plain_text_failure_reason = None;
  }

  pub fn set_service_web_plain_text_up(&mut self, service_web_plain_text_up: bool) {
    self.service_web_plain_text_up = Some(service_web_plain_text_up);
  }

  pub fn with_service_web_plain_text_up(mut self, service_web_plain_text_up: bool) -> MsgVpn {
    self.service_web_plain_text_up = Some(service_web_plain_text_up);
    self
  }

  pub fn service_web_plain_text_up(&self) -> Option<&bool> {
    self.service_web_plain_text_up.as_ref()
  }

  pub fn reset_service_web_plain_text_up(&mut self) {
    self.service_web_plain_text_up = None;
  }

  pub fn set_service_web_tls_enabled(&mut self, service_web_tls_enabled: bool) {
    self.service_web_tls_enabled = Some(service_web_tls_enabled);
  }

  pub fn with_service_web_tls_enabled(mut self, service_web_tls_enabled: bool) -> MsgVpn {
    self.service_web_tls_enabled = Some(service_web_tls_enabled);
    self
  }

  pub fn service_web_tls_enabled(&self) -> Option<&bool> {
    self.service_web_tls_enabled.as_ref()
  }

  pub fn reset_service_web_tls_enabled(&mut self) {
    self.service_web_tls_enabled = None;
  }

  pub fn set_service_web_tls_failure_reason(&mut self, service_web_tls_failure_reason: String) {
    self.service_web_tls_failure_reason = Some(service_web_tls_failure_reason);
  }

  pub fn with_service_web_tls_failure_reason(mut self, service_web_tls_failure_reason: String) -> MsgVpn {
    self.service_web_tls_failure_reason = Some(service_web_tls_failure_reason);
    self
  }

  pub fn service_web_tls_failure_reason(&self) -> Option<&String> {
    self.service_web_tls_failure_reason.as_ref()
  }

  pub fn reset_service_web_tls_failure_reason(&mut self) {
    self.service_web_tls_failure_reason = None;
  }

  pub fn set_service_web_tls_up(&mut self, service_web_tls_up: bool) {
    self.service_web_tls_up = Some(service_web_tls_up);
  }

  pub fn with_service_web_tls_up(mut self, service_web_tls_up: bool) -> MsgVpn {
    self.service_web_tls_up = Some(service_web_tls_up);
    self
  }

  pub fn service_web_tls_up(&self) -> Option<&bool> {
    self.service_web_tls_up.as_ref()
  }

  pub fn reset_service_web_tls_up(&mut self) {
    self.service_web_tls_up = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> MsgVpn {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_subscription_export_progress(&mut self, subscription_export_progress: i64) {
    self.subscription_export_progress = Some(subscription_export_progress);
  }

  pub fn with_subscription_export_progress(mut self, subscription_export_progress: i64) -> MsgVpn {
    self.subscription_export_progress = Some(subscription_export_progress);
    self
  }

  pub fn subscription_export_progress(&self) -> Option<&i64> {
    self.subscription_export_progress.as_ref()
  }

  pub fn reset_subscription_export_progress(&mut self) {
    self.subscription_export_progress = None;
  }

  pub fn set_system_manager(&mut self, system_manager: bool) {
    self.system_manager = Some(system_manager);
  }

  pub fn with_system_manager(mut self, system_manager: bool) -> MsgVpn {
    self.system_manager = Some(system_manager);
    self
  }

  pub fn system_manager(&self) -> Option<&bool> {
    self.system_manager.as_ref()
  }

  pub fn reset_system_manager(&mut self) {
    self.system_manager = None;
  }

  pub fn set_tls_allow_downgrade_to_plain_text_enabled(&mut self, tls_allow_downgrade_to_plain_text_enabled: bool) {
    self.tls_allow_downgrade_to_plain_text_enabled = Some(tls_allow_downgrade_to_plain_text_enabled);
  }

  pub fn with_tls_allow_downgrade_to_plain_text_enabled(mut self, tls_allow_downgrade_to_plain_text_enabled: bool) -> MsgVpn {
    self.tls_allow_downgrade_to_plain_text_enabled = Some(tls_allow_downgrade_to_plain_text_enabled);
    self
  }

  pub fn tls_allow_downgrade_to_plain_text_enabled(&self) -> Option<&bool> {
    self.tls_allow_downgrade_to_plain_text_enabled.as_ref()
  }

  pub fn reset_tls_allow_downgrade_to_plain_text_enabled(&mut self) {
    self.tls_allow_downgrade_to_plain_text_enabled = None;
  }

}



