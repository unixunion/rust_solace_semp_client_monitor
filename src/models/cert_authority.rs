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
pub struct CertAuthority {
  /// The name of the Certificate Authority.
  #[serde(rename = "certAuthorityName", skip_serializing_if="Option::is_none")]
  cert_authority_name: Option<String>,
  /// The scheduled CRL refresh day(s), specified as \"daily\" or a comma-separated list of days. Days must be specified as \"Sun\", \"Mon\", \"Tue\", \"Wed\", \"Thu\", \"Fri\", or \"Sat\", with no spaces, and in sorted order from Sunday to Saturday.
  #[serde(rename = "crlDayList", skip_serializing_if="Option::is_none")]
  crl_day_list: Option<String>,
  /// The timestamp of the last successful CRL download. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "crlLastDownloadTime", skip_serializing_if="Option::is_none")]
  crl_last_download_time: Option<i32>,
  /// The reason for the last CRL failure.
  #[serde(rename = "crlLastFailureReason", skip_serializing_if="Option::is_none")]
  crl_last_failure_reason: Option<String>,
  /// The timestamp of the last CRL failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "crlLastFailureTime", skip_serializing_if="Option::is_none")]
  crl_last_failure_time: Option<i32>,
  /// The scheduled time of the next CRL download. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "crlNextDownloadTime", skip_serializing_if="Option::is_none")]
  crl_next_download_time: Option<i32>,
  /// The scheduled CRL refresh time(s), specified as \"hourly\" or a comma-separated list of 24-hour times in the form hh:mm, or h:mm. There must be no spaces, and times must be in sorted order from 0:00 to 23:59.
  #[serde(rename = "crlTimeList", skip_serializing_if="Option::is_none")]
  crl_time_list: Option<String>,
  /// Indicates whether CRL revocation checking is operationally up.
  #[serde(rename = "crlUp", skip_serializing_if="Option::is_none")]
  crl_up: Option<bool>,
  /// The URL for the CRL source. This is a required attribute for CRL to be operational and the URL must be complete with http:// included.
  #[serde(rename = "crlUrl", skip_serializing_if="Option::is_none")]
  crl_url: Option<String>,
  /// The reason for the last OCSP failure.
  #[serde(rename = "ocspLastFailureReason", skip_serializing_if="Option::is_none")]
  ocsp_last_failure_reason: Option<String>,
  /// The timestamp of the last OCSP failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "ocspLastFailureTime", skip_serializing_if="Option::is_none")]
  ocsp_last_failure_time: Option<i32>,
  /// The URL involved in the last OCSP failure.
  #[serde(rename = "ocspLastFailureUrl", skip_serializing_if="Option::is_none")]
  ocsp_last_failure_url: Option<String>,
  /// Indicates whether a non-responder certificate is allowed to sign an OCSP response. Typically used with an OCSP override URL in cases where a single certificate is used to sign client certificates and OCSP responses.
  #[serde(rename = "ocspNonResponderCertEnabled", skip_serializing_if="Option::is_none")]
  ocsp_non_responder_cert_enabled: Option<bool>,
  /// The OCSP responder URL to use for overriding the one supplied in the client certificate. The URL must be complete with http:// included.
  #[serde(rename = "ocspOverrideUrl", skip_serializing_if="Option::is_none")]
  ocsp_override_url: Option<String>,
  /// The timeout in seconds to receive a response from the OCSP responder after sending a request or making the initial connection attempt.
  #[serde(rename = "ocspTimeout", skip_serializing_if="Option::is_none")]
  ocsp_timeout: Option<i64>,
  /// Indicates whether Certificate Authority revocation checking is enabled.
  #[serde(rename = "revocationCheckEnabled", skip_serializing_if="Option::is_none")]
  revocation_check_enabled: Option<bool>
}

impl CertAuthority {
  pub fn new() -> CertAuthority {
    CertAuthority {
      cert_authority_name: None,
      crl_day_list: None,
      crl_last_download_time: None,
      crl_last_failure_reason: None,
      crl_last_failure_time: None,
      crl_next_download_time: None,
      crl_time_list: None,
      crl_up: None,
      crl_url: None,
      ocsp_last_failure_reason: None,
      ocsp_last_failure_time: None,
      ocsp_last_failure_url: None,
      ocsp_non_responder_cert_enabled: None,
      ocsp_override_url: None,
      ocsp_timeout: None,
      revocation_check_enabled: None
    }
  }

  pub fn set_cert_authority_name(&mut self, cert_authority_name: String) {
    self.cert_authority_name = Some(cert_authority_name);
  }

  pub fn with_cert_authority_name(mut self, cert_authority_name: String) -> CertAuthority {
    self.cert_authority_name = Some(cert_authority_name);
    self
  }

  pub fn cert_authority_name(&self) -> Option<&String> {
    self.cert_authority_name.as_ref()
  }

  pub fn reset_cert_authority_name(&mut self) {
    self.cert_authority_name = None;
  }

  pub fn set_crl_day_list(&mut self, crl_day_list: String) {
    self.crl_day_list = Some(crl_day_list);
  }

  pub fn with_crl_day_list(mut self, crl_day_list: String) -> CertAuthority {
    self.crl_day_list = Some(crl_day_list);
    self
  }

  pub fn crl_day_list(&self) -> Option<&String> {
    self.crl_day_list.as_ref()
  }

  pub fn reset_crl_day_list(&mut self) {
    self.crl_day_list = None;
  }

  pub fn set_crl_last_download_time(&mut self, crl_last_download_time: i32) {
    self.crl_last_download_time = Some(crl_last_download_time);
  }

  pub fn with_crl_last_download_time(mut self, crl_last_download_time: i32) -> CertAuthority {
    self.crl_last_download_time = Some(crl_last_download_time);
    self
  }

  pub fn crl_last_download_time(&self) -> Option<&i32> {
    self.crl_last_download_time.as_ref()
  }

  pub fn reset_crl_last_download_time(&mut self) {
    self.crl_last_download_time = None;
  }

  pub fn set_crl_last_failure_reason(&mut self, crl_last_failure_reason: String) {
    self.crl_last_failure_reason = Some(crl_last_failure_reason);
  }

  pub fn with_crl_last_failure_reason(mut self, crl_last_failure_reason: String) -> CertAuthority {
    self.crl_last_failure_reason = Some(crl_last_failure_reason);
    self
  }

  pub fn crl_last_failure_reason(&self) -> Option<&String> {
    self.crl_last_failure_reason.as_ref()
  }

  pub fn reset_crl_last_failure_reason(&mut self) {
    self.crl_last_failure_reason = None;
  }

  pub fn set_crl_last_failure_time(&mut self, crl_last_failure_time: i32) {
    self.crl_last_failure_time = Some(crl_last_failure_time);
  }

  pub fn with_crl_last_failure_time(mut self, crl_last_failure_time: i32) -> CertAuthority {
    self.crl_last_failure_time = Some(crl_last_failure_time);
    self
  }

  pub fn crl_last_failure_time(&self) -> Option<&i32> {
    self.crl_last_failure_time.as_ref()
  }

  pub fn reset_crl_last_failure_time(&mut self) {
    self.crl_last_failure_time = None;
  }

  pub fn set_crl_next_download_time(&mut self, crl_next_download_time: i32) {
    self.crl_next_download_time = Some(crl_next_download_time);
  }

  pub fn with_crl_next_download_time(mut self, crl_next_download_time: i32) -> CertAuthority {
    self.crl_next_download_time = Some(crl_next_download_time);
    self
  }

  pub fn crl_next_download_time(&self) -> Option<&i32> {
    self.crl_next_download_time.as_ref()
  }

  pub fn reset_crl_next_download_time(&mut self) {
    self.crl_next_download_time = None;
  }

  pub fn set_crl_time_list(&mut self, crl_time_list: String) {
    self.crl_time_list = Some(crl_time_list);
  }

  pub fn with_crl_time_list(mut self, crl_time_list: String) -> CertAuthority {
    self.crl_time_list = Some(crl_time_list);
    self
  }

  pub fn crl_time_list(&self) -> Option<&String> {
    self.crl_time_list.as_ref()
  }

  pub fn reset_crl_time_list(&mut self) {
    self.crl_time_list = None;
  }

  pub fn set_crl_up(&mut self, crl_up: bool) {
    self.crl_up = Some(crl_up);
  }

  pub fn with_crl_up(mut self, crl_up: bool) -> CertAuthority {
    self.crl_up = Some(crl_up);
    self
  }

  pub fn crl_up(&self) -> Option<&bool> {
    self.crl_up.as_ref()
  }

  pub fn reset_crl_up(&mut self) {
    self.crl_up = None;
  }

  pub fn set_crl_url(&mut self, crl_url: String) {
    self.crl_url = Some(crl_url);
  }

  pub fn with_crl_url(mut self, crl_url: String) -> CertAuthority {
    self.crl_url = Some(crl_url);
    self
  }

  pub fn crl_url(&self) -> Option<&String> {
    self.crl_url.as_ref()
  }

  pub fn reset_crl_url(&mut self) {
    self.crl_url = None;
  }

  pub fn set_ocsp_last_failure_reason(&mut self, ocsp_last_failure_reason: String) {
    self.ocsp_last_failure_reason = Some(ocsp_last_failure_reason);
  }

  pub fn with_ocsp_last_failure_reason(mut self, ocsp_last_failure_reason: String) -> CertAuthority {
    self.ocsp_last_failure_reason = Some(ocsp_last_failure_reason);
    self
  }

  pub fn ocsp_last_failure_reason(&self) -> Option<&String> {
    self.ocsp_last_failure_reason.as_ref()
  }

  pub fn reset_ocsp_last_failure_reason(&mut self) {
    self.ocsp_last_failure_reason = None;
  }

  pub fn set_ocsp_last_failure_time(&mut self, ocsp_last_failure_time: i32) {
    self.ocsp_last_failure_time = Some(ocsp_last_failure_time);
  }

  pub fn with_ocsp_last_failure_time(mut self, ocsp_last_failure_time: i32) -> CertAuthority {
    self.ocsp_last_failure_time = Some(ocsp_last_failure_time);
    self
  }

  pub fn ocsp_last_failure_time(&self) -> Option<&i32> {
    self.ocsp_last_failure_time.as_ref()
  }

  pub fn reset_ocsp_last_failure_time(&mut self) {
    self.ocsp_last_failure_time = None;
  }

  pub fn set_ocsp_last_failure_url(&mut self, ocsp_last_failure_url: String) {
    self.ocsp_last_failure_url = Some(ocsp_last_failure_url);
  }

  pub fn with_ocsp_last_failure_url(mut self, ocsp_last_failure_url: String) -> CertAuthority {
    self.ocsp_last_failure_url = Some(ocsp_last_failure_url);
    self
  }

  pub fn ocsp_last_failure_url(&self) -> Option<&String> {
    self.ocsp_last_failure_url.as_ref()
  }

  pub fn reset_ocsp_last_failure_url(&mut self) {
    self.ocsp_last_failure_url = None;
  }

  pub fn set_ocsp_non_responder_cert_enabled(&mut self, ocsp_non_responder_cert_enabled: bool) {
    self.ocsp_non_responder_cert_enabled = Some(ocsp_non_responder_cert_enabled);
  }

  pub fn with_ocsp_non_responder_cert_enabled(mut self, ocsp_non_responder_cert_enabled: bool) -> CertAuthority {
    self.ocsp_non_responder_cert_enabled = Some(ocsp_non_responder_cert_enabled);
    self
  }

  pub fn ocsp_non_responder_cert_enabled(&self) -> Option<&bool> {
    self.ocsp_non_responder_cert_enabled.as_ref()
  }

  pub fn reset_ocsp_non_responder_cert_enabled(&mut self) {
    self.ocsp_non_responder_cert_enabled = None;
  }

  pub fn set_ocsp_override_url(&mut self, ocsp_override_url: String) {
    self.ocsp_override_url = Some(ocsp_override_url);
  }

  pub fn with_ocsp_override_url(mut self, ocsp_override_url: String) -> CertAuthority {
    self.ocsp_override_url = Some(ocsp_override_url);
    self
  }

  pub fn ocsp_override_url(&self) -> Option<&String> {
    self.ocsp_override_url.as_ref()
  }

  pub fn reset_ocsp_override_url(&mut self) {
    self.ocsp_override_url = None;
  }

  pub fn set_ocsp_timeout(&mut self, ocsp_timeout: i64) {
    self.ocsp_timeout = Some(ocsp_timeout);
  }

  pub fn with_ocsp_timeout(mut self, ocsp_timeout: i64) -> CertAuthority {
    self.ocsp_timeout = Some(ocsp_timeout);
    self
  }

  pub fn ocsp_timeout(&self) -> Option<&i64> {
    self.ocsp_timeout.as_ref()
  }

  pub fn reset_ocsp_timeout(&mut self) {
    self.ocsp_timeout = None;
  }

  pub fn set_revocation_check_enabled(&mut self, revocation_check_enabled: bool) {
    self.revocation_check_enabled = Some(revocation_check_enabled);
  }

  pub fn with_revocation_check_enabled(mut self, revocation_check_enabled: bool) -> CertAuthority {
    self.revocation_check_enabled = Some(revocation_check_enabled);
    self
  }

  pub fn revocation_check_enabled(&self) -> Option<&bool> {
    self.revocation_check_enabled.as_ref()
  }

  pub fn reset_revocation_check_enabled(&mut self) {
    self.revocation_check_enabled = None;
  }

}



