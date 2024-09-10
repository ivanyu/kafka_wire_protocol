package org.apache.kafka.message;

class RustTypeBytes implements RustType {
    @Override
    public String strRepr(boolean turbofish) {
        if (turbofish) {
            return "Vec::<u8>";
        } else {
            return "Vec<u8>";
        }
    }

    @Override
    public void writeDeclaration(String fieldName, RustHeaderGenerator headerGenerator, CodeBuffer buffer) {
        headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
        headerGenerator.addImportTest("crate::test_utils::serde_bytes");
        buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::bytes()\"))]%n");
        buffer.printf("#[cfg_attr(test, serde(with=\"serde_bytes\"))]%n");
        buffer.printf("pub %s: %s,%n", fieldName, strRepr(false));
    }

    @Override
    public String readExpression(String readSource, String fieldNameInRust, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::bytes::read_bytes");
        return String.format("read_bytes(%s, \"%s\", %b)", readSource, fieldNameInRust, flexible);
    }

    @Override
    public String writeExpression(String writeTarget, String object, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::bytes::write_bytes");
        return String.format("write_bytes(%s, \"%s\", &%s, %b)",
            writeTarget, object, object, flexible);
    }
}
