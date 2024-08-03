package me.ivanyu.java_tester;

import com.fasterxml.jackson.databind.JsonNode;
import org.apache.kafka.common.protocol.types.RawTaggedField;

import java.util.List;

class RawTaggedFieldsFiller {
    static void fill(List<RawTaggedField> target, JsonNode array) throws Exception {
        if (!array.isArray()) {
            throw new RuntimeException("Value must be array");
        }

        for (JsonNode value : array) {
            RawTaggedField field = new RawTaggedField(
                value.get("tag").asInt(),
                BaseCreator.getBytes(value.get("data"), "data")
            );
            target.add(field);
        }
    }
}
