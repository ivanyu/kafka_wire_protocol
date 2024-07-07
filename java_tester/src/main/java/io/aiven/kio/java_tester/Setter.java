package io.aiven.kio.java_tester;

import java.lang.reflect.Method;

class Setter {
    private final Method method;

    Setter(Method method, String setterName) throws Exception {
        if (method.getParameterCount() != 1) {
            throw new Exception("Invalid number of parameters in " + setterName);
        }
        this.method = method;
    }

    Class<?> parameterType() {
        return method.getParameterTypes()[0];
    }

    void invoke(Object instance, Object arg) throws Exception {
        method.invoke(instance, arg);
    }
}
