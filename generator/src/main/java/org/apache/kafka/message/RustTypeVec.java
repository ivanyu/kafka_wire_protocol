package org.apache.kafka.message;

class RustTypeVec implements RustType {
    final RustType inner;

    RustTypeVec(RustType inner) {
        this.inner = inner;
    }

    @Override
    public String strRepr(boolean turbofish) {
        if (turbofish) {
            return "Vec::<" + inner.strRepr(turbofish) + ">";
        } else {
            return "Vec<" + inner.strRepr(turbofish) + ">";
        }
    }

    @Override
    public void writeDeclaration(String fieldName, RustHeaderGenerator headerGenerator, CodeBuffer buffer) {
        if (inner instanceof RustTypeUuid) {
            headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
            buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::vec_elem::<Uuid>(proptest_strategies::uuid())\"))]%n");
        } else {
            headerGenerator.addImportTest("crate::test_utils::proptest_strategies");
            buffer.printf("#[cfg_attr(test, proptest(strategy = \"proptest_strategies::vec()\"))]%n");
        }
        buffer.printf("pub %s: %s,%n", fieldName, strRepr(false));
    }

    @Override
    public String readExpression(String readSource, String fieldNameInRust, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::arrays::k_read_array");
        return String.format("k_read_array::<%s>(%s, \"%s\", %b)",
            inner.strRepr(false), readSource, fieldNameInRust, flexible);
    }

    @Override
    public String writeExpression(String writeTarget, String object, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::arrays::k_write_array");
        return String.format("k_write_array(%s, \"%s\", &%s, %b)",
            writeTarget, object, object, flexible);
    }
}
