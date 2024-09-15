package org.apache.kafka.message;

import java.io.BufferedWriter;
import java.io.IOException;

class RustUtils {
    static void addGeneratedHeader(BufferedWriter writer) throws IOException {
        writer.write("// This file was generated. Do not edit.\n\n");
    }
}
