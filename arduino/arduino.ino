#include <Arduino.h>

#include "day1.hpp"
#include "utils.hpp"

int leds[] = {4, 5, 6, 7};
void blink_mode();
void (*modes[])() = {&solve_d1, &blink_mode};
char* titles[] = {"Day 1", "Blink-Mode"};
#define MODE_COUNT 2

void setup() {
    for (int i = 0; i < 4; i++) {
        pinMode(leds[i], OUTPUT);
    }
    Serial.begin(9600);
    Serial.setTimeout(1000);
}

void loop() {
    char buf[80];
    for (int i = 0; i < 5; i++) Serial.println();
    Serial.println("-------- Select: --------");
    for (size_t i = 1; i <= MODE_COUNT; i++) {
        Serial.print(i);
        Serial.print(" : ");
        Serial.println(titles[i - 1]);
    }
    char c = get_char();

    if (isDigit(c)) {
        size_t index = c - '0' - 1;
        if (index < MODE_COUNT) {
            Serial.print("You chose: ");
            Serial.println(titles[index]);

            modes[index]();
        }
    }

    delay(1);
}

void blink_mode() {
    int count = 0;

    while (!Serial.available()) {
        for (int i = 0; i < 4; i++) {
            digitalWrite(leds[i], (count >> i) & 1);
        }
        count++;
        delay(1000);
    }
    for (int i = 0; i < 4; i++) {
        digitalWrite(leds[i], LOW);
    }
    Serial.readString();
}
