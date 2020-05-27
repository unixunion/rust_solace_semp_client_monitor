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
pub struct MsgVpnAuthenticationOauthProvider {
  /// The audience claim name, indicating which part of the object to use for determining the audience.
  #[serde(rename = "audienceClaimName", skip_serializing_if="Option::is_none")]
  audience_claim_name: Option<String>,
  /// The audience claim source, indicating where to search for the audience value. The allowed values and their meaning are:  <pre> \"access-token\" - The OAuth v2 access_token. \"id-token\" - The OpenID Connect id_token. \"introspection\" - The result of introspecting the OAuth v2 access_token. </pre> 
  #[serde(rename = "audienceClaimSource", skip_serializing_if="Option::is_none")]
  audience_claim_source: Option<String>,
  /// The required audience value for a token to be considered valid.
  #[serde(rename = "audienceClaimValue", skip_serializing_if="Option::is_none")]
  audience_claim_value: Option<String>,
  /// Indicates whether audience validation is enabled.
  #[serde(rename = "audienceValidationEnabled", skip_serializing_if="Option::is_none")]
  audience_validation_enabled: Option<bool>,
  /// The number of OAuth Provider client authentications that succeeded.
  #[serde(rename = "authenticationSuccessCount", skip_serializing_if="Option::is_none")]
  authentication_success_count: Option<i64>,
  /// The authorization group claim name, indicating which part of the object to use for determining the authorization group.
  #[serde(rename = "authorizationGroupClaimName", skip_serializing_if="Option::is_none")]
  authorization_group_claim_name: Option<String>,
  /// The authorization group claim source, indicating where to search for the authorization group name. The allowed values and their meaning are:  <pre> \"access-token\" - The OAuth v2 access_token. \"id-token\" - The OpenID Connect id_token. \"introspection\" - The result of introspecting the OAuth v2 access_token. </pre> 
  #[serde(rename = "authorizationGroupClaimSource", skip_serializing_if="Option::is_none")]
  authorization_group_claim_source: Option<String>,
  /// Indicates whether OAuth based authorization is enabled and the configured authorization type for OAuth clients is overridden.
  #[serde(rename = "authorizationGroupEnabled", skip_serializing_if="Option::is_none")]
  authorization_group_enabled: Option<bool>,
  /// Indicates whether clients are disconnected when their tokens expire.
  #[serde(rename = "disconnectOnTokenExpirationEnabled", skip_serializing_if="Option::is_none")]
  disconnect_on_token_expiration_enabled: Option<bool>,
  /// Indicates whether OAuth Provider client authentication is enabled.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The reason for the last JWKS public key refresh failure.
  #[serde(rename = "jwksLastRefreshFailureReason", skip_serializing_if="Option::is_none")]
  jwks_last_refresh_failure_reason: Option<String>,
  /// The timestamp of the last JWKS public key refresh failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "jwksLastRefreshFailureTime", skip_serializing_if="Option::is_none")]
  jwks_last_refresh_failure_time: Option<i32>,
  /// The timestamp of the last JWKS public key refresh success. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "jwksLastRefreshTime", skip_serializing_if="Option::is_none")]
  jwks_last_refresh_time: Option<i32>,
  /// The timestamp of the next scheduled JWKS public key refresh. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "jwksNextScheduledRefreshTime", skip_serializing_if="Option::is_none")]
  jwks_next_scheduled_refresh_time: Option<i32>,
  /// The number of JWKS public key refresh failures.
  #[serde(rename = "jwksRefreshFailureCount", skip_serializing_if="Option::is_none")]
  jwks_refresh_failure_count: Option<i64>,
  /// The number of seconds between forced JWKS public key refreshing.
  #[serde(rename = "jwksRefreshInterval", skip_serializing_if="Option::is_none")]
  jwks_refresh_interval: Option<i32>,
  /// The URI where the OAuth provider publishes its JWKS public keys.
  #[serde(rename = "jwksUri", skip_serializing_if="Option::is_none")]
  jwks_uri: Option<String>,
  /// The number of login failures due to an incorrect audience value.
  #[serde(rename = "loginFailureIncorrectAudienceValueCount", skip_serializing_if="Option::is_none")]
  login_failure_incorrect_audience_value_count: Option<i64>,
  /// The number of login failures due to an invalid audience value.
  #[serde(rename = "loginFailureInvalidAudienceValueCount", skip_serializing_if="Option::is_none")]
  login_failure_invalid_audience_value_count: Option<i64>,
  /// The number of login failures due to an invalid authorization group value (zero-length or non-string).
  #[serde(rename = "loginFailureInvalidAuthorizationGroupValueCount", skip_serializing_if="Option::is_none")]
  login_failure_invalid_authorization_group_value_count: Option<i64>,
  /// The number of login failures due to an invalid JWT signature.
  #[serde(rename = "loginFailureInvalidJwtSignatureCount", skip_serializing_if="Option::is_none")]
  login_failure_invalid_jwt_signature_count: Option<i64>,
  /// The number of login failures due to an invalid username value.
  #[serde(rename = "loginFailureInvalidUsernameValueCount", skip_serializing_if="Option::is_none")]
  login_failure_invalid_username_value_count: Option<i64>,
  /// The number of login failures due to a mismatched username.
  #[serde(rename = "loginFailureMismatchedUsernameCount", skip_serializing_if="Option::is_none")]
  login_failure_mismatched_username_count: Option<i64>,
  /// The number of login failures due to a missing audience claim.
  #[serde(rename = "loginFailureMissingAudienceCount", skip_serializing_if="Option::is_none")]
  login_failure_missing_audience_count: Option<i64>,
  /// The number of login failures due to a missing JSON Web Key (JWK).
  #[serde(rename = "loginFailureMissingJwkCount", skip_serializing_if="Option::is_none")]
  login_failure_missing_jwk_count: Option<i64>,
  /// The number of login failures due to a missing or invalid token.
  #[serde(rename = "loginFailureMissingOrInvalidTokenCount", skip_serializing_if="Option::is_none")]
  login_failure_missing_or_invalid_token_count: Option<i64>,
  /// The number of login failures due to a missing username.
  #[serde(rename = "loginFailureMissingUsernameCount", skip_serializing_if="Option::is_none")]
  login_failure_missing_username_count: Option<i64>,
  /// The number of login failures due to a token being expired.
  #[serde(rename = "loginFailureTokenExpiredCount", skip_serializing_if="Option::is_none")]
  login_failure_token_expired_count: Option<i64>,
  /// The number of login failures due to a token introspection error response.
  #[serde(rename = "loginFailureTokenIntrospectionErroredCount", skip_serializing_if="Option::is_none")]
  login_failure_token_introspection_errored_count: Option<i64>,
  /// The number of login failures due to a failure to complete the token introspection.
  #[serde(rename = "loginFailureTokenIntrospectionFailureCount", skip_serializing_if="Option::is_none")]
  login_failure_token_introspection_failure_count: Option<i64>,
  /// The number of login failures due to a token introspection HTTPS error.
  #[serde(rename = "loginFailureTokenIntrospectionHttpsErrorCount", skip_serializing_if="Option::is_none")]
  login_failure_token_introspection_https_error_count: Option<i64>,
  /// The number of login failures due to a token introspection response being invalid.
  #[serde(rename = "loginFailureTokenIntrospectionInvalidCount", skip_serializing_if="Option::is_none")]
  login_failure_token_introspection_invalid_count: Option<i64>,
  /// The number of login failures due to a token introspection timeout.
  #[serde(rename = "loginFailureTokenIntrospectionTimeoutCount", skip_serializing_if="Option::is_none")]
  login_failure_token_introspection_timeout_count: Option<i64>,
  /// The number of login failures due to a token not being valid yet.
  #[serde(rename = "loginFailureTokenNotValidYetCount", skip_serializing_if="Option::is_none")]
  login_failure_token_not_valid_yet_count: Option<i64>,
  /// The number of login failures due to an unsupported algorithm.
  #[serde(rename = "loginFailureUnsupportedAlgCount", skip_serializing_if="Option::is_none")]
  login_failure_unsupported_alg_count: Option<i64>,
  /// The number of clients that did not provide an authorization group claim value when expected.
  #[serde(rename = "missingAuthorizationGroupCount", skip_serializing_if="Option::is_none")]
  missing_authorization_group_count: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The name of the OAuth Provider.
  #[serde(rename = "oauthProviderName", skip_serializing_if="Option::is_none")]
  oauth_provider_name: Option<String>,
  /// Indicates whether to ignore time limits and accept tokens that are not yet valid or are no longer valid.
  #[serde(rename = "tokenIgnoreTimeLimitsEnabled", skip_serializing_if="Option::is_none")]
  token_ignore_time_limits_enabled: Option<bool>,
  /// The one minute average of the time required to complete a token introspection, in milliseconds (ms).
  #[serde(rename = "tokenIntrospectionAverageTime", skip_serializing_if="Option::is_none")]
  token_introspection_average_time: Option<i32>,
  /// The reason for the last token introspection failure.
  #[serde(rename = "tokenIntrospectionLastFailureReason", skip_serializing_if="Option::is_none")]
  token_introspection_last_failure_reason: Option<String>,
  /// The timestamp of the last token introspection failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time).
  #[serde(rename = "tokenIntrospectionLastFailureTime", skip_serializing_if="Option::is_none")]
  token_introspection_last_failure_time: Option<i32>,
  /// The parameter name used to identify the token during access token introspection. A standards compliant OAuth introspection server expects \"token\".
  #[serde(rename = "tokenIntrospectionParameterName", skip_serializing_if="Option::is_none")]
  token_introspection_parameter_name: Option<String>,
  /// The number of token introspection successes.
  #[serde(rename = "tokenIntrospectionSuccessCount", skip_serializing_if="Option::is_none")]
  token_introspection_success_count: Option<i64>,
  /// The maximum time in seconds a token introspection is allowed to take.
  #[serde(rename = "tokenIntrospectionTimeout", skip_serializing_if="Option::is_none")]
  token_introspection_timeout: Option<i32>,
  /// The token introspection URI of the OAuth authentication server.
  #[serde(rename = "tokenIntrospectionUri", skip_serializing_if="Option::is_none")]
  token_introspection_uri: Option<String>,
  /// The username to use when logging into the token introspection URI.
  #[serde(rename = "tokenIntrospectionUsername", skip_serializing_if="Option::is_none")]
  token_introspection_username: Option<String>,
  /// The username claim name, indicating which part of the object to use for determining the username.
  #[serde(rename = "usernameClaimName", skip_serializing_if="Option::is_none")]
  username_claim_name: Option<String>,
  /// The username claim source, indicating where to search for the username value. The allowed values and their meaning are:  <pre> \"access-token\" - The OAuth v2 access_token. \"id-token\" - The OpenID Connect id_token. \"introspection\" - The result of introspecting the OAuth v2 access_token. </pre> 
  #[serde(rename = "usernameClaimSource", skip_serializing_if="Option::is_none")]
  username_claim_source: Option<String>,
  /// Indicates whether the API provided username will be validated against the username calculated from the token(s).
  #[serde(rename = "usernameValidateEnabled", skip_serializing_if="Option::is_none")]
  username_validate_enabled: Option<bool>
}

impl MsgVpnAuthenticationOauthProvider {
  pub fn new() -> MsgVpnAuthenticationOauthProvider {
    MsgVpnAuthenticationOauthProvider {
      audience_claim_name: None,
      audience_claim_source: None,
      audience_claim_value: None,
      audience_validation_enabled: None,
      authentication_success_count: None,
      authorization_group_claim_name: None,
      authorization_group_claim_source: None,
      authorization_group_enabled: None,
      disconnect_on_token_expiration_enabled: None,
      enabled: None,
      jwks_last_refresh_failure_reason: None,
      jwks_last_refresh_failure_time: None,
      jwks_last_refresh_time: None,
      jwks_next_scheduled_refresh_time: None,
      jwks_refresh_failure_count: None,
      jwks_refresh_interval: None,
      jwks_uri: None,
      login_failure_incorrect_audience_value_count: None,
      login_failure_invalid_audience_value_count: None,
      login_failure_invalid_authorization_group_value_count: None,
      login_failure_invalid_jwt_signature_count: None,
      login_failure_invalid_username_value_count: None,
      login_failure_mismatched_username_count: None,
      login_failure_missing_audience_count: None,
      login_failure_missing_jwk_count: None,
      login_failure_missing_or_invalid_token_count: None,
      login_failure_missing_username_count: None,
      login_failure_token_expired_count: None,
      login_failure_token_introspection_errored_count: None,
      login_failure_token_introspection_failure_count: None,
      login_failure_token_introspection_https_error_count: None,
      login_failure_token_introspection_invalid_count: None,
      login_failure_token_introspection_timeout_count: None,
      login_failure_token_not_valid_yet_count: None,
      login_failure_unsupported_alg_count: None,
      missing_authorization_group_count: None,
      msg_vpn_name: None,
      oauth_provider_name: None,
      token_ignore_time_limits_enabled: None,
      token_introspection_average_time: None,
      token_introspection_last_failure_reason: None,
      token_introspection_last_failure_time: None,
      token_introspection_parameter_name: None,
      token_introspection_success_count: None,
      token_introspection_timeout: None,
      token_introspection_uri: None,
      token_introspection_username: None,
      username_claim_name: None,
      username_claim_source: None,
      username_validate_enabled: None
    }
  }

  pub fn set_audience_claim_name(&mut self, audience_claim_name: String) {
    self.audience_claim_name = Some(audience_claim_name);
  }

  pub fn with_audience_claim_name(mut self, audience_claim_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.audience_claim_name = Some(audience_claim_name);
    self
  }

  pub fn audience_claim_name(&self) -> Option<&String> {
    self.audience_claim_name.as_ref()
  }

  pub fn reset_audience_claim_name(&mut self) {
    self.audience_claim_name = None;
  }

  pub fn set_audience_claim_source(&mut self, audience_claim_source: String) {
    self.audience_claim_source = Some(audience_claim_source);
  }

  pub fn with_audience_claim_source(mut self, audience_claim_source: String) -> MsgVpnAuthenticationOauthProvider {
    self.audience_claim_source = Some(audience_claim_source);
    self
  }

  pub fn audience_claim_source(&self) -> Option<&String> {
    self.audience_claim_source.as_ref()
  }

  pub fn reset_audience_claim_source(&mut self) {
    self.audience_claim_source = None;
  }

  pub fn set_audience_claim_value(&mut self, audience_claim_value: String) {
    self.audience_claim_value = Some(audience_claim_value);
  }

  pub fn with_audience_claim_value(mut self, audience_claim_value: String) -> MsgVpnAuthenticationOauthProvider {
    self.audience_claim_value = Some(audience_claim_value);
    self
  }

  pub fn audience_claim_value(&self) -> Option<&String> {
    self.audience_claim_value.as_ref()
  }

  pub fn reset_audience_claim_value(&mut self) {
    self.audience_claim_value = None;
  }

  pub fn set_audience_validation_enabled(&mut self, audience_validation_enabled: bool) {
    self.audience_validation_enabled = Some(audience_validation_enabled);
  }

  pub fn with_audience_validation_enabled(mut self, audience_validation_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.audience_validation_enabled = Some(audience_validation_enabled);
    self
  }

  pub fn audience_validation_enabled(&self) -> Option<&bool> {
    self.audience_validation_enabled.as_ref()
  }

  pub fn reset_audience_validation_enabled(&mut self) {
    self.audience_validation_enabled = None;
  }

  pub fn set_authentication_success_count(&mut self, authentication_success_count: i64) {
    self.authentication_success_count = Some(authentication_success_count);
  }

  pub fn with_authentication_success_count(mut self, authentication_success_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.authentication_success_count = Some(authentication_success_count);
    self
  }

  pub fn authentication_success_count(&self) -> Option<&i64> {
    self.authentication_success_count.as_ref()
  }

  pub fn reset_authentication_success_count(&mut self) {
    self.authentication_success_count = None;
  }

  pub fn set_authorization_group_claim_name(&mut self, authorization_group_claim_name: String) {
    self.authorization_group_claim_name = Some(authorization_group_claim_name);
  }

  pub fn with_authorization_group_claim_name(mut self, authorization_group_claim_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.authorization_group_claim_name = Some(authorization_group_claim_name);
    self
  }

  pub fn authorization_group_claim_name(&self) -> Option<&String> {
    self.authorization_group_claim_name.as_ref()
  }

  pub fn reset_authorization_group_claim_name(&mut self) {
    self.authorization_group_claim_name = None;
  }

  pub fn set_authorization_group_claim_source(&mut self, authorization_group_claim_source: String) {
    self.authorization_group_claim_source = Some(authorization_group_claim_source);
  }

  pub fn with_authorization_group_claim_source(mut self, authorization_group_claim_source: String) -> MsgVpnAuthenticationOauthProvider {
    self.authorization_group_claim_source = Some(authorization_group_claim_source);
    self
  }

  pub fn authorization_group_claim_source(&self) -> Option<&String> {
    self.authorization_group_claim_source.as_ref()
  }

  pub fn reset_authorization_group_claim_source(&mut self) {
    self.authorization_group_claim_source = None;
  }

  pub fn set_authorization_group_enabled(&mut self, authorization_group_enabled: bool) {
    self.authorization_group_enabled = Some(authorization_group_enabled);
  }

  pub fn with_authorization_group_enabled(mut self, authorization_group_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.authorization_group_enabled = Some(authorization_group_enabled);
    self
  }

  pub fn authorization_group_enabled(&self) -> Option<&bool> {
    self.authorization_group_enabled.as_ref()
  }

  pub fn reset_authorization_group_enabled(&mut self) {
    self.authorization_group_enabled = None;
  }

  pub fn set_disconnect_on_token_expiration_enabled(&mut self, disconnect_on_token_expiration_enabled: bool) {
    self.disconnect_on_token_expiration_enabled = Some(disconnect_on_token_expiration_enabled);
  }

  pub fn with_disconnect_on_token_expiration_enabled(mut self, disconnect_on_token_expiration_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.disconnect_on_token_expiration_enabled = Some(disconnect_on_token_expiration_enabled);
    self
  }

  pub fn disconnect_on_token_expiration_enabled(&self) -> Option<&bool> {
    self.disconnect_on_token_expiration_enabled.as_ref()
  }

  pub fn reset_disconnect_on_token_expiration_enabled(&mut self) {
    self.disconnect_on_token_expiration_enabled = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_jwks_last_refresh_failure_reason(&mut self, jwks_last_refresh_failure_reason: String) {
    self.jwks_last_refresh_failure_reason = Some(jwks_last_refresh_failure_reason);
  }

  pub fn with_jwks_last_refresh_failure_reason(mut self, jwks_last_refresh_failure_reason: String) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_last_refresh_failure_reason = Some(jwks_last_refresh_failure_reason);
    self
  }

  pub fn jwks_last_refresh_failure_reason(&self) -> Option<&String> {
    self.jwks_last_refresh_failure_reason.as_ref()
  }

  pub fn reset_jwks_last_refresh_failure_reason(&mut self) {
    self.jwks_last_refresh_failure_reason = None;
  }

  pub fn set_jwks_last_refresh_failure_time(&mut self, jwks_last_refresh_failure_time: i32) {
    self.jwks_last_refresh_failure_time = Some(jwks_last_refresh_failure_time);
  }

  pub fn with_jwks_last_refresh_failure_time(mut self, jwks_last_refresh_failure_time: i32) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_last_refresh_failure_time = Some(jwks_last_refresh_failure_time);
    self
  }

  pub fn jwks_last_refresh_failure_time(&self) -> Option<&i32> {
    self.jwks_last_refresh_failure_time.as_ref()
  }

  pub fn reset_jwks_last_refresh_failure_time(&mut self) {
    self.jwks_last_refresh_failure_time = None;
  }

  pub fn set_jwks_last_refresh_time(&mut self, jwks_last_refresh_time: i32) {
    self.jwks_last_refresh_time = Some(jwks_last_refresh_time);
  }

  pub fn with_jwks_last_refresh_time(mut self, jwks_last_refresh_time: i32) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_last_refresh_time = Some(jwks_last_refresh_time);
    self
  }

  pub fn jwks_last_refresh_time(&self) -> Option<&i32> {
    self.jwks_last_refresh_time.as_ref()
  }

  pub fn reset_jwks_last_refresh_time(&mut self) {
    self.jwks_last_refresh_time = None;
  }

  pub fn set_jwks_next_scheduled_refresh_time(&mut self, jwks_next_scheduled_refresh_time: i32) {
    self.jwks_next_scheduled_refresh_time = Some(jwks_next_scheduled_refresh_time);
  }

  pub fn with_jwks_next_scheduled_refresh_time(mut self, jwks_next_scheduled_refresh_time: i32) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_next_scheduled_refresh_time = Some(jwks_next_scheduled_refresh_time);
    self
  }

  pub fn jwks_next_scheduled_refresh_time(&self) -> Option<&i32> {
    self.jwks_next_scheduled_refresh_time.as_ref()
  }

  pub fn reset_jwks_next_scheduled_refresh_time(&mut self) {
    self.jwks_next_scheduled_refresh_time = None;
  }

  pub fn set_jwks_refresh_failure_count(&mut self, jwks_refresh_failure_count: i64) {
    self.jwks_refresh_failure_count = Some(jwks_refresh_failure_count);
  }

  pub fn with_jwks_refresh_failure_count(mut self, jwks_refresh_failure_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_refresh_failure_count = Some(jwks_refresh_failure_count);
    self
  }

  pub fn jwks_refresh_failure_count(&self) -> Option<&i64> {
    self.jwks_refresh_failure_count.as_ref()
  }

  pub fn reset_jwks_refresh_failure_count(&mut self) {
    self.jwks_refresh_failure_count = None;
  }

  pub fn set_jwks_refresh_interval(&mut self, jwks_refresh_interval: i32) {
    self.jwks_refresh_interval = Some(jwks_refresh_interval);
  }

  pub fn with_jwks_refresh_interval(mut self, jwks_refresh_interval: i32) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_refresh_interval = Some(jwks_refresh_interval);
    self
  }

  pub fn jwks_refresh_interval(&self) -> Option<&i32> {
    self.jwks_refresh_interval.as_ref()
  }

  pub fn reset_jwks_refresh_interval(&mut self) {
    self.jwks_refresh_interval = None;
  }

  pub fn set_jwks_uri(&mut self, jwks_uri: String) {
    self.jwks_uri = Some(jwks_uri);
  }

  pub fn with_jwks_uri(mut self, jwks_uri: String) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_uri = Some(jwks_uri);
    self
  }

  pub fn jwks_uri(&self) -> Option<&String> {
    self.jwks_uri.as_ref()
  }

  pub fn reset_jwks_uri(&mut self) {
    self.jwks_uri = None;
  }

  pub fn set_login_failure_incorrect_audience_value_count(&mut self, login_failure_incorrect_audience_value_count: i64) {
    self.login_failure_incorrect_audience_value_count = Some(login_failure_incorrect_audience_value_count);
  }

  pub fn with_login_failure_incorrect_audience_value_count(mut self, login_failure_incorrect_audience_value_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_incorrect_audience_value_count = Some(login_failure_incorrect_audience_value_count);
    self
  }

  pub fn login_failure_incorrect_audience_value_count(&self) -> Option<&i64> {
    self.login_failure_incorrect_audience_value_count.as_ref()
  }

  pub fn reset_login_failure_incorrect_audience_value_count(&mut self) {
    self.login_failure_incorrect_audience_value_count = None;
  }

  pub fn set_login_failure_invalid_audience_value_count(&mut self, login_failure_invalid_audience_value_count: i64) {
    self.login_failure_invalid_audience_value_count = Some(login_failure_invalid_audience_value_count);
  }

  pub fn with_login_failure_invalid_audience_value_count(mut self, login_failure_invalid_audience_value_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_invalid_audience_value_count = Some(login_failure_invalid_audience_value_count);
    self
  }

  pub fn login_failure_invalid_audience_value_count(&self) -> Option<&i64> {
    self.login_failure_invalid_audience_value_count.as_ref()
  }

  pub fn reset_login_failure_invalid_audience_value_count(&mut self) {
    self.login_failure_invalid_audience_value_count = None;
  }

  pub fn set_login_failure_invalid_authorization_group_value_count(&mut self, login_failure_invalid_authorization_group_value_count: i64) {
    self.login_failure_invalid_authorization_group_value_count = Some(login_failure_invalid_authorization_group_value_count);
  }

  pub fn with_login_failure_invalid_authorization_group_value_count(mut self, login_failure_invalid_authorization_group_value_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_invalid_authorization_group_value_count = Some(login_failure_invalid_authorization_group_value_count);
    self
  }

  pub fn login_failure_invalid_authorization_group_value_count(&self) -> Option<&i64> {
    self.login_failure_invalid_authorization_group_value_count.as_ref()
  }

  pub fn reset_login_failure_invalid_authorization_group_value_count(&mut self) {
    self.login_failure_invalid_authorization_group_value_count = None;
  }

  pub fn set_login_failure_invalid_jwt_signature_count(&mut self, login_failure_invalid_jwt_signature_count: i64) {
    self.login_failure_invalid_jwt_signature_count = Some(login_failure_invalid_jwt_signature_count);
  }

  pub fn with_login_failure_invalid_jwt_signature_count(mut self, login_failure_invalid_jwt_signature_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_invalid_jwt_signature_count = Some(login_failure_invalid_jwt_signature_count);
    self
  }

  pub fn login_failure_invalid_jwt_signature_count(&self) -> Option<&i64> {
    self.login_failure_invalid_jwt_signature_count.as_ref()
  }

  pub fn reset_login_failure_invalid_jwt_signature_count(&mut self) {
    self.login_failure_invalid_jwt_signature_count = None;
  }

  pub fn set_login_failure_invalid_username_value_count(&mut self, login_failure_invalid_username_value_count: i64) {
    self.login_failure_invalid_username_value_count = Some(login_failure_invalid_username_value_count);
  }

  pub fn with_login_failure_invalid_username_value_count(mut self, login_failure_invalid_username_value_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_invalid_username_value_count = Some(login_failure_invalid_username_value_count);
    self
  }

  pub fn login_failure_invalid_username_value_count(&self) -> Option<&i64> {
    self.login_failure_invalid_username_value_count.as_ref()
  }

  pub fn reset_login_failure_invalid_username_value_count(&mut self) {
    self.login_failure_invalid_username_value_count = None;
  }

  pub fn set_login_failure_mismatched_username_count(&mut self, login_failure_mismatched_username_count: i64) {
    self.login_failure_mismatched_username_count = Some(login_failure_mismatched_username_count);
  }

  pub fn with_login_failure_mismatched_username_count(mut self, login_failure_mismatched_username_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_mismatched_username_count = Some(login_failure_mismatched_username_count);
    self
  }

  pub fn login_failure_mismatched_username_count(&self) -> Option<&i64> {
    self.login_failure_mismatched_username_count.as_ref()
  }

  pub fn reset_login_failure_mismatched_username_count(&mut self) {
    self.login_failure_mismatched_username_count = None;
  }

  pub fn set_login_failure_missing_audience_count(&mut self, login_failure_missing_audience_count: i64) {
    self.login_failure_missing_audience_count = Some(login_failure_missing_audience_count);
  }

  pub fn with_login_failure_missing_audience_count(mut self, login_failure_missing_audience_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_missing_audience_count = Some(login_failure_missing_audience_count);
    self
  }

  pub fn login_failure_missing_audience_count(&self) -> Option<&i64> {
    self.login_failure_missing_audience_count.as_ref()
  }

  pub fn reset_login_failure_missing_audience_count(&mut self) {
    self.login_failure_missing_audience_count = None;
  }

  pub fn set_login_failure_missing_jwk_count(&mut self, login_failure_missing_jwk_count: i64) {
    self.login_failure_missing_jwk_count = Some(login_failure_missing_jwk_count);
  }

  pub fn with_login_failure_missing_jwk_count(mut self, login_failure_missing_jwk_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_missing_jwk_count = Some(login_failure_missing_jwk_count);
    self
  }

  pub fn login_failure_missing_jwk_count(&self) -> Option<&i64> {
    self.login_failure_missing_jwk_count.as_ref()
  }

  pub fn reset_login_failure_missing_jwk_count(&mut self) {
    self.login_failure_missing_jwk_count = None;
  }

  pub fn set_login_failure_missing_or_invalid_token_count(&mut self, login_failure_missing_or_invalid_token_count: i64) {
    self.login_failure_missing_or_invalid_token_count = Some(login_failure_missing_or_invalid_token_count);
  }

  pub fn with_login_failure_missing_or_invalid_token_count(mut self, login_failure_missing_or_invalid_token_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_missing_or_invalid_token_count = Some(login_failure_missing_or_invalid_token_count);
    self
  }

  pub fn login_failure_missing_or_invalid_token_count(&self) -> Option<&i64> {
    self.login_failure_missing_or_invalid_token_count.as_ref()
  }

  pub fn reset_login_failure_missing_or_invalid_token_count(&mut self) {
    self.login_failure_missing_or_invalid_token_count = None;
  }

  pub fn set_login_failure_missing_username_count(&mut self, login_failure_missing_username_count: i64) {
    self.login_failure_missing_username_count = Some(login_failure_missing_username_count);
  }

  pub fn with_login_failure_missing_username_count(mut self, login_failure_missing_username_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_missing_username_count = Some(login_failure_missing_username_count);
    self
  }

  pub fn login_failure_missing_username_count(&self) -> Option<&i64> {
    self.login_failure_missing_username_count.as_ref()
  }

  pub fn reset_login_failure_missing_username_count(&mut self) {
    self.login_failure_missing_username_count = None;
  }

  pub fn set_login_failure_token_expired_count(&mut self, login_failure_token_expired_count: i64) {
    self.login_failure_token_expired_count = Some(login_failure_token_expired_count);
  }

  pub fn with_login_failure_token_expired_count(mut self, login_failure_token_expired_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_token_expired_count = Some(login_failure_token_expired_count);
    self
  }

  pub fn login_failure_token_expired_count(&self) -> Option<&i64> {
    self.login_failure_token_expired_count.as_ref()
  }

  pub fn reset_login_failure_token_expired_count(&mut self) {
    self.login_failure_token_expired_count = None;
  }

  pub fn set_login_failure_token_introspection_errored_count(&mut self, login_failure_token_introspection_errored_count: i64) {
    self.login_failure_token_introspection_errored_count = Some(login_failure_token_introspection_errored_count);
  }

  pub fn with_login_failure_token_introspection_errored_count(mut self, login_failure_token_introspection_errored_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_token_introspection_errored_count = Some(login_failure_token_introspection_errored_count);
    self
  }

  pub fn login_failure_token_introspection_errored_count(&self) -> Option<&i64> {
    self.login_failure_token_introspection_errored_count.as_ref()
  }

  pub fn reset_login_failure_token_introspection_errored_count(&mut self) {
    self.login_failure_token_introspection_errored_count = None;
  }

  pub fn set_login_failure_token_introspection_failure_count(&mut self, login_failure_token_introspection_failure_count: i64) {
    self.login_failure_token_introspection_failure_count = Some(login_failure_token_introspection_failure_count);
  }

  pub fn with_login_failure_token_introspection_failure_count(mut self, login_failure_token_introspection_failure_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_token_introspection_failure_count = Some(login_failure_token_introspection_failure_count);
    self
  }

  pub fn login_failure_token_introspection_failure_count(&self) -> Option<&i64> {
    self.login_failure_token_introspection_failure_count.as_ref()
  }

  pub fn reset_login_failure_token_introspection_failure_count(&mut self) {
    self.login_failure_token_introspection_failure_count = None;
  }

  pub fn set_login_failure_token_introspection_https_error_count(&mut self, login_failure_token_introspection_https_error_count: i64) {
    self.login_failure_token_introspection_https_error_count = Some(login_failure_token_introspection_https_error_count);
  }

  pub fn with_login_failure_token_introspection_https_error_count(mut self, login_failure_token_introspection_https_error_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_token_introspection_https_error_count = Some(login_failure_token_introspection_https_error_count);
    self
  }

  pub fn login_failure_token_introspection_https_error_count(&self) -> Option<&i64> {
    self.login_failure_token_introspection_https_error_count.as_ref()
  }

  pub fn reset_login_failure_token_introspection_https_error_count(&mut self) {
    self.login_failure_token_introspection_https_error_count = None;
  }

  pub fn set_login_failure_token_introspection_invalid_count(&mut self, login_failure_token_introspection_invalid_count: i64) {
    self.login_failure_token_introspection_invalid_count = Some(login_failure_token_introspection_invalid_count);
  }

  pub fn with_login_failure_token_introspection_invalid_count(mut self, login_failure_token_introspection_invalid_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_token_introspection_invalid_count = Some(login_failure_token_introspection_invalid_count);
    self
  }

  pub fn login_failure_token_introspection_invalid_count(&self) -> Option<&i64> {
    self.login_failure_token_introspection_invalid_count.as_ref()
  }

  pub fn reset_login_failure_token_introspection_invalid_count(&mut self) {
    self.login_failure_token_introspection_invalid_count = None;
  }

  pub fn set_login_failure_token_introspection_timeout_count(&mut self, login_failure_token_introspection_timeout_count: i64) {
    self.login_failure_token_introspection_timeout_count = Some(login_failure_token_introspection_timeout_count);
  }

  pub fn with_login_failure_token_introspection_timeout_count(mut self, login_failure_token_introspection_timeout_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_token_introspection_timeout_count = Some(login_failure_token_introspection_timeout_count);
    self
  }

  pub fn login_failure_token_introspection_timeout_count(&self) -> Option<&i64> {
    self.login_failure_token_introspection_timeout_count.as_ref()
  }

  pub fn reset_login_failure_token_introspection_timeout_count(&mut self) {
    self.login_failure_token_introspection_timeout_count = None;
  }

  pub fn set_login_failure_token_not_valid_yet_count(&mut self, login_failure_token_not_valid_yet_count: i64) {
    self.login_failure_token_not_valid_yet_count = Some(login_failure_token_not_valid_yet_count);
  }

  pub fn with_login_failure_token_not_valid_yet_count(mut self, login_failure_token_not_valid_yet_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_token_not_valid_yet_count = Some(login_failure_token_not_valid_yet_count);
    self
  }

  pub fn login_failure_token_not_valid_yet_count(&self) -> Option<&i64> {
    self.login_failure_token_not_valid_yet_count.as_ref()
  }

  pub fn reset_login_failure_token_not_valid_yet_count(&mut self) {
    self.login_failure_token_not_valid_yet_count = None;
  }

  pub fn set_login_failure_unsupported_alg_count(&mut self, login_failure_unsupported_alg_count: i64) {
    self.login_failure_unsupported_alg_count = Some(login_failure_unsupported_alg_count);
  }

  pub fn with_login_failure_unsupported_alg_count(mut self, login_failure_unsupported_alg_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.login_failure_unsupported_alg_count = Some(login_failure_unsupported_alg_count);
    self
  }

  pub fn login_failure_unsupported_alg_count(&self) -> Option<&i64> {
    self.login_failure_unsupported_alg_count.as_ref()
  }

  pub fn reset_login_failure_unsupported_alg_count(&mut self) {
    self.login_failure_unsupported_alg_count = None;
  }

  pub fn set_missing_authorization_group_count(&mut self, missing_authorization_group_count: i64) {
    self.missing_authorization_group_count = Some(missing_authorization_group_count);
  }

  pub fn with_missing_authorization_group_count(mut self, missing_authorization_group_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.missing_authorization_group_count = Some(missing_authorization_group_count);
    self
  }

  pub fn missing_authorization_group_count(&self) -> Option<&i64> {
    self.missing_authorization_group_count.as_ref()
  }

  pub fn reset_missing_authorization_group_count(&mut self) {
    self.missing_authorization_group_count = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_oauth_provider_name(&mut self, oauth_provider_name: String) {
    self.oauth_provider_name = Some(oauth_provider_name);
  }

  pub fn with_oauth_provider_name(mut self, oauth_provider_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.oauth_provider_name = Some(oauth_provider_name);
    self
  }

  pub fn oauth_provider_name(&self) -> Option<&String> {
    self.oauth_provider_name.as_ref()
  }

  pub fn reset_oauth_provider_name(&mut self) {
    self.oauth_provider_name = None;
  }

  pub fn set_token_ignore_time_limits_enabled(&mut self, token_ignore_time_limits_enabled: bool) {
    self.token_ignore_time_limits_enabled = Some(token_ignore_time_limits_enabled);
  }

  pub fn with_token_ignore_time_limits_enabled(mut self, token_ignore_time_limits_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.token_ignore_time_limits_enabled = Some(token_ignore_time_limits_enabled);
    self
  }

  pub fn token_ignore_time_limits_enabled(&self) -> Option<&bool> {
    self.token_ignore_time_limits_enabled.as_ref()
  }

  pub fn reset_token_ignore_time_limits_enabled(&mut self) {
    self.token_ignore_time_limits_enabled = None;
  }

  pub fn set_token_introspection_average_time(&mut self, token_introspection_average_time: i32) {
    self.token_introspection_average_time = Some(token_introspection_average_time);
  }

  pub fn with_token_introspection_average_time(mut self, token_introspection_average_time: i32) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_average_time = Some(token_introspection_average_time);
    self
  }

  pub fn token_introspection_average_time(&self) -> Option<&i32> {
    self.token_introspection_average_time.as_ref()
  }

  pub fn reset_token_introspection_average_time(&mut self) {
    self.token_introspection_average_time = None;
  }

  pub fn set_token_introspection_last_failure_reason(&mut self, token_introspection_last_failure_reason: String) {
    self.token_introspection_last_failure_reason = Some(token_introspection_last_failure_reason);
  }

  pub fn with_token_introspection_last_failure_reason(mut self, token_introspection_last_failure_reason: String) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_last_failure_reason = Some(token_introspection_last_failure_reason);
    self
  }

  pub fn token_introspection_last_failure_reason(&self) -> Option<&String> {
    self.token_introspection_last_failure_reason.as_ref()
  }

  pub fn reset_token_introspection_last_failure_reason(&mut self) {
    self.token_introspection_last_failure_reason = None;
  }

  pub fn set_token_introspection_last_failure_time(&mut self, token_introspection_last_failure_time: i32) {
    self.token_introspection_last_failure_time = Some(token_introspection_last_failure_time);
  }

  pub fn with_token_introspection_last_failure_time(mut self, token_introspection_last_failure_time: i32) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_last_failure_time = Some(token_introspection_last_failure_time);
    self
  }

  pub fn token_introspection_last_failure_time(&self) -> Option<&i32> {
    self.token_introspection_last_failure_time.as_ref()
  }

  pub fn reset_token_introspection_last_failure_time(&mut self) {
    self.token_introspection_last_failure_time = None;
  }

  pub fn set_token_introspection_parameter_name(&mut self, token_introspection_parameter_name: String) {
    self.token_introspection_parameter_name = Some(token_introspection_parameter_name);
  }

  pub fn with_token_introspection_parameter_name(mut self, token_introspection_parameter_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_parameter_name = Some(token_introspection_parameter_name);
    self
  }

  pub fn token_introspection_parameter_name(&self) -> Option<&String> {
    self.token_introspection_parameter_name.as_ref()
  }

  pub fn reset_token_introspection_parameter_name(&mut self) {
    self.token_introspection_parameter_name = None;
  }

  pub fn set_token_introspection_success_count(&mut self, token_introspection_success_count: i64) {
    self.token_introspection_success_count = Some(token_introspection_success_count);
  }

  pub fn with_token_introspection_success_count(mut self, token_introspection_success_count: i64) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_success_count = Some(token_introspection_success_count);
    self
  }

  pub fn token_introspection_success_count(&self) -> Option<&i64> {
    self.token_introspection_success_count.as_ref()
  }

  pub fn reset_token_introspection_success_count(&mut self) {
    self.token_introspection_success_count = None;
  }

  pub fn set_token_introspection_timeout(&mut self, token_introspection_timeout: i32) {
    self.token_introspection_timeout = Some(token_introspection_timeout);
  }

  pub fn with_token_introspection_timeout(mut self, token_introspection_timeout: i32) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_timeout = Some(token_introspection_timeout);
    self
  }

  pub fn token_introspection_timeout(&self) -> Option<&i32> {
    self.token_introspection_timeout.as_ref()
  }

  pub fn reset_token_introspection_timeout(&mut self) {
    self.token_introspection_timeout = None;
  }

  pub fn set_token_introspection_uri(&mut self, token_introspection_uri: String) {
    self.token_introspection_uri = Some(token_introspection_uri);
  }

  pub fn with_token_introspection_uri(mut self, token_introspection_uri: String) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_uri = Some(token_introspection_uri);
    self
  }

  pub fn token_introspection_uri(&self) -> Option<&String> {
    self.token_introspection_uri.as_ref()
  }

  pub fn reset_token_introspection_uri(&mut self) {
    self.token_introspection_uri = None;
  }

  pub fn set_token_introspection_username(&mut self, token_introspection_username: String) {
    self.token_introspection_username = Some(token_introspection_username);
  }

  pub fn with_token_introspection_username(mut self, token_introspection_username: String) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_username = Some(token_introspection_username);
    self
  }

  pub fn token_introspection_username(&self) -> Option<&String> {
    self.token_introspection_username.as_ref()
  }

  pub fn reset_token_introspection_username(&mut self) {
    self.token_introspection_username = None;
  }

  pub fn set_username_claim_name(&mut self, username_claim_name: String) {
    self.username_claim_name = Some(username_claim_name);
  }

  pub fn with_username_claim_name(mut self, username_claim_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.username_claim_name = Some(username_claim_name);
    self
  }

  pub fn username_claim_name(&self) -> Option<&String> {
    self.username_claim_name.as_ref()
  }

  pub fn reset_username_claim_name(&mut self) {
    self.username_claim_name = None;
  }

  pub fn set_username_claim_source(&mut self, username_claim_source: String) {
    self.username_claim_source = Some(username_claim_source);
  }

  pub fn with_username_claim_source(mut self, username_claim_source: String) -> MsgVpnAuthenticationOauthProvider {
    self.username_claim_source = Some(username_claim_source);
    self
  }

  pub fn username_claim_source(&self) -> Option<&String> {
    self.username_claim_source.as_ref()
  }

  pub fn reset_username_claim_source(&mut self) {
    self.username_claim_source = None;
  }

  pub fn set_username_validate_enabled(&mut self, username_validate_enabled: bool) {
    self.username_validate_enabled = Some(username_validate_enabled);
  }

  pub fn with_username_validate_enabled(mut self, username_validate_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.username_validate_enabled = Some(username_validate_enabled);
    self
  }

  pub fn username_validate_enabled(&self) -> Option<&bool> {
    self.username_validate_enabled.as_ref()
  }

  pub fn reset_username_validate_enabled(&mut self) {
    self.username_validate_enabled = None;
  }

}



