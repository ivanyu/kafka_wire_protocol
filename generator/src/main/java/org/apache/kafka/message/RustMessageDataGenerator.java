package org.apache.kafka.message;

import java.io.BufferedWriter;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class RustMessageDataGenerator {
    private final MessageSpec message;
    private final short version;
    private final StructRegistry structRegistry = new StructRegistry();
    private final RustHeaderGenerator headerGenerator = new RustHeaderGenerator();
    private final CodeBuffer buffer = new CodeBuffer();

    public RustMessageDataGenerator(MessageSpec message, short version) {
        if (!message.validVersions().contains(version)) {
            throw new RuntimeException("Unsupported version: " + version);
        }
        if (message.struct().versions().contains(Short.MAX_VALUE)) {
            throw new RuntimeException("Message " + message.name() + " does " +
                    "not specify a maximum version.");
        }
        this.message = message;
        this.version = version;
    }

    public void generateAndWrite(BufferedWriter writer) throws Exception {
        generate();
        write(writer);
    }

    private void generate() throws Exception {
        structRegistry.register(message);

        String className = message.dataClassName();
        if (className.endsWith("Data")) {
            className = className.substring(0, className.length() - 4);
        }

        generateClass(true, className, message.struct());

        headerGenerator.generate();
    }

    private void generateClass(boolean isTopLevel,
                               String className,
                               StructSpec struct) throws Exception {
        headerGenerator.addImport("serde::Serialize");
        headerGenerator.addImport("serde::Deserialize");
        headerGenerator.addImportTest("proptest_derive::Arbitrary");

        buffer.printf("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]%n");
        buffer.printf("#[cfg_attr(test, derive(Arbitrary))]%n");
        buffer.printf("pub struct %s {%n", className);
        buffer.incrementIndent();
        generateFieldDeclarations(struct);
        buffer.decrementIndent();
        buffer.printf("}%n");
        buffer.printf("%n");

        switch (message.type()) {
            case REQUEST:
                headerGenerator.addImport("crate::markers::ApiMessage");
                headerGenerator.addImport("crate::markers::Request");

                generateApiMessageImpl(className);
                buffer.printf("%n");

                buffer.printf("impl Request for %s { }%n%n", className);
                break;

            case RESPONSE:
                headerGenerator.addImport("crate::markers::ApiMessage");
                headerGenerator.addImport("crate::markers::Response");

                generateApiMessageImpl(className);
                buffer.printf("%n");

                buffer.printf("impl Response for %s { }%n%n", className);
                break;

            case HEADER:
                headerGenerator.addImport("crate::markers::ApiMessage");
                headerGenerator.addImport("crate::markers::Header");

                generateApiMessageImpl(className);
                buffer.printf("%n");

                buffer.printf("impl Header for %s { }%n%n", className);
                break;

            case METADATA:
                throw new Exception("not expected");

            case DATA:
                headerGenerator.addImport("crate::markers::ApiMessage");
                headerGenerator.addImport("crate::markers::Data");

                generateApiMessageImpl(className);
                buffer.printf("%n");

                buffer.printf("impl Data for %s { }%n%n", className);
                break;
        }

        generateClassDefault(className, struct);
        buffer.printf("%n");
        generateClassConstructor(className, struct);
        buffer.printf("%n");
        generateClassReader(className, struct);
        buffer.printf("%n");
        generateClassWriter(className, struct);
        buffer.printf("%n");

        generateSubclasses(struct);

        if (isTopLevel) {
            for (Iterator<StructSpec> iter = structRegistry.commonStructs(); iter.hasNext(); ) {
                StructSpec commonStruct = iter.next();
                generateClass(false, commonStruct.name(), commonStruct);
            }

            buffer.printf("#[cfg(test)]%n");
            buffer.printf("mod tests {%n");
            buffer.incrementIndent();
            buffer.printf("use super::*;%n");
            buffer.printf("use proptest::prelude::*;%n");
            buffer.printf("%n");

            buffer.printf("#[test]%n");
            buffer.printf("fn test_java_default() {%n");
            buffer.incrementIndent();
            buffer.printf("crate::test_utils::test_java_default::<%s>(\"%s\", %d);%n", className, className, version);
            buffer.decrementIndent();
            buffer.printf("}%n");

            buffer.printf("%n");

            buffer.printf("proptest! {%n");
            buffer.incrementIndent();
            buffer.printf("#[test]%n");
            buffer.printf("fn test_serde(data: %s) {%n", className);
            buffer.incrementIndent();
            buffer.printf("crate::test_utils::test_serde(&data)?;%n");
            buffer.decrementIndent();
            buffer.printf("}%n");
            buffer.decrementIndent();
            buffer.printf("}%n");

            buffer.printf("%n");

            buffer.printf("proptest! {%n");
            buffer.incrementIndent();
            buffer.printf("#[test]%n");
            buffer.printf("fn test_java_arbitrary(data: %s) {%n", className);
            buffer.incrementIndent();
            buffer.printf("crate::test_utils::test_java_arbitrary(&data, \"%s\", %d);%n", className, version);
            buffer.decrementIndent();
            buffer.printf("}%n");
            buffer.decrementIndent();
            buffer.printf("}%n");

            buffer.decrementIndent();

            buffer.printf("}%n");
        }
    }

    private void generateApiMessageImpl(String className) {
        buffer.printf("impl ApiMessage for %s {%n", className);
        buffer.incrementIndent();

        buffer.printf("fn api_key(&self) -> i16 {%n");
        buffer.incrementIndent();
        buffer.printf("%d%n", message.apiKey().orElseGet(() -> (short) -1));
        buffer.decrementIndent();
        buffer.printf("}%n");

        buffer.printf("%n");

        buffer.printf("fn version(&self) -> i16 {%n");
        buffer.incrementIndent();
        buffer.printf("%d%n", version);
        buffer.decrementIndent();
        buffer.printf("}%n");

        buffer.decrementIndent();
        buffer.printf("}%n");
    }

    private void generateFieldDeclarations(StructSpec struct) {
        for (FieldSpec field : struct.fields()) {
            if (!field.versions().contains(version)) {
                continue;
            }

            RustFieldSpecAdaptor rustFieldSpecAdaptor = new RustFieldSpecAdaptor(field, version, headerGenerator);
            RustType type = rustFieldSpecAdaptor.fieldType();
            buffer.printf("/// %s%n", field.about());
            type.writeDeclaration(rustFieldSpecAdaptor.fieldName(), headerGenerator, buffer);
        }

        if (hasTaggedFields()) {
            headerGenerator.addImport("crate::tagged_fields::RawTaggedField");
            headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
            buffer.printf("/// Unknown tagged fields%n");
            buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::unknown_tagged_fields()\"))]%n");
            buffer.printf("pub _unknown_tagged_fields: Vec<RawTaggedField>,%n");
        }
    }

    private void generateClassConstructor(String className, StructSpec struct) {
        buffer.printf("impl %s {%n", className);
        buffer.incrementIndent();

        int stringParamCounter = 0;
        List<String> fieldsForConstructor = new ArrayList<>();
        List<String> fieldsForConstructorWithTypes = new ArrayList<>();
        List<String> valuesForTest = new ArrayList<>();
        for (FieldSpec field : struct.fields()) {
            if (!field.versions().contains(version)) {
                continue;
            }

            RustFieldSpecAdaptor rustFieldSpecAdaptor = new RustFieldSpecAdaptor(field, version, headerGenerator);
            String fieldNameInRust = rustFieldSpecAdaptor.fieldName();
            RustType type = rustFieldSpecAdaptor.fieldType();

            String fieldDefault = rustFieldSpecAdaptor.fieldDefault();
            if (type instanceof RustTypeOption
                && fieldDefault.equals("None")) {
                fieldDefault = String.format("%s::<%s>", fieldDefault, ((RustTypeOption) type).inner.strRepr(true));
            }
            valuesForTest.add(fieldDefault);

            String typeStr = type.strRepr(false);
            if (type instanceof RustTypeString) {
                stringParamCounter += 1;
                typeStr = String.format("S%d", stringParamCounter);
                fieldsForConstructor.add(String.format("%s: %s.as_ref().to_string()", fieldNameInRust, fieldNameInRust));
            } else if (type instanceof RustTypeOption && ((RustTypeOption) type).inner instanceof RustTypeString) {
                stringParamCounter += 1;
                typeStr = String.format("Option<S%d>", stringParamCounter);
                fieldsForConstructor.add(String.format("%s: %s.map(|s| s.as_ref().to_string())", fieldNameInRust, fieldNameInRust));
            } else {
                fieldsForConstructor.add(fieldNameInRust);
            }
            fieldsForConstructorWithTypes.add(String.format("%s: %s", fieldNameInRust, typeStr));
        }
        if (hasTaggedFields()) {
            fieldsForConstructor.add("_unknown_tagged_fields: vec![]");
        }

        String constructorParamsStr = String.join(", ", fieldsForConstructorWithTypes);
        if (stringParamCounter == 0) {
            buffer.printf("pub fn new(%s) -> Self {%n", constructorParamsStr);
        } else {
            String typeParamsStr = IntStream
                    .range(1, stringParamCounter + 1)
                    .mapToObj(i -> String.format("S%d: AsRef<str>", i))
                    .collect(Collectors.joining(", "));
            buffer.printf("pub fn new<%s>(%s) -> Self {%n", typeParamsStr, constructorParamsStr);
        }

        buffer.incrementIndent();

        buffer.printf("Self {%n");
        buffer.incrementIndent();
        for (String f : fieldsForConstructor) {
            buffer.printf("%s,%n", f);
        }
        buffer.decrementIndent();
        buffer.printf("}%n");

        buffer.decrementIndent();
        buffer.printf("}%n");

        buffer.decrementIndent();
        buffer.printf("}%n");

        buffer.printf("%n");

        buffer.printf("#[cfg(test)]%n");
        buffer.printf("mod tests_%s_new_and_default {%n", MessageGenerator.toSnakeCase(className));
        buffer.incrementIndent();
        buffer.printf("use super::*;%n");
        buffer.printf("%n");

        buffer.printf("#[test]%n");
        buffer.printf("fn test() {%n");
        buffer.incrementIndent();
        buffer.printf("let d = %s::new(%n", className);
        buffer.incrementIndent();

        for (String v : valuesForTest) {
            buffer.printf("%s,%n", v);
        }

        buffer.decrementIndent();
        buffer.printf(");%n", className);
        buffer.printf("assert_eq!(d, %s::default());%n", className);

        buffer.decrementIndent();
        buffer.printf("}%n");

        buffer.decrementIndent();
        buffer.printf("}%n");
    }

    private void generateClassDefault(String className, StructSpec struct) {
        buffer.printf("impl Default for %s {%n", className);
        buffer.incrementIndent();
        buffer.printf("fn default() -> Self {%n");
        buffer.incrementIndent();
        buffer.printf("%s {%n", className);
        buffer.incrementIndent();

        for (FieldSpec field : struct.fields()) {
            if (!field.versions().contains(version)) {
                continue;
            }

            RustFieldSpecAdaptor rustFieldSpecAdaptor = new RustFieldSpecAdaptor(field, version, headerGenerator);
            String fieldNameInRust = rustFieldSpecAdaptor.fieldName();
            buffer.printf("%s: %s,%n", fieldNameInRust, rustFieldSpecAdaptor.fieldDefault());
        }
        if (hasTaggedFields()) {
            buffer.printf("_unknown_tagged_fields: Vec::new(),%n");
        }

        buffer.decrementIndent();
        buffer.printf("}%n", className);
        buffer.decrementIndent();
        buffer.printf("}%n");
        buffer.decrementIndent();
        buffer.printf("}%n");
    }

    private void generateClassReader(String className, StructSpec struct) {
        headerGenerator.addImport("std::io::Read");
        headerGenerator.addImport("std::io::Result");
        headerGenerator.addImport("crate::readable_writable::Readable");
        buffer.printf("impl Readable for %s {%n", className);
        buffer.incrementIndent();
        buffer.printf("fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {%n");
        buffer.incrementIndent();

        List<String> fieldsForConstructor = new ArrayList<>();

        Map<Integer, FieldSpec> taggedFields = new HashMap<>();
        for (FieldSpec field : struct.fields()) {
            if (!field.versions().contains(version)) {
                continue;
            }

            RustFieldSpecAdaptor rustFieldSpecAdaptor = new RustFieldSpecAdaptor(field, version, headerGenerator);
            String fieldNameInRust = rustFieldSpecAdaptor.fieldName();
            fieldsForConstructor.add(fieldNameInRust);

            if (field.taggedVersions().contains(version)) {
                taggedFields.put(field.tag().get(), field);
                buffer.printf("let mut %s = %s;%n", fieldNameInRust, rustFieldSpecAdaptor.fieldDefault());
            } else {
                String readExpression = readExpression(
                        "input",
                        rustFieldSpecAdaptor.fieldType(),
                        fieldFlexibleVersions(field).contains(version),
                        rustFieldSpecAdaptor.fieldName()
                );
                buffer.printf("let %s = %s?;%n", rustFieldSpecAdaptor.fieldName(), readExpression);
            }
        }

        if (hasTaggedFields()) {
            if (taggedFields.isEmpty()) {
                buffer.printf("let tagged_fields_callback = |tag: i32, _: &[u8]| {%n");
            } else {
                buffer.printf("let tagged_fields_callback = |tag: i32, tag_data: &[u8]| {%n");
            }
            buffer.incrementIndent();

            buffer.printf("match tag {%n");
            buffer.incrementIndent();

            taggedFields.entrySet().stream().sorted(Comparator.comparingInt(Map.Entry::getKey)).forEach(kv -> {
                buffer.printf("%d => {%n", kv.getKey());
                buffer.incrementIndent();
                headerGenerator.addImport("std::io::Cursor");
                buffer.printf("let mut cur = Cursor::new(tag_data);%n");
                RustFieldSpecAdaptor rustFieldSpecAdaptor = new RustFieldSpecAdaptor(kv.getValue(), version, headerGenerator);
                String readExpression = readExpression(
                    "&mut cur",
                    rustFieldSpecAdaptor.fieldType(),
                    fieldFlexibleVersions(kv.getValue()).contains(version),
                    rustFieldSpecAdaptor.fieldName()
                );
                buffer.printf("%s = %s?;%n", rustFieldSpecAdaptor.fieldName(), readExpression);
                buffer.printf("Ok(true)%n");
                buffer.decrementIndent();
                buffer.printf("},%n");
            });

            buffer.printf("_ => Ok(false)%n");

            buffer.decrementIndent();
            buffer.printf("}%n");

            buffer.decrementIndent();
            buffer.printf("};%n");
            headerGenerator.addImport("crate::tagged_fields::read_tagged_fields");
            buffer.printf("let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;%n");

            fieldsForConstructor.add("_unknown_tagged_fields");
        }

        buffer.printf("Ok(%s {%n", className);
        buffer.incrementIndent();
        buffer.printf("%s%n", String.join(", ", fieldsForConstructor));
        buffer.decrementIndent();
        buffer.printf("})%n");

        buffer.decrementIndent();
        buffer.printf("}%n");
        buffer.decrementIndent();
        buffer.printf("}%n");
    }

    private String readExpression(String readSource, RustType type, boolean flexible, String fieldNameInRust) {
        return type.readExpression(readSource, fieldNameInRust, flexible, headerGenerator);
    }

    private void generateClassWriter(String className, StructSpec struct) {
        headerGenerator.addImport("std::io::Write");
        headerGenerator.addImport("std::io::Result");
        headerGenerator.addImport("crate::readable_writable::Writable");
        buffer.printf("impl Writable for %s {%n", className);
        buffer.incrementIndent();
        buffer.printf("fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {%n");
        buffer.incrementIndent();

        Map<Integer, FieldSpec> taggedFields = new HashMap<>();
        for (FieldSpec field : struct.fields()) {
            if (!field.versions().contains(version)) {
                continue;
            }

            RustFieldSpecAdaptor rustFieldSpecAdaptor = new RustFieldSpecAdaptor(field, version, headerGenerator);

            if (field.taggedVersions().contains(version)) {
                taggedFields.put(field.tag().get(), field);
                continue;
            }

            boolean flexible = fieldFlexibleVersions(field).contains(version);
            String writeExpression = writeExpression("output", rustFieldSpecAdaptor.fieldType(), flexible, rustFieldSpecAdaptor.fieldName());
            buffer.printf("%s?;%n", writeExpression);
        }
        if (hasTaggedFields()) {
            if (!taggedFields.isEmpty()) {
                buffer.printf("let mut known_tagged_fields = Vec::<RawTaggedField>::new();%n");
                taggedFields.entrySet().stream().sorted(Comparator.comparingInt(Map.Entry::getKey)).forEach(kv -> {
                    FieldSpec field = kv.getValue();
                    RustFieldSpecAdaptor rustFieldSpecAdaptor = new RustFieldSpecAdaptor(field, version, headerGenerator);
                    rustFieldSpecAdaptor.generateNonDefaultValueCheck(buffer, "self.");
                    buffer.incrementIndent();
                    buffer.printf("let mut cur = Cursor::new(Vec::<u8>::new());%n");
                    boolean flexible = fieldFlexibleVersions(field).contains(version);
                    String writeExpression = writeExpression("&mut cur", rustFieldSpecAdaptor.fieldType(), flexible, rustFieldSpecAdaptor.fieldName());
                    buffer.printf("%s?;%n", writeExpression);
                    buffer.printf("known_tagged_fields.push(RawTaggedField { tag: %d, data: cur.into_inner() });%n", field.tag().get());
                    buffer.decrementIndent();
                    buffer.printf("}%n");
                });
                headerGenerator.addImport("crate::tagged_fields::write_tagged_fields");
                buffer.printf("write_tagged_fields(output, &known_tagged_fields, &self._unknown_tagged_fields)?;%n");
            } else {
                headerGenerator.addImport("crate::tagged_fields::write_tagged_fields");
                buffer.printf("write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;%n");
            }
        }

        buffer.printf("Ok(())%n");
        buffer.decrementIndent();
        buffer.printf("}%n");
        buffer.decrementIndent();
        buffer.printf("}%n");
    }

    private String writeExpression(String writeTarget,
                                   RustType type,
                                   boolean flexible,
                                   String fieldNameInRust) {
        return type.writeExpression(writeTarget, String.format("self.%s", fieldNameInRust), flexible, headerGenerator);
    }

    private Versions fieldFlexibleVersions(FieldSpec field) {
        if (field.flexibleVersions().isPresent()) {
            if (!message.flexibleVersions().intersect(field.flexibleVersions().get()).
                    equals(field.flexibleVersions().get())) {
                throw new RuntimeException("The flexible versions for field " +
                        field.name() + " are " + field.flexibleVersions().get() +
                        ", which are not a subset of the flexible versions for the " +
                        "message as a whole, which are " + message.flexibleVersions());
            }
            return field.flexibleVersions().get();
        } else {
            return message.flexibleVersions();
        }
    }

    private void generateSubclasses(StructSpec struct) throws Exception {
        for (FieldSpec field : struct.fields()) {
            if (!field.versions().contains(version)) {
                continue;
            }

            if (field.type().isStructArray()) {
                FieldType.ArrayType arrayType = (FieldType.ArrayType) field.type();
                if (!structRegistry.commonStructNames().contains(arrayType.elementName())) {
                    generateClass(false, arrayType.elementType().toString(),
                            structRegistry.findStruct(field));
                }
            } else if (field.type().isStruct()) {
                if (!structRegistry.commonStructNames().contains(field.typeString())) {
                    generateClass(false, field.typeString(), structRegistry.findStruct(field));
                }
            }
        }
    }

    private boolean hasTaggedFields() {
        return message.flexibleVersions().contains(this.version);
    }

    void write(BufferedWriter writer) throws Exception {
        RustUtils.addGeneratedHeader(writer);
        headerGenerator.buffer.write(writer);
        buffer.write(writer);
    }
}
