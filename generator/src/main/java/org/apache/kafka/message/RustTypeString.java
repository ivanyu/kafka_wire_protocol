package org.apache.kafka.message;

class RustTypeString implements RustType {
    @Override
    public String strRepr(boolean turbofish) {
        return "String";
    }

    @Override
    public void writeDeclaration(String fieldName, RustHeaderGenerator headerGenerator, CodeBuffer buffer) {
        headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
        buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::string()\"))]%n");
        buffer.printf("pub %s: %s,%n", fieldName, strRepr(false));
    }

    @Override
    public String readExpression(String readSource, String fieldNameInRust, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::readable_writable::KafkaReadable");
        return String.format("String::read_ext(%s, \"%s\", %b)", readSource, fieldNameInRust, flexible);
    }

    @Override
    public String writeExpression(String writeTarget, String object, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::readable_writable::KafkaWritable");
        return String.format("%s.write_ext(%s, \"%s\", %b)",
                object, writeTarget, object, flexible);
    }
}
