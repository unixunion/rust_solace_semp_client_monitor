# MsgVpnClientRxFlow

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_name** | **String** | The name of the Client. | [optional] [default to null]
**connect_time** | **i32** | The timestamp of when the Flow from the Client connected. | [optional] [default to null]
**destination_group_error_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to a destination group error. | [optional] [default to null]
**duplicate_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to being a duplicate. | [optional] [default to null]
**endpoint_disabled_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to an eligible endpoint destination being disabled. | [optional] [default to null]
**endpoint_usage_exceeded_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to an eligible endpoint destination having its maximum message spool usage exceeded. | [optional] [default to null]
**errored_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to errors being detected. | [optional] [default to null]
**flow_id** | **i64** | The identifier (ID) of the flow. | [optional] [default to null]
**flow_name** | **String** | The name of the Flow. | [optional] [default to null]
**guaranteed_msg_count** | **i64** | The number of guaranteed messages from the Flow. | [optional] [default to null]
**last_rx_msg_id** | **i64** | The identifier (ID) of the last message received on the Flow. | [optional] [default to null]
**local_msg_count_exceeded_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to the maximum number of messages allowed on the broker being exceeded. | [optional] [default to null]
**low_priority_msg_congestion_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to congestion of low priority messages. | [optional] [default to null]
**max_msg_size_exceeded_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to the maximum allowed message size being exceeded. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**no_eligible_destinations_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to there being no eligible endpoint destination. | [optional] [default to null]
**no_local_delivery_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to no local delivery being requested. | [optional] [default to null]
**not_compatible_with_forwarding_mode_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to being incompatible with the forwarding mode of an eligible endpoint destination. | [optional] [default to null]
**out_of_order_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to being received out of order. | [optional] [default to null]
**publish_acl_denied_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to being denied by the access control list (ACL) profile for the published topic. | [optional] [default to null]
**publisher_id** | **i64** | The identifier (ID) of the publisher for the Flow. | [optional] [default to null]
**queue_not_found_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to the destination queue not being found. | [optional] [default to null]
**replication_standby_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to the Message VPN being in the replication standby state. | [optional] [default to null]
**session_name** | **String** | The name of the transacted session on the Flow. | [optional] [default to null]
**smf_ttl_exceeded_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to the message time-to-live (TTL) count being exceeded. The message TTL count is the maximum number of times the message can cross a bridge between Message VPNs. | [optional] [default to null]
**spool_file_limit_exceeded_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to all available message spool file resources being used. | [optional] [default to null]
**spool_not_ready_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to the message spool being not ready. | [optional] [default to null]
**spool_to_adb_fail_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to a failure while spooling to the Assured Delivery Blade (ADB). | [optional] [default to null]
**spool_to_disk_fail_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to a failure while spooling to the disk. | [optional] [default to null]
**spool_usage_exceeded_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to the maximum message spool usage being exceeded. | [optional] [default to null]
**sync_replication_ineligible_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to synchronous replication being ineligible. | [optional] [default to null]
**user_profile_denied_guaranteed_discarded_msg_count** | **i64** | The number of guaranteed messages from the Flow discarded due to being denied by the client profile. | [optional] [default to null]
**window_size** | **i32** | The size of the window used for guaranteed messages sent on the Flow, in messages. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


