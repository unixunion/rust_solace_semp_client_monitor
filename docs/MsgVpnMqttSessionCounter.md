# MsgVpnMqttSessionCounter

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mqtt_connack_error_tx_count** | **i64** | The number of MQTT connect acknowledgment (CONNACK) refused response packets transmitted to the Client. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_connack_tx_count** | **i64** | The number of MQTT connect acknowledgment (CONNACK) accepted response packets transmitted to the Client. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_connect_rx_count** | **i32** | The number of MQTT connect (CONNECT) request packets received from the Client. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_disconnect_rx_count** | **i64** | The number of MQTT disconnect (DISCONNECT) request packets received from the Client. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_pubcomp_tx_count** | **i64** | The number of MQTT publish complete (PUBCOMP) packets transmitted to the Client in response to a PUBREL packet. These packets are the fourth and final packet of a QoS 2 protocol exchange. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_publish_qos0_rx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 0 message delivery. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_publish_qos0_tx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets transmitted to the Client for QoS 0 message delivery. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_publish_qos1_rx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 1 message delivery. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_publish_qos1_tx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets transmitted to the Client for QoS 1 message delivery. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_publish_qos2_rx_count** | **i64** | The number of MQTT publish message (PUBLISH) request packets received from the Client for QoS 2 message delivery. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_pubrec_tx_count** | **i64** | The number of MQTT publish received (PUBREC) packets transmitted to the Client in response to a PUBLISH packet with QoS 2. These packets are the second packet of a QoS 2 protocol exchange. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]
**mqtt_pubrel_rx_count** | **i64** | The number of MQTT publish release (PUBREL) packets received from the Client in response to a PUBREC packet. These packets are the third packet of a QoS 2 protocol exchange. Deprecated since 2.16. This attribute has been moved to the MsgVpnMqttSession object. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


