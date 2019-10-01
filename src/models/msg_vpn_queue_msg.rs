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
pub struct MsgVpnQueueMsg {
  /// The size of the Message attachment, in bytes (B).
  #[serde(rename = "attachmentSize", skip_serializing_if="Option::is_none")]
  attachment_size: Option<i64>,
  /// The size of the Message content, in bytes (B).
  #[serde(rename = "contentSize", skip_serializing_if="Option::is_none")]
  content_size: Option<i64>,
  /// Indicates whether the Message is eligible for the Dead Message Queue (DMQ).
  #[serde(rename = "dmqEligible", skip_serializing_if="Option::is_none")]
  dmq_eligible: Option<bool>,
  /// The timestamp of when the Message expires. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "expiryTime", skip_serializing_if="Option::is_none")]
  expiry_time: Option<i32>,
  /// The identifier (ID) of the Message.
  #[serde(rename = "msgId", skip_serializing_if="Option::is_none")]
  msg_id: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The priority level of the Message, from 9 (highest) to 0 (lowest).
  #[serde(rename = "priority", skip_serializing_if="Option::is_none")]
  priority: Option<i32>,
  /// The identifier (ID) of the Message publisher.
  #[serde(rename = "publisherId", skip_serializing_if="Option::is_none")]
  publisher_id: Option<i64>,
  /// The name of the Queue.
  #[serde(rename = "queueName", skip_serializing_if="Option::is_none")]
  queue_name: Option<String>,
  /// The number of times the Message has been redelivered.
  #[serde(rename = "redeliveryCount", skip_serializing_if="Option::is_none")]
  redelivery_count: Option<i32>,
  /// The Message identifier (ID) on the replication mate. Applicable only to replicated messages.
  #[serde(rename = "replicatedMateMsgId", skip_serializing_if="Option::is_none")]
  replicated_mate_msg_id: Option<i64>,
  /// The replication state of the Message. The allowed values and their meaning are:  <pre> \"replicated\" - The Message is replicated to the remote Message VPN. \"not-replicated\" - The Message is not being replicated to the remote Message VPN. \"pending-replication\" - The Message is queued for replication to the remote Message VPN. </pre> 
  #[serde(rename = "replicationState", skip_serializing_if="Option::is_none")]
  replication_state: Option<String>,
  /// The timestamp of when the Message was spooled in the Queue. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "spooledTime", skip_serializing_if="Option::is_none")]
  spooled_time: Option<i32>,
  /// Indicates whether delivery of the Message has never been attempted.
  #[serde(rename = "undelivered", skip_serializing_if="Option::is_none")]
  undelivered: Option<bool>
}

impl MsgVpnQueueMsg {
  pub fn new() -> MsgVpnQueueMsg {
    MsgVpnQueueMsg {
      attachment_size: None,
      content_size: None,
      dmq_eligible: None,
      expiry_time: None,
      msg_id: None,
      msg_vpn_name: None,
      priority: None,
      publisher_id: None,
      queue_name: None,
      redelivery_count: None,
      replicated_mate_msg_id: None,
      replication_state: None,
      spooled_time: None,
      undelivered: None
    }
  }

  pub fn set_attachment_size(&mut self, attachment_size: i64) {
    self.attachment_size = Some(attachment_size);
  }

  pub fn with_attachment_size(mut self, attachment_size: i64) -> MsgVpnQueueMsg {
    self.attachment_size = Some(attachment_size);
    self
  }

  pub fn attachment_size(&self) -> Option<&i64> {
    self.attachment_size.as_ref()
  }

  pub fn reset_attachment_size(&mut self) {
    self.attachment_size = None;
  }

  pub fn set_content_size(&mut self, content_size: i64) {
    self.content_size = Some(content_size);
  }

  pub fn with_content_size(mut self, content_size: i64) -> MsgVpnQueueMsg {
    self.content_size = Some(content_size);
    self
  }

  pub fn content_size(&self) -> Option<&i64> {
    self.content_size.as_ref()
  }

  pub fn reset_content_size(&mut self) {
    self.content_size = None;
  }

  pub fn set_dmq_eligible(&mut self, dmq_eligible: bool) {
    self.dmq_eligible = Some(dmq_eligible);
  }

  pub fn with_dmq_eligible(mut self, dmq_eligible: bool) -> MsgVpnQueueMsg {
    self.dmq_eligible = Some(dmq_eligible);
    self
  }

  pub fn dmq_eligible(&self) -> Option<&bool> {
    self.dmq_eligible.as_ref()
  }

  pub fn reset_dmq_eligible(&mut self) {
    self.dmq_eligible = None;
  }

  pub fn set_expiry_time(&mut self, expiry_time: i32) {
    self.expiry_time = Some(expiry_time);
  }

  pub fn with_expiry_time(mut self, expiry_time: i32) -> MsgVpnQueueMsg {
    self.expiry_time = Some(expiry_time);
    self
  }

  pub fn expiry_time(&self) -> Option<&i32> {
    self.expiry_time.as_ref()
  }

  pub fn reset_expiry_time(&mut self) {
    self.expiry_time = None;
  }

  pub fn set_msg_id(&mut self, msg_id: i64) {
    self.msg_id = Some(msg_id);
  }

  pub fn with_msg_id(mut self, msg_id: i64) -> MsgVpnQueueMsg {
    self.msg_id = Some(msg_id);
    self
  }

  pub fn msg_id(&self) -> Option<&i64> {
    self.msg_id.as_ref()
  }

  pub fn reset_msg_id(&mut self) {
    self.msg_id = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnQueueMsg {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_priority(&mut self, priority: i32) {
    self.priority = Some(priority);
  }

  pub fn with_priority(mut self, priority: i32) -> MsgVpnQueueMsg {
    self.priority = Some(priority);
    self
  }

  pub fn priority(&self) -> Option<&i32> {
    self.priority.as_ref()
  }

  pub fn reset_priority(&mut self) {
    self.priority = None;
  }

  pub fn set_publisher_id(&mut self, publisher_id: i64) {
    self.publisher_id = Some(publisher_id);
  }

  pub fn with_publisher_id(mut self, publisher_id: i64) -> MsgVpnQueueMsg {
    self.publisher_id = Some(publisher_id);
    self
  }

  pub fn publisher_id(&self) -> Option<&i64> {
    self.publisher_id.as_ref()
  }

  pub fn reset_publisher_id(&mut self) {
    self.publisher_id = None;
  }

  pub fn set_queue_name(&mut self, queue_name: String) {
    self.queue_name = Some(queue_name);
  }

  pub fn with_queue_name(mut self, queue_name: String) -> MsgVpnQueueMsg {
    self.queue_name = Some(queue_name);
    self
  }

  pub fn queue_name(&self) -> Option<&String> {
    self.queue_name.as_ref()
  }

  pub fn reset_queue_name(&mut self) {
    self.queue_name = None;
  }

  pub fn set_redelivery_count(&mut self, redelivery_count: i32) {
    self.redelivery_count = Some(redelivery_count);
  }

  pub fn with_redelivery_count(mut self, redelivery_count: i32) -> MsgVpnQueueMsg {
    self.redelivery_count = Some(redelivery_count);
    self
  }

  pub fn redelivery_count(&self) -> Option<&i32> {
    self.redelivery_count.as_ref()
  }

  pub fn reset_redelivery_count(&mut self) {
    self.redelivery_count = None;
  }

  pub fn set_replicated_mate_msg_id(&mut self, replicated_mate_msg_id: i64) {
    self.replicated_mate_msg_id = Some(replicated_mate_msg_id);
  }

  pub fn with_replicated_mate_msg_id(mut self, replicated_mate_msg_id: i64) -> MsgVpnQueueMsg {
    self.replicated_mate_msg_id = Some(replicated_mate_msg_id);
    self
  }

  pub fn replicated_mate_msg_id(&self) -> Option<&i64> {
    self.replicated_mate_msg_id.as_ref()
  }

  pub fn reset_replicated_mate_msg_id(&mut self) {
    self.replicated_mate_msg_id = None;
  }

  pub fn set_replication_state(&mut self, replication_state: String) {
    self.replication_state = Some(replication_state);
  }

  pub fn with_replication_state(mut self, replication_state: String) -> MsgVpnQueueMsg {
    self.replication_state = Some(replication_state);
    self
  }

  pub fn replication_state(&self) -> Option<&String> {
    self.replication_state.as_ref()
  }

  pub fn reset_replication_state(&mut self) {
    self.replication_state = None;
  }

  pub fn set_spooled_time(&mut self, spooled_time: i32) {
    self.spooled_time = Some(spooled_time);
  }

  pub fn with_spooled_time(mut self, spooled_time: i32) -> MsgVpnQueueMsg {
    self.spooled_time = Some(spooled_time);
    self
  }

  pub fn spooled_time(&self) -> Option<&i32> {
    self.spooled_time.as_ref()
  }

  pub fn reset_spooled_time(&mut self) {
    self.spooled_time = None;
  }

  pub fn set_undelivered(&mut self, undelivered: bool) {
    self.undelivered = Some(undelivered);
  }

  pub fn with_undelivered(mut self, undelivered: bool) -> MsgVpnQueueMsg {
    self.undelivered = Some(undelivered);
    self
  }

  pub fn undelivered(&self) -> Option<&bool> {
    self.undelivered.as_ref()
  }

  pub fn reset_undelivered(&mut self) {
    self.undelivered = None;
  }

}



