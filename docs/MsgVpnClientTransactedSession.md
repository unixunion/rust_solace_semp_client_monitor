# MsgVpnClientTransactedSession

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_name** | **String** | The name of the Client. | [optional] [default to null]
**commit_count** | **i64** | The number of transactions committed within the Transacted Session. | [optional] [default to null]
**commit_failure_count** | **i64** | The number of transaction commit operations that failed. | [optional] [default to null]
**commit_success_count** | **i64** | The number of transaction commit operations that succeeded. | [optional] [default to null]
**consumed_msg_count** | **i64** | The number of messages consumed within the Transacted Session. | [optional] [default to null]
**end_fail_failure_count** | **i64** | The number of transaction end fail operations that failed. | [optional] [default to null]
**end_fail_success_count** | **i64** | The number of transaction end fail operations that succeeded. | [optional] [default to null]
**end_failure_count** | **i64** | The number of transaction end operations that failed. | [optional] [default to null]
**end_rollback_failure_count** | **i64** | The number of transaction end rollback operations that failed. | [optional] [default to null]
**end_rollback_success_count** | **i64** | The number of transaction end rollback operations that succeeded. | [optional] [default to null]
**end_success_count** | **i64** | The number of transaction end operations that succeeded. | [optional] [default to null]
**failure_count** | **i64** | The number of transactions that failed within the Transacted Session. | [optional] [default to null]
**forget_failure_count** | **i64** | The number of transaction forget operations that failed. | [optional] [default to null]
**forget_success_count** | **i64** | The number of transaction forget operations that succeeded. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**one_phase_commit_failure_count** | **i64** | The number of transaction one-phase commit operations that failed. | [optional] [default to null]
**one_phase_commit_success_count** | **i64** | The number of transaction one-phase commit operations that succeeded. | [optional] [default to null]
**pending_consumed_msg_count** | **i32** | The number of messages to be consumed when the transaction is committed. | [optional] [default to null]
**pending_published_msg_count** | **i32** | The number of messages to be published when the transaction is committed. | [optional] [default to null]
**prepare_failure_count** | **i64** | The number of transaction prepare operations that failed. | [optional] [default to null]
**prepare_success_count** | **i64** | The number of transaction prepare operations that succeeded. | [optional] [default to null]
**previous_transaction_state** | **String** | The state of the previous transaction. The allowed values and their meaning are:  &lt;pre&gt; \&quot;none\&quot; - The previous transaction had no state. \&quot;committed\&quot; - The previous transaction was committed. \&quot;rolled-back\&quot; - The previous transaction was rolled back. \&quot;failed\&quot; - The previous transaction failed. &lt;/pre&gt;  | [optional] [default to null]
**published_msg_count** | **i64** | The number of messages published within the Transacted Session. | [optional] [default to null]
**resume_failure_count** | **i64** | The number of transaction resume operations that failed. | [optional] [default to null]
**resume_success_count** | **i64** | The number of transaction resume operations that succeeded. | [optional] [default to null]
**retrieved_msg_count** | **i64** | The number of messages retrieved within the Transacted Session. | [optional] [default to null]
**rollback_count** | **i64** | The number of transactions rolled back within the Transacted Session. | [optional] [default to null]
**rollback_failure_count** | **i64** | The number of transaction rollback operations that failed. | [optional] [default to null]
**rollback_success_count** | **i64** | The number of transaction rollback operations that succeeded. | [optional] [default to null]
**session_name** | **String** | The name of the Transacted Session. | [optional] [default to null]
**spooled_msg_count** | **i64** | The number of messages spooled within the Transacted Session. | [optional] [default to null]
**start_failure_count** | **i64** | The number of transaction start operations that failed. | [optional] [default to null]
**start_success_count** | **i64** | The number of transaction start operations that succeeded. | [optional] [default to null]
**success_count** | **i64** | The number of transactions that succeeded within the Transacted Session. | [optional] [default to null]
**suspend_failure_count** | **i64** | The number of transaction suspend operations that failed. | [optional] [default to null]
**suspend_success_count** | **i64** | The number of transaction suspend operations that succeeded. | [optional] [default to null]
**transaction_id** | **i32** | The identifier (ID) of the transaction in the Transacted Session. | [optional] [default to null]
**transaction_state** | **String** | The state of the current transaction. The allowed values and their meaning are:  &lt;pre&gt; \&quot;in-progress\&quot; - The current transaction is in progress. \&quot;committing\&quot; - The current transaction is committing. \&quot;rolling-back\&quot; - The current transaction is rolling back. \&quot;failing\&quot; - The current transaction is failing. &lt;/pre&gt;  | [optional] [default to null]
**two_phase_commit_failure_count** | **i64** | The number of transaction two-phase commit operations that failed. | [optional] [default to null]
**two_phase_commit_success_count** | **i64** | The number of transaction two-phase commit operations that succeeded. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


