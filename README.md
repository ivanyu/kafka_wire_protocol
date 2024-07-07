Apache Kafka protocol
=====================

Apache Kafka protocol implementations generated for different languages based on Kafka code. It's done the similar way as Kafka does for Java.

At the moment, it's implemented for Rust (see in [rust/](rust/) and [RustMessageGenerator.java](java/org/apache/kafka/message/RustMessageGenerator.java)).

## Java Tester

[Java Tester](java-tester/) is a piece of Java code that uses the original Kafka serialization/deserialization classes to check that what's generated Rust code produces is correct. Java Tester is taken from [kio](https://github.com/Aiven-Open/kio).
