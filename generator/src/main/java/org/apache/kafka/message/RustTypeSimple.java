package org.apache.kafka.message;

class RustTypeSimple implements RustType {
    private final FieldType type;

    RustTypeSimple(FieldType type) {
        this.type = type;
    }

    @Override
    public String strRepr(boolean turbofish) {
        if (type instanceof FieldType.BoolFieldType) {
            return "bool";
        } else if (type instanceof FieldType.Int8FieldType) {
            return "i8";
        } else if (type instanceof FieldType.Int16FieldType) {
            return "i16";
        } else if (type instanceof FieldType.Uint16FieldType) {
            return "u16";
        } else if (type instanceof FieldType.Uint32FieldType) {
            return "u32";
        } else if (type instanceof FieldType.Int32FieldType) {
            return "i32";
        } else if (type instanceof FieldType.Int64FieldType) {
            return "i64";
        } else if (type instanceof FieldType.UUIDFieldType) {
            return "Uuid";
        } else if (type instanceof FieldType.Float64FieldType) {
            return "f64";
        }
        return "";
    }

    @Override
    public void writeDeclaration(String fieldName, RustHeaderGenerator headerGenerator, CodeBuffer buffer) {
        buffer.printf("pub %s: %s,%n", fieldName, strRepr(false));
    }

    @Override
    public String readExpression(String readSource, String fieldNameInRust, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::readable_writable::Readable");
        return String.format("%s::read(%s)", strRepr(false), readSource);
    }

    @Override
    public String writeExpression(String writeTarget, String object, boolean flexible, RustHeaderGenerator headerGenerator) {
        headerGenerator.addImport("crate::readable_writable::Writable");
        return String.format("%s.write(%s)", object, writeTarget);
    }
}
