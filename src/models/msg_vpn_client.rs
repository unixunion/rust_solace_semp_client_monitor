/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/monitor/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/monitor/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/monitor/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/monitor/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/monitor/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/monitor/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/monitor/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/monitor/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/monitor/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.  
 *
 * OpenAPI spec version: 2.16
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnClient {
  /// The name of the access control list (ACL) profile of the Client.
  #[serde(rename = "aclProfileName", skip_serializing_if="Option::is_none")]
  acl_profile_name: Option<String>,
  /// The name of the original MsgVpn which the client signaled in. Available since 2.14.
  #[serde(rename = "aliasedFromMsgVpnName", skip_serializing_if="Option::is_none")]
  aliased_from_msg_vpn_name: Option<String>,
  /// The number of Client bind failures due to endpoint being already bound.
  #[serde(rename = "alreadyBoundBindFailureCount", skip_serializing_if="Option::is_none")]
  already_bound_bind_failure_count: Option<i64>,
  /// The name of the authorization group of the Client.
  #[serde(rename = "authorizationGroupName", skip_serializing_if="Option::is_none")]
  authorization_group_name: Option<String>,
  /// The one minute average of the message rate received from the Client, in bytes per second (B/sec).
  #[serde(rename = "averageRxByteRate", skip_serializing_if="Option::is_none")]
  average_rx_byte_rate: Option<i64>,
  /// The one minute average of the message rate received from the Client, in messages per second (msg/sec).
  #[serde(rename = "averageRxMsgRate", skip_serializing_if="Option::is_none")]
  average_rx_msg_rate: Option<i64>,
  /// The one minute average of the message rate transmitted to the Client, in bytes per second (B/sec).
  #[serde(rename = "averageTxByteRate", skip_serializing_if="Option::is_none")]
  average_tx_byte_rate: Option<i64>,
  /// The one minute average of the message rate transmitted to the Client, in messages per second (msg/sec).
  #[serde(rename = "averageTxMsgRate", skip_serializing_if="Option::is_none")]
  average_tx_msg_rate: Option<i64>,
  /// The number of Client requests to bind to an endpoint.
  #[serde(rename = "bindRequestCount", skip_serializing_if="Option::is_none")]
  bind_request_count: Option<i64>,
  /// The number of successful Client requests to bind to an endpoint.
  #[serde(rename = "bindSuccessCount", skip_serializing_if="Option::is_none")]
  bind_success_count: Option<i64>,
  /// The IP address and port of the Client.
  #[serde(rename = "clientAddress", skip_serializing_if="Option::is_none")]
  client_address: Option<String>,
  /// The identifier (ID) of the Client.
  #[serde(rename = "clientId", skip_serializing_if="Option::is_none")]
  client_id: Option<i32>,
  /// The name of the Client.
  #[serde(rename = "clientName", skip_serializing_if="Option::is_none")]
  client_name: Option<String>,
  /// The name of the client profile of the Client.
  #[serde(rename = "clientProfileName", skip_serializing_if="Option::is_none")]
  client_profile_name: Option<String>,
  /// The client username of the Client used for authorization.
  #[serde(rename = "clientUsername", skip_serializing_if="Option::is_none")]
  client_username: Option<String>,
  /// The amount of client control messages received from the Client, in bytes (B).
  #[serde(rename = "controlRxByteCount", skip_serializing_if="Option::is_none")]
  control_rx_byte_count: Option<i64>,
  /// The number of client control messages received from the Client.
  #[serde(rename = "controlRxMsgCount", skip_serializing_if="Option::is_none")]
  control_rx_msg_count: Option<i64>,
  /// The amount of client control messages transmitted to the Client, in bytes (B).
  #[serde(rename = "controlTxByteCount", skip_serializing_if="Option::is_none")]
  control_tx_byte_count: Option<i64>,
  /// The number of client control messages transmitted to the Client.
  #[serde(rename = "controlTxMsgCount", skip_serializing_if="Option::is_none")]
  control_tx_msg_count: Option<i64>,
  /// The number of Client bind failures due to being denied cut-through forwarding.
  #[serde(rename = "cutThroughDeniedBindFailureCount", skip_serializing_if="Option::is_none")]
  cut_through_denied_bind_failure_count: Option<i64>,
  /// The amount of client data messages received from the Client, in bytes (B).
  #[serde(rename = "dataRxByteCount", skip_serializing_if="Option::is_none")]
  data_rx_byte_count: Option<i64>,
  /// The number of client data messages received from the Client.
  #[serde(rename = "dataRxMsgCount", skip_serializing_if="Option::is_none")]
  data_rx_msg_count: Option<i64>,
  /// The amount of client data messages transmitted to the Client, in bytes (B).
  #[serde(rename = "dataTxByteCount", skip_serializing_if="Option::is_none")]
  data_tx_byte_count: Option<i64>,
  /// The number of client data messages transmitted to the Client.
  #[serde(rename = "dataTxMsgCount", skip_serializing_if="Option::is_none")]
  data_tx_msg_count: Option<i64>,
  /// The description text of the Client.
  #[serde(rename = "description", skip_serializing_if="Option::is_none")]
  description: Option<String>,
  /// The number of Client bind failures due to endpoint being disabled.
  #[serde(rename = "disabledBindFailureCount", skip_serializing_if="Option::is_none")]
  disabled_bind_failure_count: Option<i64>,
  /// The priority of the Client's subscriptions for receiving deliver-to-one (DTO) messages published on the local broker.
  #[serde(rename = "dtoLocalPriority", skip_serializing_if="Option::is_none")]
  dto_local_priority: Option<i32>,
  /// The priority of the Client's subscriptions for receiving deliver-to-one (DTO) messages published on a remote broker.
  #[serde(rename = "dtoNetworkPriority", skip_serializing_if="Option::is_none")]
  dto_network_priority: Option<i32>,
  /// Indicates whether message eliding is enabled for the Client.
  #[serde(rename = "eliding", skip_serializing_if="Option::is_none")]
  eliding: Option<bool>,
  /// The number of topics requiring message eliding for the Client.
  #[serde(rename = "elidingTopicCount", skip_serializing_if="Option::is_none")]
  eliding_topic_count: Option<i32>,
  /// The peak number of topics requiring message eliding for the Client.
  #[serde(rename = "elidingTopicPeakCount", skip_serializing_if="Option::is_none")]
  eliding_topic_peak_count: Option<i32>,
  /// The number of Client bind failures due to being denied guaranteed messaging.
  #[serde(rename = "guaranteedDeniedBindFailureCount", skip_serializing_if="Option::is_none")]
  guaranteed_denied_bind_failure_count: Option<i64>,
  /// The number of Client bind failures due to an invalid selector.
  #[serde(rename = "invalidSelectorBindFailureCount", skip_serializing_if="Option::is_none")]
  invalid_selector_bind_failure_count: Option<i64>,
  /// Indicates whether the large-message event has been raised for the Client.
  #[serde(rename = "largeMsgEventRaised", skip_serializing_if="Option::is_none")]
  large_msg_event_raised: Option<bool>,
  /// The number of login request messages received from the Client.
  #[serde(rename = "loginRxMsgCount", skip_serializing_if="Option::is_none")]
  login_rx_msg_count: Option<i64>,
  /// The number of login response messages transmitted to the Client.
  #[serde(rename = "loginTxMsgCount", skip_serializing_if="Option::is_none")]
  login_tx_msg_count: Option<i64>,
  /// The number of Client bind failures due to the endpoint maximum bind count being exceeded.
  #[serde(rename = "maxBindCountExceededBindFailureCount", skip_serializing_if="Option::is_none")]
  max_bind_count_exceeded_bind_failure_count: Option<i64>,
  /// Indicates whether the max-eliding-topic-count event has been raised for the Client.
  #[serde(rename = "maxElidingTopicCountEventRaised", skip_serializing_if="Option::is_none")]
  max_eliding_topic_count_event_raised: Option<bool>,
  /// The number of MQTT connect acknowledgment (CONNACK) refused response packets transmitted to the Client.
  #[serde(rename = "mqttConnackErrorTxCount", skip_serializing_if="Option::is_none")]
  mqtt_connack_error_tx_count: Option<i64>,
  /// The number of MQTT connect acknowledgment (CONNACK) accepted response packets transmitted to the Client.
  #[serde(rename = "mqttConnackTxCount", skip_serializing_if="Option::is_none")]
  mqtt_connack_tx_count: Option<i64>,
  /// The number of MQTT connect (CONNECT) request packets received from the Client.
  #[serde(rename = "mqttConnectRxCount", skip_serializing_if="Option::is_none")]
  mqtt_connect_rx_count: Option<i64>,
  /// The number of MQTT disconnect (DISCONNECT) request packets received from the Client.
  #[serde(rename = "mqttDisconnectRxCount", skip_serializing_if="Option::is_none")]
  mqtt_disconnect_rx_count: Option<i64>,
  /// The number of MQTT ping request (PINGREQ) packets received from the Client.
  #[serde(rename = "mqttPingreqRxCount", skip_serializing_if="Option::is_none")]
  mqtt_pingreq_rx_count: Option<i64>,
  /// The number of MQTT ping response (PINGRESP) packets transmitted to the Client.
  #[serde(rename = "mqttPingrespTxCount", skip_serializing_if="Option::is_none")]
  mqtt_pingresp_tx_count: Option<i64>,
  /// The number of MQTT publish acknowledgement (PUBACK) response packets received from the Client.
  #[serde(rename = "mqttPubackRxCount", skip_serializing_if="Option::is_none")]
  mqtt_puback_rx_count: Option<i64>,
  /// The number of MQTT publish acknowledgement (PUBACK) response packets transmitted to the Client.
  #[serde(rename = "mqttPubackTxCount", skip_serializing_if="Option::is_none")]
  mqtt_puback_tx_count: Option<i64>,
  /// The number of MQTT publish complete (PUBCOMP) packets transmitted to the Client in response to a PUBREL packet. These packets are the fourth and final packet of a QoS 2 protocol exchange.
  #[serde(rename = "mqttPubcompTxCount", skip_serializing_if="Option::is_none")]
  mqtt_pubcomp_tx_count: Option<i64>,
  /// The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 0 message delivery.
  #[serde(rename = "mqttPublishQos0RxCount", skip_serializing_if="Option::is_none")]
  mqtt_publish_qos0_rx_count: Option<i64>,
  /// The number of MQTT publish message (PUBLISH) request packets transmitted to the Client for QoS 0 message delivery.
  #[serde(rename = "mqttPublishQos0TxCount", skip_serializing_if="Option::is_none")]
  mqtt_publish_qos0_tx_count: Option<i64>,
  /// The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 1 message delivery.
  #[serde(rename = "mqttPublishQos1RxCount", skip_serializing_if="Option::is_none")]
  mqtt_publish_qos1_rx_count: Option<i64>,
  /// The number of MQTT publish message (PUBLISH) request packets transmitted to the Client for QoS 1 message delivery.
  #[serde(rename = "mqttPublishQos1TxCount", skip_serializing_if="Option::is_none")]
  mqtt_publish_qos1_tx_count: Option<i64>,
  /// The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 2 message delivery.
  #[serde(rename = "mqttPublishQos2RxCount", skip_serializing_if="Option::is_none")]
  mqtt_publish_qos2_rx_count: Option<i64>,
  /// The number of MQTT publish received (PUBREC) packets transmitted to the Client in response to a PUBLISH packet with QoS 2. These packets are the second packet of a QoS 2 protocol exchange.
  #[serde(rename = "mqttPubrecTxCount", skip_serializing_if="Option::is_none")]
  mqtt_pubrec_tx_count: Option<i64>,
  /// The number of MQTT publish release (PUBREL) packets received from the Client in response to a PUBREC packet. These packets are the third packet of a QoS 2 protocol exchange.
  #[serde(rename = "mqttPubrelRxCount", skip_serializing_if="Option::is_none")]
  mqtt_pubrel_rx_count: Option<i64>,
  /// The number of MQTT subscribe acknowledgement (SUBACK) failure response packets transmitted to the Client.
  #[serde(rename = "mqttSubackErrorTxCount", skip_serializing_if="Option::is_none")]
  mqtt_suback_error_tx_count: Option<i64>,
  /// The number of MQTT subscribe acknowledgement (SUBACK) response packets transmitted to the Client.
  #[serde(rename = "mqttSubackTxCount", skip_serializing_if="Option::is_none")]
  mqtt_suback_tx_count: Option<i64>,
  /// The number of MQTT subscribe (SUBSCRIBE) request packets received from the Client to create one or more topic subscriptions.
  #[serde(rename = "mqttSubscribeRxCount", skip_serializing_if="Option::is_none")]
  mqtt_subscribe_rx_count: Option<i64>,
  /// The number of MQTT unsubscribe acknowledgement (UNSUBACK) response packets transmitted to the Client.
  #[serde(rename = "mqttUnsubackTxCount", skip_serializing_if="Option::is_none")]
  mqtt_unsuback_tx_count: Option<i64>,
  /// The number of MQTT unsubscribe (UNSUBSCRIBE) request packets received from the Client to remove one or more topic subscriptions.
  #[serde(rename = "mqttUnsubscribeRxCount", skip_serializing_if="Option::is_none")]
  mqtt_unsubscribe_rx_count: Option<i64>,
  /// The number of messages from the Client discarded due to message spool congestion primarily caused by message promotion.
  #[serde(rename = "msgSpoolCongestionRxDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  msg_spool_congestion_rx_discarded_msg_count: Option<i64>,
  /// The number of messages from the Client discarded by the message spool.
  #[serde(rename = "msgSpoolRxDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  msg_spool_rx_discarded_msg_count: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// Indicates whether not to deliver messages to the Client if it published them.
  #[serde(rename = "noLocalDelivery", skip_serializing_if="Option::is_none")]
  no_local_delivery: Option<bool>,
  /// The number of messages from the Client discarded due to no matching subscription found.
  #[serde(rename = "noSubscriptionMatchRxDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  no_subscription_match_rx_discarded_msg_count: Option<i64>,
  /// The original value of the client username used for Client authentication.
  #[serde(rename = "originalClientUsername", skip_serializing_if="Option::is_none")]
  original_client_username: Option<String>,
  /// The number of Client bind failures due to other reasons.
  #[serde(rename = "otherBindFailureCount", skip_serializing_if="Option::is_none")]
  other_bind_failure_count: Option<i64>,
  /// The platform the Client application software was built for, which may include the OS and API type.
  #[serde(rename = "platform", skip_serializing_if="Option::is_none")]
  platform: Option<String>,
  /// The number of messages from the Client discarded due to the publish topic being denied by the Access Control List (ACL) profile.
  #[serde(rename = "publishTopicAclRxDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  publish_topic_acl_rx_discarded_msg_count: Option<i64>,
  /// The amount of HTTP request messages received from the Client, in bytes (B).
  #[serde(rename = "restHttpRequestRxByteCount", skip_serializing_if="Option::is_none")]
  rest_http_request_rx_byte_count: Option<i64>,
  /// The number of HTTP request messages received from the Client.
  #[serde(rename = "restHttpRequestRxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_request_rx_msg_count: Option<i64>,
  /// The amount of HTTP request messages transmitted to the Client, in bytes (B).
  #[serde(rename = "restHttpRequestTxByteCount", skip_serializing_if="Option::is_none")]
  rest_http_request_tx_byte_count: Option<i64>,
  /// The number of HTTP request messages transmitted to the Client.
  #[serde(rename = "restHttpRequestTxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_request_tx_msg_count: Option<i64>,
  /// The number of HTTP client/server error response messages received from the Client.
  #[serde(rename = "restHttpResponseErrorRxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_response_error_rx_msg_count: Option<i64>,
  /// The number of HTTP client/server error response messages transmitted to the Client.
  #[serde(rename = "restHttpResponseErrorTxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_response_error_tx_msg_count: Option<i64>,
  /// The amount of HTTP response messages received from the Client, in bytes (B).
  #[serde(rename = "restHttpResponseRxByteCount", skip_serializing_if="Option::is_none")]
  rest_http_response_rx_byte_count: Option<i64>,
  /// The number of HTTP response messages received from the Client.
  #[serde(rename = "restHttpResponseRxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_response_rx_msg_count: Option<i64>,
  /// The number of HTTP successful response messages received from the Client.
  #[serde(rename = "restHttpResponseSuccessRxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_response_success_rx_msg_count: Option<i64>,
  /// The number of HTTP successful response messages transmitted to the Client.
  #[serde(rename = "restHttpResponseSuccessTxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_response_success_tx_msg_count: Option<i64>,
  /// The number of HTTP wait for reply timeout response messages received from the Client.
  #[serde(rename = "restHttpResponseTimeoutRxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_response_timeout_rx_msg_count: Option<i64>,
  /// The number of HTTP wait for reply timeout response messages transmitted to the Client.
  #[serde(rename = "restHttpResponseTimeoutTxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_response_timeout_tx_msg_count: Option<i64>,
  /// The amount of HTTP response messages transmitted to the Client, in bytes (B).
  #[serde(rename = "restHttpResponseTxByteCount", skip_serializing_if="Option::is_none")]
  rest_http_response_tx_byte_count: Option<i64>,
  /// The number of HTTP response messages transmitted to the Client.
  #[serde(rename = "restHttpResponseTxMsgCount", skip_serializing_if="Option::is_none")]
  rest_http_response_tx_msg_count: Option<i64>,
  /// The amount of messages received from the Client, in bytes (B).
  #[serde(rename = "rxByteCount", skip_serializing_if="Option::is_none")]
  rx_byte_count: Option<i64>,
  /// The current message rate received from the Client, in bytes per second (B/sec).
  #[serde(rename = "rxByteRate", skip_serializing_if="Option::is_none")]
  rx_byte_rate: Option<i64>,
  /// The number of messages discarded during reception from the Client.
  #[serde(rename = "rxDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  rx_discarded_msg_count: Option<i64>,
  /// The number of messages received from the Client.
  #[serde(rename = "rxMsgCount", skip_serializing_if="Option::is_none")]
  rx_msg_count: Option<i64>,
  /// The current message rate received from the Client, in messages per second (msg/sec).
  #[serde(rename = "rxMsgRate", skip_serializing_if="Option::is_none")]
  rx_msg_rate: Option<i64>,
  /// The timestamp of when the Client will be disconnected by the broker. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). Available since 2.13.
  #[serde(rename = "scheduledDisconnectTime", skip_serializing_if="Option::is_none")]
  scheduled_disconnect_time: Option<i32>,
  /// Indicates whether the Client is a slow subscriber and blocks for a few seconds when receiving messages.
  #[serde(rename = "slowSubscriber", skip_serializing_if="Option::is_none")]
  slow_subscriber: Option<bool>,
  /// The date the Client application software was built.
  #[serde(rename = "softwareDate", skip_serializing_if="Option::is_none")]
  software_date: Option<String>,
  /// The version of the Client application software.
  #[serde(rename = "softwareVersion", skip_serializing_if="Option::is_none")]
  software_version: Option<String>,
  /// The description of the TLS cipher used by the Client, which may include cipher name, key exchange and encryption algorithms.
  #[serde(rename = "tlsCipherDescription", skip_serializing_if="Option::is_none")]
  tls_cipher_description: Option<String>,
  /// Indicates whether the Client TLS connection was downgraded to plain-text to increase performance.
  #[serde(rename = "tlsDowngradedToPlainText", skip_serializing_if="Option::is_none")]
  tls_downgraded_to_plain_text: Option<bool>,
  /// The version of TLS used by the Client.
  #[serde(rename = "tlsVersion", skip_serializing_if="Option::is_none")]
  tls_version: Option<String>,
  /// The number of messages from the Client discarded due to an error while parsing the publish topic.
  #[serde(rename = "topicParseErrorRxDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  topic_parse_error_rx_discarded_msg_count: Option<i64>,
  /// The amount of messages transmitted to the Client, in bytes (B).
  #[serde(rename = "txByteCount", skip_serializing_if="Option::is_none")]
  tx_byte_count: Option<i64>,
  /// The current message rate transmitted to the Client, in bytes per second (B/sec).
  #[serde(rename = "txByteRate", skip_serializing_if="Option::is_none")]
  tx_byte_rate: Option<i64>,
  /// The number of messages discarded during transmission to the Client.
  #[serde(rename = "txDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  tx_discarded_msg_count: Option<i64>,
  /// The number of messages transmitted to the Client.
  #[serde(rename = "txMsgCount", skip_serializing_if="Option::is_none")]
  tx_msg_count: Option<i64>,
  /// The current message rate transmitted to the Client, in messages per second (msg/sec).
  #[serde(rename = "txMsgRate", skip_serializing_if="Option::is_none")]
  tx_msg_rate: Option<i64>,
  /// The amount of time in seconds since the Client connected.
  #[serde(rename = "uptime", skip_serializing_if="Option::is_none")]
  uptime: Option<i32>,
  /// The user description for the Client, which may include computer name and process ID.
  #[serde(rename = "user", skip_serializing_if="Option::is_none")]
  user: Option<String>,
  /// The virtual router used by the Client. The allowed values and their meaning are:  <pre> \"primary\" - The Client is using the primary virtual router. \"backup\" - The Client is using the backup virtual router. \"internal\" - The Client is using the internal virtual router. \"unknown\" - The Client virtual router is unknown. </pre> 
  #[serde(rename = "virtualRouter", skip_serializing_if="Option::is_none")]
  virtual_router: Option<String>,
  /// The maximum web transport timeout for the Client being inactive, in seconds.
  #[serde(rename = "webInactiveTimeout", skip_serializing_if="Option::is_none")]
  web_inactive_timeout: Option<i32>,
  /// The maximum web transport message payload size which excludes the size of the message header, in bytes.
  #[serde(rename = "webMaxPayload", skip_serializing_if="Option::is_none")]
  web_max_payload: Option<i64>,
  /// The number of messages from the Client discarded due to an error while parsing the web message.
  #[serde(rename = "webParseErrorRxDiscardedMsgCount", skip_serializing_if="Option::is_none")]
  web_parse_error_rx_discarded_msg_count: Option<i64>,
  /// The remaining web transport timeout for the Client being inactive, in seconds.
  #[serde(rename = "webRemainingTimeout", skip_serializing_if="Option::is_none")]
  web_remaining_timeout: Option<i32>,
  /// The amount of web transport messages received from the Client, in bytes (B).
  #[serde(rename = "webRxByteCount", skip_serializing_if="Option::is_none")]
  web_rx_byte_count: Option<i64>,
  /// The type of encoding used during reception from the Client. The allowed values and their meaning are:  <pre> \"binary\" - The Client is using binary encoding. \"base64\" - The Client is using base64 encoding. \"illegal\" - The Client is using an illegal encoding type. </pre> 
  #[serde(rename = "webRxEncoding", skip_serializing_if="Option::is_none")]
  web_rx_encoding: Option<String>,
  /// The number of web transport messages received from the Client.
  #[serde(rename = "webRxMsgCount", skip_serializing_if="Option::is_none")]
  web_rx_msg_count: Option<i64>,
  /// The type of web transport used during reception from the Client. The allowed values and their meaning are:  <pre> \"ws-binary\" - The Client is using WebSocket binary transport. \"http-binary-streaming\" - The Client is using HTTP binary streaming transport. \"http-binary\" - The Client is using HTTP binary transport. \"http-base64\" - The Client is using HTTP base64 transport. </pre> 
  #[serde(rename = "webRxProtocol", skip_serializing_if="Option::is_none")]
  web_rx_protocol: Option<String>,
  /// The number of web transport requests received from the Client (HTTP only). Not available for WebSockets.
  #[serde(rename = "webRxRequestCount", skip_serializing_if="Option::is_none")]
  web_rx_request_count: Option<i64>,
  /// The number of web transport responses transmitted to the Client on the receive connection (HTTP only). Not available for WebSockets.
  #[serde(rename = "webRxResponseCount", skip_serializing_if="Option::is_none")]
  web_rx_response_count: Option<i64>,
  /// The TCP state of the receive connection from the Client. When fully operational, should be: established. See RFC 793 for further details. The allowed values and their meaning are:  <pre> \"closed\" - No connection state at all. \"listen\" - Waiting for a connection request from any remote TCP and port. \"syn-sent\" - Waiting for a matching connection request after having sent a connection request. \"syn-received\" - Waiting for a confirming connection request acknowledgment after having both received and sent a connection request. \"established\" - An open connection, data received can be delivered to the user. \"close-wait\" - Waiting for a connection termination request from the local user. \"fin-wait-1\" - Waiting for a connection termination request from the remote TCP, or an acknowledgment of the connection termination request previously sent. \"closing\" - Waiting for a connection termination request acknowledgment from the remote TCP. \"last-ack\" - Waiting for an acknowledgment of the connection termination request previously sent to the remote TCP. \"fin-wait-2\" - Waiting for a connection termination request from the remote TCP. \"time-wait\" - Waiting for enough time to pass to be sure the remote TCP received the acknowledgment of its connection termination request. </pre> 
  #[serde(rename = "webRxTcpState", skip_serializing_if="Option::is_none")]
  web_rx_tcp_state: Option<String>,
  /// The description of the TLS cipher received from the Client, which may include cipher name, key exchange and encryption algorithms.
  #[serde(rename = "webRxTlsCipherDescription", skip_serializing_if="Option::is_none")]
  web_rx_tls_cipher_description: Option<String>,
  /// The version of TLS used during reception from the Client.
  #[serde(rename = "webRxTlsVersion", skip_serializing_if="Option::is_none")]
  web_rx_tls_version: Option<String>,
  /// The identifier (ID) of the web transport session for the Client.
  #[serde(rename = "webSessionId", skip_serializing_if="Option::is_none")]
  web_session_id: Option<String>,
  /// The amount of web transport messages transmitted to the Client, in bytes (B).
  #[serde(rename = "webTxByteCount", skip_serializing_if="Option::is_none")]
  web_tx_byte_count: Option<i64>,
  /// The type of encoding used during transmission to the Client. The allowed values and their meaning are:  <pre> \"binary\" - The Client is using binary encoding. \"base64\" - The Client is using base64 encoding. \"illegal\" - The Client is using an illegal encoding type. </pre> 
  #[serde(rename = "webTxEncoding", skip_serializing_if="Option::is_none")]
  web_tx_encoding: Option<String>,
  /// The number of web transport messages transmitted to the Client.
  #[serde(rename = "webTxMsgCount", skip_serializing_if="Option::is_none")]
  web_tx_msg_count: Option<i64>,
  /// The type of web transport used during transmission to the Client. The allowed values and their meaning are:  <pre> \"ws-binary\" - The Client is using WebSocket binary transport. \"http-binary-streaming\" - The Client is using HTTP binary streaming transport. \"http-binary\" - The Client is using HTTP binary transport. \"http-base64\" - The Client is using HTTP base64 transport. </pre> 
  #[serde(rename = "webTxProtocol", skip_serializing_if="Option::is_none")]
  web_tx_protocol: Option<String>,
  /// The number of web transport requests transmitted to the Client (HTTP only). Not available for WebSockets.
  #[serde(rename = "webTxRequestCount", skip_serializing_if="Option::is_none")]
  web_tx_request_count: Option<i64>,
  /// The number of web transport responses received from the Client on the transmit connection (HTTP only). Not available for WebSockets.
  #[serde(rename = "webTxResponseCount", skip_serializing_if="Option::is_none")]
  web_tx_response_count: Option<i64>,
  /// The TCP state of the transmit connection to the Client. When fully operational, should be: established. See RFC 793 for further details. The allowed values and their meaning are:  <pre> \"closed\" - No connection state at all. \"listen\" - Waiting for a connection request from any remote TCP and port. \"syn-sent\" - Waiting for a matching connection request after having sent a connection request. \"syn-received\" - Waiting for a confirming connection request acknowledgment after having both received and sent a connection request. \"established\" - An open connection, data received can be delivered to the user. \"close-wait\" - Waiting for a connection termination request from the local user. \"fin-wait-1\" - Waiting for a connection termination request from the remote TCP, or an acknowledgment of the connection termination request previously sent. \"closing\" - Waiting for a connection termination request acknowledgment from the remote TCP. \"last-ack\" - Waiting for an acknowledgment of the connection termination request previously sent to the remote TCP. \"fin-wait-2\" - Waiting for a connection termination request from the remote TCP. \"time-wait\" - Waiting for enough time to pass to be sure the remote TCP received the acknowledgment of its connection termination request. </pre> 
  #[serde(rename = "webTxTcpState", skip_serializing_if="Option::is_none")]
  web_tx_tcp_state: Option<String>,
  /// The description of the TLS cipher transmitted to the Client, which may include cipher name, key exchange and encryption algorithms.
  #[serde(rename = "webTxTlsCipherDescription", skip_serializing_if="Option::is_none")]
  web_tx_tls_cipher_description: Option<String>,
  /// The version of TLS used during transmission to the Client.
  #[serde(rename = "webTxTlsVersion", skip_serializing_if="Option::is_none")]
  web_tx_tls_version: Option<String>
}

impl MsgVpnClient {
  pub fn new() -> MsgVpnClient {
    MsgVpnClient {
      acl_profile_name: None,
      aliased_from_msg_vpn_name: None,
      already_bound_bind_failure_count: None,
      authorization_group_name: None,
      average_rx_byte_rate: None,
      average_rx_msg_rate: None,
      average_tx_byte_rate: None,
      average_tx_msg_rate: None,
      bind_request_count: None,
      bind_success_count: None,
      client_address: None,
      client_id: None,
      client_name: None,
      client_profile_name: None,
      client_username: None,
      control_rx_byte_count: None,
      control_rx_msg_count: None,
      control_tx_byte_count: None,
      control_tx_msg_count: None,
      cut_through_denied_bind_failure_count: None,
      data_rx_byte_count: None,
      data_rx_msg_count: None,
      data_tx_byte_count: None,
      data_tx_msg_count: None,
      description: None,
      disabled_bind_failure_count: None,
      dto_local_priority: None,
      dto_network_priority: None,
      eliding: None,
      eliding_topic_count: None,
      eliding_topic_peak_count: None,
      guaranteed_denied_bind_failure_count: None,
      invalid_selector_bind_failure_count: None,
      large_msg_event_raised: None,
      login_rx_msg_count: None,
      login_tx_msg_count: None,
      max_bind_count_exceeded_bind_failure_count: None,
      max_eliding_topic_count_event_raised: None,
      mqtt_connack_error_tx_count: None,
      mqtt_connack_tx_count: None,
      mqtt_connect_rx_count: None,
      mqtt_disconnect_rx_count: None,
      mqtt_pingreq_rx_count: None,
      mqtt_pingresp_tx_count: None,
      mqtt_puback_rx_count: None,
      mqtt_puback_tx_count: None,
      mqtt_pubcomp_tx_count: None,
      mqtt_publish_qos0_rx_count: None,
      mqtt_publish_qos0_tx_count: None,
      mqtt_publish_qos1_rx_count: None,
      mqtt_publish_qos1_tx_count: None,
      mqtt_publish_qos2_rx_count: None,
      mqtt_pubrec_tx_count: None,
      mqtt_pubrel_rx_count: None,
      mqtt_suback_error_tx_count: None,
      mqtt_suback_tx_count: None,
      mqtt_subscribe_rx_count: None,
      mqtt_unsuback_tx_count: None,
      mqtt_unsubscribe_rx_count: None,
      msg_spool_congestion_rx_discarded_msg_count: None,
      msg_spool_rx_discarded_msg_count: None,
      msg_vpn_name: None,
      no_local_delivery: None,
      no_subscription_match_rx_discarded_msg_count: None,
      original_client_username: None,
      other_bind_failure_count: None,
      platform: None,
      publish_topic_acl_rx_discarded_msg_count: None,
      rest_http_request_rx_byte_count: None,
      rest_http_request_rx_msg_count: None,
      rest_http_request_tx_byte_count: None,
      rest_http_request_tx_msg_count: None,
      rest_http_response_error_rx_msg_count: None,
      rest_http_response_error_tx_msg_count: None,
      rest_http_response_rx_byte_count: None,
      rest_http_response_rx_msg_count: None,
      rest_http_response_success_rx_msg_count: None,
      rest_http_response_success_tx_msg_count: None,
      rest_http_response_timeout_rx_msg_count: None,
      rest_http_response_timeout_tx_msg_count: None,
      rest_http_response_tx_byte_count: None,
      rest_http_response_tx_msg_count: None,
      rx_byte_count: None,
      rx_byte_rate: None,
      rx_discarded_msg_count: None,
      rx_msg_count: None,
      rx_msg_rate: None,
      scheduled_disconnect_time: None,
      slow_subscriber: None,
      software_date: None,
      software_version: None,
      tls_cipher_description: None,
      tls_downgraded_to_plain_text: None,
      tls_version: None,
      topic_parse_error_rx_discarded_msg_count: None,
      tx_byte_count: None,
      tx_byte_rate: None,
      tx_discarded_msg_count: None,
      tx_msg_count: None,
      tx_msg_rate: None,
      uptime: None,
      user: None,
      virtual_router: None,
      web_inactive_timeout: None,
      web_max_payload: None,
      web_parse_error_rx_discarded_msg_count: None,
      web_remaining_timeout: None,
      web_rx_byte_count: None,
      web_rx_encoding: None,
      web_rx_msg_count: None,
      web_rx_protocol: None,
      web_rx_request_count: None,
      web_rx_response_count: None,
      web_rx_tcp_state: None,
      web_rx_tls_cipher_description: None,
      web_rx_tls_version: None,
      web_session_id: None,
      web_tx_byte_count: None,
      web_tx_encoding: None,
      web_tx_msg_count: None,
      web_tx_protocol: None,
      web_tx_request_count: None,
      web_tx_response_count: None,
      web_tx_tcp_state: None,
      web_tx_tls_cipher_description: None,
      web_tx_tls_version: None
    }
  }

  pub fn set_acl_profile_name(&mut self, acl_profile_name: String) {
    self.acl_profile_name = Some(acl_profile_name);
  }

  pub fn with_acl_profile_name(mut self, acl_profile_name: String) -> MsgVpnClient {
    self.acl_profile_name = Some(acl_profile_name);
    self
  }

  pub fn acl_profile_name(&self) -> Option<&String> {
    self.acl_profile_name.as_ref()
  }

  pub fn reset_acl_profile_name(&mut self) {
    self.acl_profile_name = None;
  }

  pub fn set_aliased_from_msg_vpn_name(&mut self, aliased_from_msg_vpn_name: String) {
    self.aliased_from_msg_vpn_name = Some(aliased_from_msg_vpn_name);
  }

  pub fn with_aliased_from_msg_vpn_name(mut self, aliased_from_msg_vpn_name: String) -> MsgVpnClient {
    self.aliased_from_msg_vpn_name = Some(aliased_from_msg_vpn_name);
    self
  }

  pub fn aliased_from_msg_vpn_name(&self) -> Option<&String> {
    self.aliased_from_msg_vpn_name.as_ref()
  }

  pub fn reset_aliased_from_msg_vpn_name(&mut self) {
    self.aliased_from_msg_vpn_name = None;
  }

  pub fn set_already_bound_bind_failure_count(&mut self, already_bound_bind_failure_count: i64) {
    self.already_bound_bind_failure_count = Some(already_bound_bind_failure_count);
  }

  pub fn with_already_bound_bind_failure_count(mut self, already_bound_bind_failure_count: i64) -> MsgVpnClient {
    self.already_bound_bind_failure_count = Some(already_bound_bind_failure_count);
    self
  }

  pub fn already_bound_bind_failure_count(&self) -> Option<&i64> {
    self.already_bound_bind_failure_count.as_ref()
  }

  pub fn reset_already_bound_bind_failure_count(&mut self) {
    self.already_bound_bind_failure_count = None;
  }

  pub fn set_authorization_group_name(&mut self, authorization_group_name: String) {
    self.authorization_group_name = Some(authorization_group_name);
  }

  pub fn with_authorization_group_name(mut self, authorization_group_name: String) -> MsgVpnClient {
    self.authorization_group_name = Some(authorization_group_name);
    self
  }

  pub fn authorization_group_name(&self) -> Option<&String> {
    self.authorization_group_name.as_ref()
  }

  pub fn reset_authorization_group_name(&mut self) {
    self.authorization_group_name = None;
  }

  pub fn set_average_rx_byte_rate(&mut self, average_rx_byte_rate: i64) {
    self.average_rx_byte_rate = Some(average_rx_byte_rate);
  }

  pub fn with_average_rx_byte_rate(mut self, average_rx_byte_rate: i64) -> MsgVpnClient {
    self.average_rx_byte_rate = Some(average_rx_byte_rate);
    self
  }

  pub fn average_rx_byte_rate(&self) -> Option<&i64> {
    self.average_rx_byte_rate.as_ref()
  }

  pub fn reset_average_rx_byte_rate(&mut self) {
    self.average_rx_byte_rate = None;
  }

  pub fn set_average_rx_msg_rate(&mut self, average_rx_msg_rate: i64) {
    self.average_rx_msg_rate = Some(average_rx_msg_rate);
  }

  pub fn with_average_rx_msg_rate(mut self, average_rx_msg_rate: i64) -> MsgVpnClient {
    self.average_rx_msg_rate = Some(average_rx_msg_rate);
    self
  }

  pub fn average_rx_msg_rate(&self) -> Option<&i64> {
    self.average_rx_msg_rate.as_ref()
  }

  pub fn reset_average_rx_msg_rate(&mut self) {
    self.average_rx_msg_rate = None;
  }

  pub fn set_average_tx_byte_rate(&mut self, average_tx_byte_rate: i64) {
    self.average_tx_byte_rate = Some(average_tx_byte_rate);
  }

  pub fn with_average_tx_byte_rate(mut self, average_tx_byte_rate: i64) -> MsgVpnClient {
    self.average_tx_byte_rate = Some(average_tx_byte_rate);
    self
  }

  pub fn average_tx_byte_rate(&self) -> Option<&i64> {
    self.average_tx_byte_rate.as_ref()
  }

  pub fn reset_average_tx_byte_rate(&mut self) {
    self.average_tx_byte_rate = None;
  }

  pub fn set_average_tx_msg_rate(&mut self, average_tx_msg_rate: i64) {
    self.average_tx_msg_rate = Some(average_tx_msg_rate);
  }

  pub fn with_average_tx_msg_rate(mut self, average_tx_msg_rate: i64) -> MsgVpnClient {
    self.average_tx_msg_rate = Some(average_tx_msg_rate);
    self
  }

  pub fn average_tx_msg_rate(&self) -> Option<&i64> {
    self.average_tx_msg_rate.as_ref()
  }

  pub fn reset_average_tx_msg_rate(&mut self) {
    self.average_tx_msg_rate = None;
  }

  pub fn set_bind_request_count(&mut self, bind_request_count: i64) {
    self.bind_request_count = Some(bind_request_count);
  }

  pub fn with_bind_request_count(mut self, bind_request_count: i64) -> MsgVpnClient {
    self.bind_request_count = Some(bind_request_count);
    self
  }

  pub fn bind_request_count(&self) -> Option<&i64> {
    self.bind_request_count.as_ref()
  }

  pub fn reset_bind_request_count(&mut self) {
    self.bind_request_count = None;
  }

  pub fn set_bind_success_count(&mut self, bind_success_count: i64) {
    self.bind_success_count = Some(bind_success_count);
  }

  pub fn with_bind_success_count(mut self, bind_success_count: i64) -> MsgVpnClient {
    self.bind_success_count = Some(bind_success_count);
    self
  }

  pub fn bind_success_count(&self) -> Option<&i64> {
    self.bind_success_count.as_ref()
  }

  pub fn reset_bind_success_count(&mut self) {
    self.bind_success_count = None;
  }

  pub fn set_client_address(&mut self, client_address: String) {
    self.client_address = Some(client_address);
  }

  pub fn with_client_address(mut self, client_address: String) -> MsgVpnClient {
    self.client_address = Some(client_address);
    self
  }

  pub fn client_address(&self) -> Option<&String> {
    self.client_address.as_ref()
  }

  pub fn reset_client_address(&mut self) {
    self.client_address = None;
  }

  pub fn set_client_id(&mut self, client_id: i32) {
    self.client_id = Some(client_id);
  }

  pub fn with_client_id(mut self, client_id: i32) -> MsgVpnClient {
    self.client_id = Some(client_id);
    self
  }

  pub fn client_id(&self) -> Option<&i32> {
    self.client_id.as_ref()
  }

  pub fn reset_client_id(&mut self) {
    self.client_id = None;
  }

  pub fn set_client_name(&mut self, client_name: String) {
    self.client_name = Some(client_name);
  }

  pub fn with_client_name(mut self, client_name: String) -> MsgVpnClient {
    self.client_name = Some(client_name);
    self
  }

  pub fn client_name(&self) -> Option<&String> {
    self.client_name.as_ref()
  }

  pub fn reset_client_name(&mut self) {
    self.client_name = None;
  }

  pub fn set_client_profile_name(&mut self, client_profile_name: String) {
    self.client_profile_name = Some(client_profile_name);
  }

  pub fn with_client_profile_name(mut self, client_profile_name: String) -> MsgVpnClient {
    self.client_profile_name = Some(client_profile_name);
    self
  }

  pub fn client_profile_name(&self) -> Option<&String> {
    self.client_profile_name.as_ref()
  }

  pub fn reset_client_profile_name(&mut self) {
    self.client_profile_name = None;
  }

  pub fn set_client_username(&mut self, client_username: String) {
    self.client_username = Some(client_username);
  }

  pub fn with_client_username(mut self, client_username: String) -> MsgVpnClient {
    self.client_username = Some(client_username);
    self
  }

  pub fn client_username(&self) -> Option<&String> {
    self.client_username.as_ref()
  }

  pub fn reset_client_username(&mut self) {
    self.client_username = None;
  }

  pub fn set_control_rx_byte_count(&mut self, control_rx_byte_count: i64) {
    self.control_rx_byte_count = Some(control_rx_byte_count);
  }

  pub fn with_control_rx_byte_count(mut self, control_rx_byte_count: i64) -> MsgVpnClient {
    self.control_rx_byte_count = Some(control_rx_byte_count);
    self
  }

  pub fn control_rx_byte_count(&self) -> Option<&i64> {
    self.control_rx_byte_count.as_ref()
  }

  pub fn reset_control_rx_byte_count(&mut self) {
    self.control_rx_byte_count = None;
  }

  pub fn set_control_rx_msg_count(&mut self, control_rx_msg_count: i64) {
    self.control_rx_msg_count = Some(control_rx_msg_count);
  }

  pub fn with_control_rx_msg_count(mut self, control_rx_msg_count: i64) -> MsgVpnClient {
    self.control_rx_msg_count = Some(control_rx_msg_count);
    self
  }

  pub fn control_rx_msg_count(&self) -> Option<&i64> {
    self.control_rx_msg_count.as_ref()
  }

  pub fn reset_control_rx_msg_count(&mut self) {
    self.control_rx_msg_count = None;
  }

  pub fn set_control_tx_byte_count(&mut self, control_tx_byte_count: i64) {
    self.control_tx_byte_count = Some(control_tx_byte_count);
  }

  pub fn with_control_tx_byte_count(mut self, control_tx_byte_count: i64) -> MsgVpnClient {
    self.control_tx_byte_count = Some(control_tx_byte_count);
    self
  }

  pub fn control_tx_byte_count(&self) -> Option<&i64> {
    self.control_tx_byte_count.as_ref()
  }

  pub fn reset_control_tx_byte_count(&mut self) {
    self.control_tx_byte_count = None;
  }

  pub fn set_control_tx_msg_count(&mut self, control_tx_msg_count: i64) {
    self.control_tx_msg_count = Some(control_tx_msg_count);
  }

  pub fn with_control_tx_msg_count(mut self, control_tx_msg_count: i64) -> MsgVpnClient {
    self.control_tx_msg_count = Some(control_tx_msg_count);
    self
  }

  pub fn control_tx_msg_count(&self) -> Option<&i64> {
    self.control_tx_msg_count.as_ref()
  }

  pub fn reset_control_tx_msg_count(&mut self) {
    self.control_tx_msg_count = None;
  }

  pub fn set_cut_through_denied_bind_failure_count(&mut self, cut_through_denied_bind_failure_count: i64) {
    self.cut_through_denied_bind_failure_count = Some(cut_through_denied_bind_failure_count);
  }

  pub fn with_cut_through_denied_bind_failure_count(mut self, cut_through_denied_bind_failure_count: i64) -> MsgVpnClient {
    self.cut_through_denied_bind_failure_count = Some(cut_through_denied_bind_failure_count);
    self
  }

  pub fn cut_through_denied_bind_failure_count(&self) -> Option<&i64> {
    self.cut_through_denied_bind_failure_count.as_ref()
  }

  pub fn reset_cut_through_denied_bind_failure_count(&mut self) {
    self.cut_through_denied_bind_failure_count = None;
  }

  pub fn set_data_rx_byte_count(&mut self, data_rx_byte_count: i64) {
    self.data_rx_byte_count = Some(data_rx_byte_count);
  }

  pub fn with_data_rx_byte_count(mut self, data_rx_byte_count: i64) -> MsgVpnClient {
    self.data_rx_byte_count = Some(data_rx_byte_count);
    self
  }

  pub fn data_rx_byte_count(&self) -> Option<&i64> {
    self.data_rx_byte_count.as_ref()
  }

  pub fn reset_data_rx_byte_count(&mut self) {
    self.data_rx_byte_count = None;
  }

  pub fn set_data_rx_msg_count(&mut self, data_rx_msg_count: i64) {
    self.data_rx_msg_count = Some(data_rx_msg_count);
  }

  pub fn with_data_rx_msg_count(mut self, data_rx_msg_count: i64) -> MsgVpnClient {
    self.data_rx_msg_count = Some(data_rx_msg_count);
    self
  }

  pub fn data_rx_msg_count(&self) -> Option<&i64> {
    self.data_rx_msg_count.as_ref()
  }

  pub fn reset_data_rx_msg_count(&mut self) {
    self.data_rx_msg_count = None;
  }

  pub fn set_data_tx_byte_count(&mut self, data_tx_byte_count: i64) {
    self.data_tx_byte_count = Some(data_tx_byte_count);
  }

  pub fn with_data_tx_byte_count(mut self, data_tx_byte_count: i64) -> MsgVpnClient {
    self.data_tx_byte_count = Some(data_tx_byte_count);
    self
  }

  pub fn data_tx_byte_count(&self) -> Option<&i64> {
    self.data_tx_byte_count.as_ref()
  }

  pub fn reset_data_tx_byte_count(&mut self) {
    self.data_tx_byte_count = None;
  }

  pub fn set_data_tx_msg_count(&mut self, data_tx_msg_count: i64) {
    self.data_tx_msg_count = Some(data_tx_msg_count);
  }

  pub fn with_data_tx_msg_count(mut self, data_tx_msg_count: i64) -> MsgVpnClient {
    self.data_tx_msg_count = Some(data_tx_msg_count);
    self
  }

  pub fn data_tx_msg_count(&self) -> Option<&i64> {
    self.data_tx_msg_count.as_ref()
  }

  pub fn reset_data_tx_msg_count(&mut self) {
    self.data_tx_msg_count = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> MsgVpnClient {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_disabled_bind_failure_count(&mut self, disabled_bind_failure_count: i64) {
    self.disabled_bind_failure_count = Some(disabled_bind_failure_count);
  }

  pub fn with_disabled_bind_failure_count(mut self, disabled_bind_failure_count: i64) -> MsgVpnClient {
    self.disabled_bind_failure_count = Some(disabled_bind_failure_count);
    self
  }

  pub fn disabled_bind_failure_count(&self) -> Option<&i64> {
    self.disabled_bind_failure_count.as_ref()
  }

  pub fn reset_disabled_bind_failure_count(&mut self) {
    self.disabled_bind_failure_count = None;
  }

  pub fn set_dto_local_priority(&mut self, dto_local_priority: i32) {
    self.dto_local_priority = Some(dto_local_priority);
  }

  pub fn with_dto_local_priority(mut self, dto_local_priority: i32) -> MsgVpnClient {
    self.dto_local_priority = Some(dto_local_priority);
    self
  }

  pub fn dto_local_priority(&self) -> Option<&i32> {
    self.dto_local_priority.as_ref()
  }

  pub fn reset_dto_local_priority(&mut self) {
    self.dto_local_priority = None;
  }

  pub fn set_dto_network_priority(&mut self, dto_network_priority: i32) {
    self.dto_network_priority = Some(dto_network_priority);
  }

  pub fn with_dto_network_priority(mut self, dto_network_priority: i32) -> MsgVpnClient {
    self.dto_network_priority = Some(dto_network_priority);
    self
  }

  pub fn dto_network_priority(&self) -> Option<&i32> {
    self.dto_network_priority.as_ref()
  }

  pub fn reset_dto_network_priority(&mut self) {
    self.dto_network_priority = None;
  }

  pub fn set_eliding(&mut self, eliding: bool) {
    self.eliding = Some(eliding);
  }

  pub fn with_eliding(mut self, eliding: bool) -> MsgVpnClient {
    self.eliding = Some(eliding);
    self
  }

  pub fn eliding(&self) -> Option<&bool> {
    self.eliding.as_ref()
  }

  pub fn reset_eliding(&mut self) {
    self.eliding = None;
  }

  pub fn set_eliding_topic_count(&mut self, eliding_topic_count: i32) {
    self.eliding_topic_count = Some(eliding_topic_count);
  }

  pub fn with_eliding_topic_count(mut self, eliding_topic_count: i32) -> MsgVpnClient {
    self.eliding_topic_count = Some(eliding_topic_count);
    self
  }

  pub fn eliding_topic_count(&self) -> Option<&i32> {
    self.eliding_topic_count.as_ref()
  }

  pub fn reset_eliding_topic_count(&mut self) {
    self.eliding_topic_count = None;
  }

  pub fn set_eliding_topic_peak_count(&mut self, eliding_topic_peak_count: i32) {
    self.eliding_topic_peak_count = Some(eliding_topic_peak_count);
  }

  pub fn with_eliding_topic_peak_count(mut self, eliding_topic_peak_count: i32) -> MsgVpnClient {
    self.eliding_topic_peak_count = Some(eliding_topic_peak_count);
    self
  }

  pub fn eliding_topic_peak_count(&self) -> Option<&i32> {
    self.eliding_topic_peak_count.as_ref()
  }

  pub fn reset_eliding_topic_peak_count(&mut self) {
    self.eliding_topic_peak_count = None;
  }

  pub fn set_guaranteed_denied_bind_failure_count(&mut self, guaranteed_denied_bind_failure_count: i64) {
    self.guaranteed_denied_bind_failure_count = Some(guaranteed_denied_bind_failure_count);
  }

  pub fn with_guaranteed_denied_bind_failure_count(mut self, guaranteed_denied_bind_failure_count: i64) -> MsgVpnClient {
    self.guaranteed_denied_bind_failure_count = Some(guaranteed_denied_bind_failure_count);
    self
  }

  pub fn guaranteed_denied_bind_failure_count(&self) -> Option<&i64> {
    self.guaranteed_denied_bind_failure_count.as_ref()
  }

  pub fn reset_guaranteed_denied_bind_failure_count(&mut self) {
    self.guaranteed_denied_bind_failure_count = None;
  }

  pub fn set_invalid_selector_bind_failure_count(&mut self, invalid_selector_bind_failure_count: i64) {
    self.invalid_selector_bind_failure_count = Some(invalid_selector_bind_failure_count);
  }

  pub fn with_invalid_selector_bind_failure_count(mut self, invalid_selector_bind_failure_count: i64) -> MsgVpnClient {
    self.invalid_selector_bind_failure_count = Some(invalid_selector_bind_failure_count);
    self
  }

  pub fn invalid_selector_bind_failure_count(&self) -> Option<&i64> {
    self.invalid_selector_bind_failure_count.as_ref()
  }

  pub fn reset_invalid_selector_bind_failure_count(&mut self) {
    self.invalid_selector_bind_failure_count = None;
  }

  pub fn set_large_msg_event_raised(&mut self, large_msg_event_raised: bool) {
    self.large_msg_event_raised = Some(large_msg_event_raised);
  }

  pub fn with_large_msg_event_raised(mut self, large_msg_event_raised: bool) -> MsgVpnClient {
    self.large_msg_event_raised = Some(large_msg_event_raised);
    self
  }

  pub fn large_msg_event_raised(&self) -> Option<&bool> {
    self.large_msg_event_raised.as_ref()
  }

  pub fn reset_large_msg_event_raised(&mut self) {
    self.large_msg_event_raised = None;
  }

  pub fn set_login_rx_msg_count(&mut self, login_rx_msg_count: i64) {
    self.login_rx_msg_count = Some(login_rx_msg_count);
  }

  pub fn with_login_rx_msg_count(mut self, login_rx_msg_count: i64) -> MsgVpnClient {
    self.login_rx_msg_count = Some(login_rx_msg_count);
    self
  }

  pub fn login_rx_msg_count(&self) -> Option<&i64> {
    self.login_rx_msg_count.as_ref()
  }

  pub fn reset_login_rx_msg_count(&mut self) {
    self.login_rx_msg_count = None;
  }

  pub fn set_login_tx_msg_count(&mut self, login_tx_msg_count: i64) {
    self.login_tx_msg_count = Some(login_tx_msg_count);
  }

  pub fn with_login_tx_msg_count(mut self, login_tx_msg_count: i64) -> MsgVpnClient {
    self.login_tx_msg_count = Some(login_tx_msg_count);
    self
  }

  pub fn login_tx_msg_count(&self) -> Option<&i64> {
    self.login_tx_msg_count.as_ref()
  }

  pub fn reset_login_tx_msg_count(&mut self) {
    self.login_tx_msg_count = None;
  }

  pub fn set_max_bind_count_exceeded_bind_failure_count(&mut self, max_bind_count_exceeded_bind_failure_count: i64) {
    self.max_bind_count_exceeded_bind_failure_count = Some(max_bind_count_exceeded_bind_failure_count);
  }

  pub fn with_max_bind_count_exceeded_bind_failure_count(mut self, max_bind_count_exceeded_bind_failure_count: i64) -> MsgVpnClient {
    self.max_bind_count_exceeded_bind_failure_count = Some(max_bind_count_exceeded_bind_failure_count);
    self
  }

  pub fn max_bind_count_exceeded_bind_failure_count(&self) -> Option<&i64> {
    self.max_bind_count_exceeded_bind_failure_count.as_ref()
  }

  pub fn reset_max_bind_count_exceeded_bind_failure_count(&mut self) {
    self.max_bind_count_exceeded_bind_failure_count = None;
  }

  pub fn set_max_eliding_topic_count_event_raised(&mut self, max_eliding_topic_count_event_raised: bool) {
    self.max_eliding_topic_count_event_raised = Some(max_eliding_topic_count_event_raised);
  }

  pub fn with_max_eliding_topic_count_event_raised(mut self, max_eliding_topic_count_event_raised: bool) -> MsgVpnClient {
    self.max_eliding_topic_count_event_raised = Some(max_eliding_topic_count_event_raised);
    self
  }

  pub fn max_eliding_topic_count_event_raised(&self) -> Option<&bool> {
    self.max_eliding_topic_count_event_raised.as_ref()
  }

  pub fn reset_max_eliding_topic_count_event_raised(&mut self) {
    self.max_eliding_topic_count_event_raised = None;
  }

  pub fn set_mqtt_connack_error_tx_count(&mut self, mqtt_connack_error_tx_count: i64) {
    self.mqtt_connack_error_tx_count = Some(mqtt_connack_error_tx_count);
  }

  pub fn with_mqtt_connack_error_tx_count(mut self, mqtt_connack_error_tx_count: i64) -> MsgVpnClient {
    self.mqtt_connack_error_tx_count = Some(mqtt_connack_error_tx_count);
    self
  }

  pub fn mqtt_connack_error_tx_count(&self) -> Option<&i64> {
    self.mqtt_connack_error_tx_count.as_ref()
  }

  pub fn reset_mqtt_connack_error_tx_count(&mut self) {
    self.mqtt_connack_error_tx_count = None;
  }

  pub fn set_mqtt_connack_tx_count(&mut self, mqtt_connack_tx_count: i64) {
    self.mqtt_connack_tx_count = Some(mqtt_connack_tx_count);
  }

  pub fn with_mqtt_connack_tx_count(mut self, mqtt_connack_tx_count: i64) -> MsgVpnClient {
    self.mqtt_connack_tx_count = Some(mqtt_connack_tx_count);
    self
  }

  pub fn mqtt_connack_tx_count(&self) -> Option<&i64> {
    self.mqtt_connack_tx_count.as_ref()
  }

  pub fn reset_mqtt_connack_tx_count(&mut self) {
    self.mqtt_connack_tx_count = None;
  }

  pub fn set_mqtt_connect_rx_count(&mut self, mqtt_connect_rx_count: i64) {
    self.mqtt_connect_rx_count = Some(mqtt_connect_rx_count);
  }

  pub fn with_mqtt_connect_rx_count(mut self, mqtt_connect_rx_count: i64) -> MsgVpnClient {
    self.mqtt_connect_rx_count = Some(mqtt_connect_rx_count);
    self
  }

  pub fn mqtt_connect_rx_count(&self) -> Option<&i64> {
    self.mqtt_connect_rx_count.as_ref()
  }

  pub fn reset_mqtt_connect_rx_count(&mut self) {
    self.mqtt_connect_rx_count = None;
  }

  pub fn set_mqtt_disconnect_rx_count(&mut self, mqtt_disconnect_rx_count: i64) {
    self.mqtt_disconnect_rx_count = Some(mqtt_disconnect_rx_count);
  }

  pub fn with_mqtt_disconnect_rx_count(mut self, mqtt_disconnect_rx_count: i64) -> MsgVpnClient {
    self.mqtt_disconnect_rx_count = Some(mqtt_disconnect_rx_count);
    self
  }

  pub fn mqtt_disconnect_rx_count(&self) -> Option<&i64> {
    self.mqtt_disconnect_rx_count.as_ref()
  }

  pub fn reset_mqtt_disconnect_rx_count(&mut self) {
    self.mqtt_disconnect_rx_count = None;
  }

  pub fn set_mqtt_pingreq_rx_count(&mut self, mqtt_pingreq_rx_count: i64) {
    self.mqtt_pingreq_rx_count = Some(mqtt_pingreq_rx_count);
  }

  pub fn with_mqtt_pingreq_rx_count(mut self, mqtt_pingreq_rx_count: i64) -> MsgVpnClient {
    self.mqtt_pingreq_rx_count = Some(mqtt_pingreq_rx_count);
    self
  }

  pub fn mqtt_pingreq_rx_count(&self) -> Option<&i64> {
    self.mqtt_pingreq_rx_count.as_ref()
  }

  pub fn reset_mqtt_pingreq_rx_count(&mut self) {
    self.mqtt_pingreq_rx_count = None;
  }

  pub fn set_mqtt_pingresp_tx_count(&mut self, mqtt_pingresp_tx_count: i64) {
    self.mqtt_pingresp_tx_count = Some(mqtt_pingresp_tx_count);
  }

  pub fn with_mqtt_pingresp_tx_count(mut self, mqtt_pingresp_tx_count: i64) -> MsgVpnClient {
    self.mqtt_pingresp_tx_count = Some(mqtt_pingresp_tx_count);
    self
  }

  pub fn mqtt_pingresp_tx_count(&self) -> Option<&i64> {
    self.mqtt_pingresp_tx_count.as_ref()
  }

  pub fn reset_mqtt_pingresp_tx_count(&mut self) {
    self.mqtt_pingresp_tx_count = None;
  }

  pub fn set_mqtt_puback_rx_count(&mut self, mqtt_puback_rx_count: i64) {
    self.mqtt_puback_rx_count = Some(mqtt_puback_rx_count);
  }

  pub fn with_mqtt_puback_rx_count(mut self, mqtt_puback_rx_count: i64) -> MsgVpnClient {
    self.mqtt_puback_rx_count = Some(mqtt_puback_rx_count);
    self
  }

  pub fn mqtt_puback_rx_count(&self) -> Option<&i64> {
    self.mqtt_puback_rx_count.as_ref()
  }

  pub fn reset_mqtt_puback_rx_count(&mut self) {
    self.mqtt_puback_rx_count = None;
  }

  pub fn set_mqtt_puback_tx_count(&mut self, mqtt_puback_tx_count: i64) {
    self.mqtt_puback_tx_count = Some(mqtt_puback_tx_count);
  }

  pub fn with_mqtt_puback_tx_count(mut self, mqtt_puback_tx_count: i64) -> MsgVpnClient {
    self.mqtt_puback_tx_count = Some(mqtt_puback_tx_count);
    self
  }

  pub fn mqtt_puback_tx_count(&self) -> Option<&i64> {
    self.mqtt_puback_tx_count.as_ref()
  }

  pub fn reset_mqtt_puback_tx_count(&mut self) {
    self.mqtt_puback_tx_count = None;
  }

  pub fn set_mqtt_pubcomp_tx_count(&mut self, mqtt_pubcomp_tx_count: i64) {
    self.mqtt_pubcomp_tx_count = Some(mqtt_pubcomp_tx_count);
  }

  pub fn with_mqtt_pubcomp_tx_count(mut self, mqtt_pubcomp_tx_count: i64) -> MsgVpnClient {
    self.mqtt_pubcomp_tx_count = Some(mqtt_pubcomp_tx_count);
    self
  }

  pub fn mqtt_pubcomp_tx_count(&self) -> Option<&i64> {
    self.mqtt_pubcomp_tx_count.as_ref()
  }

  pub fn reset_mqtt_pubcomp_tx_count(&mut self) {
    self.mqtt_pubcomp_tx_count = None;
  }

  pub fn set_mqtt_publish_qos0_rx_count(&mut self, mqtt_publish_qos0_rx_count: i64) {
    self.mqtt_publish_qos0_rx_count = Some(mqtt_publish_qos0_rx_count);
  }

  pub fn with_mqtt_publish_qos0_rx_count(mut self, mqtt_publish_qos0_rx_count: i64) -> MsgVpnClient {
    self.mqtt_publish_qos0_rx_count = Some(mqtt_publish_qos0_rx_count);
    self
  }

  pub fn mqtt_publish_qos0_rx_count(&self) -> Option<&i64> {
    self.mqtt_publish_qos0_rx_count.as_ref()
  }

  pub fn reset_mqtt_publish_qos0_rx_count(&mut self) {
    self.mqtt_publish_qos0_rx_count = None;
  }

  pub fn set_mqtt_publish_qos0_tx_count(&mut self, mqtt_publish_qos0_tx_count: i64) {
    self.mqtt_publish_qos0_tx_count = Some(mqtt_publish_qos0_tx_count);
  }

  pub fn with_mqtt_publish_qos0_tx_count(mut self, mqtt_publish_qos0_tx_count: i64) -> MsgVpnClient {
    self.mqtt_publish_qos0_tx_count = Some(mqtt_publish_qos0_tx_count);
    self
  }

  pub fn mqtt_publish_qos0_tx_count(&self) -> Option<&i64> {
    self.mqtt_publish_qos0_tx_count.as_ref()
  }

  pub fn reset_mqtt_publish_qos0_tx_count(&mut self) {
    self.mqtt_publish_qos0_tx_count = None;
  }

  pub fn set_mqtt_publish_qos1_rx_count(&mut self, mqtt_publish_qos1_rx_count: i64) {
    self.mqtt_publish_qos1_rx_count = Some(mqtt_publish_qos1_rx_count);
  }

  pub fn with_mqtt_publish_qos1_rx_count(mut self, mqtt_publish_qos1_rx_count: i64) -> MsgVpnClient {
    self.mqtt_publish_qos1_rx_count = Some(mqtt_publish_qos1_rx_count);
    self
  }

  pub fn mqtt_publish_qos1_rx_count(&self) -> Option<&i64> {
    self.mqtt_publish_qos1_rx_count.as_ref()
  }

  pub fn reset_mqtt_publish_qos1_rx_count(&mut self) {
    self.mqtt_publish_qos1_rx_count = None;
  }

  pub fn set_mqtt_publish_qos1_tx_count(&mut self, mqtt_publish_qos1_tx_count: i64) {
    self.mqtt_publish_qos1_tx_count = Some(mqtt_publish_qos1_tx_count);
  }

  pub fn with_mqtt_publish_qos1_tx_count(mut self, mqtt_publish_qos1_tx_count: i64) -> MsgVpnClient {
    self.mqtt_publish_qos1_tx_count = Some(mqtt_publish_qos1_tx_count);
    self
  }

  pub fn mqtt_publish_qos1_tx_count(&self) -> Option<&i64> {
    self.mqtt_publish_qos1_tx_count.as_ref()
  }

  pub fn reset_mqtt_publish_qos1_tx_count(&mut self) {
    self.mqtt_publish_qos1_tx_count = None;
  }

  pub fn set_mqtt_publish_qos2_rx_count(&mut self, mqtt_publish_qos2_rx_count: i64) {
    self.mqtt_publish_qos2_rx_count = Some(mqtt_publish_qos2_rx_count);
  }

  pub fn with_mqtt_publish_qos2_rx_count(mut self, mqtt_publish_qos2_rx_count: i64) -> MsgVpnClient {
    self.mqtt_publish_qos2_rx_count = Some(mqtt_publish_qos2_rx_count);
    self
  }

  pub fn mqtt_publish_qos2_rx_count(&self) -> Option<&i64> {
    self.mqtt_publish_qos2_rx_count.as_ref()
  }

  pub fn reset_mqtt_publish_qos2_rx_count(&mut self) {
    self.mqtt_publish_qos2_rx_count = None;
  }

  pub fn set_mqtt_pubrec_tx_count(&mut self, mqtt_pubrec_tx_count: i64) {
    self.mqtt_pubrec_tx_count = Some(mqtt_pubrec_tx_count);
  }

  pub fn with_mqtt_pubrec_tx_count(mut self, mqtt_pubrec_tx_count: i64) -> MsgVpnClient {
    self.mqtt_pubrec_tx_count = Some(mqtt_pubrec_tx_count);
    self
  }

  pub fn mqtt_pubrec_tx_count(&self) -> Option<&i64> {
    self.mqtt_pubrec_tx_count.as_ref()
  }

  pub fn reset_mqtt_pubrec_tx_count(&mut self) {
    self.mqtt_pubrec_tx_count = None;
  }

  pub fn set_mqtt_pubrel_rx_count(&mut self, mqtt_pubrel_rx_count: i64) {
    self.mqtt_pubrel_rx_count = Some(mqtt_pubrel_rx_count);
  }

  pub fn with_mqtt_pubrel_rx_count(mut self, mqtt_pubrel_rx_count: i64) -> MsgVpnClient {
    self.mqtt_pubrel_rx_count = Some(mqtt_pubrel_rx_count);
    self
  }

  pub fn mqtt_pubrel_rx_count(&self) -> Option<&i64> {
    self.mqtt_pubrel_rx_count.as_ref()
  }

  pub fn reset_mqtt_pubrel_rx_count(&mut self) {
    self.mqtt_pubrel_rx_count = None;
  }

  pub fn set_mqtt_suback_error_tx_count(&mut self, mqtt_suback_error_tx_count: i64) {
    self.mqtt_suback_error_tx_count = Some(mqtt_suback_error_tx_count);
  }

  pub fn with_mqtt_suback_error_tx_count(mut self, mqtt_suback_error_tx_count: i64) -> MsgVpnClient {
    self.mqtt_suback_error_tx_count = Some(mqtt_suback_error_tx_count);
    self
  }

  pub fn mqtt_suback_error_tx_count(&self) -> Option<&i64> {
    self.mqtt_suback_error_tx_count.as_ref()
  }

  pub fn reset_mqtt_suback_error_tx_count(&mut self) {
    self.mqtt_suback_error_tx_count = None;
  }

  pub fn set_mqtt_suback_tx_count(&mut self, mqtt_suback_tx_count: i64) {
    self.mqtt_suback_tx_count = Some(mqtt_suback_tx_count);
  }

  pub fn with_mqtt_suback_tx_count(mut self, mqtt_suback_tx_count: i64) -> MsgVpnClient {
    self.mqtt_suback_tx_count = Some(mqtt_suback_tx_count);
    self
  }

  pub fn mqtt_suback_tx_count(&self) -> Option<&i64> {
    self.mqtt_suback_tx_count.as_ref()
  }

  pub fn reset_mqtt_suback_tx_count(&mut self) {
    self.mqtt_suback_tx_count = None;
  }

  pub fn set_mqtt_subscribe_rx_count(&mut self, mqtt_subscribe_rx_count: i64) {
    self.mqtt_subscribe_rx_count = Some(mqtt_subscribe_rx_count);
  }

  pub fn with_mqtt_subscribe_rx_count(mut self, mqtt_subscribe_rx_count: i64) -> MsgVpnClient {
    self.mqtt_subscribe_rx_count = Some(mqtt_subscribe_rx_count);
    self
  }

  pub fn mqtt_subscribe_rx_count(&self) -> Option<&i64> {
    self.mqtt_subscribe_rx_count.as_ref()
  }

  pub fn reset_mqtt_subscribe_rx_count(&mut self) {
    self.mqtt_subscribe_rx_count = None;
  }

  pub fn set_mqtt_unsuback_tx_count(&mut self, mqtt_unsuback_tx_count: i64) {
    self.mqtt_unsuback_tx_count = Some(mqtt_unsuback_tx_count);
  }

  pub fn with_mqtt_unsuback_tx_count(mut self, mqtt_unsuback_tx_count: i64) -> MsgVpnClient {
    self.mqtt_unsuback_tx_count = Some(mqtt_unsuback_tx_count);
    self
  }

  pub fn mqtt_unsuback_tx_count(&self) -> Option<&i64> {
    self.mqtt_unsuback_tx_count.as_ref()
  }

  pub fn reset_mqtt_unsuback_tx_count(&mut self) {
    self.mqtt_unsuback_tx_count = None;
  }

  pub fn set_mqtt_unsubscribe_rx_count(&mut self, mqtt_unsubscribe_rx_count: i64) {
    self.mqtt_unsubscribe_rx_count = Some(mqtt_unsubscribe_rx_count);
  }

  pub fn with_mqtt_unsubscribe_rx_count(mut self, mqtt_unsubscribe_rx_count: i64) -> MsgVpnClient {
    self.mqtt_unsubscribe_rx_count = Some(mqtt_unsubscribe_rx_count);
    self
  }

  pub fn mqtt_unsubscribe_rx_count(&self) -> Option<&i64> {
    self.mqtt_unsubscribe_rx_count.as_ref()
  }

  pub fn reset_mqtt_unsubscribe_rx_count(&mut self) {
    self.mqtt_unsubscribe_rx_count = None;
  }

  pub fn set_msg_spool_congestion_rx_discarded_msg_count(&mut self, msg_spool_congestion_rx_discarded_msg_count: i64) {
    self.msg_spool_congestion_rx_discarded_msg_count = Some(msg_spool_congestion_rx_discarded_msg_count);
  }

  pub fn with_msg_spool_congestion_rx_discarded_msg_count(mut self, msg_spool_congestion_rx_discarded_msg_count: i64) -> MsgVpnClient {
    self.msg_spool_congestion_rx_discarded_msg_count = Some(msg_spool_congestion_rx_discarded_msg_count);
    self
  }

  pub fn msg_spool_congestion_rx_discarded_msg_count(&self) -> Option<&i64> {
    self.msg_spool_congestion_rx_discarded_msg_count.as_ref()
  }

  pub fn reset_msg_spool_congestion_rx_discarded_msg_count(&mut self) {
    self.msg_spool_congestion_rx_discarded_msg_count = None;
  }

  pub fn set_msg_spool_rx_discarded_msg_count(&mut self, msg_spool_rx_discarded_msg_count: i64) {
    self.msg_spool_rx_discarded_msg_count = Some(msg_spool_rx_discarded_msg_count);
  }

  pub fn with_msg_spool_rx_discarded_msg_count(mut self, msg_spool_rx_discarded_msg_count: i64) -> MsgVpnClient {
    self.msg_spool_rx_discarded_msg_count = Some(msg_spool_rx_discarded_msg_count);
    self
  }

  pub fn msg_spool_rx_discarded_msg_count(&self) -> Option<&i64> {
    self.msg_spool_rx_discarded_msg_count.as_ref()
  }

  pub fn reset_msg_spool_rx_discarded_msg_count(&mut self) {
    self.msg_spool_rx_discarded_msg_count = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnClient {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_no_local_delivery(&mut self, no_local_delivery: bool) {
    self.no_local_delivery = Some(no_local_delivery);
  }

  pub fn with_no_local_delivery(mut self, no_local_delivery: bool) -> MsgVpnClient {
    self.no_local_delivery = Some(no_local_delivery);
    self
  }

  pub fn no_local_delivery(&self) -> Option<&bool> {
    self.no_local_delivery.as_ref()
  }

  pub fn reset_no_local_delivery(&mut self) {
    self.no_local_delivery = None;
  }

  pub fn set_no_subscription_match_rx_discarded_msg_count(&mut self, no_subscription_match_rx_discarded_msg_count: i64) {
    self.no_subscription_match_rx_discarded_msg_count = Some(no_subscription_match_rx_discarded_msg_count);
  }

  pub fn with_no_subscription_match_rx_discarded_msg_count(mut self, no_subscription_match_rx_discarded_msg_count: i64) -> MsgVpnClient {
    self.no_subscription_match_rx_discarded_msg_count = Some(no_subscription_match_rx_discarded_msg_count);
    self
  }

  pub fn no_subscription_match_rx_discarded_msg_count(&self) -> Option<&i64> {
    self.no_subscription_match_rx_discarded_msg_count.as_ref()
  }

  pub fn reset_no_subscription_match_rx_discarded_msg_count(&mut self) {
    self.no_subscription_match_rx_discarded_msg_count = None;
  }

  pub fn set_original_client_username(&mut self, original_client_username: String) {
    self.original_client_username = Some(original_client_username);
  }

  pub fn with_original_client_username(mut self, original_client_username: String) -> MsgVpnClient {
    self.original_client_username = Some(original_client_username);
    self
  }

  pub fn original_client_username(&self) -> Option<&String> {
    self.original_client_username.as_ref()
  }

  pub fn reset_original_client_username(&mut self) {
    self.original_client_username = None;
  }

  pub fn set_other_bind_failure_count(&mut self, other_bind_failure_count: i64) {
    self.other_bind_failure_count = Some(other_bind_failure_count);
  }

  pub fn with_other_bind_failure_count(mut self, other_bind_failure_count: i64) -> MsgVpnClient {
    self.other_bind_failure_count = Some(other_bind_failure_count);
    self
  }

  pub fn other_bind_failure_count(&self) -> Option<&i64> {
    self.other_bind_failure_count.as_ref()
  }

  pub fn reset_other_bind_failure_count(&mut self) {
    self.other_bind_failure_count = None;
  }

  pub fn set_platform(&mut self, platform: String) {
    self.platform = Some(platform);
  }

  pub fn with_platform(mut self, platform: String) -> MsgVpnClient {
    self.platform = Some(platform);
    self
  }

  pub fn platform(&self) -> Option<&String> {
    self.platform.as_ref()
  }

  pub fn reset_platform(&mut self) {
    self.platform = None;
  }

  pub fn set_publish_topic_acl_rx_discarded_msg_count(&mut self, publish_topic_acl_rx_discarded_msg_count: i64) {
    self.publish_topic_acl_rx_discarded_msg_count = Some(publish_topic_acl_rx_discarded_msg_count);
  }

  pub fn with_publish_topic_acl_rx_discarded_msg_count(mut self, publish_topic_acl_rx_discarded_msg_count: i64) -> MsgVpnClient {
    self.publish_topic_acl_rx_discarded_msg_count = Some(publish_topic_acl_rx_discarded_msg_count);
    self
  }

  pub fn publish_topic_acl_rx_discarded_msg_count(&self) -> Option<&i64> {
    self.publish_topic_acl_rx_discarded_msg_count.as_ref()
  }

  pub fn reset_publish_topic_acl_rx_discarded_msg_count(&mut self) {
    self.publish_topic_acl_rx_discarded_msg_count = None;
  }

  pub fn set_rest_http_request_rx_byte_count(&mut self, rest_http_request_rx_byte_count: i64) {
    self.rest_http_request_rx_byte_count = Some(rest_http_request_rx_byte_count);
  }

  pub fn with_rest_http_request_rx_byte_count(mut self, rest_http_request_rx_byte_count: i64) -> MsgVpnClient {
    self.rest_http_request_rx_byte_count = Some(rest_http_request_rx_byte_count);
    self
  }

  pub fn rest_http_request_rx_byte_count(&self) -> Option<&i64> {
    self.rest_http_request_rx_byte_count.as_ref()
  }

  pub fn reset_rest_http_request_rx_byte_count(&mut self) {
    self.rest_http_request_rx_byte_count = None;
  }

  pub fn set_rest_http_request_rx_msg_count(&mut self, rest_http_request_rx_msg_count: i64) {
    self.rest_http_request_rx_msg_count = Some(rest_http_request_rx_msg_count);
  }

  pub fn with_rest_http_request_rx_msg_count(mut self, rest_http_request_rx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_request_rx_msg_count = Some(rest_http_request_rx_msg_count);
    self
  }

  pub fn rest_http_request_rx_msg_count(&self) -> Option<&i64> {
    self.rest_http_request_rx_msg_count.as_ref()
  }

  pub fn reset_rest_http_request_rx_msg_count(&mut self) {
    self.rest_http_request_rx_msg_count = None;
  }

  pub fn set_rest_http_request_tx_byte_count(&mut self, rest_http_request_tx_byte_count: i64) {
    self.rest_http_request_tx_byte_count = Some(rest_http_request_tx_byte_count);
  }

  pub fn with_rest_http_request_tx_byte_count(mut self, rest_http_request_tx_byte_count: i64) -> MsgVpnClient {
    self.rest_http_request_tx_byte_count = Some(rest_http_request_tx_byte_count);
    self
  }

  pub fn rest_http_request_tx_byte_count(&self) -> Option<&i64> {
    self.rest_http_request_tx_byte_count.as_ref()
  }

  pub fn reset_rest_http_request_tx_byte_count(&mut self) {
    self.rest_http_request_tx_byte_count = None;
  }

  pub fn set_rest_http_request_tx_msg_count(&mut self, rest_http_request_tx_msg_count: i64) {
    self.rest_http_request_tx_msg_count = Some(rest_http_request_tx_msg_count);
  }

  pub fn with_rest_http_request_tx_msg_count(mut self, rest_http_request_tx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_request_tx_msg_count = Some(rest_http_request_tx_msg_count);
    self
  }

  pub fn rest_http_request_tx_msg_count(&self) -> Option<&i64> {
    self.rest_http_request_tx_msg_count.as_ref()
  }

  pub fn reset_rest_http_request_tx_msg_count(&mut self) {
    self.rest_http_request_tx_msg_count = None;
  }

  pub fn set_rest_http_response_error_rx_msg_count(&mut self, rest_http_response_error_rx_msg_count: i64) {
    self.rest_http_response_error_rx_msg_count = Some(rest_http_response_error_rx_msg_count);
  }

  pub fn with_rest_http_response_error_rx_msg_count(mut self, rest_http_response_error_rx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_response_error_rx_msg_count = Some(rest_http_response_error_rx_msg_count);
    self
  }

  pub fn rest_http_response_error_rx_msg_count(&self) -> Option<&i64> {
    self.rest_http_response_error_rx_msg_count.as_ref()
  }

  pub fn reset_rest_http_response_error_rx_msg_count(&mut self) {
    self.rest_http_response_error_rx_msg_count = None;
  }

  pub fn set_rest_http_response_error_tx_msg_count(&mut self, rest_http_response_error_tx_msg_count: i64) {
    self.rest_http_response_error_tx_msg_count = Some(rest_http_response_error_tx_msg_count);
  }

  pub fn with_rest_http_response_error_tx_msg_count(mut self, rest_http_response_error_tx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_response_error_tx_msg_count = Some(rest_http_response_error_tx_msg_count);
    self
  }

  pub fn rest_http_response_error_tx_msg_count(&self) -> Option<&i64> {
    self.rest_http_response_error_tx_msg_count.as_ref()
  }

  pub fn reset_rest_http_response_error_tx_msg_count(&mut self) {
    self.rest_http_response_error_tx_msg_count = None;
  }

  pub fn set_rest_http_response_rx_byte_count(&mut self, rest_http_response_rx_byte_count: i64) {
    self.rest_http_response_rx_byte_count = Some(rest_http_response_rx_byte_count);
  }

  pub fn with_rest_http_response_rx_byte_count(mut self, rest_http_response_rx_byte_count: i64) -> MsgVpnClient {
    self.rest_http_response_rx_byte_count = Some(rest_http_response_rx_byte_count);
    self
  }

  pub fn rest_http_response_rx_byte_count(&self) -> Option<&i64> {
    self.rest_http_response_rx_byte_count.as_ref()
  }

  pub fn reset_rest_http_response_rx_byte_count(&mut self) {
    self.rest_http_response_rx_byte_count = None;
  }

  pub fn set_rest_http_response_rx_msg_count(&mut self, rest_http_response_rx_msg_count: i64) {
    self.rest_http_response_rx_msg_count = Some(rest_http_response_rx_msg_count);
  }

  pub fn with_rest_http_response_rx_msg_count(mut self, rest_http_response_rx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_response_rx_msg_count = Some(rest_http_response_rx_msg_count);
    self
  }

  pub fn rest_http_response_rx_msg_count(&self) -> Option<&i64> {
    self.rest_http_response_rx_msg_count.as_ref()
  }

  pub fn reset_rest_http_response_rx_msg_count(&mut self) {
    self.rest_http_response_rx_msg_count = None;
  }

  pub fn set_rest_http_response_success_rx_msg_count(&mut self, rest_http_response_success_rx_msg_count: i64) {
    self.rest_http_response_success_rx_msg_count = Some(rest_http_response_success_rx_msg_count);
  }

  pub fn with_rest_http_response_success_rx_msg_count(mut self, rest_http_response_success_rx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_response_success_rx_msg_count = Some(rest_http_response_success_rx_msg_count);
    self
  }

  pub fn rest_http_response_success_rx_msg_count(&self) -> Option<&i64> {
    self.rest_http_response_success_rx_msg_count.as_ref()
  }

  pub fn reset_rest_http_response_success_rx_msg_count(&mut self) {
    self.rest_http_response_success_rx_msg_count = None;
  }

  pub fn set_rest_http_response_success_tx_msg_count(&mut self, rest_http_response_success_tx_msg_count: i64) {
    self.rest_http_response_success_tx_msg_count = Some(rest_http_response_success_tx_msg_count);
  }

  pub fn with_rest_http_response_success_tx_msg_count(mut self, rest_http_response_success_tx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_response_success_tx_msg_count = Some(rest_http_response_success_tx_msg_count);
    self
  }

  pub fn rest_http_response_success_tx_msg_count(&self) -> Option<&i64> {
    self.rest_http_response_success_tx_msg_count.as_ref()
  }

  pub fn reset_rest_http_response_success_tx_msg_count(&mut self) {
    self.rest_http_response_success_tx_msg_count = None;
  }

  pub fn set_rest_http_response_timeout_rx_msg_count(&mut self, rest_http_response_timeout_rx_msg_count: i64) {
    self.rest_http_response_timeout_rx_msg_count = Some(rest_http_response_timeout_rx_msg_count);
  }

  pub fn with_rest_http_response_timeout_rx_msg_count(mut self, rest_http_response_timeout_rx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_response_timeout_rx_msg_count = Some(rest_http_response_timeout_rx_msg_count);
    self
  }

  pub fn rest_http_response_timeout_rx_msg_count(&self) -> Option<&i64> {
    self.rest_http_response_timeout_rx_msg_count.as_ref()
  }

  pub fn reset_rest_http_response_timeout_rx_msg_count(&mut self) {
    self.rest_http_response_timeout_rx_msg_count = None;
  }

  pub fn set_rest_http_response_timeout_tx_msg_count(&mut self, rest_http_response_timeout_tx_msg_count: i64) {
    self.rest_http_response_timeout_tx_msg_count = Some(rest_http_response_timeout_tx_msg_count);
  }

  pub fn with_rest_http_response_timeout_tx_msg_count(mut self, rest_http_response_timeout_tx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_response_timeout_tx_msg_count = Some(rest_http_response_timeout_tx_msg_count);
    self
  }

  pub fn rest_http_response_timeout_tx_msg_count(&self) -> Option<&i64> {
    self.rest_http_response_timeout_tx_msg_count.as_ref()
  }

  pub fn reset_rest_http_response_timeout_tx_msg_count(&mut self) {
    self.rest_http_response_timeout_tx_msg_count = None;
  }

  pub fn set_rest_http_response_tx_byte_count(&mut self, rest_http_response_tx_byte_count: i64) {
    self.rest_http_response_tx_byte_count = Some(rest_http_response_tx_byte_count);
  }

  pub fn with_rest_http_response_tx_byte_count(mut self, rest_http_response_tx_byte_count: i64) -> MsgVpnClient {
    self.rest_http_response_tx_byte_count = Some(rest_http_response_tx_byte_count);
    self
  }

  pub fn rest_http_response_tx_byte_count(&self) -> Option<&i64> {
    self.rest_http_response_tx_byte_count.as_ref()
  }

  pub fn reset_rest_http_response_tx_byte_count(&mut self) {
    self.rest_http_response_tx_byte_count = None;
  }

  pub fn set_rest_http_response_tx_msg_count(&mut self, rest_http_response_tx_msg_count: i64) {
    self.rest_http_response_tx_msg_count = Some(rest_http_response_tx_msg_count);
  }

  pub fn with_rest_http_response_tx_msg_count(mut self, rest_http_response_tx_msg_count: i64) -> MsgVpnClient {
    self.rest_http_response_tx_msg_count = Some(rest_http_response_tx_msg_count);
    self
  }

  pub fn rest_http_response_tx_msg_count(&self) -> Option<&i64> {
    self.rest_http_response_tx_msg_count.as_ref()
  }

  pub fn reset_rest_http_response_tx_msg_count(&mut self) {
    self.rest_http_response_tx_msg_count = None;
  }

  pub fn set_rx_byte_count(&mut self, rx_byte_count: i64) {
    self.rx_byte_count = Some(rx_byte_count);
  }

  pub fn with_rx_byte_count(mut self, rx_byte_count: i64) -> MsgVpnClient {
    self.rx_byte_count = Some(rx_byte_count);
    self
  }

  pub fn rx_byte_count(&self) -> Option<&i64> {
    self.rx_byte_count.as_ref()
  }

  pub fn reset_rx_byte_count(&mut self) {
    self.rx_byte_count = None;
  }

  pub fn set_rx_byte_rate(&mut self, rx_byte_rate: i64) {
    self.rx_byte_rate = Some(rx_byte_rate);
  }

  pub fn with_rx_byte_rate(mut self, rx_byte_rate: i64) -> MsgVpnClient {
    self.rx_byte_rate = Some(rx_byte_rate);
    self
  }

  pub fn rx_byte_rate(&self) -> Option<&i64> {
    self.rx_byte_rate.as_ref()
  }

  pub fn reset_rx_byte_rate(&mut self) {
    self.rx_byte_rate = None;
  }

  pub fn set_rx_discarded_msg_count(&mut self, rx_discarded_msg_count: i64) {
    self.rx_discarded_msg_count = Some(rx_discarded_msg_count);
  }

  pub fn with_rx_discarded_msg_count(mut self, rx_discarded_msg_count: i64) -> MsgVpnClient {
    self.rx_discarded_msg_count = Some(rx_discarded_msg_count);
    self
  }

  pub fn rx_discarded_msg_count(&self) -> Option<&i64> {
    self.rx_discarded_msg_count.as_ref()
  }

  pub fn reset_rx_discarded_msg_count(&mut self) {
    self.rx_discarded_msg_count = None;
  }

  pub fn set_rx_msg_count(&mut self, rx_msg_count: i64) {
    self.rx_msg_count = Some(rx_msg_count);
  }

  pub fn with_rx_msg_count(mut self, rx_msg_count: i64) -> MsgVpnClient {
    self.rx_msg_count = Some(rx_msg_count);
    self
  }

  pub fn rx_msg_count(&self) -> Option<&i64> {
    self.rx_msg_count.as_ref()
  }

  pub fn reset_rx_msg_count(&mut self) {
    self.rx_msg_count = None;
  }

  pub fn set_rx_msg_rate(&mut self, rx_msg_rate: i64) {
    self.rx_msg_rate = Some(rx_msg_rate);
  }

  pub fn with_rx_msg_rate(mut self, rx_msg_rate: i64) -> MsgVpnClient {
    self.rx_msg_rate = Some(rx_msg_rate);
    self
  }

  pub fn rx_msg_rate(&self) -> Option<&i64> {
    self.rx_msg_rate.as_ref()
  }

  pub fn reset_rx_msg_rate(&mut self) {
    self.rx_msg_rate = None;
  }

  pub fn set_scheduled_disconnect_time(&mut self, scheduled_disconnect_time: i32) {
    self.scheduled_disconnect_time = Some(scheduled_disconnect_time);
  }

  pub fn with_scheduled_disconnect_time(mut self, scheduled_disconnect_time: i32) -> MsgVpnClient {
    self.scheduled_disconnect_time = Some(scheduled_disconnect_time);
    self
  }

  pub fn scheduled_disconnect_time(&self) -> Option<&i32> {
    self.scheduled_disconnect_time.as_ref()
  }

  pub fn reset_scheduled_disconnect_time(&mut self) {
    self.scheduled_disconnect_time = None;
  }

  pub fn set_slow_subscriber(&mut self, slow_subscriber: bool) {
    self.slow_subscriber = Some(slow_subscriber);
  }

  pub fn with_slow_subscriber(mut self, slow_subscriber: bool) -> MsgVpnClient {
    self.slow_subscriber = Some(slow_subscriber);
    self
  }

  pub fn slow_subscriber(&self) -> Option<&bool> {
    self.slow_subscriber.as_ref()
  }

  pub fn reset_slow_subscriber(&mut self) {
    self.slow_subscriber = None;
  }

  pub fn set_software_date(&mut self, software_date: String) {
    self.software_date = Some(software_date);
  }

  pub fn with_software_date(mut self, software_date: String) -> MsgVpnClient {
    self.software_date = Some(software_date);
    self
  }

  pub fn software_date(&self) -> Option<&String> {
    self.software_date.as_ref()
  }

  pub fn reset_software_date(&mut self) {
    self.software_date = None;
  }

  pub fn set_software_version(&mut self, software_version: String) {
    self.software_version = Some(software_version);
  }

  pub fn with_software_version(mut self, software_version: String) -> MsgVpnClient {
    self.software_version = Some(software_version);
    self
  }

  pub fn software_version(&self) -> Option<&String> {
    self.software_version.as_ref()
  }

  pub fn reset_software_version(&mut self) {
    self.software_version = None;
  }

  pub fn set_tls_cipher_description(&mut self, tls_cipher_description: String) {
    self.tls_cipher_description = Some(tls_cipher_description);
  }

  pub fn with_tls_cipher_description(mut self, tls_cipher_description: String) -> MsgVpnClient {
    self.tls_cipher_description = Some(tls_cipher_description);
    self
  }

  pub fn tls_cipher_description(&self) -> Option<&String> {
    self.tls_cipher_description.as_ref()
  }

  pub fn reset_tls_cipher_description(&mut self) {
    self.tls_cipher_description = None;
  }

  pub fn set_tls_downgraded_to_plain_text(&mut self, tls_downgraded_to_plain_text: bool) {
    self.tls_downgraded_to_plain_text = Some(tls_downgraded_to_plain_text);
  }

  pub fn with_tls_downgraded_to_plain_text(mut self, tls_downgraded_to_plain_text: bool) -> MsgVpnClient {
    self.tls_downgraded_to_plain_text = Some(tls_downgraded_to_plain_text);
    self
  }

  pub fn tls_downgraded_to_plain_text(&self) -> Option<&bool> {
    self.tls_downgraded_to_plain_text.as_ref()
  }

  pub fn reset_tls_downgraded_to_plain_text(&mut self) {
    self.tls_downgraded_to_plain_text = None;
  }

  pub fn set_tls_version(&mut self, tls_version: String) {
    self.tls_version = Some(tls_version);
  }

  pub fn with_tls_version(mut self, tls_version: String) -> MsgVpnClient {
    self.tls_version = Some(tls_version);
    self
  }

  pub fn tls_version(&self) -> Option<&String> {
    self.tls_version.as_ref()
  }

  pub fn reset_tls_version(&mut self) {
    self.tls_version = None;
  }

  pub fn set_topic_parse_error_rx_discarded_msg_count(&mut self, topic_parse_error_rx_discarded_msg_count: i64) {
    self.topic_parse_error_rx_discarded_msg_count = Some(topic_parse_error_rx_discarded_msg_count);
  }

  pub fn with_topic_parse_error_rx_discarded_msg_count(mut self, topic_parse_error_rx_discarded_msg_count: i64) -> MsgVpnClient {
    self.topic_parse_error_rx_discarded_msg_count = Some(topic_parse_error_rx_discarded_msg_count);
    self
  }

  pub fn topic_parse_error_rx_discarded_msg_count(&self) -> Option<&i64> {
    self.topic_parse_error_rx_discarded_msg_count.as_ref()
  }

  pub fn reset_topic_parse_error_rx_discarded_msg_count(&mut self) {
    self.topic_parse_error_rx_discarded_msg_count = None;
  }

  pub fn set_tx_byte_count(&mut self, tx_byte_count: i64) {
    self.tx_byte_count = Some(tx_byte_count);
  }

  pub fn with_tx_byte_count(mut self, tx_byte_count: i64) -> MsgVpnClient {
    self.tx_byte_count = Some(tx_byte_count);
    self
  }

  pub fn tx_byte_count(&self) -> Option<&i64> {
    self.tx_byte_count.as_ref()
  }

  pub fn reset_tx_byte_count(&mut self) {
    self.tx_byte_count = None;
  }

  pub fn set_tx_byte_rate(&mut self, tx_byte_rate: i64) {
    self.tx_byte_rate = Some(tx_byte_rate);
  }

  pub fn with_tx_byte_rate(mut self, tx_byte_rate: i64) -> MsgVpnClient {
    self.tx_byte_rate = Some(tx_byte_rate);
    self
  }

  pub fn tx_byte_rate(&self) -> Option<&i64> {
    self.tx_byte_rate.as_ref()
  }

  pub fn reset_tx_byte_rate(&mut self) {
    self.tx_byte_rate = None;
  }

  pub fn set_tx_discarded_msg_count(&mut self, tx_discarded_msg_count: i64) {
    self.tx_discarded_msg_count = Some(tx_discarded_msg_count);
  }

  pub fn with_tx_discarded_msg_count(mut self, tx_discarded_msg_count: i64) -> MsgVpnClient {
    self.tx_discarded_msg_count = Some(tx_discarded_msg_count);
    self
  }

  pub fn tx_discarded_msg_count(&self) -> Option<&i64> {
    self.tx_discarded_msg_count.as_ref()
  }

  pub fn reset_tx_discarded_msg_count(&mut self) {
    self.tx_discarded_msg_count = None;
  }

  pub fn set_tx_msg_count(&mut self, tx_msg_count: i64) {
    self.tx_msg_count = Some(tx_msg_count);
  }

  pub fn with_tx_msg_count(mut self, tx_msg_count: i64) -> MsgVpnClient {
    self.tx_msg_count = Some(tx_msg_count);
    self
  }

  pub fn tx_msg_count(&self) -> Option<&i64> {
    self.tx_msg_count.as_ref()
  }

  pub fn reset_tx_msg_count(&mut self) {
    self.tx_msg_count = None;
  }

  pub fn set_tx_msg_rate(&mut self, tx_msg_rate: i64) {
    self.tx_msg_rate = Some(tx_msg_rate);
  }

  pub fn with_tx_msg_rate(mut self, tx_msg_rate: i64) -> MsgVpnClient {
    self.tx_msg_rate = Some(tx_msg_rate);
    self
  }

  pub fn tx_msg_rate(&self) -> Option<&i64> {
    self.tx_msg_rate.as_ref()
  }

  pub fn reset_tx_msg_rate(&mut self) {
    self.tx_msg_rate = None;
  }

  pub fn set_uptime(&mut self, uptime: i32) {
    self.uptime = Some(uptime);
  }

  pub fn with_uptime(mut self, uptime: i32) -> MsgVpnClient {
    self.uptime = Some(uptime);
    self
  }

  pub fn uptime(&self) -> Option<&i32> {
    self.uptime.as_ref()
  }

  pub fn reset_uptime(&mut self) {
    self.uptime = None;
  }

  pub fn set_user(&mut self, user: String) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: String) -> MsgVpnClient {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&String> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

  pub fn set_virtual_router(&mut self, virtual_router: String) {
    self.virtual_router = Some(virtual_router);
  }

  pub fn with_virtual_router(mut self, virtual_router: String) -> MsgVpnClient {
    self.virtual_router = Some(virtual_router);
    self
  }

  pub fn virtual_router(&self) -> Option<&String> {
    self.virtual_router.as_ref()
  }

  pub fn reset_virtual_router(&mut self) {
    self.virtual_router = None;
  }

  pub fn set_web_inactive_timeout(&mut self, web_inactive_timeout: i32) {
    self.web_inactive_timeout = Some(web_inactive_timeout);
  }

  pub fn with_web_inactive_timeout(mut self, web_inactive_timeout: i32) -> MsgVpnClient {
    self.web_inactive_timeout = Some(web_inactive_timeout);
    self
  }

  pub fn web_inactive_timeout(&self) -> Option<&i32> {
    self.web_inactive_timeout.as_ref()
  }

  pub fn reset_web_inactive_timeout(&mut self) {
    self.web_inactive_timeout = None;
  }

  pub fn set_web_max_payload(&mut self, web_max_payload: i64) {
    self.web_max_payload = Some(web_max_payload);
  }

  pub fn with_web_max_payload(mut self, web_max_payload: i64) -> MsgVpnClient {
    self.web_max_payload = Some(web_max_payload);
    self
  }

  pub fn web_max_payload(&self) -> Option<&i64> {
    self.web_max_payload.as_ref()
  }

  pub fn reset_web_max_payload(&mut self) {
    self.web_max_payload = None;
  }

  pub fn set_web_parse_error_rx_discarded_msg_count(&mut self, web_parse_error_rx_discarded_msg_count: i64) {
    self.web_parse_error_rx_discarded_msg_count = Some(web_parse_error_rx_discarded_msg_count);
  }

  pub fn with_web_parse_error_rx_discarded_msg_count(mut self, web_parse_error_rx_discarded_msg_count: i64) -> MsgVpnClient {
    self.web_parse_error_rx_discarded_msg_count = Some(web_parse_error_rx_discarded_msg_count);
    self
  }

  pub fn web_parse_error_rx_discarded_msg_count(&self) -> Option<&i64> {
    self.web_parse_error_rx_discarded_msg_count.as_ref()
  }

  pub fn reset_web_parse_error_rx_discarded_msg_count(&mut self) {
    self.web_parse_error_rx_discarded_msg_count = None;
  }

  pub fn set_web_remaining_timeout(&mut self, web_remaining_timeout: i32) {
    self.web_remaining_timeout = Some(web_remaining_timeout);
  }

  pub fn with_web_remaining_timeout(mut self, web_remaining_timeout: i32) -> MsgVpnClient {
    self.web_remaining_timeout = Some(web_remaining_timeout);
    self
  }

  pub fn web_remaining_timeout(&self) -> Option<&i32> {
    self.web_remaining_timeout.as_ref()
  }

  pub fn reset_web_remaining_timeout(&mut self) {
    self.web_remaining_timeout = None;
  }

  pub fn set_web_rx_byte_count(&mut self, web_rx_byte_count: i64) {
    self.web_rx_byte_count = Some(web_rx_byte_count);
  }

  pub fn with_web_rx_byte_count(mut self, web_rx_byte_count: i64) -> MsgVpnClient {
    self.web_rx_byte_count = Some(web_rx_byte_count);
    self
  }

  pub fn web_rx_byte_count(&self) -> Option<&i64> {
    self.web_rx_byte_count.as_ref()
  }

  pub fn reset_web_rx_byte_count(&mut self) {
    self.web_rx_byte_count = None;
  }

  pub fn set_web_rx_encoding(&mut self, web_rx_encoding: String) {
    self.web_rx_encoding = Some(web_rx_encoding);
  }

  pub fn with_web_rx_encoding(mut self, web_rx_encoding: String) -> MsgVpnClient {
    self.web_rx_encoding = Some(web_rx_encoding);
    self
  }

  pub fn web_rx_encoding(&self) -> Option<&String> {
    self.web_rx_encoding.as_ref()
  }

  pub fn reset_web_rx_encoding(&mut self) {
    self.web_rx_encoding = None;
  }

  pub fn set_web_rx_msg_count(&mut self, web_rx_msg_count: i64) {
    self.web_rx_msg_count = Some(web_rx_msg_count);
  }

  pub fn with_web_rx_msg_count(mut self, web_rx_msg_count: i64) -> MsgVpnClient {
    self.web_rx_msg_count = Some(web_rx_msg_count);
    self
  }

  pub fn web_rx_msg_count(&self) -> Option<&i64> {
    self.web_rx_msg_count.as_ref()
  }

  pub fn reset_web_rx_msg_count(&mut self) {
    self.web_rx_msg_count = None;
  }

  pub fn set_web_rx_protocol(&mut self, web_rx_protocol: String) {
    self.web_rx_protocol = Some(web_rx_protocol);
  }

  pub fn with_web_rx_protocol(mut self, web_rx_protocol: String) -> MsgVpnClient {
    self.web_rx_protocol = Some(web_rx_protocol);
    self
  }

  pub fn web_rx_protocol(&self) -> Option<&String> {
    self.web_rx_protocol.as_ref()
  }

  pub fn reset_web_rx_protocol(&mut self) {
    self.web_rx_protocol = None;
  }

  pub fn set_web_rx_request_count(&mut self, web_rx_request_count: i64) {
    self.web_rx_request_count = Some(web_rx_request_count);
  }

  pub fn with_web_rx_request_count(mut self, web_rx_request_count: i64) -> MsgVpnClient {
    self.web_rx_request_count = Some(web_rx_request_count);
    self
  }

  pub fn web_rx_request_count(&self) -> Option<&i64> {
    self.web_rx_request_count.as_ref()
  }

  pub fn reset_web_rx_request_count(&mut self) {
    self.web_rx_request_count = None;
  }

  pub fn set_web_rx_response_count(&mut self, web_rx_response_count: i64) {
    self.web_rx_response_count = Some(web_rx_response_count);
  }

  pub fn with_web_rx_response_count(mut self, web_rx_response_count: i64) -> MsgVpnClient {
    self.web_rx_response_count = Some(web_rx_response_count);
    self
  }

  pub fn web_rx_response_count(&self) -> Option<&i64> {
    self.web_rx_response_count.as_ref()
  }

  pub fn reset_web_rx_response_count(&mut self) {
    self.web_rx_response_count = None;
  }

  pub fn set_web_rx_tcp_state(&mut self, web_rx_tcp_state: String) {
    self.web_rx_tcp_state = Some(web_rx_tcp_state);
  }

  pub fn with_web_rx_tcp_state(mut self, web_rx_tcp_state: String) -> MsgVpnClient {
    self.web_rx_tcp_state = Some(web_rx_tcp_state);
    self
  }

  pub fn web_rx_tcp_state(&self) -> Option<&String> {
    self.web_rx_tcp_state.as_ref()
  }

  pub fn reset_web_rx_tcp_state(&mut self) {
    self.web_rx_tcp_state = None;
  }

  pub fn set_web_rx_tls_cipher_description(&mut self, web_rx_tls_cipher_description: String) {
    self.web_rx_tls_cipher_description = Some(web_rx_tls_cipher_description);
  }

  pub fn with_web_rx_tls_cipher_description(mut self, web_rx_tls_cipher_description: String) -> MsgVpnClient {
    self.web_rx_tls_cipher_description = Some(web_rx_tls_cipher_description);
    self
  }

  pub fn web_rx_tls_cipher_description(&self) -> Option<&String> {
    self.web_rx_tls_cipher_description.as_ref()
  }

  pub fn reset_web_rx_tls_cipher_description(&mut self) {
    self.web_rx_tls_cipher_description = None;
  }

  pub fn set_web_rx_tls_version(&mut self, web_rx_tls_version: String) {
    self.web_rx_tls_version = Some(web_rx_tls_version);
  }

  pub fn with_web_rx_tls_version(mut self, web_rx_tls_version: String) -> MsgVpnClient {
    self.web_rx_tls_version = Some(web_rx_tls_version);
    self
  }

  pub fn web_rx_tls_version(&self) -> Option<&String> {
    self.web_rx_tls_version.as_ref()
  }

  pub fn reset_web_rx_tls_version(&mut self) {
    self.web_rx_tls_version = None;
  }

  pub fn set_web_session_id(&mut self, web_session_id: String) {
    self.web_session_id = Some(web_session_id);
  }

  pub fn with_web_session_id(mut self, web_session_id: String) -> MsgVpnClient {
    self.web_session_id = Some(web_session_id);
    self
  }

  pub fn web_session_id(&self) -> Option<&String> {
    self.web_session_id.as_ref()
  }

  pub fn reset_web_session_id(&mut self) {
    self.web_session_id = None;
  }

  pub fn set_web_tx_byte_count(&mut self, web_tx_byte_count: i64) {
    self.web_tx_byte_count = Some(web_tx_byte_count);
  }

  pub fn with_web_tx_byte_count(mut self, web_tx_byte_count: i64) -> MsgVpnClient {
    self.web_tx_byte_count = Some(web_tx_byte_count);
    self
  }

  pub fn web_tx_byte_count(&self) -> Option<&i64> {
    self.web_tx_byte_count.as_ref()
  }

  pub fn reset_web_tx_byte_count(&mut self) {
    self.web_tx_byte_count = None;
  }

  pub fn set_web_tx_encoding(&mut self, web_tx_encoding: String) {
    self.web_tx_encoding = Some(web_tx_encoding);
  }

  pub fn with_web_tx_encoding(mut self, web_tx_encoding: String) -> MsgVpnClient {
    self.web_tx_encoding = Some(web_tx_encoding);
    self
  }

  pub fn web_tx_encoding(&self) -> Option<&String> {
    self.web_tx_encoding.as_ref()
  }

  pub fn reset_web_tx_encoding(&mut self) {
    self.web_tx_encoding = None;
  }

  pub fn set_web_tx_msg_count(&mut self, web_tx_msg_count: i64) {
    self.web_tx_msg_count = Some(web_tx_msg_count);
  }

  pub fn with_web_tx_msg_count(mut self, web_tx_msg_count: i64) -> MsgVpnClient {
    self.web_tx_msg_count = Some(web_tx_msg_count);
    self
  }

  pub fn web_tx_msg_count(&self) -> Option<&i64> {
    self.web_tx_msg_count.as_ref()
  }

  pub fn reset_web_tx_msg_count(&mut self) {
    self.web_tx_msg_count = None;
  }

  pub fn set_web_tx_protocol(&mut self, web_tx_protocol: String) {
    self.web_tx_protocol = Some(web_tx_protocol);
  }

  pub fn with_web_tx_protocol(mut self, web_tx_protocol: String) -> MsgVpnClient {
    self.web_tx_protocol = Some(web_tx_protocol);
    self
  }

  pub fn web_tx_protocol(&self) -> Option<&String> {
    self.web_tx_protocol.as_ref()
  }

  pub fn reset_web_tx_protocol(&mut self) {
    self.web_tx_protocol = None;
  }

  pub fn set_web_tx_request_count(&mut self, web_tx_request_count: i64) {
    self.web_tx_request_count = Some(web_tx_request_count);
  }

  pub fn with_web_tx_request_count(mut self, web_tx_request_count: i64) -> MsgVpnClient {
    self.web_tx_request_count = Some(web_tx_request_count);
    self
  }

  pub fn web_tx_request_count(&self) -> Option<&i64> {
    self.web_tx_request_count.as_ref()
  }

  pub fn reset_web_tx_request_count(&mut self) {
    self.web_tx_request_count = None;
  }

  pub fn set_web_tx_response_count(&mut self, web_tx_response_count: i64) {
    self.web_tx_response_count = Some(web_tx_response_count);
  }

  pub fn with_web_tx_response_count(mut self, web_tx_response_count: i64) -> MsgVpnClient {
    self.web_tx_response_count = Some(web_tx_response_count);
    self
  }

  pub fn web_tx_response_count(&self) -> Option<&i64> {
    self.web_tx_response_count.as_ref()
  }

  pub fn reset_web_tx_response_count(&mut self) {
    self.web_tx_response_count = None;
  }

  pub fn set_web_tx_tcp_state(&mut self, web_tx_tcp_state: String) {
    self.web_tx_tcp_state = Some(web_tx_tcp_state);
  }

  pub fn with_web_tx_tcp_state(mut self, web_tx_tcp_state: String) -> MsgVpnClient {
    self.web_tx_tcp_state = Some(web_tx_tcp_state);
    self
  }

  pub fn web_tx_tcp_state(&self) -> Option<&String> {
    self.web_tx_tcp_state.as_ref()
  }

  pub fn reset_web_tx_tcp_state(&mut self) {
    self.web_tx_tcp_state = None;
  }

  pub fn set_web_tx_tls_cipher_description(&mut self, web_tx_tls_cipher_description: String) {
    self.web_tx_tls_cipher_description = Some(web_tx_tls_cipher_description);
  }

  pub fn with_web_tx_tls_cipher_description(mut self, web_tx_tls_cipher_description: String) -> MsgVpnClient {
    self.web_tx_tls_cipher_description = Some(web_tx_tls_cipher_description);
    self
  }

  pub fn web_tx_tls_cipher_description(&self) -> Option<&String> {
    self.web_tx_tls_cipher_description.as_ref()
  }

  pub fn reset_web_tx_tls_cipher_description(&mut self) {
    self.web_tx_tls_cipher_description = None;
  }

  pub fn set_web_tx_tls_version(&mut self, web_tx_tls_version: String) {
    self.web_tx_tls_version = Some(web_tx_tls_version);
  }

  pub fn with_web_tx_tls_version(mut self, web_tx_tls_version: String) -> MsgVpnClient {
    self.web_tx_tls_version = Some(web_tx_tls_version);
    self
  }

  pub fn web_tx_tls_version(&self) -> Option<&String> {
    self.web_tx_tls_version.as_ref()
  }

  pub fn reset_web_tx_tls_version(&mut self) {
    self.web_tx_tls_version = None;
  }

}



