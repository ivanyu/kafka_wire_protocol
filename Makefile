.PHONY: test
test: build/install/java-tester/bin/java-tester src/schema/mod.rs
	cd rust; cargo test

build/install/java-tester/bin/java-tester:
	./gradlew :java-tester:installDist

src/schema/mod.rs:
	./gradlew processMessagesRust
