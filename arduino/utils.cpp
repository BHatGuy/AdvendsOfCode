#include "utils.hpp"

#include <Arduino.h>

char get_char() {
    while (!Serial.available())
        ;
    return Serial.read();
}

List::List() : head{nullptr} {}

List::~List() {
    Node* curr = head;
    while (curr) {
        Node* next = curr->next;
        delete curr;
        curr = next;
    }
}

void List::append(void* val) {
    if (!head) {
        head = new Node(val);
        return;
    }
    Node* curr = head;
    while (curr->next) curr = curr->next;
    curr->next = new Node(val);
}

size_t List::length() {
    size_t count = 0;
    Node* curr = head;
    while (curr) {
        curr = curr->next;
        count++;
    }
    return count;
}

bool List::contains(void* val, bool (*comp)(void*, void*)) {
    Node* curr = head;
    while (curr) {
        if (comp(curr->val, val)) {
            return true;
        }
        curr = curr->next;
    }
    return false;
}

Node::Node(void* val) : val{val}, next{nullptr} {}

Node::~Node() {
    delete val;
}