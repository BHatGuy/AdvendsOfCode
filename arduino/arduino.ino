#include <Arduino.h>

#include "day1.hpp"
#include "day2.hpp"
#include "day3.hpp"
#include "day4.hpp"
#include "day5.hpp"
#include "utils.hpp"

int leds[] = {4, 5, 6, 7};
void blink_mode();
void (*modes[])() = {&solve_d1, &solve_d2, &solve_d3, &solve_d4, &solve_d5, &blink_mode};
char* titles[] = {"Day 1", "Day 2", "Day 3 (Not working)", "Day 4", "Day 5", "Blink-Mode"};
#define MODE_COUNT sizeof(titles) / sizeof(char*)

void setup() {
    for (int i = 0; i < 4; i++) {
        pinMode(leds[i], OUTPUT);
    }
    Serial.begin(9600);
    Serial.setTimeout(6000000);
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
    String s = Serial.readStringUntil('\r');
    int index = s.toInt();

    if (index > 0 && index <= MODE_COUNT) {
        index--;
        Serial.print("You chose: ");
        Serial.println(titles[index]);

        modes[index]();
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
        delay(3333);
    }
    for (int i = 0; i < 4; i++) {
        digitalWrite(leds[i], LOW);
    }
    Serial.readString();
}
