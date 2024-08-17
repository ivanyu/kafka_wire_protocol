package org.apache.kafka.message;

interface RustType {
    String strRepr(boolean turbofish);
    void writeDeclaration(String fieldName, RustHeaderGenerator headerGenerator, CodeBuffer buffer);
    String readExpression(String readSource, String fieldNameInRust, boolean flexible, RustHeaderGenerator headerGenerator);
    String writeExpression(String writeTarget, String object, boolean flexible, RustHeaderGenerator headerGenerator);
};
