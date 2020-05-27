# MsgVpnAuthorizationGroup

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acl_profile_name** | **String** | The ACL Profile of the LDAP Authorization Group. | [optional] [default to null]
**authorization_group_name** | **String** | The name of the LDAP Authorization Group. Special care is needed if the group name contains special characters such as &#39;#&#39;, &#39;+&#39;, &#39;;&#39;, &#39;&#x3D;&#39; as the value of the group name returned from the LDAP server might prepend those characters with &#39;\\&#39;. For example a group name called &#39;test#,lab,com&#39; will be returned from the LDAP server as &#39;test\\#,lab,com&#39;. | [optional] [default to null]
**client_profile_name** | **String** | The Client Profile of the LDAP Authorization Group. | [optional] [default to null]
**enabled** | **bool** | Indicates whether the LDAP Authorization Group is enabled. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


