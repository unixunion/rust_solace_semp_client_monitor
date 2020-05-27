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
pub struct MsgVpnClientTransactedSession {
  /// The name of the Client.
  #[serde(rename = "clientName", skip_serializing_if="Option::is_none")]
  client_name: Option<String>,
  /// The number of transactions committed within the Transacted Session.
  #[serde(rename = "commitCount", skip_serializing_if="Option::is_none")]
  commit_count: Option<i64>,
  /// The number of transaction commit operations that failed.
  #[serde(rename = "commitFailureCount", skip_serializing_if="Option::is_none")]
  commit_failure_count: Option<i64>,
  /// The number of transaction commit operations that succeeded.
  #[serde(rename = "commitSuccessCount", skip_serializing_if="Option::is_none")]
  commit_success_count: Option<i64>,
  /// The number of messages consumed within the Transacted Session.
  #[serde(rename = "consumedMsgCount", skip_serializing_if="Option::is_none")]
  consumed_msg_count: Option<i64>,
  /// The number of transaction end fail operations that failed.
  #[serde(rename = "endFailFailureCount", skip_serializing_if="Option::is_none")]
  end_fail_failure_count: Option<i64>,
  /// The number of transaction end fail operations that succeeded.
  #[serde(rename = "endFailSuccessCount", skip_serializing_if="Option::is_none")]
  end_fail_success_count: Option<i64>,
  /// The number of transaction end operations that failed.
  #[serde(rename = "endFailureCount", skip_serializing_if="Option::is_none")]
  end_failure_count: Option<i64>,
  /// The number of transaction end rollback operations that failed.
  #[serde(rename = "endRollbackFailureCount", skip_serializing_if="Option::is_none")]
  end_rollback_failure_count: Option<i64>,
  /// The number of transaction end rollback operations that succeeded.
  #[serde(rename = "endRollbackSuccessCount", skip_serializing_if="Option::is_none")]
  end_rollback_success_count: Option<i64>,
  /// The number of transaction end operations that succeeded.
  #[serde(rename = "endSuccessCount", skip_serializing_if="Option::is_none")]
  end_success_count: Option<i64>,
  /// The number of transactions that failed within the Transacted Session.
  #[serde(rename = "failureCount", skip_serializing_if="Option::is_none")]
  failure_count: Option<i64>,
  /// The number of transaction forget operations that failed.
  #[serde(rename = "forgetFailureCount", skip_serializing_if="Option::is_none")]
  forget_failure_count: Option<i64>,
  /// The number of transaction forget operations that succeeded.
  #[serde(rename = "forgetSuccessCount", skip_serializing_if="Option::is_none")]
  forget_success_count: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The number of transaction one-phase commit operations that failed.
  #[serde(rename = "onePhaseCommitFailureCount", skip_serializing_if="Option::is_none")]
  one_phase_commit_failure_count: Option<i64>,
  /// The number of transaction one-phase commit operations that succeeded.
  #[serde(rename = "onePhaseCommitSuccessCount", skip_serializing_if="Option::is_none")]
  one_phase_commit_success_count: Option<i64>,
  /// The number of messages to be consumed when the transaction is committed.
  #[serde(rename = "pendingConsumedMsgCount", skip_serializing_if="Option::is_none")]
  pending_consumed_msg_count: Option<i32>,
  /// The number of messages to be published when the transaction is committed.
  #[serde(rename = "pendingPublishedMsgCount", skip_serializing_if="Option::is_none")]
  pending_published_msg_count: Option<i32>,
  /// The number of transaction prepare operations that failed.
  #[serde(rename = "prepareFailureCount", skip_serializing_if="Option::is_none")]
  prepare_failure_count: Option<i64>,
  /// The number of transaction prepare operations that succeeded.
  #[serde(rename = "prepareSuccessCount", skip_serializing_if="Option::is_none")]
  prepare_success_count: Option<i64>,
  /// The state of the previous transaction. The allowed values and their meaning are:  <pre> \"none\" - The previous transaction had no state. \"committed\" - The previous transaction was committed. \"rolled-back\" - The previous transaction was rolled back. \"failed\" - The previous transaction failed. </pre> 
  #[serde(rename = "previousTransactionState", skip_serializing_if="Option::is_none")]
  previous_transaction_state: Option<String>,
  /// The number of messages published within the Transacted Session.
  #[serde(rename = "publishedMsgCount", skip_serializing_if="Option::is_none")]
  published_msg_count: Option<i64>,
  /// The number of transaction resume operations that failed.
  #[serde(rename = "resumeFailureCount", skip_serializing_if="Option::is_none")]
  resume_failure_count: Option<i64>,
  /// The number of transaction resume operations that succeeded.
  #[serde(rename = "resumeSuccessCount", skip_serializing_if="Option::is_none")]
  resume_success_count: Option<i64>,
  /// The number of messages retrieved within the Transacted Session.
  #[serde(rename = "retrievedMsgCount", skip_serializing_if="Option::is_none")]
  retrieved_msg_count: Option<i64>,
  /// The number of transactions rolled back within the Transacted Session.
  #[serde(rename = "rollbackCount", skip_serializing_if="Option::is_none")]
  rollback_count: Option<i64>,
  /// The number of transaction rollback operations that failed.
  #[serde(rename = "rollbackFailureCount", skip_serializing_if="Option::is_none")]
  rollback_failure_count: Option<i64>,
  /// The number of transaction rollback operations that succeeded.
  #[serde(rename = "rollbackSuccessCount", skip_serializing_if="Option::is_none")]
  rollback_success_count: Option<i64>,
  /// The name of the Transacted Session.
  #[serde(rename = "sessionName", skip_serializing_if="Option::is_none")]
  session_name: Option<String>,
  /// The number of messages spooled within the Transacted Session.
  #[serde(rename = "spooledMsgCount", skip_serializing_if="Option::is_none")]
  spooled_msg_count: Option<i64>,
  /// The number of transaction start operations that failed.
  #[serde(rename = "startFailureCount", skip_serializing_if="Option::is_none")]
  start_failure_count: Option<i64>,
  /// The number of transaction start operations that succeeded.
  #[serde(rename = "startSuccessCount", skip_serializing_if="Option::is_none")]
  start_success_count: Option<i64>,
  /// The number of transactions that succeeded within the Transacted Session.
  #[serde(rename = "successCount", skip_serializing_if="Option::is_none")]
  success_count: Option<i64>,
  /// The number of transaction suspend operations that failed.
  #[serde(rename = "suspendFailureCount", skip_serializing_if="Option::is_none")]
  suspend_failure_count: Option<i64>,
  /// The number of transaction suspend operations that succeeded.
  #[serde(rename = "suspendSuccessCount", skip_serializing_if="Option::is_none")]
  suspend_success_count: Option<i64>,
  /// The identifier (ID) of the transaction in the Transacted Session.
  #[serde(rename = "transactionId", skip_serializing_if="Option::is_none")]
  transaction_id: Option<i32>,
  /// The state of the current transaction. The allowed values and their meaning are:  <pre> \"in-progress\" - The current transaction is in progress. \"committing\" - The current transaction is committing. \"rolling-back\" - The current transaction is rolling back. \"failing\" - The current transaction is failing. </pre> 
  #[serde(rename = "transactionState", skip_serializing_if="Option::is_none")]
  transaction_state: Option<String>,
  /// The number of transaction two-phase commit operations that failed.
  #[serde(rename = "twoPhaseCommitFailureCount", skip_serializing_if="Option::is_none")]
  two_phase_commit_failure_count: Option<i64>,
  /// The number of transaction two-phase commit operations that succeeded.
  #[serde(rename = "twoPhaseCommitSuccessCount", skip_serializing_if="Option::is_none")]
  two_phase_commit_success_count: Option<i64>
}

impl MsgVpnClientTransactedSession {
  pub fn new() -> MsgVpnClientTransactedSession {
    MsgVpnClientTransactedSession {
      client_name: None,
      commit_count: None,
      commit_failure_count: None,
      commit_success_count: None,
      consumed_msg_count: None,
      end_fail_failure_count: None,
      end_fail_success_count: None,
      end_failure_count: None,
      end_rollback_failure_count: None,
      end_rollback_success_count: None,
      end_success_count: None,
      failure_count: None,
      forget_failure_count: None,
      forget_success_count: None,
      msg_vpn_name: None,
      one_phase_commit_failure_count: None,
      one_phase_commit_success_count: None,
      pending_consumed_msg_count: None,
      pending_published_msg_count: None,
      prepare_failure_count: None,
      prepare_success_count: None,
      previous_transaction_state: None,
      published_msg_count: None,
      resume_failure_count: None,
      resume_success_count: None,
      retrieved_msg_count: None,
      rollback_count: None,
      rollback_failure_count: None,
      rollback_success_count: None,
      session_name: None,
      spooled_msg_count: None,
      start_failure_count: None,
      start_success_count: None,
      success_count: None,
      suspend_failure_count: None,
      suspend_success_count: None,
      transaction_id: None,
      transaction_state: None,
      two_phase_commit_failure_count: None,
      two_phase_commit_success_count: None
    }
  }

  pub fn set_client_name(&mut self, client_name: String) {
    self.client_name = Some(client_name);
  }

  pub fn with_client_name(mut self, client_name: String) -> MsgVpnClientTransactedSession {
    self.client_name = Some(client_name);
    self
  }

  pub fn client_name(&self) -> Option<&String> {
    self.client_name.as_ref()
  }

  pub fn reset_client_name(&mut self) {
    self.client_name = None;
  }

  pub fn set_commit_count(&mut self, commit_count: i64) {
    self.commit_count = Some(commit_count);
  }

  pub fn with_commit_count(mut self, commit_count: i64) -> MsgVpnClientTransactedSession {
    self.commit_count = Some(commit_count);
    self
  }

  pub fn commit_count(&self) -> Option<&i64> {
    self.commit_count.as_ref()
  }

  pub fn reset_commit_count(&mut self) {
    self.commit_count = None;
  }

  pub fn set_commit_failure_count(&mut self, commit_failure_count: i64) {
    self.commit_failure_count = Some(commit_failure_count);
  }

  pub fn with_commit_failure_count(mut self, commit_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.commit_failure_count = Some(commit_failure_count);
    self
  }

  pub fn commit_failure_count(&self) -> Option<&i64> {
    self.commit_failure_count.as_ref()
  }

  pub fn reset_commit_failure_count(&mut self) {
    self.commit_failure_count = None;
  }

  pub fn set_commit_success_count(&mut self, commit_success_count: i64) {
    self.commit_success_count = Some(commit_success_count);
  }

  pub fn with_commit_success_count(mut self, commit_success_count: i64) -> MsgVpnClientTransactedSession {
    self.commit_success_count = Some(commit_success_count);
    self
  }

  pub fn commit_success_count(&self) -> Option<&i64> {
    self.commit_success_count.as_ref()
  }

  pub fn reset_commit_success_count(&mut self) {
    self.commit_success_count = None;
  }

  pub fn set_consumed_msg_count(&mut self, consumed_msg_count: i64) {
    self.consumed_msg_count = Some(consumed_msg_count);
  }

  pub fn with_consumed_msg_count(mut self, consumed_msg_count: i64) -> MsgVpnClientTransactedSession {
    self.consumed_msg_count = Some(consumed_msg_count);
    self
  }

  pub fn consumed_msg_count(&self) -> Option<&i64> {
    self.consumed_msg_count.as_ref()
  }

  pub fn reset_consumed_msg_count(&mut self) {
    self.consumed_msg_count = None;
  }

  pub fn set_end_fail_failure_count(&mut self, end_fail_failure_count: i64) {
    self.end_fail_failure_count = Some(end_fail_failure_count);
  }

  pub fn with_end_fail_failure_count(mut self, end_fail_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.end_fail_failure_count = Some(end_fail_failure_count);
    self
  }

  pub fn end_fail_failure_count(&self) -> Option<&i64> {
    self.end_fail_failure_count.as_ref()
  }

  pub fn reset_end_fail_failure_count(&mut self) {
    self.end_fail_failure_count = None;
  }

  pub fn set_end_fail_success_count(&mut self, end_fail_success_count: i64) {
    self.end_fail_success_count = Some(end_fail_success_count);
  }

  pub fn with_end_fail_success_count(mut self, end_fail_success_count: i64) -> MsgVpnClientTransactedSession {
    self.end_fail_success_count = Some(end_fail_success_count);
    self
  }

  pub fn end_fail_success_count(&self) -> Option<&i64> {
    self.end_fail_success_count.as_ref()
  }

  pub fn reset_end_fail_success_count(&mut self) {
    self.end_fail_success_count = None;
  }

  pub fn set_end_failure_count(&mut self, end_failure_count: i64) {
    self.end_failure_count = Some(end_failure_count);
  }

  pub fn with_end_failure_count(mut self, end_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.end_failure_count = Some(end_failure_count);
    self
  }

  pub fn end_failure_count(&self) -> Option<&i64> {
    self.end_failure_count.as_ref()
  }

  pub fn reset_end_failure_count(&mut self) {
    self.end_failure_count = None;
  }

  pub fn set_end_rollback_failure_count(&mut self, end_rollback_failure_count: i64) {
    self.end_rollback_failure_count = Some(end_rollback_failure_count);
  }

  pub fn with_end_rollback_failure_count(mut self, end_rollback_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.end_rollback_failure_count = Some(end_rollback_failure_count);
    self
  }

  pub fn end_rollback_failure_count(&self) -> Option<&i64> {
    self.end_rollback_failure_count.as_ref()
  }

  pub fn reset_end_rollback_failure_count(&mut self) {
    self.end_rollback_failure_count = None;
  }

  pub fn set_end_rollback_success_count(&mut self, end_rollback_success_count: i64) {
    self.end_rollback_success_count = Some(end_rollback_success_count);
  }

  pub fn with_end_rollback_success_count(mut self, end_rollback_success_count: i64) -> MsgVpnClientTransactedSession {
    self.end_rollback_success_count = Some(end_rollback_success_count);
    self
  }

  pub fn end_rollback_success_count(&self) -> Option<&i64> {
    self.end_rollback_success_count.as_ref()
  }

  pub fn reset_end_rollback_success_count(&mut self) {
    self.end_rollback_success_count = None;
  }

  pub fn set_end_success_count(&mut self, end_success_count: i64) {
    self.end_success_count = Some(end_success_count);
  }

  pub fn with_end_success_count(mut self, end_success_count: i64) -> MsgVpnClientTransactedSession {
    self.end_success_count = Some(end_success_count);
    self
  }

  pub fn end_success_count(&self) -> Option<&i64> {
    self.end_success_count.as_ref()
  }

  pub fn reset_end_success_count(&mut self) {
    self.end_success_count = None;
  }

  pub fn set_failure_count(&mut self, failure_count: i64) {
    self.failure_count = Some(failure_count);
  }

  pub fn with_failure_count(mut self, failure_count: i64) -> MsgVpnClientTransactedSession {
    self.failure_count = Some(failure_count);
    self
  }

  pub fn failure_count(&self) -> Option<&i64> {
    self.failure_count.as_ref()
  }

  pub fn reset_failure_count(&mut self) {
    self.failure_count = None;
  }

  pub fn set_forget_failure_count(&mut self, forget_failure_count: i64) {
    self.forget_failure_count = Some(forget_failure_count);
  }

  pub fn with_forget_failure_count(mut self, forget_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.forget_failure_count = Some(forget_failure_count);
    self
  }

  pub fn forget_failure_count(&self) -> Option<&i64> {
    self.forget_failure_count.as_ref()
  }

  pub fn reset_forget_failure_count(&mut self) {
    self.forget_failure_count = None;
  }

  pub fn set_forget_success_count(&mut self, forget_success_count: i64) {
    self.forget_success_count = Some(forget_success_count);
  }

  pub fn with_forget_success_count(mut self, forget_success_count: i64) -> MsgVpnClientTransactedSession {
    self.forget_success_count = Some(forget_success_count);
    self
  }

  pub fn forget_success_count(&self) -> Option<&i64> {
    self.forget_success_count.as_ref()
  }

  pub fn reset_forget_success_count(&mut self) {
    self.forget_success_count = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnClientTransactedSession {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_one_phase_commit_failure_count(&mut self, one_phase_commit_failure_count: i64) {
    self.one_phase_commit_failure_count = Some(one_phase_commit_failure_count);
  }

  pub fn with_one_phase_commit_failure_count(mut self, one_phase_commit_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.one_phase_commit_failure_count = Some(one_phase_commit_failure_count);
    self
  }

  pub fn one_phase_commit_failure_count(&self) -> Option<&i64> {
    self.one_phase_commit_failure_count.as_ref()
  }

  pub fn reset_one_phase_commit_failure_count(&mut self) {
    self.one_phase_commit_failure_count = None;
  }

  pub fn set_one_phase_commit_success_count(&mut self, one_phase_commit_success_count: i64) {
    self.one_phase_commit_success_count = Some(one_phase_commit_success_count);
  }

  pub fn with_one_phase_commit_success_count(mut self, one_phase_commit_success_count: i64) -> MsgVpnClientTransactedSession {
    self.one_phase_commit_success_count = Some(one_phase_commit_success_count);
    self
  }

  pub fn one_phase_commit_success_count(&self) -> Option<&i64> {
    self.one_phase_commit_success_count.as_ref()
  }

  pub fn reset_one_phase_commit_success_count(&mut self) {
    self.one_phase_commit_success_count = None;
  }

  pub fn set_pending_consumed_msg_count(&mut self, pending_consumed_msg_count: i32) {
    self.pending_consumed_msg_count = Some(pending_consumed_msg_count);
  }

  pub fn with_pending_consumed_msg_count(mut self, pending_consumed_msg_count: i32) -> MsgVpnClientTransactedSession {
    self.pending_consumed_msg_count = Some(pending_consumed_msg_count);
    self
  }

  pub fn pending_consumed_msg_count(&self) -> Option<&i32> {
    self.pending_consumed_msg_count.as_ref()
  }

  pub fn reset_pending_consumed_msg_count(&mut self) {
    self.pending_consumed_msg_count = None;
  }

  pub fn set_pending_published_msg_count(&mut self, pending_published_msg_count: i32) {
    self.pending_published_msg_count = Some(pending_published_msg_count);
  }

  pub fn with_pending_published_msg_count(mut self, pending_published_msg_count: i32) -> MsgVpnClientTransactedSession {
    self.pending_published_msg_count = Some(pending_published_msg_count);
    self
  }

  pub fn pending_published_msg_count(&self) -> Option<&i32> {
    self.pending_published_msg_count.as_ref()
  }

  pub fn reset_pending_published_msg_count(&mut self) {
    self.pending_published_msg_count = None;
  }

  pub fn set_prepare_failure_count(&mut self, prepare_failure_count: i64) {
    self.prepare_failure_count = Some(prepare_failure_count);
  }

  pub fn with_prepare_failure_count(mut self, prepare_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.prepare_failure_count = Some(prepare_failure_count);
    self
  }

  pub fn prepare_failure_count(&self) -> Option<&i64> {
    self.prepare_failure_count.as_ref()
  }

  pub fn reset_prepare_failure_count(&mut self) {
    self.prepare_failure_count = None;
  }

  pub fn set_prepare_success_count(&mut self, prepare_success_count: i64) {
    self.prepare_success_count = Some(prepare_success_count);
  }

  pub fn with_prepare_success_count(mut self, prepare_success_count: i64) -> MsgVpnClientTransactedSession {
    self.prepare_success_count = Some(prepare_success_count);
    self
  }

  pub fn prepare_success_count(&self) -> Option<&i64> {
    self.prepare_success_count.as_ref()
  }

  pub fn reset_prepare_success_count(&mut self) {
    self.prepare_success_count = None;
  }

  pub fn set_previous_transaction_state(&mut self, previous_transaction_state: String) {
    self.previous_transaction_state = Some(previous_transaction_state);
  }

  pub fn with_previous_transaction_state(mut self, previous_transaction_state: String) -> MsgVpnClientTransactedSession {
    self.previous_transaction_state = Some(previous_transaction_state);
    self
  }

  pub fn previous_transaction_state(&self) -> Option<&String> {
    self.previous_transaction_state.as_ref()
  }

  pub fn reset_previous_transaction_state(&mut self) {
    self.previous_transaction_state = None;
  }

  pub fn set_published_msg_count(&mut self, published_msg_count: i64) {
    self.published_msg_count = Some(published_msg_count);
  }

  pub fn with_published_msg_count(mut self, published_msg_count: i64) -> MsgVpnClientTransactedSession {
    self.published_msg_count = Some(published_msg_count);
    self
  }

  pub fn published_msg_count(&self) -> Option<&i64> {
    self.published_msg_count.as_ref()
  }

  pub fn reset_published_msg_count(&mut self) {
    self.published_msg_count = None;
  }

  pub fn set_resume_failure_count(&mut self, resume_failure_count: i64) {
    self.resume_failure_count = Some(resume_failure_count);
  }

  pub fn with_resume_failure_count(mut self, resume_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.resume_failure_count = Some(resume_failure_count);
    self
  }

  pub fn resume_failure_count(&self) -> Option<&i64> {
    self.resume_failure_count.as_ref()
  }

  pub fn reset_resume_failure_count(&mut self) {
    self.resume_failure_count = None;
  }

  pub fn set_resume_success_count(&mut self, resume_success_count: i64) {
    self.resume_success_count = Some(resume_success_count);
  }

  pub fn with_resume_success_count(mut self, resume_success_count: i64) -> MsgVpnClientTransactedSession {
    self.resume_success_count = Some(resume_success_count);
    self
  }

  pub fn resume_success_count(&self) -> Option<&i64> {
    self.resume_success_count.as_ref()
  }

  pub fn reset_resume_success_count(&mut self) {
    self.resume_success_count = None;
  }

  pub fn set_retrieved_msg_count(&mut self, retrieved_msg_count: i64) {
    self.retrieved_msg_count = Some(retrieved_msg_count);
  }

  pub fn with_retrieved_msg_count(mut self, retrieved_msg_count: i64) -> MsgVpnClientTransactedSession {
    self.retrieved_msg_count = Some(retrieved_msg_count);
    self
  }

  pub fn retrieved_msg_count(&self) -> Option<&i64> {
    self.retrieved_msg_count.as_ref()
  }

  pub fn reset_retrieved_msg_count(&mut self) {
    self.retrieved_msg_count = None;
  }

  pub fn set_rollback_count(&mut self, rollback_count: i64) {
    self.rollback_count = Some(rollback_count);
  }

  pub fn with_rollback_count(mut self, rollback_count: i64) -> MsgVpnClientTransactedSession {
    self.rollback_count = Some(rollback_count);
    self
  }

  pub fn rollback_count(&self) -> Option<&i64> {
    self.rollback_count.as_ref()
  }

  pub fn reset_rollback_count(&mut self) {
    self.rollback_count = None;
  }

  pub fn set_rollback_failure_count(&mut self, rollback_failure_count: i64) {
    self.rollback_failure_count = Some(rollback_failure_count);
  }

  pub fn with_rollback_failure_count(mut self, rollback_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.rollback_failure_count = Some(rollback_failure_count);
    self
  }

  pub fn rollback_failure_count(&self) -> Option<&i64> {
    self.rollback_failure_count.as_ref()
  }

  pub fn reset_rollback_failure_count(&mut self) {
    self.rollback_failure_count = None;
  }

  pub fn set_rollback_success_count(&mut self, rollback_success_count: i64) {
    self.rollback_success_count = Some(rollback_success_count);
  }

  pub fn with_rollback_success_count(mut self, rollback_success_count: i64) -> MsgVpnClientTransactedSession {
    self.rollback_success_count = Some(rollback_success_count);
    self
  }

  pub fn rollback_success_count(&self) -> Option<&i64> {
    self.rollback_success_count.as_ref()
  }

  pub fn reset_rollback_success_count(&mut self) {
    self.rollback_success_count = None;
  }

  pub fn set_session_name(&mut self, session_name: String) {
    self.session_name = Some(session_name);
  }

  pub fn with_session_name(mut self, session_name: String) -> MsgVpnClientTransactedSession {
    self.session_name = Some(session_name);
    self
  }

  pub fn session_name(&self) -> Option<&String> {
    self.session_name.as_ref()
  }

  pub fn reset_session_name(&mut self) {
    self.session_name = None;
  }

  pub fn set_spooled_msg_count(&mut self, spooled_msg_count: i64) {
    self.spooled_msg_count = Some(spooled_msg_count);
  }

  pub fn with_spooled_msg_count(mut self, spooled_msg_count: i64) -> MsgVpnClientTransactedSession {
    self.spooled_msg_count = Some(spooled_msg_count);
    self
  }

  pub fn spooled_msg_count(&self) -> Option<&i64> {
    self.spooled_msg_count.as_ref()
  }

  pub fn reset_spooled_msg_count(&mut self) {
    self.spooled_msg_count = None;
  }

  pub fn set_start_failure_count(&mut self, start_failure_count: i64) {
    self.start_failure_count = Some(start_failure_count);
  }

  pub fn with_start_failure_count(mut self, start_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.start_failure_count = Some(start_failure_count);
    self
  }

  pub fn start_failure_count(&self) -> Option<&i64> {
    self.start_failure_count.as_ref()
  }

  pub fn reset_start_failure_count(&mut self) {
    self.start_failure_count = None;
  }

  pub fn set_start_success_count(&mut self, start_success_count: i64) {
    self.start_success_count = Some(start_success_count);
  }

  pub fn with_start_success_count(mut self, start_success_count: i64) -> MsgVpnClientTransactedSession {
    self.start_success_count = Some(start_success_count);
    self
  }

  pub fn start_success_count(&self) -> Option<&i64> {
    self.start_success_count.as_ref()
  }

  pub fn reset_start_success_count(&mut self) {
    self.start_success_count = None;
  }

  pub fn set_success_count(&mut self, success_count: i64) {
    self.success_count = Some(success_count);
  }

  pub fn with_success_count(mut self, success_count: i64) -> MsgVpnClientTransactedSession {
    self.success_count = Some(success_count);
    self
  }

  pub fn success_count(&self) -> Option<&i64> {
    self.success_count.as_ref()
  }

  pub fn reset_success_count(&mut self) {
    self.success_count = None;
  }

  pub fn set_suspend_failure_count(&mut self, suspend_failure_count: i64) {
    self.suspend_failure_count = Some(suspend_failure_count);
  }

  pub fn with_suspend_failure_count(mut self, suspend_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.suspend_failure_count = Some(suspend_failure_count);
    self
  }

  pub fn suspend_failure_count(&self) -> Option<&i64> {
    self.suspend_failure_count.as_ref()
  }

  pub fn reset_suspend_failure_count(&mut self) {
    self.suspend_failure_count = None;
  }

  pub fn set_suspend_success_count(&mut self, suspend_success_count: i64) {
    self.suspend_success_count = Some(suspend_success_count);
  }

  pub fn with_suspend_success_count(mut self, suspend_success_count: i64) -> MsgVpnClientTransactedSession {
    self.suspend_success_count = Some(suspend_success_count);
    self
  }

  pub fn suspend_success_count(&self) -> Option<&i64> {
    self.suspend_success_count.as_ref()
  }

  pub fn reset_suspend_success_count(&mut self) {
    self.suspend_success_count = None;
  }

  pub fn set_transaction_id(&mut self, transaction_id: i32) {
    self.transaction_id = Some(transaction_id);
  }

  pub fn with_transaction_id(mut self, transaction_id: i32) -> MsgVpnClientTransactedSession {
    self.transaction_id = Some(transaction_id);
    self
  }

  pub fn transaction_id(&self) -> Option<&i32> {
    self.transaction_id.as_ref()
  }

  pub fn reset_transaction_id(&mut self) {
    self.transaction_id = None;
  }

  pub fn set_transaction_state(&mut self, transaction_state: String) {
    self.transaction_state = Some(transaction_state);
  }

  pub fn with_transaction_state(mut self, transaction_state: String) -> MsgVpnClientTransactedSession {
    self.transaction_state = Some(transaction_state);
    self
  }

  pub fn transaction_state(&self) -> Option<&String> {
    self.transaction_state.as_ref()
  }

  pub fn reset_transaction_state(&mut self) {
    self.transaction_state = None;
  }

  pub fn set_two_phase_commit_failure_count(&mut self, two_phase_commit_failure_count: i64) {
    self.two_phase_commit_failure_count = Some(two_phase_commit_failure_count);
  }

  pub fn with_two_phase_commit_failure_count(mut self, two_phase_commit_failure_count: i64) -> MsgVpnClientTransactedSession {
    self.two_phase_commit_failure_count = Some(two_phase_commit_failure_count);
    self
  }

  pub fn two_phase_commit_failure_count(&self) -> Option<&i64> {
    self.two_phase_commit_failure_count.as_ref()
  }

  pub fn reset_two_phase_commit_failure_count(&mut self) {
    self.two_phase_commit_failure_count = None;
  }

  pub fn set_two_phase_commit_success_count(&mut self, two_phase_commit_success_count: i64) {
    self.two_phase_commit_success_count = Some(two_phase_commit_success_count);
  }

  pub fn with_two_phase_commit_success_count(mut self, two_phase_commit_success_count: i64) -> MsgVpnClientTransactedSession {
    self.two_phase_commit_success_count = Some(two_phase_commit_success_count);
    self
  }

  pub fn two_phase_commit_success_count(&self) -> Option<&i64> {
    self.two_phase_commit_success_count.as_ref()
  }

  pub fn reset_two_phase_commit_success_count(&mut self) {
    self.two_phase_commit_success_count = None;
  }

}



