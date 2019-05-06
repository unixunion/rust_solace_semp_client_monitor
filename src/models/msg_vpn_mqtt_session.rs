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
pub struct MsgVpnMqttSession {
  /// Indicates whether the Client requested a clean (newly created) MQTT Session when connecting. If not clean (already existing), then previously stored messages for QoS 1 subscriptions are delivered.
  #[serde(rename = "clean", skip_serializing_if="Option::is_none")]
  clean: Option<bool>,
  /// The name of the MQTT Session Client.
  #[serde(rename = "clientName", skip_serializing_if="Option::is_none")]
  client_name: Option<String>,
  #[serde(rename = "counter", skip_serializing_if="Option::is_none")]
  counter: Option<::models::MsgVpnMqttSessionCounter>,
  /// Indicates whether the MQTT Session was created by a Management API.
  #[serde(rename = "createdByManagement", skip_serializing_if="Option::is_none")]
  created_by_management: Option<bool>,
  /// Indicates whether the MQTT Session is enabled.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet.
  #[serde(rename = "mqttSessionClientId", skip_serializing_if="Option::is_none")]
  mqtt_session_client_id: Option<String>,
  /// The virtual router of the MQTT Session. The allowed values and their meaning are:  <pre> \"primary\" - The MQTT Session belongs to the primary virtual router. \"backup\" - The MQTT Session belongs to the backup virtual router. </pre> 
  #[serde(rename = "mqttSessionVirtualRouter", skip_serializing_if="Option::is_none")]
  mqtt_session_virtual_router: Option<String>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The Client Username which owns the MQTT Session.
  #[serde(rename = "owner", skip_serializing_if="Option::is_none")]
  owner: Option<String>,
  /// The name of the MQTT Session Queue.
  #[serde(rename = "queueName", skip_serializing_if="Option::is_none")]
  queue_name: Option<String>,
  /// Indicates whether the MQTT Session has the Will message specified by the Client. The Will message is published if the Client disconnects without sending the MQTT DISCONNECT packet.
  #[serde(rename = "will", skip_serializing_if="Option::is_none")]
  will: Option<bool>
}

impl MsgVpnMqttSession {
  pub fn new() -> MsgVpnMqttSession {
    MsgVpnMqttSession {
      clean: None,
      client_name: None,
      counter: None,
      created_by_management: None,
      enabled: None,
      mqtt_session_client_id: None,
      mqtt_session_virtual_router: None,
      msg_vpn_name: None,
      owner: None,
      queue_name: None,
      will: None
    }
  }

  pub fn set_clean(&mut self, clean: bool) {
    self.clean = Some(clean);
  }

  pub fn with_clean(mut self, clean: bool) -> MsgVpnMqttSession {
    self.clean = Some(clean);
    self
  }

  pub fn clean(&self) -> Option<&bool> {
    self.clean.as_ref()
  }

  pub fn reset_clean(&mut self) {
    self.clean = None;
  }

  pub fn set_client_name(&mut self, client_name: String) {
    self.client_name = Some(client_name);
  }

  pub fn with_client_name(mut self, client_name: String) -> MsgVpnMqttSession {
    self.client_name = Some(client_name);
    self
  }

  pub fn client_name(&self) -> Option<&String> {
    self.client_name.as_ref()
  }

  pub fn reset_client_name(&mut self) {
    self.client_name = None;
  }

  pub fn set_counter(&mut self, counter: ::models::MsgVpnMqttSessionCounter) {
    self.counter = Some(counter);
  }

  pub fn with_counter(mut self, counter: ::models::MsgVpnMqttSessionCounter) -> MsgVpnMqttSession {
    self.counter = Some(counter);
    self
  }

  pub fn counter(&self) -> Option<&::models::MsgVpnMqttSessionCounter> {
    self.counter.as_ref()
  }

  pub fn reset_counter(&mut self) {
    self.counter = None;
  }

  pub fn set_created_by_management(&mut self, created_by_management: bool) {
    self.created_by_management = Some(created_by_management);
  }

  pub fn with_created_by_management(mut self, created_by_management: bool) -> MsgVpnMqttSession {
    self.created_by_management = Some(created_by_management);
    self
  }

  pub fn created_by_management(&self) -> Option<&bool> {
    self.created_by_management.as_ref()
  }

  pub fn reset_created_by_management(&mut self) {
    self.created_by_management = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnMqttSession {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_mqtt_session_client_id(&mut self, mqtt_session_client_id: String) {
    self.mqtt_session_client_id = Some(mqtt_session_client_id);
  }

  pub fn with_mqtt_session_client_id(mut self, mqtt_session_client_id: String) -> MsgVpnMqttSession {
    self.mqtt_session_client_id = Some(mqtt_session_client_id);
    self
  }

  pub fn mqtt_session_client_id(&self) -> Option<&String> {
    self.mqtt_session_client_id.as_ref()
  }

  pub fn reset_mqtt_session_client_id(&mut self) {
    self.mqtt_session_client_id = None;
  }

  pub fn set_mqtt_session_virtual_router(&mut self, mqtt_session_virtual_router: String) {
    self.mqtt_session_virtual_router = Some(mqtt_session_virtual_router);
  }

  pub fn with_mqtt_session_virtual_router(mut self, mqtt_session_virtual_router: String) -> MsgVpnMqttSession {
    self.mqtt_session_virtual_router = Some(mqtt_session_virtual_router);
    self
  }

  pub fn mqtt_session_virtual_router(&self) -> Option<&String> {
    self.mqtt_session_virtual_router.as_ref()
  }

  pub fn reset_mqtt_session_virtual_router(&mut self) {
    self.mqtt_session_virtual_router = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnMqttSession {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_owner(&mut self, owner: String) {
    self.owner = Some(owner);
  }

  pub fn with_owner(mut self, owner: String) -> MsgVpnMqttSession {
    self.owner = Some(owner);
    self
  }

  pub fn owner(&self) -> Option<&String> {
    self.owner.as_ref()
  }

  pub fn reset_owner(&mut self) {
    self.owner = None;
  }

  pub fn set_queue_name(&mut self, queue_name: String) {
    self.queue_name = Some(queue_name);
  }

  pub fn with_queue_name(mut self, queue_name: String) -> MsgVpnMqttSession {
    self.queue_name = Some(queue_name);
    self
  }

  pub fn queue_name(&self) -> Option<&String> {
    self.queue_name.as_ref()
  }

  pub fn reset_queue_name(&mut self) {
    self.queue_name = None;
  }

  pub fn set_will(&mut self, will: bool) {
    self.will = Some(will);
  }

  pub fn with_will(mut self, will: bool) -> MsgVpnMqttSession {
    self.will = Some(will);
    self
  }

  pub fn will(&self) -> Option<&bool> {
    self.will.as_ref()
  }

  pub fn reset_will(&mut self) {
    self.will = None;
  }

}



