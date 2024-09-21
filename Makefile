.PHONY: clean
clean:
	./gradlew clean

.PHONY: clean_generated
clean_generated:
	rm -rf rust/src/schema
	rm -rf rust/src/api_message_type.rs

.PHONY: clean_all
clean_all: clean clean_generated

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

.PHONY: doc
doc:
	cd rust; cargo doc
