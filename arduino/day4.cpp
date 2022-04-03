#include <Arduino.h>
#include <string.h>

#include "MD5.hpp"
#include "day4.hpp"
#include "utils.hpp"

void solve_d4() {
    String secret = Serial.readStringUntil('\r');
    Serial.println(secret);
    for (unsigned long i = 1; i > 0; i++) {
        String guess = secret + String(i);
        unsigned char* hash = MD5::make_hash((char*)guess.c_str());
        char* md5str = MD5::make_digest(hash, 16);
        if (md5str[0] == '0' && md5str[1] == '0' && md5str[2] == '0' && md5str[3] == '0' && md5str[4] == '0') {
            Serial.println();
            Serial.println(md5str);
            Serial.println(i);
            Serial.println();
        }
        if (md5str[0] == '0' && md5str[1] == '0' && md5str[2] == '0' && md5str[3] == '0' && md5str[4] == '0' && md5str[5] == '0') {
            Serial.println();
            Serial.println(md5str);
            Serial.println(i);
            free(md5str);
            free(hash);
            return;
        }

        if (i % 500 == 0) {
            Serial.print("\r");
            Serial.print(i);
            Serial.print(" ");
            Serial.print(md5str);
        }
        free(md5str);
        free(hash);
    }
}