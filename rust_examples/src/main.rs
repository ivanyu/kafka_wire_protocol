use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use kafka_wire_protocol::api_message_type::ApiMessageType;
use kafka_wire_protocol::markers::ApiMessage;
use kafka_wire_protocol::readable_writable::{Readable, Writable};
use kafka_wire_protocol::schema::api_versions_request::v3::ApiVersionsRequest;
use kafka_wire_protocol::schema::api_versions_response::v3::ApiVersionsResponse;
use kafka_wire_protocol::schema::request_header::v2::RequestHeader;
use kafka_wire_protocol::schema::response_header::v0::ResponseHeader;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::net::{Shutdown, TcpStream};
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::runners::SyncRunner;
use testcontainers::GenericImage;

const KAFKA_VERSION: &str = "3.9.0";
const KAFKA_PORT: ContainerPort = ContainerPort::Tcp(9092);

fn main() {
    // Start Kafka in a container.
    let container = GenericImage::new("apache/kafka", KAFKA_VERSION)
        .with_exposed_port(KAFKA_PORT)
        .with_wait_for(WaitFor::message_on_stdout("Kafka Server started"))
        .start()
        .unwrap();
    let port = container.get_host_port_ipv4(KAFKA_PORT).unwrap();

    // Connect to Kafka.
    let mut tcp_stream = TcpStream::connect(
        format!("127.0.0.1:{port}")).unwrap();

    // Prepare the request buffer.
    let mut request_cur: Cursor<Vec<u8>> = Cursor::<Vec<u8>>::new(Vec::new());
    request_cur.write_i32::<BigEndian>(0).unwrap(); // size placeholder

    let request = ApiVersionsRequest::new("client", "123");
    let api_message_type: ApiMessageType = ApiMessageType::from_api_key(request.api_key());

    let request_header_version = api_message_type.request_header_version(request.version());
    println!("Request header version: {}", request_header_version);
    assert_eq!(request_header_version, 2);

    let response_header_version = api_message_type.response_header_version(request.version());
    println!("Request header version: {}", response_header_version);
    assert_eq!(response_header_version, 0);

    // Write the header.
    let correlation_id = 1234;
    let client_id = "test-client";
    let request_header = RequestHeader::new(
        request.api_key(),
        request.version(),
        correlation_id,
        Some(client_id),
    );
    request_header.write(&mut request_cur).unwrap();

    // Write the request.
    request.write(&mut request_cur).unwrap();

    // Write the real size on top of the placeholder.
    let size = (request_cur.position() - 4) as i32;
    request_cur.seek(SeekFrom::Start(0)).unwrap();
    request_cur.write_i32::<BigEndian>(size).unwrap();

    // Send the request to Kafka.
    tcp_stream.write(request_cur.get_ref()).unwrap();
    tcp_stream.flush().unwrap();

    // Read the response from the socket.
    let response_size = tcp_stream.read_i32::<BigEndian>().unwrap() as usize;
    let mut response_buf = vec![0; response_size];
    let read_size = tcp_stream.read(&mut response_buf).unwrap();
    assert_eq!(read_size, response_size);
    let mut response_cur = Cursor::new(response_buf);

    // Read the response header.
    let response_header = ResponseHeader::read(&mut response_cur).unwrap();
    assert_eq!(response_header.correlation_id, correlation_id);
    println!("{:?}", response_header);

    // Read the response.
    let response = ApiVersionsResponse::read(&mut response_cur).unwrap();
    println!("{:?}", response);

    // Close the socket.
    let _ = tcp_stream.shutdown(Shutdown::Both);
}
