#include "day1.hpp"

#include <Arduino.h>
#include <string.h>

#include "utils.hpp"

void solve_d1() {
    int count = 0;
    int i = 1;
    int first_basement = -1;

    while(true) {
        char c = get_char();
        Serial.print(c);
        if (c == '(') {
            count++;
        } else if (c == ')') {
            count--;
        } else if (c == '\r'){
            break;
        } 
        else {
            Serial.print(c);
            Serial.println(" Illegal char!");
            break;
        }
        if(count < 0 && first_basement == -1) {
            first_basement = i;
        }
        i++;
    }
    Serial.println();
    Serial.print("Final floor: ");
    Serial.println(count);
    Serial.print("First basement pos: ");
    Serial.println(first_basement);
}