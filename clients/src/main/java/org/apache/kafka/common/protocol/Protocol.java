/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package org.apache.kafka.common.protocol;

import org.apache.kafka.common.message.RequestHeaderData;
import org.apache.kafka.common.message.ResponseHeaderData;
import org.apache.kafka.common.protocol.types.BoundField;
import org.apache.kafka.common.protocol.types.Schema;
import org.apache.kafka.common.protocol.types.TaggedFields;
import org.apache.kafka.common.protocol.types.Type;

import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.Map;
import java.util.Set;

public class Protocol {

    private static String indentString(int size) {
        return " ".repeat(Math.max(0, size));
    }

    private static void schemaToBnfHtml(Schema schema, StringBuilder b, int indentSize) {
        final String indentStr = indentString(indentSize);
        final Map<String, Type> subTypes = new LinkedHashMap<>();

        // Top level fields
        for (BoundField field: schema.fields()) {
            Type type = field.def.type;
            if (type.isArray()) {
                b.append("[");
                b.append(field.def.name);
                b.append("] ");
                if (!subTypes.containsKey(field.def.name)) {
                    subTypes.put(field.def.name, type.arrayElementType().get());
                }
            } else if (type instanceof TaggedFields) {
                b.append("_tagged_fields ");
            } else {
                b.append(field.def.name);
                b.append(" ");
                if (!subTypes.containsKey(field.def.name))
                    subTypes.put(field.def.name, type);
            }
        }
        b.append("\n");

        // Sub Types/Schemas
        for (Map.Entry<String, Type> entry: subTypes.entrySet()) {
            if (entry.getValue() instanceof Schema) {
                // Complex Schema Type
                b.append(indentStr);
                b.append(entry.getKey());
                b.append(" => ");
                schemaToBnfHtml((Schema) entry.getValue(), b, indentSize + 2);
            } else {
                // Standard Field Type
                b.append(indentStr);
                b.append(entry.getKey());
                b.append(" => ");
                b.append(entry.getValue());
                b.append("\n");
            }
        }
    }

    private static void populateSchemaFields(Schema schema, Set<BoundField> fields) {
        for (BoundField field: schema.fields()) {
            fields.add(field);
            if (field.def.type.isArray()) {
                Type innerType = field.def.type.arrayElementType().get();
                if (innerType instanceof Schema)
                    populateSchemaFields((Schema) innerType, fields);
            } else if (field.def.type instanceof Schema)
                populateSchemaFields((Schema) field.def.type, fields);
        }
    }

    private static void schemaToFieldTableHtml(Schema schema, StringBuilder b) {
        Set<BoundField> fields = new LinkedHashSet<>();
        populateSchemaFields(schema, fields);

        b.append("<table class=\"data-table\"><tbody>\n");
        b.append("<tr>");
        b.append("<th>Field</th>\n");
        b.append("<th>Description</th>\n");
        b.append("</tr>");
        for (BoundField field : fields) {
            b.append("<tr>\n");
            b.append("<td>");
            b.append(field.def.name);
            b.append("</td>");
            b.append("<td>");
            if (field.def.type instanceof TaggedFields) {
                TaggedFields taggedFields = (TaggedFields) field.def.type;
                // Only include the field in the table if there are actually tags defined
                if (taggedFields.numFields() > 0) {
                    b.append("<table class=\"data-table\"><tbody>\n");
                    b.append("<tr>");
                    b.append("<th>Tag</th>\n");
                    b.append("<th>Tagged field</th>\n");
                    b.append("<th>Description</th>\n");
                    b.append("</tr>");
                    taggedFields.fields().forEach((tag, taggedField) -> {
                        b.append("<tr>\n");
                        b.append("<td>");
                        b.append(tag);
                        b.append("</td>");
                        b.append("<td>");
                        b.append(taggedField.name);
                        b.append("</td>");
                        b.append("<td>");
                        b.append(taggedField.docString);
                        if (taggedField.type.isArray()) {
                            Type innerType = taggedField.type.arrayElementType().get();
                            if (innerType instanceof Schema) {
                                schemaToFieldTableHtml((Schema) innerType, b);
                            }
                        } else if (taggedField.type instanceof Schema) {
                            schemaToFieldTableHtml((Schema) taggedField.type, b);
                        }
                        b.append("</td>");
                        b.append("</tr>\n");
                    });
                    b.append("</tbody></table>\n");
                } else {
                    b.append(field.def.docString);
                }
            } else {
                b.append(field.def.docString);
            }
            b.append("</td>");
            b.append("</tr>\n");
        }
        b.append("</tbody></table>\n");
    }

    public static String toHtml() {
        final StringBuilder b = new StringBuilder();
        b.append("<h5>Headers:</h5>\n");

        for (int i = RequestHeaderData.LOWEST_SUPPORTED_VERSION; i <= RequestHeaderData.HIGHEST_SUPPORTED_VERSION; i++) {
            b.append("<pre>");
            b.append("Request Header v").append(i).append(" => ");
            schemaToBnfHtml(RequestHeaderData.SCHEMAS[i], b, 2);
            b.append("</pre>\n");
            schemaToFieldTableHtml(RequestHeaderData.SCHEMAS[i], b);
        }
        for (int i = ResponseHeaderData.LOWEST_SUPPORTED_VERSION; i <= ResponseHeaderData.HIGHEST_SUPPORTED_VERSION; i++) {
            b.append("<pre>");
            b.append("Response Header v").append(i).append(" => ");
            schemaToBnfHtml(ResponseHeaderData.SCHEMAS[i], b, 2);
            b.append("</pre>\n");
            schemaToFieldTableHtml(ResponseHeaderData.SCHEMAS[i], b);
        }
        for (ApiKeys key : ApiKeys.clientApis()) {
            // Key
            b.append("<h5>");
            b.append("<a name=\"The_Messages_" + key.name + "\">");
            b.append(key.name);
            b.append(" API (Key: ");
            b.append(key.id);
            b.append("):</a></h5>\n\n");
            // Requests
            b.append("<b>Requests:</b><br>\n");
            Schema[] requests = key.messageType.requestSchemas();
            for (short version = key.oldestVersion(); version <= key.latestVersion(); version++) {
                Schema schema = requests[version];
                if (schema == null)
                    throw new IllegalStateException("Unexpected null schema for " + key + " with version " + version);
                // Schema
                b.append("<div>");
                // Version header
                b.append("<pre>");
                b.append(key.name);
                b.append(" Request (Version: ");
                b.append(version);
                b.append(") => ");
                schemaToBnfHtml(schema, b, 2);
                b.append("</pre>");

                if (!key.isVersionEnabled(version, false)) {
                    b.append("<p>This version of the request is unstable.</p>");
                }

                b.append("<p><b>Request header version:</b> ");
                b.append(key.requestHeaderVersion(version));
                b.append("</p>\n");

                schemaToFieldTableHtml(schema, b);
                b.append("</div>\n");
            }

            // Responses
            b.append("<b>Responses:</b><br>\n");
            Schema[] responses = key.messageType.responseSchemas();
            for (int version = key.oldestVersion(); version < key.latestVersion(); version++) {
                Schema schema = responses[version];
                if (schema == null)
                    throw new IllegalStateException("Unexpected null schema for " + key + " with version " + version);
                // Schema
                b.append("<div>");
                // Version header
                b.append("<pre>");
                b.append(key.name);
                b.append(" Response (Version: ");
                b.append(version);
                b.append(") => ");
                schemaToBnfHtml(responses[version], b, 2);
                b.append("</pre>");

                b.append("<p><b>Response header version:</b> ");
                b.append(key.responseHeaderVersion((short) version));
                b.append("</p>\n");

                schemaToFieldTableHtml(responses[version], b);
                b.append("</div>\n");
            }
        }

        return b.toString();
    }

    public static void main(String[] args) {
        System.out.println(toHtml());
    }

}
