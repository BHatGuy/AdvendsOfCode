#include <Arduino.h>
#include <string.h>

#include "MD5.hpp"
#include "day4.hpp"
#include "utils.hpp"

bool is_vowel(char c) { return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'; }

void solve_d5() {
    uint16_t count = 0;
    while (true) {
        String s = Serial.readStringUntil('\r');
        if (s.length() == 0) break;

        uint8_t vcount = 0;
        bool doublec = false;
        bool bad = false;
        char last = 0;
        for (char c : s) {
            if (is_vowel(c)) vcount++;
            if (c == last) doublec = true;
            if ((last == 'a' && c == 'b') || (last == 'c' && c == 'd') || (last == 'p' && c == 'q') ||
                (last == 'x' && c == 'y')) {
                    bad = true;
                    break;
            }
            last = c;
        }
        if(vcount >= 3 && doublec && !bad) {
            count++;
        }
    }
    Serial.println(count);
}