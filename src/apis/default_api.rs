/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters \"q1\" and \"q2\" ; with values \"val1\" and \"val2\" respectively /SEMP/v2/monitor/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/monitor/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/monitor/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/monitor/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/monitor/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/monitor/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/monitor/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/monitor/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/monitor/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.    
 *
 * OpenAPI spec version: 2.12.00902000014
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;
use std::unimplemented;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct DefaultApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DefaultApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DefaultApiClient<C> {
        DefaultApiClient {
            configuration: configuration,
        }
    }
}

pub trait DefaultApi {
    fn get_about_user_msg_vpn(&self, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::AboutUserMsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn get_about_user_msg_vpns(&self, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::AboutUserMsgVpnsResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster(&self, dmr_cluster_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_link(&self, dmr_cluster_name: &str, remote_node_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_link_channel(&self, dmr_cluster_name: &str, remote_node_name: &str, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkChannelResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_link_channels(&self, dmr_cluster_name: &str, remote_node_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkChannelsResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_link_remote_address(&self, dmr_cluster_name: &str, remote_node_name: &str, remote_address: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkRemoteAddressResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_link_remote_addresses(&self, dmr_cluster_name: &str, remote_node_name: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkRemoteAddressesResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_link_tls_trusted_common_name(&self, dmr_cluster_name: &str, remote_node_name: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_link_tls_trusted_common_names(&self, dmr_cluster_name: &str, remote_node_name: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_links(&self, dmr_cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinksResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_topology_issue(&self, dmr_cluster_name: &str, topology_issue: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterTopologyIssueResponse, Error = Error<serde_json::Value>>>;
    fn get_dmr_cluster_topology_issues(&self, dmr_cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterTopologyIssuesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn(&self, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_client_connect_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, client_connect_exception_address: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_client_connect_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_publish_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, publish_exception_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_publish_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_subscribe_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, subscribe_exception_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_subscribe_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_local_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, local_subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeLocalSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_local_subscriptions(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeLocalSubscriptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_remote_msg_vpns(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_remote_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_remote_subscriptions(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_tls_trusted_common_name(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_tls_trusted_common_names(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client(&self, msg_vpn_name: &str, client_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_connection(&self, msg_vpn_name: &str, client_name: &str, client_address: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientConnectionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_connections(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientConnectionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_rx_flow(&self, msg_vpn_name: &str, client_name: &str, flow_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientRxFlowResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_rx_flows(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientRxFlowsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_subscription(&self, msg_vpn_name: &str, client_name: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_subscriptions(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientSubscriptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_transacted_session(&self, msg_vpn_name: &str, client_name: &str, session_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientTransactedSessionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_transacted_sessions(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientTransactedSessionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_tx_flow(&self, msg_vpn_name: &str, client_name: &str, flow_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientTxFlowResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_tx_flows(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientTxFlowsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_config_sync_remote_node(&self, msg_vpn_name: &str, remote_node_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnConfigSyncRemoteNodeResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, topic_prefix: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, home_cluster_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClusterResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_clusters(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClustersResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_instance_remote_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_instance_remote_topics(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_instances(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstancesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_topics(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_clusters(&self, msg_vpn_name: &str, cache_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClustersResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_dmr_bridge(&self, msg_vpn_name: &str, remote_node_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDmrBridgeResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_mqtt_retain_cache(&self, msg_vpn_name: &str, cache_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttRetainCacheResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_mqtt_session_subscriptions(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_msg(&self, msg_vpn_name: &str, queue_name: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueMsgResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_msgs(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueMsgsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_priorities(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueuePrioritiesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_priority(&self, msg_vpn_name: &str, queue_name: &str, priority: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueuePriorityResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_subscription(&self, msg_vpn_name: &str, queue_name: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_subscriptions(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_tx_flow(&self, msg_vpn_name: &str, queue_name: &str, flow_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueTxFlowResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_tx_flows(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueTxFlowsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_replay_log_msg(&self, msg_vpn_name: &str, replay_log_name: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogMsgResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_replay_log_msgs(&self, msg_vpn_name: &str, replay_log_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogMsgsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_queue_bindings(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_rest_consumers(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumersResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_topic_endpoint_msg(&self, msg_vpn_name: &str, topic_endpoint_name: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointMsgResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_topic_endpoint_msgs(&self, msg_vpn_name: &str, topic_endpoint_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointMsgsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_topic_endpoint_priorities(&self, msg_vpn_name: &str, topic_endpoint_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointPrioritiesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_topic_endpoint_priority(&self, msg_vpn_name: &str, topic_endpoint_name: &str, priority: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointPriorityResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_topic_endpoint_tx_flow(&self, msg_vpn_name: &str, topic_endpoint_name: &str, flow_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointTxFlowResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_topic_endpoint_tx_flows(&self, msg_vpn_name: &str, topic_endpoint_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointTxFlowsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_transaction(&self, msg_vpn_name: &str, xid: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_transaction_consumer_msg(&self, msg_vpn_name: &str, xid: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionConsumerMsgResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_transaction_consumer_msgs(&self, msg_vpn_name: &str, xid: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionConsumerMsgsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_transaction_publisher_msg(&self, msg_vpn_name: &str, xid: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionPublisherMsgResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_transaction_publisher_msgs(&self, msg_vpn_name: &str, xid: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionPublisherMsgsResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>DefaultApi for DefaultApiClient<C> {
    fn get_about_user_msg_vpn(&self, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::AboutUserMsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/about/user/msgVpns/{msgVpnName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::AboutUserMsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_about_user_msg_vpns(&self, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::AboutUserMsgVpnsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/about/user/msgVpns?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::AboutUserMsgVpnsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster(&self, dmr_cluster_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_link(&self, dmr_cluster_name: &str, remote_node_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/links/{remoteNodeName}?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name, remoteNodeName=remote_node_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterLinkResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_link_channel(&self, dmr_cluster_name: &str, remote_node_name: &str, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkChannelResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/links/{remoteNodeName}/channels/{msgVpnName}?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name, remoteNodeName=remote_node_name, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterLinkChannelResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_link_channels(&self, dmr_cluster_name: &str, remote_node_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkChannelsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/links/{remoteNodeName}/channels?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name, remoteNodeName=remote_node_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterLinkChannelsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_link_remote_address(&self, dmr_cluster_name: &str, remote_node_name: &str, remote_address: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkRemoteAddressResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses/{remoteAddress}?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name, remoteNodeName=remote_node_name, remoteAddress=remote_address);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterLinkRemoteAddressResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_link_remote_addresses(&self, dmr_cluster_name: &str, remote_node_name: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkRemoteAddressesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name, remoteNodeName=remote_node_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterLinkRemoteAddressesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_link_tls_trusted_common_name(&self, dmr_cluster_name: &str, remote_node_name: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames/{tlsTrustedCommonName}?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name, remoteNodeName=remote_node_name, tlsTrustedCommonName=tls_trusted_common_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterLinkTlsTrustedCommonNameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_link_tls_trusted_common_names(&self, dmr_cluster_name: &str, remote_node_name: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinkTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name, remoteNodeName=remote_node_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterLinkTlsTrustedCommonNamesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_links(&self, dmr_cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterLinksResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/links?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterLinksResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_topology_issue(&self, dmr_cluster_name: &str, topology_issue: &str, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterTopologyIssueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/topologyIssues/{topologyIssue}?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name, topologyIssue=topology_issue);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterTopologyIssueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_dmr_cluster_topology_issues(&self, dmr_cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::DmrClusterTopologyIssuesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/dmrClusters/{dmrClusterName}/topologyIssues?{}", configuration.base_path, query_string, dmrClusterName=dmr_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::DmrClusterTopologyIssuesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn(&self, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_client_connect_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, client_connect_exception_address: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, clientConnectExceptionAddress=client_connect_exception_address);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileClientConnectExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_client_connect_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileClientConnectExceptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_publish_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, publish_exception_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, topicSyntax=topic_syntax, publishExceptionTopic=publish_exception_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfilePublishExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_publish_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfilePublishExceptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_subscribe_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, subscribe_exception_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, topicSyntax=topic_syntax, subscribeExceptionTopic=subscribe_exception_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileSubscribeExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_subscribe_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileSubscribeExceptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, authorizationGroupName=authorization_group_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAuthorizationGroupResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_local_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, local_subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeLocalSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/localSubscriptions/{localSubscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, localSubscriptionTopic=local_subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeLocalSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_local_subscriptions(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeLocalSubscriptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/localSubscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeLocalSubscriptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, remoteMsgVpnName=remote_msg_vpn_name, remoteMsgVpnLocation=remote_msg_vpn_location, remoteMsgVpnInterface=remote_msg_vpn_interface);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteMsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_remote_msg_vpns(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteMsgVpnsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_remote_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, remoteSubscriptionTopic=remote_subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_remote_subscriptions(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteSubscriptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_tls_trusted_common_name(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, tlsTrustedCommonName=tls_trusted_common_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeTlsTrustedCommonNameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_tls_trusted_common_names(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeTlsTrustedCommonNamesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client(&self, msg_vpn_name: &str, client_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_connection(&self, msg_vpn_name: &str, client_name: &str, client_address: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientConnectionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/connections/{clientAddress}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name, clientAddress=client_address);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientConnectionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_connections(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientConnectionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/connections?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientConnectionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientProfiles/{clientProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientProfileName=client_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_rx_flow(&self, msg_vpn_name: &str, client_name: &str, flow_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientRxFlowResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/rxFlows/{flowId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name, flowId=flow_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientRxFlowResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_rx_flows(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientRxFlowsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/rxFlows?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientRxFlowsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_subscription(&self, msg_vpn_name: &str, client_name: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_subscriptions(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientSubscriptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/subscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientSubscriptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_transacted_session(&self, msg_vpn_name: &str, client_name: &str, session_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientTransactedSessionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions/{sessionName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name, sessionName=session_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientTransactedSessionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_transacted_sessions(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientTransactedSessionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientTransactedSessionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_tx_flow(&self, msg_vpn_name: &str, client_name: &str, flow_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientTxFlowResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/txFlows/{flowId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name, flowId=flow_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientTxFlowResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_tx_flows(&self, msg_vpn_name: &str, client_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientTxFlowsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clients/{clientName}/txFlows?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientName=client_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientTxFlowsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientUsernames/{clientUsername}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientUsername=client_username);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientUsernameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_config_sync_remote_node(&self, msg_vpn_name: &str, remote_node_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnConfigSyncRemoteNodeResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/configSyncRemoteNodes/{remoteNodeName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, remoteNodeName=remote_node_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnConfigSyncRemoteNodeResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, topic_prefix: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name, topicPrefix=topic_prefix);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, home_cluster_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/remoteGlobalCachingHomeClusters/{homeClusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name, homeClusterName=home_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_instance_remote_global_caching_home_clusters(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClustersResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/remoteGlobalCachingHomeClusters?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceRemoteGlobalCachingHomeClustersResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_instance_remote_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/remoteTopics/{topic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name, topic=topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_instance_remote_topics(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/remoteTopics?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceRemoteTopicsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_instances(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstancesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstancesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, topic=topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_topics(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterTopicsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_clusters(&self, msg_vpn_name: &str, cache_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClustersResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClustersResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_dmr_bridge(&self, msg_vpn_name: &str, remote_node_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDmrBridgeResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, remoteNodeName=remote_node_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDmrBridgeResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, connectionFactoryName=connection_factory_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiConnectionFactoryResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiQueues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiTopics/{topicName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicName=topic_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_mqtt_retain_cache(&self, msg_vpn_name: &str, cache_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttRetainCacheResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttRetainCacheResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_mqtt_session_subscriptions(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionSubscriptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_msg(&self, msg_vpn_name: &str, queue_name: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueMsgResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/msgs/{msgId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name, msgId=msg_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueMsgResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_msgs(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueMsgsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/msgs?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueMsgsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_priorities(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueuePrioritiesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/priorities?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueuePrioritiesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_priority(&self, msg_vpn_name: &str, queue_name: &str, priority: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueuePriorityResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/priorities/{priority}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name, priority=priority);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueuePriorityResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_subscription(&self, msg_vpn_name: &str, queue_name: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_subscriptions(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/subscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueSubscriptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_tx_flow(&self, msg_vpn_name: &str, queue_name: &str, flow_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueTxFlowResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/txFlows/{flowId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name, flowId=flow_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueTxFlowResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_tx_flows(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueTxFlowsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/txFlows?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueTxFlowsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replayLogs/{replayLogName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replayLogName=replay_log_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplayLogResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_replay_log_msg(&self, msg_vpn_name: &str, replay_log_name: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogMsgResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replayLogs/{replayLogName}/msgs/{msgId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replayLogName=replay_log_name, msgId=msg_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplayLogMsgResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_replay_log_msgs(&self, msg_vpn_name: &str, replay_log_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogMsgsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replayLogs/{replayLogName}/msgs?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replayLogName=replay_log_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplayLogMsgsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replicatedTopic=replicated_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplicatedTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, queueBindingName=queue_binding_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointQueueBindingResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_queue_bindings(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointQueueBindingsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name, tlsTrustedCommonName=tls_trusted_common_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_rest_consumers(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumersResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumersResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_topic_endpoint_msg(&self, msg_vpn_name: &str, topic_endpoint_name: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointMsgResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs/{msgId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name, msgId=msg_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointMsgResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_topic_endpoint_msgs(&self, msg_vpn_name: &str, topic_endpoint_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointMsgsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointMsgsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_topic_endpoint_priorities(&self, msg_vpn_name: &str, topic_endpoint_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointPrioritiesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/priorities?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointPrioritiesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_topic_endpoint_priority(&self, msg_vpn_name: &str, topic_endpoint_name: &str, priority: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointPriorityResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/priorities/{priority}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name, priority=priority);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointPriorityResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_topic_endpoint_tx_flow(&self, msg_vpn_name: &str, topic_endpoint_name: &str, flow_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointTxFlowResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/txFlows/{flowId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name, flowId=flow_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointTxFlowResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_topic_endpoint_tx_flows(&self, msg_vpn_name: &str, topic_endpoint_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointTxFlowsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/txFlows?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointTxFlowsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_transaction(&self, msg_vpn_name: &str, xid: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/transactions/{xid}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, xid=xid);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTransactionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_transaction_consumer_msg(&self, msg_vpn_name: &str, xid: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionConsumerMsgResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/transactions/{xid}/consumerMsgs/{msgId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, xid=xid, msgId=msg_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTransactionConsumerMsgResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_transaction_consumer_msgs(&self, msg_vpn_name: &str, xid: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionConsumerMsgsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/transactions/{xid}/consumerMsgs?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, xid=xid);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTransactionConsumerMsgsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_transaction_publisher_msg(&self, msg_vpn_name: &str, xid: &str, msg_id: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionPublisherMsgResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/transactions/{xid}/publisherMsgs/{msgId}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, xid=xid, msgId=msg_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTransactionPublisherMsgResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_transaction_publisher_msgs(&self, msg_vpn_name: &str, xid: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTransactionPublisherMsgsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/transactions/{xid}/publisherMsgs?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, xid=xid);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTransactionPublisherMsgsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
