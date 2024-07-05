package io.aiven.kio.java_tester;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Map;

import com.fasterxml.jackson.databind.JsonNode;

class Schema {
    final JsonNode schemaJson;
    final Map<String, JsonNode> commonStructs;

    Schema(JsonNode schemaJson, Map<String, JsonNode> commonStructs) {
        this.schemaJson = schemaJson;
        this.commonStructs = commonStructs;
    }

    String type() {
        return schemaJson.get("type").asText();
    }

    Schema fieldSchema(String fieldName) throws Exception {
        Iterator<JsonNode> fields;
        if (schemaJson.get("fields") != null) {
            fields = schemaJson.get("fields").elements();
        } else {
            String type = type().replace("[]", "");
            fields = commonStructs.get(type).get("fields").elements();
        }

        while (fields.hasNext()) {
            JsonNode field = fields.next();
            if (field.get("name").asText().equals(fieldName)) {
                return new Schema(field, commonStructs);
            }
        }

        return switch (fieldName) {
            case "TimeoutMs" -> fieldSchema("timeoutMs");
            case "ValidateOnly" -> fieldSchema("validateOnly");
            case "MemberAssignment" -> fieldSchema("memberAssignment");
            case "GroupId" -> fieldSchema("groupId");
            case "IsKRaftController" -> fieldSchema("isKRaftController");
            default -> throw new Exception("field " + fieldName + " not found");
        };
    }

    boolean isTaggedVersion(short version) {
        JsonNode taggedVersions = schemaJson.get("taggedVersions");
        if (taggedVersions == null) {
            return false;
        }
        short taggedVersionFrom = Short.parseShort(taggedVersions.asText().replace("+", ""));
        return version >= taggedVersionFrom;
    }

    List<String> knownFieldNames() {
        Iterator<JsonNode> fields;
        if (schemaJson.get("fields") != null) {
            fields = schemaJson.get("fields").elements();
        } else {
            String type = type().replace("[]", "");
            fields = commonStructs.get(type).get("fields").elements();
        }

        List<String> knownNames = new ArrayList<>();
        while (fields.hasNext()) {
            JsonNode field = fields.next();
            String name = field.get("name").asText();
            knownNames.add(name);
        }
        return knownNames;
    }
}
