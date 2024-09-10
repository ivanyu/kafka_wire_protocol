package org.apache.kafka.message;

class RustTypeUuid implements RustType {
    @Override
    public String strRepr(boolean turbofish) {
        return "Uuid";
    }

    @Override
    public void writeDeclaration(String fieldName, RustHeaderGenerator headerGenerator, CodeBuffer buffer) {
        headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
        buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::uuid()\"))]%n");
        buffer.printf("pub %s: %s,%n", fieldName, strRepr(false));
    }

    @Override
    public String readExpression(String readSource, String fieldNameInRust, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("uuid::Uuid");
        return String.format("Uuid::read(%s)", readSource);
    }

    @Override
    public String writeExpression(String writeTarget, String object, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::readable_writable::Writable");
        return String.format("%s.write(%s)", object, writeTarget);
    }
}
