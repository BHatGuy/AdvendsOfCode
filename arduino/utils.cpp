#include "utils.hpp"

#include <Arduino.h>

char get_char() {
    while (!Serial.available())
        ;
    return Serial.read();
}

void read_line(char* buf, size_t size, bool echo) {
    for (size_t i = 0; i < size - 1; i++) {
        buf[i] = get_char();
        if (buf[i] == '\r' || buf[i] == '\n') {
            buf[i] = 0;
            Serial.println();
            return;
        }
        if (echo) {
            Serial.write(buf[i]);
        }
    }
    buf[size - 1] = 0;
}