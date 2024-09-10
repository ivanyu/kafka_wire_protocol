package org.apache.kafka.message;

class RustTypeOption implements RustType {
    final RustType inner;

    RustTypeOption(RustType inner) {
        this.inner = inner;
    }

    @Override
    public String strRepr(boolean turbofish) {
        if (turbofish) {
            return "Option::<" + inner.strRepr(turbofish) + ">";
        } else {
            return "Option<" + inner.strRepr(turbofish) + ">";
        }
    }

    @Override
    public void writeDeclaration(String fieldName, RustHeaderGenerator headerGenerator, CodeBuffer buffer) {
        if (inner instanceof RustTypeString) {
            headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
            buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::optional_string()\"))]%n");
        } else if (inner instanceof RustTypeBytes) {
            headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
            headerGenerator.addImportTest("crate::test_utils::serde_option_bytes");
            buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::optional_bytes()\"))]%n");
            buffer.printf("#[cfg_attr(test, serde(with=\"serde_option_bytes\"))]%n");
        } else if (inner instanceof RustTypeVec) {
            headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
            buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::optional_vec()\"))]%n");
        }
        buffer.printf("pub %s: %s,%n", fieldName, strRepr(false));
    }

    @Override
    public String readExpression(String readSource, String fieldNameInRust, boolean flexible, RustHeaderGenerator headerGenerator) {
        if (inner instanceof RustTypeString) {
            headerGenerator.addImport("crate::readable_writable::KafkaReadable");
            return String.format("Option::<String>::read_ext(%s, \"%s\", %b)", readSource, fieldNameInRust, flexible);
        } else if (inner instanceof RustTypeBytes) {
            headerGenerator.addImport("crate::bytes::read_nullable_bytes");
            return String.format("read_nullable_bytes(%s, \"%s\", %b)", readSource, fieldNameInRust, flexible);
        } else if (inner instanceof RustTypeVec) {
            RustTypeVec innerVec = (RustTypeVec) inner;
            headerGenerator.addImport("crate::arrays::read_nullable_array");
            return String.format("read_nullable_array::<%s>(%s, \"%s\", %b)",
                innerVec.inner.strRepr(false), readSource, fieldNameInRust, flexible);
        } else {
            headerGenerator.addImport("crate::readable_writable::KafkaReadable");
            return String.format("(if i8::read(input)? < 0 { Ok(None) } else { %s.map(Some) })",
                    inner.readExpression(readSource, fieldNameInRust, flexible, headerGenerator));
        }
    }

    @Override
    public String writeExpression(String writeTarget, String object, boolean flexible, RustHeaderGenerator headerGenerator) {
        if (inner instanceof RustTypeString) {
            return inner.writeExpression(writeTarget, object, flexible, headerGenerator);
        } else if (inner instanceof RustTypeBytes) {
            headerGenerator.addImport("crate::bytes::write_nullable_bytes");
            return String.format("write_nullable_bytes(%s, \"%s\", %s.as_deref(), %b)",
                writeTarget, object, object, flexible);
        } else if (inner instanceof RustTypeVec) {
            headerGenerator.addImport("crate::arrays::write_nullable_array");
            return String.format("write_nullable_array(%s, \"%s\", %s.as_deref(), %b)",
                writeTarget, object, object, flexible);
        } else {
            String result = String.format("(if let Some(v) = &%s {", object);
            result += String.format(" 1_i8.write(%s)?; ", writeTarget);
            String writeExpression = inner.writeExpression(writeTarget, "v", flexible, headerGenerator);
            result += String.format("%s", writeExpression);
            result += " } else { ";
            result += String.format("(-1_i8).write(%s)", writeTarget);
            result += " })";
            return result;
        }
    }
}
