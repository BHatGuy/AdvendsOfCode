#include <Arduino.h>
#include <string.h>

#include "day1.hpp"
#include "utils.hpp"

void solve_d2() {
    unsigned long sum = 0;
    unsigned long lsum = 0;
    int count = 0;
    
    while (true) {
        String s = Serial.readStringUntil('\r');
        if (s.length() == 0) break;
        int i1 = s.indexOf('x');
        int i2 = s.lastIndexOf('x');

        int a = s.substring(0, i1).toInt();
        int b = s.substring(i1 + 1, i2).toInt();
        int c = s.substring(i2 + 1).toInt();

        int area = 2 * a * b + 2 * b * c + 2 * c * a + min(min(a * b, b * c), c * a);
        int len = 2 * a + 2 * b + 2 * c - 2 * max(max(a, b), c) + a * b * c;
        sum += area;
        lsum += len;
        count++;
    }
    Serial.print("Total area: ");
    Serial.println(sum);
    Serial.print("Total length: ");
    Serial.println(lsum);
}