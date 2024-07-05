package io.aiven.kio.java_tester;

import com.fasterxml.jackson.databind.node.NullNode;
import java.lang.reflect.Method;
import java.util.AbstractCollection;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Iterator;
import java.util.List;

import com.fasterxml.jackson.databind.JsonNode;

class CollectionCreator extends BaseCreator {
    private final JsonNode fieldValue;
    private final String fieldName;
    private final Schema fieldSchema;

    CollectionCreator(RootMessageInfo rootMessageInfo,
                      JsonNode fieldValue, String fieldName, Schema fieldSchema) throws Exception {
        super(rootMessageInfo);
        if (!fieldValue.isArray() && !fieldValue.isNull()) {
            throw new Exception("The value of " + fieldName + " must be array but was " + fieldValue);
        }
        this.fieldValue = fieldValue;
        this.fieldName = fieldName;
        this.fieldSchema = fieldSchema;
    }

    AbstractCollection<Object> createAbstractCollection(
        Class<AbstractCollection<Object>> collectionClazz
    ) throws Exception {
        if (fieldValue.isNull()) {
            return null;
        }
        AbstractCollection<Object> collection = collectionClazz.getDeclaredConstructor().newInstance();
        Class<?> elementClazz = getCollectionElementClass(collectionClazz);
        fillCollectionFromChildren(elementClazz, collection);
        return collectionClazz.cast(collection);
    }

    private static Class<Object> getCollectionElementClass(Class<AbstractCollection<Object>> collectionClazz)
        throws Exception {
        // Try to estimate the element class based on the `find` method.
        // `find` is expected to be present in all `AbstractCollection`s of interest.
        for (Method method : collectionClazz.getDeclaredMethods()) {
            if (method.getName().equals("find")) {
                return (Class<Object>) method.getReturnType();
            }
        }
        throw new Exception("No 'find' method for " + collectionClazz);
    }

    List<?> createList() throws Exception {
        if (fieldValue.isNull()) {
            return null;
        }
        final String elementTypeInSchema;
        {
            String tmp = fieldSchema.type();
            if (!tmp.startsWith("[]")) {
                throw new Exception("Unexpected type " + tmp);
            }
            elementTypeInSchema = tmp.substring(2);
        }

        Class<?> elementClazz = switch (elementTypeInSchema) {
            case "int8" -> Byte.class;
            case "int16" -> Short.class;
            case "int32" -> Integer.class;
            case "int64" -> Long.class;
            case "string" -> String.class;
            default -> rootMessageInfo.rootClazz.declaredClasses()
                .filter(c -> c.getName().endsWith("$" + elementTypeInSchema))
                .findFirst().get();
        };

        List<Object> list = new ArrayList<>();
        fillCollectionFromChildren(elementClazz, list);
        return list;
    }

    private void fillCollectionFromChildren(
        Class<?> elementClazz, Collection<Object> collection
    ) throws Exception {
        if (!fieldValue.isArray()) {
            throw new Exception("The value of " + fieldName + " must be array but was " + fieldValue);
        }

        Iterator<JsonNode> elements = fieldValue.elements();
        while (elements.hasNext()) {
            JsonNode elementValue = elements.next();
            Object elementObj;
            if (elementClazz.equals(Byte.class)) {
                elementObj = getByte(elementValue, fieldName);
            } else if (elementClazz.equals(Short.class)) {
                elementObj = getShort(elementValue, fieldName);
            } else if (elementClazz.equals(Integer.class)) {
                elementObj = getInt(elementValue, fieldName);
            } else if (elementClazz.equals(Long.class)) {
                elementObj = getLong(elementValue, fieldName);
            } else if (elementClazz.equals(String.class)) {
                elementObj = getString(elementValue, fieldName);
            } else {
                elementObj = new ObjectCreator<>(rootMessageInfo, new EntityClass<>(elementClazz), fieldSchema)
                    .create(elementValue);
            }
            collection.add(elementObj);
        }
    }
}
