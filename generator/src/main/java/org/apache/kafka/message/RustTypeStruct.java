package org.apache.kafka.message;

class RustTypeStruct implements RustType {
    private final String name;

    RustTypeStruct(String name) {
        this.name = name;
    }

    @Override
    public String strRepr(boolean turbofish) {
        return name;
    }

    @Override
    public void writeDeclaration(String fieldName, RustHeaderGenerator headerGenerator, CodeBuffer buffer) {
        buffer.printf("pub %s: %s,%n", fieldName, strRepr(false));
    }

    @Override
    public String readExpression(String readSource, String fieldNameInRust, boolean flexible, RustHeaderGenerator headerGenerator) {
        return String.format("%s::read(%s)", name, readSource);
    }

    @Override
    public String writeExpression(String writeTarget, String object, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::readable_writable::Writable");
        return String.format("%s.write(%s)", object, writeTarget);
    }
}
