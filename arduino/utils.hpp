#pragma once

#include <Arduino.h>
#include <avr/pgmspace.h>

extern HardwareSerial Serial;

char get_char();

class List;
class Node {
    friend List;
    Node* next;
    void* val;  // TODO: generic

   public:
    Node(void* val);
    ~Node();
};

class List {
    Node* head;

   public:
    List();
    ~List();
    void append(void* val);
    bool contains(void* val, bool (*comp)(void*, void*));
    size_t length();

    // TODO: operators, iterate
};
