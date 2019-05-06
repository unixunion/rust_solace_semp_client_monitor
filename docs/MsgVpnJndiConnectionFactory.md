# MsgVpnJndiConnectionFactory

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_duplicate_client_id_enabled** | **bool** | Indicates whether new JMS connections can use the same Client identifier (ID) as an existing connection. | [optional] [default to null]
**client_description** | **String** | The description of the Client. | [optional] [default to null]
**client_id** | **String** | The Client identifier (ID). If not specified, a unique value for it will be generated. | [optional] [default to null]
**connection_factory_name** | **String** | The name of the JMS Connection Factory. | [optional] [default to null]
**dto_receive_override_enabled** | **bool** | Indicates whether overriding by the Subscriber (Consumer) of the deliver-to-one (DTO) property on messages is enabled. When enabled, the Subscriber can receive all DTO tagged messages. | [optional] [default to null]
**dto_receive_subscriber_local_priority** | **i32** | The priority for receiving deliver-to-one (DTO) messages by the Subscriber (Consumer) if the messages are published on the local Router that the Subscriber is directly connected to. | [optional] [default to null]
**dto_receive_subscriber_network_priority** | **i32** | The priority for receiving deliver-to-one (DTO) messages by the Subscriber (Consumer) if the messages are published on a remote Router. | [optional] [default to null]
**dto_send_enabled** | **bool** | Indicates whether the deliver-to-one (DTO) property is enabled on messages sent by the Publisher (Producer). | [optional] [default to null]
**dynamic_endpoint_create_durable_enabled** | **bool** | Indicates whether a durable endpoint will be dynamically created on the Router when the client calls \&quot;Session.createDurableSubscriber()\&quot; or \&quot;Session.createQueue()\&quot;. The created endpoint respects the message time-to-live (TTL) according to the \&quot;dynamicEndpointRespectTtlEnabled\&quot; property. | [optional] [default to null]
**dynamic_endpoint_respect_ttl_enabled** | **bool** | Indicates whether dynamically created durable and non-durable endpoints respect the message time-to-live (TTL) property. | [optional] [default to null]
**guaranteed_receive_ack_timeout** | **i32** | The timeout for sending the acknowledgement (ACK) for guaranteed messages received by the Subscriber (Consumer), in milliseconds. | [optional] [default to null]
**guaranteed_receive_window_size** | **i32** | The size of the window for guaranteed messages received by the Subscriber (Consumer), in messages. | [optional] [default to null]
**guaranteed_receive_window_size_ack_threshold** | **i32** | The threshold for sending the acknowledgement (ACK) for guaranteed messages received by the Subscriber (Consumer) as a percentage of the \&quot;guaranteedReceiveWindowSize\&quot; value. | [optional] [default to null]
**guaranteed_send_ack_timeout** | **i32** | The timeout for receiving the acknowledgement (ACK) for guaranteed messages sent by the Publisher (Producer), in milliseconds. | [optional] [default to null]
**guaranteed_send_window_size** | **i32** | The size of the window for non-persistent guaranteed messages sent by the Publisher (Producer), in messages. For persistent messages the window size is fixed at 1. | [optional] [default to null]
**messaging_default_delivery_mode** | **String** | The default delivery mode for messages sent by the Publisher (Producer). The allowed values and their meaning are:  &lt;pre&gt; \&quot;persistent\&quot; - Router spools messages (persists in the Message Spool) as part of the send operation. \&quot;non-persistent\&quot; - Router does not spool messages (does not persist in the Message Spool) as part of the send operation. &lt;/pre&gt;  | [optional] [default to null]
**messaging_default_dmq_eligible_enabled** | **bool** | Indicates whether messages sent by the Publisher (Producer) are Dead Message Queue (DMQ) eligible by default. | [optional] [default to null]
**messaging_default_eliding_eligible_enabled** | **bool** | Indicates whether messages sent by the Publisher (Producer) are Eliding eligible by default. | [optional] [default to null]
**messaging_jmsx_user_id_enabled** | **bool** | Indicates whether to include (add or replace) the JMSXUserID property in messages sent by the Publisher (Producer). | [optional] [default to null]
**messaging_text_in_xml_payload_enabled** | **bool** | Indicates whether encoding of JMS text messages in Publisher (Producer) messages is as XML payload. When disabled, JMS text messages are encoded as a binary attachment. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**transport_compression_level** | **i32** | The ZLIB compression level for the connection to the Router. The value \&quot;0\&quot; means no compression, and the value \&quot;-1\&quot; means the compression level is specified in the JNDI Properties file. | [optional] [default to null]
**transport_connect_retry_count** | **i32** | The maximum number of retry attempts to establish an initial connection to the host (Router) or list of hosts (Routers). The value \&quot;0\&quot; means a single attempt (no retries), and the value \&quot;-1\&quot; means to retry forever. | [optional] [default to null]
**transport_connect_retry_per_host_count** | **i32** | The maximum number of retry attempts to establish an initial connection to each host (Router) on the list of hosts (Routers). The value \&quot;0\&quot; means a single attempt (no retries), and the value \&quot;-1\&quot; means to retry forever. | [optional] [default to null]
**transport_connect_timeout** | **i32** | The timeout for establishing an initial connection to the Router, in milliseconds. | [optional] [default to null]
**transport_direct_transport_enabled** | **bool** | Enable or disable usage of the Direct Transport mode for sending non-persistent messages. When disabled, the Guaranteed Transport mode is used. | [optional] [default to null]
**transport_keepalive_count** | **i32** | The maximum number of consecutive application-level keepalive messages sent without the Router response before the connection to the Router is closed. | [optional] [default to null]
**transport_keepalive_enabled** | **bool** | Indicates whether application-level keepalive messages are used to maintain a connection with the Router. | [optional] [default to null]
**transport_keepalive_interval** | **i32** | The interval between application-level keepalive messages, in milliseconds. | [optional] [default to null]
**transport_msg_callback_on_io_thread_enabled** | **bool** | Indicates whether delivery of asynchronous messages is done directly from the I/O thread. | [optional] [default to null]
**transport_optimize_direct_enabled** | **bool** | Indicates whether optimization for the Direct Transport delivery mode is enabled. If enabled, the client application is limited to one Publisher (Producer) and one non-durable Subscriber (Consumer). | [optional] [default to null]
**transport_port** | **i32** | The connection port number on the Router for SMF clients. The value \&quot;-1\&quot; means the port is specified in the JNDI Properties file. | [optional] [default to null]
**transport_read_timeout** | **i32** | The timeout for reading a reply from the Router, in milliseconds. | [optional] [default to null]
**transport_receive_buffer_size** | **i32** | The size of the receive socket buffer, in bytes. It corresponds to the SO_RCVBUF socket option. | [optional] [default to null]
**transport_reconnect_retry_count** | **i32** | The maximum number of attempts to reconnect to the host (Router) or list of hosts (Routers) after the connection has been lost. The value \&quot;-1\&quot; means to retry forever. | [optional] [default to null]
**transport_reconnect_retry_wait** | **i32** | The amount of time before making another attempt to connect or reconnect to the host (Router) after the connection has been lost, in milliseconds. | [optional] [default to null]
**transport_send_buffer_size** | **i32** | The size of the send socket buffer, in bytes. It corresponds to the SO_SNDBUF socket option. | [optional] [default to null]
**transport_tcp_no_delay_enabled** | **bool** | Indicates whether the TCP_NODELAY option is enabled, which disables Nagle&#39;s algorithm for TCP/IP congestion control (RFC 896). | [optional] [default to null]
**xa_enabled** | **bool** | Indicates whether this is an XA Connection Factory. When enabled, the Connection Factory can be cast to \&quot;XAConnectionFactory\&quot;, \&quot;XAQueueConnectionFactory\&quot; or \&quot;XATopicConnectionFactory\&quot;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


