/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/monitor/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/monitor/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/monitor/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/monitor/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/monitor/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/monitor/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/monitor/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/monitor/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/monitor/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/monitor/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.  
 *
 * OpenAPI spec version: 2.16
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// MsgVpnCounter : The counters for the Message VPN. Deprecated since 2.16. All attributes in this object have been moved to the MsgVpn object.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnCounter {
  /// The amount of client control messages received from clients by the Message VPN, in bytes (B). Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "controlRxByteCount", skip_serializing_if="Option::is_none")]
  control_rx_byte_count: Option<i64>,
  /// The number of client control messages received from clients by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "controlRxMsgCount", skip_serializing_if="Option::is_none")]
  control_rx_msg_count: Option<i64>,
  /// The amount of client control messages transmitted to clients by the Message VPN, in bytes (B). Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "controlTxByteCount", skip_serializing_if="Option::is_none")]
  control_tx_byte_count: Option<i64>,
  /// The number of client control messages transmitted to clients by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "controlTxMsgCount", skip_serializing_if="Option::is_none")]
  control_tx_msg_count: Option<i64>,
  /// The amount of client data messages received from clients by the Message VPN, in bytes (B). Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "dataRxByteCount", skip_serializing_if="Option::is_none")]
  data_rx_byte_count: Option<i64>,
  /// The number of client data messages received from clients by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "dataRxMsgCount", skip_serializing_if="Option::is_none")]
  data_rx_msg_count: Option<i64>,
  /// The amount of client data messages transmitted to clients by the Message VPN, in bytes (B). Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "dataTxByteCount", skip_serializing_if="Option::is_none")]
  data_tx_byte_count: Option<i64>,
  /// The number of client data messages transmitted to clients by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "dataTxMsgCount", skip_serializing_if="Option::is_none")]
  data_tx_msg_count: Option<i64>,
  /// The number of messages discarded during reception by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "discardedRxMsgCount", skip_serializing_if="Option::is_none")]
  discarded_rx_msg_count: Option<i32>,
  /// The number of messages discarded during transmission by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "discardedTxMsgCount", skip_serializing_if="Option::is_none")]
  discarded_tx_msg_count: Option<i32>,
  /// The number of login request messages received by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "loginRxMsgCount", skip_serializing_if="Option::is_none")]
  login_rx_msg_count: Option<i64>,
  /// The number of login response messages transmitted by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "loginTxMsgCount", skip_serializing_if="Option::is_none")]
  login_tx_msg_count: Option<i64>,
  /// The number of guaranteed messages received by the Message VPN. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "msgSpoolRxMsgCount", skip_serializing_if="Option::is_none")]
  msg_spool_rx_msg_count: Option<i64>,
  /// The number of guaranteed messages transmitted by the Message VPN. One message to multiple clients is counted as one message. Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "msgSpoolTxMsgCount", skip_serializing_if="Option::is_none")]
  msg_spool_tx_msg_count: Option<i64>,
  /// The amount of TLS messages received by the Message VPN, in bytes (B). Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "tlsRxByteCount", skip_serializing_if="Option::is_none")]
  tls_rx_byte_count: Option<i64>,
  /// The amount of TLS messages transmitted by the Message VPN, in bytes (B). Deprecated since 2.16. This attribute has been moved to the MsgVpn object.
  #[serde(rename = "tlsTxByteCount", skip_serializing_if="Option::is_none")]
  tls_tx_byte_count: Option<i64>
}

impl MsgVpnCounter {
  /// The counters for the Message VPN. Deprecated since 2.16. All attributes in this object have been moved to the MsgVpn object.
  pub fn new() -> MsgVpnCounter {
    MsgVpnCounter {
      control_rx_byte_count: None,
      control_rx_msg_count: None,
      control_tx_byte_count: None,
      control_tx_msg_count: None,
      data_rx_byte_count: None,
      data_rx_msg_count: None,
      data_tx_byte_count: None,
      data_tx_msg_count: None,
      discarded_rx_msg_count: None,
      discarded_tx_msg_count: None,
      login_rx_msg_count: None,
      login_tx_msg_count: None,
      msg_spool_rx_msg_count: None,
      msg_spool_tx_msg_count: None,
      tls_rx_byte_count: None,
      tls_tx_byte_count: None
    }
  }

  pub fn set_control_rx_byte_count(&mut self, control_rx_byte_count: i64) {
    self.control_rx_byte_count = Some(control_rx_byte_count);
  }

  pub fn with_control_rx_byte_count(mut self, control_rx_byte_count: i64) -> MsgVpnCounter {
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

  pub fn with_control_rx_msg_count(mut self, control_rx_msg_count: i64) -> MsgVpnCounter {
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

  pub fn with_control_tx_byte_count(mut self, control_tx_byte_count: i64) -> MsgVpnCounter {
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

  pub fn with_control_tx_msg_count(mut self, control_tx_msg_count: i64) -> MsgVpnCounter {
    self.control_tx_msg_count = Some(control_tx_msg_count);
    self
  }

  pub fn control_tx_msg_count(&self) -> Option<&i64> {
    self.control_tx_msg_count.as_ref()
  }

  pub fn reset_control_tx_msg_count(&mut self) {
    self.control_tx_msg_count = None;
  }

  pub fn set_data_rx_byte_count(&mut self, data_rx_byte_count: i64) {
    self.data_rx_byte_count = Some(data_rx_byte_count);
  }

  pub fn with_data_rx_byte_count(mut self, data_rx_byte_count: i64) -> MsgVpnCounter {
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

  pub fn with_data_rx_msg_count(mut self, data_rx_msg_count: i64) -> MsgVpnCounter {
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

  pub fn with_data_tx_byte_count(mut self, data_tx_byte_count: i64) -> MsgVpnCounter {
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

  pub fn with_data_tx_msg_count(mut self, data_tx_msg_count: i64) -> MsgVpnCounter {
    self.data_tx_msg_count = Some(data_tx_msg_count);
    self
  }

  pub fn data_tx_msg_count(&self) -> Option<&i64> {
    self.data_tx_msg_count.as_ref()
  }

  pub fn reset_data_tx_msg_count(&mut self) {
    self.data_tx_msg_count = None;
  }

  pub fn set_discarded_rx_msg_count(&mut self, discarded_rx_msg_count: i32) {
    self.discarded_rx_msg_count = Some(discarded_rx_msg_count);
  }

  pub fn with_discarded_rx_msg_count(mut self, discarded_rx_msg_count: i32) -> MsgVpnCounter {
    self.discarded_rx_msg_count = Some(discarded_rx_msg_count);
    self
  }

  pub fn discarded_rx_msg_count(&self) -> Option<&i32> {
    self.discarded_rx_msg_count.as_ref()
  }

  pub fn reset_discarded_rx_msg_count(&mut self) {
    self.discarded_rx_msg_count = None;
  }

  pub fn set_discarded_tx_msg_count(&mut self, discarded_tx_msg_count: i32) {
    self.discarded_tx_msg_count = Some(discarded_tx_msg_count);
  }

  pub fn with_discarded_tx_msg_count(mut self, discarded_tx_msg_count: i32) -> MsgVpnCounter {
    self.discarded_tx_msg_count = Some(discarded_tx_msg_count);
    self
  }

  pub fn discarded_tx_msg_count(&self) -> Option<&i32> {
    self.discarded_tx_msg_count.as_ref()
  }

  pub fn reset_discarded_tx_msg_count(&mut self) {
    self.discarded_tx_msg_count = None;
  }

  pub fn set_login_rx_msg_count(&mut self, login_rx_msg_count: i64) {
    self.login_rx_msg_count = Some(login_rx_msg_count);
  }

  pub fn with_login_rx_msg_count(mut self, login_rx_msg_count: i64) -> MsgVpnCounter {
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

  pub fn with_login_tx_msg_count(mut self, login_tx_msg_count: i64) -> MsgVpnCounter {
    self.login_tx_msg_count = Some(login_tx_msg_count);
    self
  }

  pub fn login_tx_msg_count(&self) -> Option<&i64> {
    self.login_tx_msg_count.as_ref()
  }

  pub fn reset_login_tx_msg_count(&mut self) {
    self.login_tx_msg_count = None;
  }

  pub fn set_msg_spool_rx_msg_count(&mut self, msg_spool_rx_msg_count: i64) {
    self.msg_spool_rx_msg_count = Some(msg_spool_rx_msg_count);
  }

  pub fn with_msg_spool_rx_msg_count(mut self, msg_spool_rx_msg_count: i64) -> MsgVpnCounter {
    self.msg_spool_rx_msg_count = Some(msg_spool_rx_msg_count);
    self
  }

  pub fn msg_spool_rx_msg_count(&self) -> Option<&i64> {
    self.msg_spool_rx_msg_count.as_ref()
  }

  pub fn reset_msg_spool_rx_msg_count(&mut self) {
    self.msg_spool_rx_msg_count = None;
  }

  pub fn set_msg_spool_tx_msg_count(&mut self, msg_spool_tx_msg_count: i64) {
    self.msg_spool_tx_msg_count = Some(msg_spool_tx_msg_count);
  }

  pub fn with_msg_spool_tx_msg_count(mut self, msg_spool_tx_msg_count: i64) -> MsgVpnCounter {
    self.msg_spool_tx_msg_count = Some(msg_spool_tx_msg_count);
    self
  }

  pub fn msg_spool_tx_msg_count(&self) -> Option<&i64> {
    self.msg_spool_tx_msg_count.as_ref()
  }

  pub fn reset_msg_spool_tx_msg_count(&mut self) {
    self.msg_spool_tx_msg_count = None;
  }

  pub fn set_tls_rx_byte_count(&mut self, tls_rx_byte_count: i64) {
    self.tls_rx_byte_count = Some(tls_rx_byte_count);
  }

  pub fn with_tls_rx_byte_count(mut self, tls_rx_byte_count: i64) -> MsgVpnCounter {
    self.tls_rx_byte_count = Some(tls_rx_byte_count);
    self
  }

  pub fn tls_rx_byte_count(&self) -> Option<&i64> {
    self.tls_rx_byte_count.as_ref()
  }

  pub fn reset_tls_rx_byte_count(&mut self) {
    self.tls_rx_byte_count = None;
  }

  pub fn set_tls_tx_byte_count(&mut self, tls_tx_byte_count: i64) {
    self.tls_tx_byte_count = Some(tls_tx_byte_count);
  }

  pub fn with_tls_tx_byte_count(mut self, tls_tx_byte_count: i64) -> MsgVpnCounter {
    self.tls_tx_byte_count = Some(tls_tx_byte_count);
    self
  }

  pub fn tls_tx_byte_count(&self) -> Option<&i64> {
    self.tls_tx_byte_count.as_ref()
  }

  pub fn reset_tls_tx_byte_count(&mut self) {
    self.tls_tx_byte_count = None;
  }

}



