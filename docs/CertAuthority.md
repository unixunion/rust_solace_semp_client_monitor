# CertAuthority

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cert_authority_name** | **String** | The name of the Certificate Authority. | [optional] [default to null]
**crl_day_list** | **String** | The scheduled CRL refresh day(s), specified as \&quot;daily\&quot; or a comma-separated list of days. Days must be specified as \&quot;Sun\&quot;, \&quot;Mon\&quot;, \&quot;Tue\&quot;, \&quot;Wed\&quot;, \&quot;Thu\&quot;, \&quot;Fri\&quot;, or \&quot;Sat\&quot;, with no spaces, and in sorted order from Sunday to Saturday. | [optional] [default to null]
**crl_last_download_time** | **i32** | The timestamp of the last successful CRL download. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**crl_last_failure_reason** | **String** | The reason for the last CRL failure. | [optional] [default to null]
**crl_last_failure_time** | **i32** | The timestamp of the last CRL failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**crl_next_download_time** | **i32** | The scheduled time of the next CRL download. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**crl_time_list** | **String** | The scheduled CRL refresh time(s), specified as \&quot;hourly\&quot; or a comma-separated list of 24-hour times in the form hh:mm, or h:mm. There must be no spaces, and times must be in sorted order from 0:00 to 23:59. | [optional] [default to null]
**crl_up** | **bool** | Indicates whether CRL revocation checking is operationally up. | [optional] [default to null]
**crl_url** | **String** | The URL for the CRL source. This is a required attribute for CRL to be operational and the URL must be complete with http:// included. | [optional] [default to null]
**ocsp_last_failure_reason** | **String** | The reason for the last OCSP failure. | [optional] [default to null]
**ocsp_last_failure_time** | **i32** | The timestamp of the last OCSP failure. This value represents the number of seconds since 1970-01-01 00:00:00 UTC (Unix time). | [optional] [default to null]
**ocsp_last_failure_url** | **String** | The URL involved in the last OCSP failure. | [optional] [default to null]
**ocsp_non_responder_cert_enabled** | **bool** | Indicates whether a non-responder certificate is allowed to sign an OCSP response. Typically used with an OCSP override URL in cases where a single certificate is used to sign client certificates and OCSP responses. | [optional] [default to null]
**ocsp_override_url** | **String** | The OCSP responder URL to use for overriding the one supplied in the client certificate. The URL must be complete with http:// included. | [optional] [default to null]
**ocsp_timeout** | **i64** | The timeout in seconds to receive a response from the OCSP responder after sending a request or making the initial connection attempt. | [optional] [default to null]
**revocation_check_enabled** | **bool** | Indicates whether Certificate Authority revocation checking is enabled. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


