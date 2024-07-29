.PHONY: java_tester
java_tester: build/install/java-tester/bin/java-tester

build/install/java-tester/bin/java-tester:
	./gradlew :java-tester:installDist

.PHONY: generate
generate: src/schema/mod.rs

src/schema/mod.rs:
	./gradlew processMessagesRust

.PHONY: test
test: generate java_tester
	cd rust; cargo test
