//! This library is a generated implementation of Apache Kafka wire protocol.
//!
//! The library contains the protocol messages and some convenience code,
//! but this is not a complete client (or server) implementation.
//!
//! The message structs and the corresponding serialization and deserialization code are generated
//! based on the Kafka JSON definitions.
//!
//! Message versions up to Kafka 3.8.0 are supported at the moment.
//!
//! # Message schema
//!
//! Kafka has a number of request-response messages with versions.
//! In the [`schema`] module you can find particular message types of particular versions.
//!
//! # Usage
//!
//! ## Serialization
//! [`Writable`](crate::readable_writable::Writable) is implemented for each message structure,
//! so they have the [`write`](crate::readable_writable::Writable::write) function.
//! This function takes an [`Write`](std::io::Write) instance, for example, [`Cursor`](std::io::Cursor).
//!
//! ```no_run
//! use std::io::Cursor;
//! use kafka_wire_protocol::readable_writable::Writable;
//! use kafka_wire_protocol::schema::api_versions_request::v3::ApiVersionsRequest;
//!
//! let mut request_cur: Cursor<Vec<u8>> = Cursor::new(Vec::new());
//! let request = ApiVersionsRequest::new("client", "123");
//! request.write(&mut request_cur).unwrap();
//! ```
//!
//! ## Deserialization
//! [`Readable`](crate::readable_writable::Readable) is implemented for each message structure,
//! so they have the [`read`](crate::readable_writable::Readable::read) function.
//! This function takes an [`Read`](std::io::Read) instance, for example, [`Cursor`](std::io::Cursor).
//!
//! ```no_run
//! use std::io::Cursor;
//! use kafka_wire_protocol::readable_writable::Readable;
//! use kafka_wire_protocol::schema::api_versions_response::v3::ApiVersionsResponse;
//!
//! let mut response_buf = vec![0; 100];  // real buffer comes e.g. from network
//! let mut response_cur = Cursor::new(response_buf);
//! let response = ApiVersionsResponse::read(&mut response_cur).unwrap();
//! ```
//!
//! ## Request and response header versions
//!
//! [`ApiMessageType`](crate::api_message_type::ApiMessageType) is an utility struct
//! that helps programmatically get the versions of request and response headers required for
//! the particular message type and version.
//!
//! ```
//! use kafka_wire_protocol::api_message_type::ApiMessageType;
//!
//! let api_message_type: ApiMessageType = ApiMessageType::from_api_key(0);
//! assert_eq!(api_message_type.request_header_version(1), 1);
//! assert_eq!(api_message_type.response_header_version(1), 0);
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
//! use kafka_wire_protocol::api_message_type::ApiMessageType;
//! use kafka_wire_protocol::markers::ApiMessage;
//! use kafka_wire_protocol::readable_writable::{Readable, Writable};
//! use kafka_wire_protocol::schema::api_versions_request::v3::ApiVersionsRequest;
//! use kafka_wire_protocol::schema::api_versions_response::v3::ApiVersionsResponse;
//! use kafka_wire_protocol::schema::request_header::v2::RequestHeader;
//! use kafka_wire_protocol::schema::response_header::v0::ResponseHeader;
//! use std::io::{Cursor, Read, Seek, SeekFrom, Write};
//! use std::net::{Shutdown, TcpStream};
//!
//! // Connect to Kafka.
//! let mut tcp_stream = TcpStream::connect("127.0.0.1:9092").unwrap();
//!
//! // Prepare the request buffer.
//! let mut request_cur: Cursor<Vec<u8>> = Cursor::<Vec<u8>>::new(Vec::new());
//! request_cur.write_i32::<BigEndian>(0).unwrap(); // size placeholder
//!
//! let request = ApiVersionsRequest::new("client", "123");
//! let api_message_type: ApiMessageType = ApiMessageType::from_api_key(request.api_key());
//!
//! let request_header_version = api_message_type.request_header_version(request.version());
//! println!("Request header version: {}", request_header_version);
//! assert_eq!(request_header_version, 2);
//!
//! let response_header_version = api_message_type.response_header_version(request.version());
//! println!("Request header version: {}", response_header_version);
//! assert_eq!(response_header_version, 0);
//!
//! // Write the header.
//! let correlation_id = 1234;
//! let client_id = "test-client";
//! let request_header = RequestHeader::new(
//!     request.api_key(),
//!     request.version(),
//!     correlation_id,
//!     Some(client_id),
//! );
//! request_header.write(&mut request_cur).unwrap();
//!
//! // Write the request.
//! request.write(&mut request_cur).unwrap();
//!
//! // Write the real size on top of the placeholder.
//! let size = (request_cur.position() - 4) as i32;
//! request_cur.seek(SeekFrom::Start(0)).unwrap();
//! request_cur.write_i32::<BigEndian>(size).unwrap();
//!
//! // Send the request to Kafka.
//! tcp_stream.write(request_cur.get_ref()).unwrap();
//! tcp_stream.flush().unwrap();
//!
//! // Read the response from the socket.
//! let response_size = tcp_stream.read_i32::<BigEndian>().unwrap() as usize;
//! let mut response_buf = vec![0; response_size];
//! let read_size = tcp_stream.read(&mut response_buf).unwrap();
//! assert_eq!(read_size, response_size);
//! let mut response_cur = Cursor::new(response_buf);
//!
//! // Read the response header.
//! let response_header = ResponseHeader::read(&mut response_cur).unwrap();
//! assert_eq!(response_header.correlation_id, correlation_id);
//! println!("{:?}", response_header);
//!
//! // Read the response.
//! let response = ApiVersionsResponse::read(&mut response_cur).unwrap();
//! println!("{:?}", response);
//!
//! // Close the socket.
//! let _ = tcp_stream.shutdown(Shutdown::Both);
//! ```

#[cfg(test)] mod test_utils;
pub mod schema;
pub mod api_message_type;
pub mod tagged_fields;
pub mod markers;
pub mod readable_writable;
mod arrays;
mod primitives;
mod strings;
mod bytes;
mod utils;
