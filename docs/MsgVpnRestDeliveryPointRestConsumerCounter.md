# MsgVpnRestDeliveryPointRestConsumerCounter

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**http_request_connection_close_tx_msg_count** | **i64** | The number of HTTP POST request messages transmitted to the REST Consumer to close the connection. | [optional] [default to null]
**http_request_outstanding_tx_msg_count** | **i64** | The number of HTTP POST request messages transmitted to the REST Consumer that are waiting for a response. | [optional] [default to null]
**http_request_timed_out_tx_msg_count** | **i64** | The number of HTTP POST request messages transmitted to the REST Consumer that have timed out. | [optional] [default to null]
**http_request_tx_byte_count** | **i64** | The amount of HTTP POST request messages transmitted to the REST Consumer, in bytes (B). | [optional] [default to null]
**http_request_tx_msg_count** | **i32** | The number of HTTP POST request messages transmitted to the REST Consumer. | [optional] [default to null]
**http_response_error_rx_msg_count** | **i64** | The number of HTTP POST client/server error response messages received from the REST Consumer. | [optional] [default to null]
**http_response_rx_byte_count** | **i64** | The amount of HTTP POST response messages received from the REST Consumer, in bytes (B). | [optional] [default to null]
**http_response_rx_msg_count** | **i64** | The number of HTTP POST response messages received from the REST Consumer. | [optional] [default to null]
**http_response_success_rx_msg_count** | **i64** | The number of HTTP POST successful response messages received from the REST Consumer. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


