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
pub struct DmrCluster {
  /// Enable or disable basic authentication for Cluster Links.
  #[serde(rename = "authenticationBasicEnabled", skip_serializing_if="Option::is_none")]
  authentication_basic_enabled: Option<bool>,
  /// The type of basic authentication to use for Cluster Links. The allowed values and their meaning are:  <pre> \"internal\" - Use locally configured password. \"none\" - No authentication. </pre> 
  #[serde(rename = "authenticationBasicType", skip_serializing_if="Option::is_none")]
  authentication_basic_type: Option<String>,
  /// Enable or disable client certificate authentication for Cluster Links.
  #[serde(rename = "authenticationClientCertEnabled", skip_serializing_if="Option::is_none")]
  authentication_client_cert_enabled: Option<bool>,
  /// Enable or disable direct messaging only. Guaranteed messages will not be transmitted through the cluster.
  #[serde(rename = "directOnlyEnabled", skip_serializing_if="Option::is_none")]
  direct_only_enabled: Option<bool>,
  /// The name of the Cluster.
  #[serde(rename = "dmrClusterName", skip_serializing_if="Option::is_none")]
  dmr_cluster_name: Option<String>,
  /// Enable or disable the Cluster.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The failure reason for the Cluster being down.
  #[serde(rename = "failureReason", skip_serializing_if="Option::is_none")]
  failure_reason: Option<String>,
  /// The name of this node in the Cluster. This is the name that this broker (or redundant group of brokers) is know by to other nodes in the Cluster. The name is chosen automatically to be either this broker's Router Name or Mate Router Name, depending on which Active Standby Role (primary or backup) this broker plays in its redundancy group.
  #[serde(rename = "nodeName", skip_serializing_if="Option::is_none")]
  node_name: Option<String>,
  /// Enable or disable the enforcing of the common name provided by the remote broker against the list of trusted common names configured for the Link. If enabled, the certificate's common name must match one of the trusted common names for the Link to be accepted.
  #[serde(rename = "tlsServerCertEnforceTrustedCommonNameEnabled", skip_serializing_if="Option::is_none")]
  tls_server_cert_enforce_trusted_common_name_enabled: Option<bool>,
  /// The maximum allowed depth of a certificate chain. The depth of a chain is defined as the number of signing CA certificates that are present in the chain back to a trusted self-signed root CA certificate.
  #[serde(rename = "tlsServerCertMaxChainDepth", skip_serializing_if="Option::is_none")]
  tls_server_cert_max_chain_depth: Option<i64>,
  /// Enable or disable the validation of the \"Not Before\" and \"Not After\" validity dates in the certificate. When disabled, the certificate is accepted even if the certificate is not valid based on these dates.
  #[serde(rename = "tlsServerCertValidateDateEnabled", skip_serializing_if="Option::is_none")]
  tls_server_cert_validate_date_enabled: Option<bool>,
  /// Indicates whether the Cluster is operationally up.
  #[serde(rename = "up", skip_serializing_if="Option::is_none")]
  up: Option<bool>,
  /// The amount of time in seconds since the Cluster was up.
  #[serde(rename = "uptime", skip_serializing_if="Option::is_none")]
  uptime: Option<i64>
}

impl DmrCluster {
  pub fn new() -> DmrCluster {
    DmrCluster {
      authentication_basic_enabled: None,
      authentication_basic_type: None,
      authentication_client_cert_enabled: None,
      direct_only_enabled: None,
      dmr_cluster_name: None,
      enabled: None,
      failure_reason: None,
      node_name: None,
      tls_server_cert_enforce_trusted_common_name_enabled: None,
      tls_server_cert_max_chain_depth: None,
      tls_server_cert_validate_date_enabled: None,
      up: None,
      uptime: None
    }
  }

  pub fn set_authentication_basic_enabled(&mut self, authentication_basic_enabled: bool) {
    self.authentication_basic_enabled = Some(authentication_basic_enabled);
  }

  pub fn with_authentication_basic_enabled(mut self, authentication_basic_enabled: bool) -> DmrCluster {
    self.authentication_basic_enabled = Some(authentication_basic_enabled);
    self
  }

  pub fn authentication_basic_enabled(&self) -> Option<&bool> {
    self.authentication_basic_enabled.as_ref()
  }

  pub fn reset_authentication_basic_enabled(&mut self) {
    self.authentication_basic_enabled = None;
  }

  pub fn set_authentication_basic_type(&mut self, authentication_basic_type: String) {
    self.authentication_basic_type = Some(authentication_basic_type);
  }

  pub fn with_authentication_basic_type(mut self, authentication_basic_type: String) -> DmrCluster {
    self.authentication_basic_type = Some(authentication_basic_type);
    self
  }

  pub fn authentication_basic_type(&self) -> Option<&String> {
    self.authentication_basic_type.as_ref()
  }

  pub fn reset_authentication_basic_type(&mut self) {
    self.authentication_basic_type = None;
  }

  pub fn set_authentication_client_cert_enabled(&mut self, authentication_client_cert_enabled: bool) {
    self.authentication_client_cert_enabled = Some(authentication_client_cert_enabled);
  }

  pub fn with_authentication_client_cert_enabled(mut self, authentication_client_cert_enabled: bool) -> DmrCluster {
    self.authentication_client_cert_enabled = Some(authentication_client_cert_enabled);
    self
  }

  pub fn authentication_client_cert_enabled(&self) -> Option<&bool> {
    self.authentication_client_cert_enabled.as_ref()
  }

  pub fn reset_authentication_client_cert_enabled(&mut self) {
    self.authentication_client_cert_enabled = None;
  }

  pub fn set_direct_only_enabled(&mut self, direct_only_enabled: bool) {
    self.direct_only_enabled = Some(direct_only_enabled);
  }

  pub fn with_direct_only_enabled(mut self, direct_only_enabled: bool) -> DmrCluster {
    self.direct_only_enabled = Some(direct_only_enabled);
    self
  }

  pub fn direct_only_enabled(&self) -> Option<&bool> {
    self.direct_only_enabled.as_ref()
  }

  pub fn reset_direct_only_enabled(&mut self) {
    self.direct_only_enabled = None;
  }

  pub fn set_dmr_cluster_name(&mut self, dmr_cluster_name: String) {
    self.dmr_cluster_name = Some(dmr_cluster_name);
  }

  pub fn with_dmr_cluster_name(mut self, dmr_cluster_name: String) -> DmrCluster {
    self.dmr_cluster_name = Some(dmr_cluster_name);
    self
  }

  pub fn dmr_cluster_name(&self) -> Option<&String> {
    self.dmr_cluster_name.as_ref()
  }

  pub fn reset_dmr_cluster_name(&mut self) {
    self.dmr_cluster_name = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> DmrCluster {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_failure_reason(&mut self, failure_reason: String) {
    self.failure_reason = Some(failure_reason);
  }

  pub fn with_failure_reason(mut self, failure_reason: String) -> DmrCluster {
    self.failure_reason = Some(failure_reason);
    self
  }

  pub fn failure_reason(&self) -> Option<&String> {
    self.failure_reason.as_ref()
  }

  pub fn reset_failure_reason(&mut self) {
    self.failure_reason = None;
  }

  pub fn set_node_name(&mut self, node_name: String) {
    self.node_name = Some(node_name);
  }

  pub fn with_node_name(mut self, node_name: String) -> DmrCluster {
    self.node_name = Some(node_name);
    self
  }

  pub fn node_name(&self) -> Option<&String> {
    self.node_name.as_ref()
  }

  pub fn reset_node_name(&mut self) {
    self.node_name = None;
  }

  pub fn set_tls_server_cert_enforce_trusted_common_name_enabled(&mut self, tls_server_cert_enforce_trusted_common_name_enabled: bool) {
    self.tls_server_cert_enforce_trusted_common_name_enabled = Some(tls_server_cert_enforce_trusted_common_name_enabled);
  }

  pub fn with_tls_server_cert_enforce_trusted_common_name_enabled(mut self, tls_server_cert_enforce_trusted_common_name_enabled: bool) -> DmrCluster {
    self.tls_server_cert_enforce_trusted_common_name_enabled = Some(tls_server_cert_enforce_trusted_common_name_enabled);
    self
  }

  pub fn tls_server_cert_enforce_trusted_common_name_enabled(&self) -> Option<&bool> {
    self.tls_server_cert_enforce_trusted_common_name_enabled.as_ref()
  }

  pub fn reset_tls_server_cert_enforce_trusted_common_name_enabled(&mut self) {
    self.tls_server_cert_enforce_trusted_common_name_enabled = None;
  }

  pub fn set_tls_server_cert_max_chain_depth(&mut self, tls_server_cert_max_chain_depth: i64) {
    self.tls_server_cert_max_chain_depth = Some(tls_server_cert_max_chain_depth);
  }

  pub fn with_tls_server_cert_max_chain_depth(mut self, tls_server_cert_max_chain_depth: i64) -> DmrCluster {
    self.tls_server_cert_max_chain_depth = Some(tls_server_cert_max_chain_depth);
    self
  }

  pub fn tls_server_cert_max_chain_depth(&self) -> Option<&i64> {
    self.tls_server_cert_max_chain_depth.as_ref()
  }

  pub fn reset_tls_server_cert_max_chain_depth(&mut self) {
    self.tls_server_cert_max_chain_depth = None;
  }

  pub fn set_tls_server_cert_validate_date_enabled(&mut self, tls_server_cert_validate_date_enabled: bool) {
    self.tls_server_cert_validate_date_enabled = Some(tls_server_cert_validate_date_enabled);
  }

  pub fn with_tls_server_cert_validate_date_enabled(mut self, tls_server_cert_validate_date_enabled: bool) -> DmrCluster {
    self.tls_server_cert_validate_date_enabled = Some(tls_server_cert_validate_date_enabled);
    self
  }

  pub fn tls_server_cert_validate_date_enabled(&self) -> Option<&bool> {
    self.tls_server_cert_validate_date_enabled.as_ref()
  }

  pub fn reset_tls_server_cert_validate_date_enabled(&mut self) {
    self.tls_server_cert_validate_date_enabled = None;
  }

  pub fn set_up(&mut self, up: bool) {
    self.up = Some(up);
  }

  pub fn with_up(mut self, up: bool) -> DmrCluster {
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

  pub fn with_uptime(mut self, uptime: i64) -> DmrCluster {
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



