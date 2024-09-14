# Apache Kafka wire protocol

Apache Kafka protocol implementations generated for different languages based on Kafka code.

## What is it and how it works

The wire protocol of Apache Kafka is defined as a set of [JSON files](clients/src/main/resources/common/message). There is a Gradle task `processMessages` which converts these JSON definitions into actual Java classes. This project aims to do the same, but for other programming languages. At the moment, its focus is Rust, but Go is also planned. (For Python, have a look at [Kio](https://github.com/Aiven-open/kio)).

The library contains the protocol messages and some convenience code, but this is not a complete client (or server) implementation. 

This project contains the unaltered code of the `clients` module and slightly altered code of `generator` module from the [Apache Kafka code base](https://github.com/apache/kafka). The additions are following:

1. The supporting Rust code and non-generated tests, located in [rust/](rust/).
2. The Rust examples, located in [rust_examples/](rust_examples/).
3. [RustMessageGenerator.java](java/org/apache/kafka/message/RustMessageGenerator.java) and its supporting classes with the `Rust` prefixes, which contain the actual generation logic.
4. The `processMessagesRust` Gradle task for running the generator.
5. [Java Tester](java-tester/) for testing against the real Kafka Java code (see below).

### Testing

Serialization/deserialization is the crucial part of the protocol implementation, which must be correct for the library to be useful. The library is tested on several levels:
1. Basic handwritten unit tests.
2. Pure Rust serialization/deserialization property-based tests with [proptest](https://crates.io/crates/proptest).
3. Serialization/deserialization property-based tests that run against the real Java Kafka code. Their purpose is to ensure that the generated Rust code and the upstream Java code understand the protocol completely identical. See the [Java Tester](#testing-with-java-tester) section below.
4. Fuzzing tests for deserialization.
5. Integration tests against a real Kafka instance running in Docker.

#### Testing with Java Tester

[Java Tester](java-tester/) is a piece of Java code that uses the original Kafka serialization/deserialization classes to check that what's generated Rust code produces is correct. Java Tester is taken from [Kio](https://github.com/Aiven-Open/kio).

The Rust test code runs Java Tester and sends test cases in the JSON format to its standard input, expecting the result at the standard output. The result is either a success or a failure with the accompanying error message and/or stack trace.

Test functions use the property-based testing generator to generate random protocol message structures, serialize them in the binary and JSON formats, and send them to Java Tester. The latter reconstructs the message based on the JSON representation, serializes it and compares that the serialized value is identical to what Rust produced, thus ensuring the Rust code does serialization correctly.

See [java_tester.rs](rust/src/test_utils/java_tester.rs) for details.

## Development

Run
```shell
make test
```
to run the generator and execute the tests.

## License

This code base--both the forked Kafka code and the original code--is provided under the Apache 2.0 license. 
