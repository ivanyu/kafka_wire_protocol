package io.aiven.kio.java_tester;

import java.nio.ByteBuffer;
import java.util.Base64;
import java.util.UUID;

import org.apache.kafka.common.Uuid;

import com.fasterxml.jackson.databind.JsonNode;

abstract class BaseCreator {
    protected final RootMessageInfo rootMessageInfo;

    BaseCreator(RootMessageInfo rootMessageInfo) {
        this.rootMessageInfo = rootMessageInfo;
    }

    protected static byte getByte(JsonNode fieldValue, String fieldName) throws Exception {
        if (!fieldValue.isIntegralNumber()) {
            throw new Exception("Expected byte in field " + fieldName + " but got " + fieldValue);
        }

        long longValue = fieldValue.asLong();
        if ((byte) longValue != longValue) {
            throw new Exception("Invalid byte value in field " + fieldName + ": " + longValue);
        }

        return (byte) longValue;
    }

    protected static short getShort(JsonNode fieldValue, String fieldName) throws Exception {
        if (!fieldValue.isIntegralNumber()) {
            throw new Exception("Expected short in field " + fieldName + " but got " + fieldValue);
        }

        long longValue = fieldValue.asLong();
        if ((short) longValue != longValue) {
            throw new Exception("Invalid short value in field " + fieldName + ": " + longValue);
        }

        return (short) longValue;
    }

    protected static int getInt(JsonNode fieldValue, String fieldName) throws Exception {
        if (!fieldValue.isIntegralNumber()) {
            throw new Exception("Expected int in field " + fieldName + " but got " + fieldValue);
        }

        long longValue = fieldValue.asLong();
        if ((int) longValue != longValue) {
            throw new Exception("Invalid int value in field " + fieldName + ": " + longValue);
        }

        return (int) longValue;
    }

    protected static long getLong(JsonNode fieldValue, String fieldName) throws Exception {
        if (!fieldValue.isIntegralNumber()) {
            throw new Exception("Expected int in field " + fieldName + " but got " + fieldValue);
        }
        return fieldValue.asLong();
    }

    protected static double getDouble(JsonNode fieldValue, String fieldName) throws Exception {
        if (!fieldValue.isFloatingPointNumber()) {
            throw new Exception("Expected double in field " + fieldName + " but got " + fieldValue);
        }
        return fieldValue.asDouble();
    }

    protected static boolean getBoolean(JsonNode fieldValue, String fieldName) throws Exception {
        if (!fieldValue.isBoolean()) {
            throw new Exception("Expected boolean in field " + fieldName + " but got " + fieldValue);
        }
        return fieldValue.asBoolean();
    }

    protected static String getString(JsonNode fieldValue, String fieldName) throws Exception {
        if (fieldValue.isNull()) {
            return null;
        }

        if (!fieldValue.isTextual()) {
            throw new Exception("Expected string in field " + fieldName + " but got " + fieldValue);
        }
        return fieldValue.asText();
    }

    protected static Uuid getUuid(JsonNode fieldValue, String fieldName) throws Exception {
        String str = getString(fieldValue, fieldName);
        if (str == null) {
            return null;
        }
        UUID tmpUuid = UUID.fromString(str);
        return new Uuid(tmpUuid.getMostSignificantBits(), tmpUuid.getLeastSignificantBits());
    }

    protected static ByteBuffer getByteBuffer(JsonNode fieldValue, String fieldName) throws Exception {
        String str = getString(fieldValue, fieldName);
        if (str == null) {
            return null;
        }
        return ByteBuffer.wrap(Base64.getDecoder().decode(str));
    }

    protected static byte[] getBytes(JsonNode fieldValue, String fieldName) throws Exception {
        String str = getString(fieldValue, fieldName);
        if (str == null) {
            return null;
        }
        return Base64.getDecoder().decode(str);
    }
}
