package io.aiven.kio.java_tester;

import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.Stream;

class EntityClass<T> {
    private final Class<T> clazz;
    private final Map<String, Method> setters;

    EntityClass(Class<T> clazz) {
        this.clazz = clazz;
        this.setters = Arrays.stream(clazz.getDeclaredMethods())
            .filter(m -> m.getName().startsWith("set"))
            .collect(Collectors.toMap(Method::getName, m -> m));
    }

    T newInstance() throws Exception {
        return clazz.getDeclaredConstructor().newInstance();
    }

    Setter setterForFieldName(String fieldName) throws Exception {
        String setterName = "set" + fieldName;
        Method method = setters.get(setterName);
        if (method == null) {
            throw new Exception("Setter method " + setterName + " not found");
        }
        return new Setter(method, setterName);
    }

    public Stream<Class<?>> declaredClasses() {
        return Arrays.stream(clazz.getDeclaredClasses());
    }
}
