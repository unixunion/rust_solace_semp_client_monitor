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
pub struct Broker {
  /// The client certificate revocation checking mode used when a client authenticates with a client certificate. The allowed values and their meaning are:  <pre> \"none\" - Do not perform any certificate revocation checking. \"ocsp\" - Use the Open Certificate Status Protcol (OCSP) for certificate revocation checking. \"crl\" - Use Certificate Revocation Lists (CRL) for certificate revocation checking. \"ocsp-crl\" - Use OCSP first, but if OCSP fails to return an unambiguous result, then check via CRL. </pre> 
  #[serde(rename = "authClientCertRevocationCheckMode", skip_serializing_if="Option::is_none")]
  auth_client_cert_revocation_check_mode: Option<String>,
  /// The one minute average of the message rate received by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "averageRxByteRate", skip_serializing_if="Option::is_none")]
  average_rx_byte_rate: Option<i64>,
  /// The one minute average of the compressed message rate received by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "averageRxCompressedByteRate", skip_serializing_if="Option::is_none")]
  average_rx_compressed_byte_rate: Option<i64>,
  /// The one minute average of the message rate received by the Broker, in messages per second (msg/sec). Available since 2.14.
  #[serde(rename = "averageRxMsgRate", skip_serializing_if="Option::is_none")]
  average_rx_msg_rate: Option<i64>,
  /// The one minute average of the uncompressed message rate received by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "averageRxUncompressedByteRate", skip_serializing_if="Option::is_none")]
  average_rx_uncompressed_byte_rate: Option<i64>,
  /// The one minute average of the message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "averageTxByteRate", skip_serializing_if="Option::is_none")]
  average_tx_byte_rate: Option<i64>,
  /// The one minute average of the compressed message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "averageTxCompressedByteRate", skip_serializing_if="Option::is_none")]
  average_tx_compressed_byte_rate: Option<i64>,
  /// The one minute average of the message rate transmitted by the Broker, in messages per second (msg/sec). Available since 2.14.
  #[serde(rename = "averageTxMsgRate", skip_serializing_if="Option::is_none")]
  average_tx_msg_rate: Option<i64>,
  /// The one minute average of the uncompressed message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "averageTxUncompressedByteRate", skip_serializing_if="Option::is_none")]
  average_tx_uncompressed_byte_rate: Option<i64>,
  /// The amount of messages received from clients by the Broker, in bytes (B). Available since 2.14.
  #[serde(rename = "rxByteCount", skip_serializing_if="Option::is_none")]
  rx_byte_count: Option<i64>,
  /// The current message rate received by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "rxByteRate", skip_serializing_if="Option::is_none")]
  rx_byte_rate: Option<i64>,
  /// The amount of compressed messages received by the Broker, in bytes (B). Available since 2.14.
  #[serde(rename = "rxCompressedByteCount", skip_serializing_if="Option::is_none")]
  rx_compressed_byte_count: Option<i64>,
  /// The current compressed message rate received by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "rxCompressedByteRate", skip_serializing_if="Option::is_none")]
  rx_compressed_byte_rate: Option<i64>,
  /// The compression ratio for messages received by the Broker. Available since 2.14.
  #[serde(rename = "rxCompressionRatio", skip_serializing_if="Option::is_none")]
  rx_compression_ratio: Option<String>,
  /// The number of messages received from clients by the Broker. Available since 2.14.
  #[serde(rename = "rxMsgCount", skip_serializing_if="Option::is_none")]
  rx_msg_count: Option<i64>,
  /// The current message rate received by the Broker, in messages per second (msg/sec). Available since 2.14.
  #[serde(rename = "rxMsgRate", skip_serializing_if="Option::is_none")]
  rx_msg_rate: Option<i64>,
  /// The amount of uncompressed messages received by the Broker, in bytes (B). Available since 2.14.
  #[serde(rename = "rxUncompressedByteCount", skip_serializing_if="Option::is_none")]
  rx_uncompressed_byte_count: Option<i64>,
  /// The current uncompressed message rate received by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "rxUncompressedByteRate", skip_serializing_if="Option::is_none")]
  rx_uncompressed_byte_rate: Option<i64>,
  /// Indicates whether incoming TLS version 1.0 connections are blocked. When blocked, existing TLS 1.0 connections from Clients and SEMP users remain connected while new connections are blocked. Note that support for TLS 1.0 will eventually be discontinued, at which time TLS 1.0 connections will be blocked regardless of this setting.
  #[serde(rename = "tlsBlockVersion10Enabled", skip_serializing_if="Option::is_none")]
  tls_block_version10_enabled: Option<bool>,
  /// Indicates whether TLS version 1.1 connections are blocked. When blocked, all existing incoming and outgoing TLS 1.1 connections with Clients, SEMP users, and LDAP servers remain connected while new connections are blocked. Note that support for TLS 1.1 will eventually be discontinued, at which time TLS 1.1 connections will be blocked regardless of this setting.
  #[serde(rename = "tlsBlockVersion11Enabled", skip_serializing_if="Option::is_none")]
  tls_block_version11_enabled: Option<bool>,
  /// The colon-separated list of default cipher suites for TLS management connections.
  #[serde(rename = "tlsCipherSuiteManagementDefaultList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_management_default_list: Option<String>,
  /// The colon-separated list of cipher suites used for TLS management connections (e.g. SEMP, LDAP). The value \"default\" implies all supported suites ordered from most secure to least secure.
  #[serde(rename = "tlsCipherSuiteManagementList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_management_list: Option<String>,
  /// The colon-separated list of supported cipher suites for TLS management connections.
  #[serde(rename = "tlsCipherSuiteManagementSupportedList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_management_supported_list: Option<String>,
  /// The colon-separated list of default cipher suites for TLS data connections.
  #[serde(rename = "tlsCipherSuiteMsgBackboneDefaultList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_msg_backbone_default_list: Option<String>,
  /// The colon-separated list of cipher suites used for TLS data connections (e.g. client pub/sub). The value \"default\" implies all supported suites ordered from most secure to least secure.
  #[serde(rename = "tlsCipherSuiteMsgBackboneList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_msg_backbone_list: Option<String>,
  /// The colon-separated list of supported cipher suites for TLS data connections.
  #[serde(rename = "tlsCipherSuiteMsgBackboneSupportedList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_msg_backbone_supported_list: Option<String>,
  /// The colon-separated list of default cipher suites for TLS secure shell connections.
  #[serde(rename = "tlsCipherSuiteSecureShellDefaultList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_secure_shell_default_list: Option<String>,
  /// The colon-separated list of cipher suites used for TLS secure shell connections (e.g. SSH, SFTP, SCP). The value \"default\" implies all supported suites ordered from most secure to least secure.
  #[serde(rename = "tlsCipherSuiteSecureShellList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_secure_shell_list: Option<String>,
  /// The colon-separated list of supported cipher suites for TLS secure shell connections.
  #[serde(rename = "tlsCipherSuiteSecureShellSupportedList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_secure_shell_supported_list: Option<String>,
  /// Indicates whether protection against the CRIME exploit is enabled. When enabled, TLS+compressed messaging performance is degraded. This protection should only be disabled if sufficient ACL and authentication features are being employed such that a potential attacker does not have sufficient access to trigger the exploit.
  #[serde(rename = "tlsCrimeExploitProtectionEnabled", skip_serializing_if="Option::is_none")]
  tls_crime_exploit_protection_enabled: Option<bool>,
  /// The TLS ticket lifetime in seconds. When a client connects with TLS, a session with a session ticket is created using the TLS ticket lifetime which determines how long the client has to resume the session.
  #[serde(rename = "tlsTicketLifetime", skip_serializing_if="Option::is_none")]
  tls_ticket_lifetime: Option<i32>,
  /// The comma-separated list of supported TLS versions.
  #[serde(rename = "tlsVersionSupportedList", skip_serializing_if="Option::is_none")]
  tls_version_supported_list: Option<String>,
  /// The amount of messages transmitted to clients by the Broker, in bytes (B). Available since 2.14.
  #[serde(rename = "txByteCount", skip_serializing_if="Option::is_none")]
  tx_byte_count: Option<i64>,
  /// The current message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "txByteRate", skip_serializing_if="Option::is_none")]
  tx_byte_rate: Option<i64>,
  /// The amount of compressed messages transmitted by the Broker, in bytes (B). Available since 2.14.
  #[serde(rename = "txCompressedByteCount", skip_serializing_if="Option::is_none")]
  tx_compressed_byte_count: Option<i64>,
  /// The current compressed message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "txCompressedByteRate", skip_serializing_if="Option::is_none")]
  tx_compressed_byte_rate: Option<i64>,
  /// The compression ratio for messages transmitted by the Broker. Available since 2.14.
  #[serde(rename = "txCompressionRatio", skip_serializing_if="Option::is_none")]
  tx_compression_ratio: Option<String>,
  /// The number of messages transmitted to clients by the Broker. Available since 2.14.
  #[serde(rename = "txMsgCount", skip_serializing_if="Option::is_none")]
  tx_msg_count: Option<i64>,
  /// The current message rate transmitted by the Broker, in messages per second (msg/sec). Available since 2.14.
  #[serde(rename = "txMsgRate", skip_serializing_if="Option::is_none")]
  tx_msg_rate: Option<i64>,
  /// The amount of uncompressed messages transmitted by the Broker, in bytes (B). Available since 2.14.
  #[serde(rename = "txUncompressedByteCount", skip_serializing_if="Option::is_none")]
  tx_uncompressed_byte_count: Option<i64>,
  /// The current uncompressed message rate transmitted by the Broker, in bytes per second (B/sec). Available since 2.14.
  #[serde(rename = "txUncompressedByteRate", skip_serializing_if="Option::is_none")]
  tx_uncompressed_byte_rate: Option<i64>
}

impl Broker {
  pub fn new() -> Broker {
    Broker {
      auth_client_cert_revocation_check_mode: None,
      average_rx_byte_rate: None,
      average_rx_compressed_byte_rate: None,
      average_rx_msg_rate: None,
      average_rx_uncompressed_byte_rate: None,
      average_tx_byte_rate: None,
      average_tx_compressed_byte_rate: None,
      average_tx_msg_rate: None,
      average_tx_uncompressed_byte_rate: None,
      rx_byte_count: None,
      rx_byte_rate: None,
      rx_compressed_byte_count: None,
      rx_compressed_byte_rate: None,
      rx_compression_ratio: None,
      rx_msg_count: None,
      rx_msg_rate: None,
      rx_uncompressed_byte_count: None,
      rx_uncompressed_byte_rate: None,
      tls_block_version10_enabled: None,
      tls_block_version11_enabled: None,
      tls_cipher_suite_management_default_list: None,
      tls_cipher_suite_management_list: None,
      tls_cipher_suite_management_supported_list: None,
      tls_cipher_suite_msg_backbone_default_list: None,
      tls_cipher_suite_msg_backbone_list: None,
      tls_cipher_suite_msg_backbone_supported_list: None,
      tls_cipher_suite_secure_shell_default_list: None,
      tls_cipher_suite_secure_shell_list: None,
      tls_cipher_suite_secure_shell_supported_list: None,
      tls_crime_exploit_protection_enabled: None,
      tls_ticket_lifetime: None,
      tls_version_supported_list: None,
      tx_byte_count: None,
      tx_byte_rate: None,
      tx_compressed_byte_count: None,
      tx_compressed_byte_rate: None,
      tx_compression_ratio: None,
      tx_msg_count: None,
      tx_msg_rate: None,
      tx_uncompressed_byte_count: None,
      tx_uncompressed_byte_rate: None
    }
  }

  pub fn set_auth_client_cert_revocation_check_mode(&mut self, auth_client_cert_revocation_check_mode: String) {
    self.auth_client_cert_revocation_check_mode = Some(auth_client_cert_revocation_check_mode);
  }

  pub fn with_auth_client_cert_revocation_check_mode(mut self, auth_client_cert_revocation_check_mode: String) -> Broker {
    self.auth_client_cert_revocation_check_mode = Some(auth_client_cert_revocation_check_mode);
    self
  }

  pub fn auth_client_cert_revocation_check_mode(&self) -> Option<&String> {
    self.auth_client_cert_revocation_check_mode.as_ref()
  }

  pub fn reset_auth_client_cert_revocation_check_mode(&mut self) {
    self.auth_client_cert_revocation_check_mode = None;
  }

  pub fn set_average_rx_byte_rate(&mut self, average_rx_byte_rate: i64) {
    self.average_rx_byte_rate = Some(average_rx_byte_rate);
  }

  pub fn with_average_rx_byte_rate(mut self, average_rx_byte_rate: i64) -> Broker {
    self.average_rx_byte_rate = Some(average_rx_byte_rate);
    self
  }

  pub fn average_rx_byte_rate(&self) -> Option<&i64> {
    self.average_rx_byte_rate.as_ref()
  }

  pub fn reset_average_rx_byte_rate(&mut self) {
    self.average_rx_byte_rate = None;
  }

  pub fn set_average_rx_compressed_byte_rate(&mut self, average_rx_compressed_byte_rate: i64) {
    self.average_rx_compressed_byte_rate = Some(average_rx_compressed_byte_rate);
  }

  pub fn with_average_rx_compressed_byte_rate(mut self, average_rx_compressed_byte_rate: i64) -> Broker {
    self.average_rx_compressed_byte_rate = Some(average_rx_compressed_byte_rate);
    self
  }

  pub fn average_rx_compressed_byte_rate(&self) -> Option<&i64> {
    self.average_rx_compressed_byte_rate.as_ref()
  }

  pub fn reset_average_rx_compressed_byte_rate(&mut self) {
    self.average_rx_compressed_byte_rate = None;
  }

  pub fn set_average_rx_msg_rate(&mut self, average_rx_msg_rate: i64) {
    self.average_rx_msg_rate = Some(average_rx_msg_rate);
  }

  pub fn with_average_rx_msg_rate(mut self, average_rx_msg_rate: i64) -> Broker {
    self.average_rx_msg_rate = Some(average_rx_msg_rate);
    self
  }

  pub fn average_rx_msg_rate(&self) -> Option<&i64> {
    self.average_rx_msg_rate.as_ref()
  }

  pub fn reset_average_rx_msg_rate(&mut self) {
    self.average_rx_msg_rate = None;
  }

  pub fn set_average_rx_uncompressed_byte_rate(&mut self, average_rx_uncompressed_byte_rate: i64) {
    self.average_rx_uncompressed_byte_rate = Some(average_rx_uncompressed_byte_rate);
  }

  pub fn with_average_rx_uncompressed_byte_rate(mut self, average_rx_uncompressed_byte_rate: i64) -> Broker {
    self.average_rx_uncompressed_byte_rate = Some(average_rx_uncompressed_byte_rate);
    self
  }

  pub fn average_rx_uncompressed_byte_rate(&self) -> Option<&i64> {
    self.average_rx_uncompressed_byte_rate.as_ref()
  }

  pub fn reset_average_rx_uncompressed_byte_rate(&mut self) {
    self.average_rx_uncompressed_byte_rate = None;
  }

  pub fn set_average_tx_byte_rate(&mut self, average_tx_byte_rate: i64) {
    self.average_tx_byte_rate = Some(average_tx_byte_rate);
  }

  pub fn with_average_tx_byte_rate(mut self, average_tx_byte_rate: i64) -> Broker {
    self.average_tx_byte_rate = Some(average_tx_byte_rate);
    self
  }

  pub fn average_tx_byte_rate(&self) -> Option<&i64> {
    self.average_tx_byte_rate.as_ref()
  }

  pub fn reset_average_tx_byte_rate(&mut self) {
    self.average_tx_byte_rate = None;
  }

  pub fn set_average_tx_compressed_byte_rate(&mut self, average_tx_compressed_byte_rate: i64) {
    self.average_tx_compressed_byte_rate = Some(average_tx_compressed_byte_rate);
  }

  pub fn with_average_tx_compressed_byte_rate(mut self, average_tx_compressed_byte_rate: i64) -> Broker {
    self.average_tx_compressed_byte_rate = Some(average_tx_compressed_byte_rate);
    self
  }

  pub fn average_tx_compressed_byte_rate(&self) -> Option<&i64> {
    self.average_tx_compressed_byte_rate.as_ref()
  }

  pub fn reset_average_tx_compressed_byte_rate(&mut self) {
    self.average_tx_compressed_byte_rate = None;
  }

  pub fn set_average_tx_msg_rate(&mut self, average_tx_msg_rate: i64) {
    self.average_tx_msg_rate = Some(average_tx_msg_rate);
  }

  pub fn with_average_tx_msg_rate(mut self, average_tx_msg_rate: i64) -> Broker {
    self.average_tx_msg_rate = Some(average_tx_msg_rate);
    self
  }

  pub fn average_tx_msg_rate(&self) -> Option<&i64> {
    self.average_tx_msg_rate.as_ref()
  }

  pub fn reset_average_tx_msg_rate(&mut self) {
    self.average_tx_msg_rate = None;
  }

  pub fn set_average_tx_uncompressed_byte_rate(&mut self, average_tx_uncompressed_byte_rate: i64) {
    self.average_tx_uncompressed_byte_rate = Some(average_tx_uncompressed_byte_rate);
  }

  pub fn with_average_tx_uncompressed_byte_rate(mut self, average_tx_uncompressed_byte_rate: i64) -> Broker {
    self.average_tx_uncompressed_byte_rate = Some(average_tx_uncompressed_byte_rate);
    self
  }

  pub fn average_tx_uncompressed_byte_rate(&self) -> Option<&i64> {
    self.average_tx_uncompressed_byte_rate.as_ref()
  }

  pub fn reset_average_tx_uncompressed_byte_rate(&mut self) {
    self.average_tx_uncompressed_byte_rate = None;
  }

  pub fn set_rx_byte_count(&mut self, rx_byte_count: i64) {
    self.rx_byte_count = Some(rx_byte_count);
  }

  pub fn with_rx_byte_count(mut self, rx_byte_count: i64) -> Broker {
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

  pub fn with_rx_byte_rate(mut self, rx_byte_rate: i64) -> Broker {
    self.rx_byte_rate = Some(rx_byte_rate);
    self
  }

  pub fn rx_byte_rate(&self) -> Option<&i64> {
    self.rx_byte_rate.as_ref()
  }

  pub fn reset_rx_byte_rate(&mut self) {
    self.rx_byte_rate = None;
  }

  pub fn set_rx_compressed_byte_count(&mut self, rx_compressed_byte_count: i64) {
    self.rx_compressed_byte_count = Some(rx_compressed_byte_count);
  }

  pub fn with_rx_compressed_byte_count(mut self, rx_compressed_byte_count: i64) -> Broker {
    self.rx_compressed_byte_count = Some(rx_compressed_byte_count);
    self
  }

  pub fn rx_compressed_byte_count(&self) -> Option<&i64> {
    self.rx_compressed_byte_count.as_ref()
  }

  pub fn reset_rx_compressed_byte_count(&mut self) {
    self.rx_compressed_byte_count = None;
  }

  pub fn set_rx_compressed_byte_rate(&mut self, rx_compressed_byte_rate: i64) {
    self.rx_compressed_byte_rate = Some(rx_compressed_byte_rate);
  }

  pub fn with_rx_compressed_byte_rate(mut self, rx_compressed_byte_rate: i64) -> Broker {
    self.rx_compressed_byte_rate = Some(rx_compressed_byte_rate);
    self
  }

  pub fn rx_compressed_byte_rate(&self) -> Option<&i64> {
    self.rx_compressed_byte_rate.as_ref()
  }

  pub fn reset_rx_compressed_byte_rate(&mut self) {
    self.rx_compressed_byte_rate = None;
  }

  pub fn set_rx_compression_ratio(&mut self, rx_compression_ratio: String) {
    self.rx_compression_ratio = Some(rx_compression_ratio);
  }

  pub fn with_rx_compression_ratio(mut self, rx_compression_ratio: String) -> Broker {
    self.rx_compression_ratio = Some(rx_compression_ratio);
    self
  }

  pub fn rx_compression_ratio(&self) -> Option<&String> {
    self.rx_compression_ratio.as_ref()
  }

  pub fn reset_rx_compression_ratio(&mut self) {
    self.rx_compression_ratio = None;
  }

  pub fn set_rx_msg_count(&mut self, rx_msg_count: i64) {
    self.rx_msg_count = Some(rx_msg_count);
  }

  pub fn with_rx_msg_count(mut self, rx_msg_count: i64) -> Broker {
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

  pub fn with_rx_msg_rate(mut self, rx_msg_rate: i64) -> Broker {
    self.rx_msg_rate = Some(rx_msg_rate);
    self
  }

  pub fn rx_msg_rate(&self) -> Option<&i64> {
    self.rx_msg_rate.as_ref()
  }

  pub fn reset_rx_msg_rate(&mut self) {
    self.rx_msg_rate = None;
  }

  pub fn set_rx_uncompressed_byte_count(&mut self, rx_uncompressed_byte_count: i64) {
    self.rx_uncompressed_byte_count = Some(rx_uncompressed_byte_count);
  }

  pub fn with_rx_uncompressed_byte_count(mut self, rx_uncompressed_byte_count: i64) -> Broker {
    self.rx_uncompressed_byte_count = Some(rx_uncompressed_byte_count);
    self
  }

  pub fn rx_uncompressed_byte_count(&self) -> Option<&i64> {
    self.rx_uncompressed_byte_count.as_ref()
  }

  pub fn reset_rx_uncompressed_byte_count(&mut self) {
    self.rx_uncompressed_byte_count = None;
  }

  pub fn set_rx_uncompressed_byte_rate(&mut self, rx_uncompressed_byte_rate: i64) {
    self.rx_uncompressed_byte_rate = Some(rx_uncompressed_byte_rate);
  }

  pub fn with_rx_uncompressed_byte_rate(mut self, rx_uncompressed_byte_rate: i64) -> Broker {
    self.rx_uncompressed_byte_rate = Some(rx_uncompressed_byte_rate);
    self
  }

  pub fn rx_uncompressed_byte_rate(&self) -> Option<&i64> {
    self.rx_uncompressed_byte_rate.as_ref()
  }

  pub fn reset_rx_uncompressed_byte_rate(&mut self) {
    self.rx_uncompressed_byte_rate = None;
  }

  pub fn set_tls_block_version10_enabled(&mut self, tls_block_version10_enabled: bool) {
    self.tls_block_version10_enabled = Some(tls_block_version10_enabled);
  }

  pub fn with_tls_block_version10_enabled(mut self, tls_block_version10_enabled: bool) -> Broker {
    self.tls_block_version10_enabled = Some(tls_block_version10_enabled);
    self
  }

  pub fn tls_block_version10_enabled(&self) -> Option<&bool> {
    self.tls_block_version10_enabled.as_ref()
  }

  pub fn reset_tls_block_version10_enabled(&mut self) {
    self.tls_block_version10_enabled = None;
  }

  pub fn set_tls_block_version11_enabled(&mut self, tls_block_version11_enabled: bool) {
    self.tls_block_version11_enabled = Some(tls_block_version11_enabled);
  }

  pub fn with_tls_block_version11_enabled(mut self, tls_block_version11_enabled: bool) -> Broker {
    self.tls_block_version11_enabled = Some(tls_block_version11_enabled);
    self
  }

  pub fn tls_block_version11_enabled(&self) -> Option<&bool> {
    self.tls_block_version11_enabled.as_ref()
  }

  pub fn reset_tls_block_version11_enabled(&mut self) {
    self.tls_block_version11_enabled = None;
  }

  pub fn set_tls_cipher_suite_management_default_list(&mut self, tls_cipher_suite_management_default_list: String) {
    self.tls_cipher_suite_management_default_list = Some(tls_cipher_suite_management_default_list);
  }

  pub fn with_tls_cipher_suite_management_default_list(mut self, tls_cipher_suite_management_default_list: String) -> Broker {
    self.tls_cipher_suite_management_default_list = Some(tls_cipher_suite_management_default_list);
    self
  }

  pub fn tls_cipher_suite_management_default_list(&self) -> Option<&String> {
    self.tls_cipher_suite_management_default_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_management_default_list(&mut self) {
    self.tls_cipher_suite_management_default_list = None;
  }

  pub fn set_tls_cipher_suite_management_list(&mut self, tls_cipher_suite_management_list: String) {
    self.tls_cipher_suite_management_list = Some(tls_cipher_suite_management_list);
  }

  pub fn with_tls_cipher_suite_management_list(mut self, tls_cipher_suite_management_list: String) -> Broker {
    self.tls_cipher_suite_management_list = Some(tls_cipher_suite_management_list);
    self
  }

  pub fn tls_cipher_suite_management_list(&self) -> Option<&String> {
    self.tls_cipher_suite_management_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_management_list(&mut self) {
    self.tls_cipher_suite_management_list = None;
  }

  pub fn set_tls_cipher_suite_management_supported_list(&mut self, tls_cipher_suite_management_supported_list: String) {
    self.tls_cipher_suite_management_supported_list = Some(tls_cipher_suite_management_supported_list);
  }

  pub fn with_tls_cipher_suite_management_supported_list(mut self, tls_cipher_suite_management_supported_list: String) -> Broker {
    self.tls_cipher_suite_management_supported_list = Some(tls_cipher_suite_management_supported_list);
    self
  }

  pub fn tls_cipher_suite_management_supported_list(&self) -> Option<&String> {
    self.tls_cipher_suite_management_supported_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_management_supported_list(&mut self) {
    self.tls_cipher_suite_management_supported_list = None;
  }

  pub fn set_tls_cipher_suite_msg_backbone_default_list(&mut self, tls_cipher_suite_msg_backbone_default_list: String) {
    self.tls_cipher_suite_msg_backbone_default_list = Some(tls_cipher_suite_msg_backbone_default_list);
  }

  pub fn with_tls_cipher_suite_msg_backbone_default_list(mut self, tls_cipher_suite_msg_backbone_default_list: String) -> Broker {
    self.tls_cipher_suite_msg_backbone_default_list = Some(tls_cipher_suite_msg_backbone_default_list);
    self
  }

  pub fn tls_cipher_suite_msg_backbone_default_list(&self) -> Option<&String> {
    self.tls_cipher_suite_msg_backbone_default_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_msg_backbone_default_list(&mut self) {
    self.tls_cipher_suite_msg_backbone_default_list = None;
  }

  pub fn set_tls_cipher_suite_msg_backbone_list(&mut self, tls_cipher_suite_msg_backbone_list: String) {
    self.tls_cipher_suite_msg_backbone_list = Some(tls_cipher_suite_msg_backbone_list);
  }

  pub fn with_tls_cipher_suite_msg_backbone_list(mut self, tls_cipher_suite_msg_backbone_list: String) -> Broker {
    self.tls_cipher_suite_msg_backbone_list = Some(tls_cipher_suite_msg_backbone_list);
    self
  }

  pub fn tls_cipher_suite_msg_backbone_list(&self) -> Option<&String> {
    self.tls_cipher_suite_msg_backbone_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_msg_backbone_list(&mut self) {
    self.tls_cipher_suite_msg_backbone_list = None;
  }

  pub fn set_tls_cipher_suite_msg_backbone_supported_list(&mut self, tls_cipher_suite_msg_backbone_supported_list: String) {
    self.tls_cipher_suite_msg_backbone_supported_list = Some(tls_cipher_suite_msg_backbone_supported_list);
  }

  pub fn with_tls_cipher_suite_msg_backbone_supported_list(mut self, tls_cipher_suite_msg_backbone_supported_list: String) -> Broker {
    self.tls_cipher_suite_msg_backbone_supported_list = Some(tls_cipher_suite_msg_backbone_supported_list);
    self
  }

  pub fn tls_cipher_suite_msg_backbone_supported_list(&self) -> Option<&String> {
    self.tls_cipher_suite_msg_backbone_supported_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_msg_backbone_supported_list(&mut self) {
    self.tls_cipher_suite_msg_backbone_supported_list = None;
  }

  pub fn set_tls_cipher_suite_secure_shell_default_list(&mut self, tls_cipher_suite_secure_shell_default_list: String) {
    self.tls_cipher_suite_secure_shell_default_list = Some(tls_cipher_suite_secure_shell_default_list);
  }

  pub fn with_tls_cipher_suite_secure_shell_default_list(mut self, tls_cipher_suite_secure_shell_default_list: String) -> Broker {
    self.tls_cipher_suite_secure_shell_default_list = Some(tls_cipher_suite_secure_shell_default_list);
    self
  }

  pub fn tls_cipher_suite_secure_shell_default_list(&self) -> Option<&String> {
    self.tls_cipher_suite_secure_shell_default_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_secure_shell_default_list(&mut self) {
    self.tls_cipher_suite_secure_shell_default_list = None;
  }

  pub fn set_tls_cipher_suite_secure_shell_list(&mut self, tls_cipher_suite_secure_shell_list: String) {
    self.tls_cipher_suite_secure_shell_list = Some(tls_cipher_suite_secure_shell_list);
  }

  pub fn with_tls_cipher_suite_secure_shell_list(mut self, tls_cipher_suite_secure_shell_list: String) -> Broker {
    self.tls_cipher_suite_secure_shell_list = Some(tls_cipher_suite_secure_shell_list);
    self
  }

  pub fn tls_cipher_suite_secure_shell_list(&self) -> Option<&String> {
    self.tls_cipher_suite_secure_shell_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_secure_shell_list(&mut self) {
    self.tls_cipher_suite_secure_shell_list = None;
  }

  pub fn set_tls_cipher_suite_secure_shell_supported_list(&mut self, tls_cipher_suite_secure_shell_supported_list: String) {
    self.tls_cipher_suite_secure_shell_supported_list = Some(tls_cipher_suite_secure_shell_supported_list);
  }

  pub fn with_tls_cipher_suite_secure_shell_supported_list(mut self, tls_cipher_suite_secure_shell_supported_list: String) -> Broker {
    self.tls_cipher_suite_secure_shell_supported_list = Some(tls_cipher_suite_secure_shell_supported_list);
    self
  }

  pub fn tls_cipher_suite_secure_shell_supported_list(&self) -> Option<&String> {
    self.tls_cipher_suite_secure_shell_supported_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_secure_shell_supported_list(&mut self) {
    self.tls_cipher_suite_secure_shell_supported_list = None;
  }

  pub fn set_tls_crime_exploit_protection_enabled(&mut self, tls_crime_exploit_protection_enabled: bool) {
    self.tls_crime_exploit_protection_enabled = Some(tls_crime_exploit_protection_enabled);
  }

  pub fn with_tls_crime_exploit_protection_enabled(mut self, tls_crime_exploit_protection_enabled: bool) -> Broker {
    self.tls_crime_exploit_protection_enabled = Some(tls_crime_exploit_protection_enabled);
    self
  }

  pub fn tls_crime_exploit_protection_enabled(&self) -> Option<&bool> {
    self.tls_crime_exploit_protection_enabled.as_ref()
  }

  pub fn reset_tls_crime_exploit_protection_enabled(&mut self) {
    self.tls_crime_exploit_protection_enabled = None;
  }

  pub fn set_tls_ticket_lifetime(&mut self, tls_ticket_lifetime: i32) {
    self.tls_ticket_lifetime = Some(tls_ticket_lifetime);
  }

  pub fn with_tls_ticket_lifetime(mut self, tls_ticket_lifetime: i32) -> Broker {
    self.tls_ticket_lifetime = Some(tls_ticket_lifetime);
    self
  }

  pub fn tls_ticket_lifetime(&self) -> Option<&i32> {
    self.tls_ticket_lifetime.as_ref()
  }

  pub fn reset_tls_ticket_lifetime(&mut self) {
    self.tls_ticket_lifetime = None;
  }

  pub fn set_tls_version_supported_list(&mut self, tls_version_supported_list: String) {
    self.tls_version_supported_list = Some(tls_version_supported_list);
  }

  pub fn with_tls_version_supported_list(mut self, tls_version_supported_list: String) -> Broker {
    self.tls_version_supported_list = Some(tls_version_supported_list);
    self
  }

  pub fn tls_version_supported_list(&self) -> Option<&String> {
    self.tls_version_supported_list.as_ref()
  }

  pub fn reset_tls_version_supported_list(&mut self) {
    self.tls_version_supported_list = None;
  }

  pub fn set_tx_byte_count(&mut self, tx_byte_count: i64) {
    self.tx_byte_count = Some(tx_byte_count);
  }

  pub fn with_tx_byte_count(mut self, tx_byte_count: i64) -> Broker {
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

  pub fn with_tx_byte_rate(mut self, tx_byte_rate: i64) -> Broker {
    self.tx_byte_rate = Some(tx_byte_rate);
    self
  }

  pub fn tx_byte_rate(&self) -> Option<&i64> {
    self.tx_byte_rate.as_ref()
  }

  pub fn reset_tx_byte_rate(&mut self) {
    self.tx_byte_rate = None;
  }

  pub fn set_tx_compressed_byte_count(&mut self, tx_compressed_byte_count: i64) {
    self.tx_compressed_byte_count = Some(tx_compressed_byte_count);
  }

  pub fn with_tx_compressed_byte_count(mut self, tx_compressed_byte_count: i64) -> Broker {
    self.tx_compressed_byte_count = Some(tx_compressed_byte_count);
    self
  }

  pub fn tx_compressed_byte_count(&self) -> Option<&i64> {
    self.tx_compressed_byte_count.as_ref()
  }

  pub fn reset_tx_compressed_byte_count(&mut self) {
    self.tx_compressed_byte_count = None;
  }

  pub fn set_tx_compressed_byte_rate(&mut self, tx_compressed_byte_rate: i64) {
    self.tx_compressed_byte_rate = Some(tx_compressed_byte_rate);
  }

  pub fn with_tx_compressed_byte_rate(mut self, tx_compressed_byte_rate: i64) -> Broker {
    self.tx_compressed_byte_rate = Some(tx_compressed_byte_rate);
    self
  }

  pub fn tx_compressed_byte_rate(&self) -> Option<&i64> {
    self.tx_compressed_byte_rate.as_ref()
  }

  pub fn reset_tx_compressed_byte_rate(&mut self) {
    self.tx_compressed_byte_rate = None;
  }

  pub fn set_tx_compression_ratio(&mut self, tx_compression_ratio: String) {
    self.tx_compression_ratio = Some(tx_compression_ratio);
  }

  pub fn with_tx_compression_ratio(mut self, tx_compression_ratio: String) -> Broker {
    self.tx_compression_ratio = Some(tx_compression_ratio);
    self
  }

  pub fn tx_compression_ratio(&self) -> Option<&String> {
    self.tx_compression_ratio.as_ref()
  }

  pub fn reset_tx_compression_ratio(&mut self) {
    self.tx_compression_ratio = None;
  }

  pub fn set_tx_msg_count(&mut self, tx_msg_count: i64) {
    self.tx_msg_count = Some(tx_msg_count);
  }

  pub fn with_tx_msg_count(mut self, tx_msg_count: i64) -> Broker {
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

  pub fn with_tx_msg_rate(mut self, tx_msg_rate: i64) -> Broker {
    self.tx_msg_rate = Some(tx_msg_rate);
    self
  }

  pub fn tx_msg_rate(&self) -> Option<&i64> {
    self.tx_msg_rate.as_ref()
  }

  pub fn reset_tx_msg_rate(&mut self) {
    self.tx_msg_rate = None;
  }

  pub fn set_tx_uncompressed_byte_count(&mut self, tx_uncompressed_byte_count: i64) {
    self.tx_uncompressed_byte_count = Some(tx_uncompressed_byte_count);
  }

  pub fn with_tx_uncompressed_byte_count(mut self, tx_uncompressed_byte_count: i64) -> Broker {
    self.tx_uncompressed_byte_count = Some(tx_uncompressed_byte_count);
    self
  }

  pub fn tx_uncompressed_byte_count(&self) -> Option<&i64> {
    self.tx_uncompressed_byte_count.as_ref()
  }

  pub fn reset_tx_uncompressed_byte_count(&mut self) {
    self.tx_uncompressed_byte_count = None;
  }

  pub fn set_tx_uncompressed_byte_rate(&mut self, tx_uncompressed_byte_rate: i64) {
    self.tx_uncompressed_byte_rate = Some(tx_uncompressed_byte_rate);
  }

  pub fn with_tx_uncompressed_byte_rate(mut self, tx_uncompressed_byte_rate: i64) -> Broker {
    self.tx_uncompressed_byte_rate = Some(tx_uncompressed_byte_rate);
    self
  }

  pub fn tx_uncompressed_byte_rate(&self) -> Option<&i64> {
    self.tx_uncompressed_byte_rate.as_ref()
  }

  pub fn reset_tx_uncompressed_byte_rate(&mut self) {
    self.tx_uncompressed_byte_rate = None;
  }

}



