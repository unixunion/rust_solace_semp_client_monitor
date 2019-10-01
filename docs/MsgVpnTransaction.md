# MsgVpnTransaction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **i32** | The identifier (ID) of the Client. | [optional] [default to null]
**client_name** | **String** | The name of the Client. | [optional] [default to null]
**client_username** | **String** | The username of the Client. | [optional] [default to null]
**idle_timeout** | **i32** | The number of seconds before an idle Transaction may be automatically rolled back and freed. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**replicated** | **bool** | Indicates whether the Transaction is replicated. | [optional] [default to null]
**session_name** | **String** | The name of the Transacted Session for the Transaction. | [optional] [default to null]
**state** | **String** | The state of the Transaction. The allowed values and their meaning are:  &lt;pre&gt; \&quot;active\&quot; - The Transaction was started. \&quot;suspended\&quot; - The Transaction was suspended. \&quot;idle\&quot; - The Transaction was ended. \&quot;prepared\&quot; - The Transaction was prepared. \&quot;complete\&quot; - The Transaction was committed or rolled back. &lt;/pre&gt;  | [optional] [default to null]
**time_in_state** | **i32** | The number of seconds the Transaction has remained in the current state. | [optional] [default to null]
**_type** | **String** | The type of Transaction. The allowed values and their meaning are:  &lt;pre&gt; \&quot;xa\&quot; - The Transaction is an XA Transaction. \&quot;local\&quot; - The Transaction is a local Transaction. &lt;/pre&gt;  | [optional] [default to null]
**xid** | **String** | The identifier (ID) of the Transaction. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


