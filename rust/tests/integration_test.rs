use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use paste::paste;
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::runners::SyncRunner;
use testcontainers::GenericImage;

use kafka_wire_protocol::api_message_type::ApiMessageType;
use kafka_wire_protocol::markers::{Request, Response};
use kafka_wire_protocol::readable_writable::{Readable, Writable};

struct Connection {
    stream: TcpStream,
    client_id: Option<String>,
    correlation_id: i32,
}

impl Connection {
    fn new<A: ToSocketAddrs>(addr: A) -> Self {
        Connection {
            stream: TcpStream::connect(addr).unwrap(),
            client_id: Some(String::from("test_client")),
            correlation_id: 0,
        }
    }

    fn send_request<TReq, TResp>(&mut self, request: TReq) -> TResp
    where
        TReq: Request + Writable,
        TResp: Response + Readable,
    {
        let api_message_type: ApiMessageType = ApiMessageType::from_api_key(request.api_key());
        let request_api_version = request.version();

        let mut cur: Cursor<Vec<u8>> = Cursor::<Vec<u8>>::new(Vec::new());

        cur.write_i32::<BigEndian>(0).unwrap(); // size placeholder

        match api_message_type.request_header_version(request_api_version) {
            1 => {
                let header = kafka_wire_protocol::schema::request_header::v1::RequestHeader
                ::new(
                    api_message_type.api_key,
                    request_api_version,
                    self.correlation_id,
                    self.client_id.clone(),
                );
                header.write(&mut cur).unwrap();
            }

            2 => {
                let header = kafka_wire_protocol::schema::request_header::v2::RequestHeader
                ::new(
                    api_message_type.api_key,
                    request_api_version,
                    self.correlation_id,
                    self.client_id.clone(),
                );
                header.write(&mut cur).unwrap();
            }

            v => panic!("Unexpected version {v}")
        };
        self.correlation_id += 1;

        request.write(&mut cur).unwrap();

        // Write the real size on top of the placeholder.
        let size = (cur.position() - 4) as i32;
        cur.seek(SeekFrom::Start(0)).unwrap();
        cur.write_i32::<BigEndian>(size).unwrap();

        self.stream.write(cur.get_ref()).unwrap();
        self.stream.flush().unwrap();

        let response_size = self.stream.read_i32::<BigEndian>().unwrap() as usize;
        let mut response_buf = vec![0; response_size];
        let read_size = self.stream.read(&mut response_buf).unwrap();
        assert_eq!(read_size, response_size);

        let mut response_cur = Cursor::new(response_buf);
        let resp_correlation_id = match api_message_type.response_header_version(request_api_version) {
            0 =>
                kafka_wire_protocol::schema::response_header::v0::ResponseHeader
                ::read(&mut response_cur).unwrap().correlation_id,

            1 =>
                kafka_wire_protocol::schema::response_header::v1::ResponseHeader
                ::read(&mut response_cur).unwrap().correlation_id,

            v => panic!("Unexpected version {v}")
        };
        assert_eq!(resp_correlation_id, self.correlation_id - 1);

        TResp::read(&mut response_cur).unwrap()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // We don't care about possible errors.
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}

const KAFKA_VERSION: &str = "4.0.0";
const KAFKA_PORT: ContainerPort = ContainerPort::Tcp(9092);

#[test]
fn test_integration() {
    let container = GenericImage::new("apache/kafka", KAFKA_VERSION)
        .with_exposed_port(KAFKA_PORT)
        .with_wait_for(WaitFor::message_on_stdout("Kafka Server started"))
        .start()
        .unwrap();
    let port = container.get_host_port_ipv4(KAFKA_PORT).unwrap();
    let mut connection = Connection::new(format!("127.0.0.1:{port}"));

    test_api_versions(&mut connection);
    test_create_topics(&mut connection);
    test_alter_configs(&mut connection);
    test_describe_configs(&mut connection);
    test_metadata(&mut connection);
}

fn test_api_versions(connection: &mut Connection) {
    macro_rules! check {
        ($response: ident) => {
            assert_eq!($response.error_code, 0);
            assert!($response.api_keys.len() > 0);
        };
    }

    macro_rules! test_api_versions_v0_v2 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::api_versions_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::api_versions_response::[<v $version>]::*;
                }

                let request = ApiVersionsRequest::new();
                let response: ApiVersionsResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    macro_rules! test_api_versions_v3_v4 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::api_versions_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::api_versions_response::[<v $version>]::*;
                }

                let request = ApiVersionsRequest::new("client".to_string(), "123".to_string());
                let response: ApiVersionsResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    test_api_versions_v0_v2!(0);
    test_api_versions_v0_v2!(1);
    test_api_versions_v0_v2!(2);
    test_api_versions_v3_v4!(3);
    test_api_versions_v3_v4!(4);
}

fn test_create_topics(connection: &mut Connection) {
    macro_rules! check {
        ($response: ident) => {
            assert_eq!($response.topics.len(), 1);
            for t in $response.topics {
                assert_eq!(t.error_code, 0);
            }
        };
    }

    macro_rules! test_create_topics_v2_v7 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::create_topics_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::create_topics_response::[<v $version>]::*;
                }

                let request = CreateTopicsRequest::new(
                    vec![
                        CreatableTopic::new(
                            format!("topic{}", stringify!($version)),
                            1, 1, vec![], vec![],
                        )
                    ],
                    10_000, false);
                let response: CreateTopicsResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    test_create_topics_v2_v7!(2);
    test_create_topics_v2_v7!(3);
    test_create_topics_v2_v7!(4);
    test_create_topics_v2_v7!(5);
    test_create_topics_v2_v7!(6);
    test_create_topics_v2_v7!(7);
}

fn test_alter_configs(connection: &mut Connection) {
    macro_rules! check {
        ($response: ident) => {
            assert_eq!($response.responses.len(), 1);
        };
    }

    macro_rules! test_alter_configs_v0_v2 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::alter_configs_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::alter_configs_response::[<v $version>]::*;
                }

                let request = AlterConfigsRequest::new(vec![
                    AlterConfigsResource::new(
                        2,
                        "topic0",
                        vec![AlterableConfig::new("retention.ms", Some("10"))]
                    )
                ], true);
                let response: AlterConfigsResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    test_alter_configs_v0_v2!(0);
    test_alter_configs_v0_v2!(1);
    test_alter_configs_v0_v2!(2);
}

fn test_describe_configs(connection: &mut Connection) {
    macro_rules! check {
        ($response: ident) => {
            assert!($response.results.len() > 0);
            assert_eq!($response.results[0].error_code, 0);
        };
    }

    macro_rules! test_describe_configs_v1_v2 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::describe_configs_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::describe_configs_response::[<v $version>]::*;
                }

                let request = DescribeConfigsRequest::new(vec![
                    DescribeConfigsResource::new(2, String::from("topic2"), None)
                ], true);
                let response: DescribeConfigsResponse = connection.send_request(request);
                if (response.results[0].error_code != 0) {
                    println!("{:?}", response);
                }
                check!(response);
            }
        }
    }

    macro_rules! test_describe_configs_v3_v4 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::describe_configs_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::describe_configs_response::[<v $version>]::*;
                }

                let request = DescribeConfigsRequest::new(vec![
                    DescribeConfigsResource::new(2, String::from("topic2"), None)
                ], true, true);
                let response: DescribeConfigsResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    test_describe_configs_v1_v2!(1);
    test_describe_configs_v1_v2!(2);
    test_describe_configs_v3_v4!(3);
    test_describe_configs_v3_v4!(4);
}

fn test_metadata(connection: &mut Connection) {
    macro_rules! check {
        ($response: ident) => {
            assert_eq!($response.brokers.len(), 1);
            assert!($response.topics.len() > 1);
        };
    }

    macro_rules! test_metadata_v0 {
        (0) => {
            {
                use kafka_wire_protocol::schema::metadata_request::v0::*;
                use kafka_wire_protocol::schema::metadata_response::v0::*;

                let request = MetadataRequest::new(vec![]);
                let response: MetadataResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    macro_rules! test_metadata_v1_v3 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::metadata_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::metadata_response::[<v $version>]::*;
                }

                let request = MetadataRequest::new(None);
                let response: MetadataResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    macro_rules! test_metadata_v4_v7 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::metadata_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::metadata_response::[<v $version>]::*;
                }

                let request = MetadataRequest::new(None, false);
                let response: MetadataResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    macro_rules! test_metadata_v8_v10 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::metadata_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::metadata_response::[<v $version>]::*;
                }

                let request = MetadataRequest::new(None, false, true, true);
                let response: MetadataResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    macro_rules! test_metadata_v11_v13 {
        ($version: literal) => {
            {
                paste! {
                    use kafka_wire_protocol::schema::metadata_request::[<v $version>]::*;
                    use kafka_wire_protocol::schema::metadata_response::[<v $version>]::*;
                }

                let request = MetadataRequest::new(None, false, true);
                let response: MetadataResponse = connection.send_request(request);
                check!(response);
            }
        }
    }

    test_metadata_v0!(0);
    test_metadata_v1_v3!(1);
    test_metadata_v1_v3!(2);
    test_metadata_v1_v3!(3);
    test_metadata_v4_v7!(4);
    test_metadata_v4_v7!(5);
    test_metadata_v4_v7!(6);
    test_metadata_v4_v7!(7);
    test_metadata_v8_v10!(8);
    test_metadata_v8_v10!(9);
    test_metadata_v8_v10!(10);
    test_metadata_v11_v13!(11);
    test_metadata_v11_v13!(12);
    test_metadata_v11_v13!(13);
}
