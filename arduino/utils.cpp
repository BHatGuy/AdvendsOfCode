#include "utils.hpp"

#include <Arduino.h>

char get_char() {
    while (!Serial.available())
        ;
    return Serial.read();
}