/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters \"q1\" and \"q2\" ; with values \"val1\" and \"val2\" respectively /SEMP/v2/monitor/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/monitor/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/monitor/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/monitor/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/monitor/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/monitor/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/monitor/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/monitor/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/monitor/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.    
 *
 * OpenAPI spec version: 2.12.00902000014
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnRestDeliveryPointQueueBinding {
  /// Indicates whether the authority for the request-target is replaced with that configured for the REST Consumer remote.
  #[serde(rename = "gatewayReplaceTargetAuthorityEnabled", skip_serializing_if="Option::is_none")]
  gateway_replace_target_authority_enabled: Option<bool>,
  /// The reason for the last REST Delivery Point queue binding failure.
  #[serde(rename = "lastFailureReason", skip_serializing_if="Option::is_none")]
  last_failure_reason: Option<String>,
  /// The timestamp of the last REST Delivery Point queue binding failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "lastFailureTime", skip_serializing_if="Option::is_none")]
  last_failure_time: Option<i32>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The POST request-target string being used when sending POST requests to a REST Consumer.
  #[serde(rename = "postRequestTarget", skip_serializing_if="Option::is_none")]
  post_request_target: Option<String>,
  /// The name of a queue in the Message VPN.
  #[serde(rename = "queueBindingName", skip_serializing_if="Option::is_none")]
  queue_binding_name: Option<String>,
  /// The name of the REST Delivery Point.
  #[serde(rename = "restDeliveryPointName", skip_serializing_if="Option::is_none")]
  rest_delivery_point_name: Option<String>,
  /// Indicates whether the operational state of the REST Delivery Point queue binding is up.
  #[serde(rename = "up", skip_serializing_if="Option::is_none")]
  up: Option<bool>,
  /// The amount of time in seconds since the REST Delivery Point queue binding was up.
  #[serde(rename = "uptime", skip_serializing_if="Option::is_none")]
  uptime: Option<i64>
}

impl MsgVpnRestDeliveryPointQueueBinding {
  pub fn new() -> MsgVpnRestDeliveryPointQueueBinding {
    MsgVpnRestDeliveryPointQueueBinding {
      gateway_replace_target_authority_enabled: None,
      last_failure_reason: None,
      last_failure_time: None,
      msg_vpn_name: None,
      post_request_target: None,
      queue_binding_name: None,
      rest_delivery_point_name: None,
      up: None,
      uptime: None
    }
  }

  pub fn set_gateway_replace_target_authority_enabled(&mut self, gateway_replace_target_authority_enabled: bool) {
    self.gateway_replace_target_authority_enabled = Some(gateway_replace_target_authority_enabled);
  }

  pub fn with_gateway_replace_target_authority_enabled(mut self, gateway_replace_target_authority_enabled: bool) -> MsgVpnRestDeliveryPointQueueBinding {
    self.gateway_replace_target_authority_enabled = Some(gateway_replace_target_authority_enabled);
    self
  }

  pub fn gateway_replace_target_authority_enabled(&self) -> Option<&bool> {
    self.gateway_replace_target_authority_enabled.as_ref()
  }

  pub fn reset_gateway_replace_target_authority_enabled(&mut self) {
    self.gateway_replace_target_authority_enabled = None;
  }

  pub fn set_last_failure_reason(&mut self, last_failure_reason: String) {
    self.last_failure_reason = Some(last_failure_reason);
  }

  pub fn with_last_failure_reason(mut self, last_failure_reason: String) -> MsgVpnRestDeliveryPointQueueBinding {
    self.last_failure_reason = Some(last_failure_reason);
    self
  }

  pub fn last_failure_reason(&self) -> Option<&String> {
    self.last_failure_reason.as_ref()
  }

  pub fn reset_last_failure_reason(&mut self) {
    self.last_failure_reason = None;
  }

  pub fn set_last_failure_time(&mut self, last_failure_time: i32) {
    self.last_failure_time = Some(last_failure_time);
  }

  pub fn with_last_failure_time(mut self, last_failure_time: i32) -> MsgVpnRestDeliveryPointQueueBinding {
    self.last_failure_time = Some(last_failure_time);
    self
  }

  pub fn last_failure_time(&self) -> Option<&i32> {
    self.last_failure_time.as_ref()
  }

  pub fn reset_last_failure_time(&mut self) {
    self.last_failure_time = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnRestDeliveryPointQueueBinding {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_post_request_target(&mut self, post_request_target: String) {
    self.post_request_target = Some(post_request_target);
  }

  pub fn with_post_request_target(mut self, post_request_target: String) -> MsgVpnRestDeliveryPointQueueBinding {
    self.post_request_target = Some(post_request_target);
    self
  }

  pub fn post_request_target(&self) -> Option<&String> {
    self.post_request_target.as_ref()
  }

  pub fn reset_post_request_target(&mut self) {
    self.post_request_target = None;
  }

  pub fn set_queue_binding_name(&mut self, queue_binding_name: String) {
    self.queue_binding_name = Some(queue_binding_name);
  }

  pub fn with_queue_binding_name(mut self, queue_binding_name: String) -> MsgVpnRestDeliveryPointQueueBinding {
    self.queue_binding_name = Some(queue_binding_name);
    self
  }

  pub fn queue_binding_name(&self) -> Option<&String> {
    self.queue_binding_name.as_ref()
  }

  pub fn reset_queue_binding_name(&mut self) {
    self.queue_binding_name = None;
  }

  pub fn set_rest_delivery_point_name(&mut self, rest_delivery_point_name: String) {
    self.rest_delivery_point_name = Some(rest_delivery_point_name);
  }

  pub fn with_rest_delivery_point_name(mut self, rest_delivery_point_name: String) -> MsgVpnRestDeliveryPointQueueBinding {
    self.rest_delivery_point_name = Some(rest_delivery_point_name);
    self
  }

  pub fn rest_delivery_point_name(&self) -> Option<&String> {
    self.rest_delivery_point_name.as_ref()
  }

  pub fn reset_rest_delivery_point_name(&mut self) {
    self.rest_delivery_point_name = None;
  }

  pub fn set_up(&mut self, up: bool) {
    self.up = Some(up);
  }

  pub fn with_up(mut self, up: bool) -> MsgVpnRestDeliveryPointQueueBinding {
    self.up = Some(up);
    self
  }

  pub fn up(&self) -> Option<&bool> {
    self.up.as_ref()
  }

  pub fn reset_up(&mut self) {
    self.up = None;
  }

  pub fn set_uptime(&mut self, uptime: i64) {
    self.uptime = Some(uptime);
  }

  pub fn with_uptime(mut self, uptime: i64) -> MsgVpnRestDeliveryPointQueueBinding {
    self.uptime = Some(uptime);
    self
  }

  pub fn uptime(&self) -> Option<&i64> {
    self.uptime.as_ref()
  }

  pub fn reset_uptime(&mut self) {
    self.uptime = None;
  }

}



