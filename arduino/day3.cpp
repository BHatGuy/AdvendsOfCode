#include "day3.hpp"

#include <Arduino.h>
#include <string.h>

#include "utils.hpp"

struct Point {
    int x;
    int y;
};

bool compare_point(void* a, void* b) {
    Point* p1 = reinterpret_cast<Point*>(a);
    Point* p2 = reinterpret_cast<Point*>(b);
    return p1->x == p2->x && p1->y == p2->y;
}

void solve_d3() {
    List* l = new List();
    int x = 0;
    int y = 0;
    l->append(new Point{x, y});
    while (true) {
        char c = get_char();
        Serial.print(c);

        if (c == '>') {
            x++;
        } else if (c == '<') {
            x--;
        } else if (c == '^') {
            y++;
        } else if (c == 'v') {
            y--;
        } else if (c == '\r') {
            break;
        }
        Point* p = new Point{x, y};
        if (!l->contains(p, compare_point)) {
            l->append(p);
        }
    }
    Serial.print("Visited houses: ");
    Serial.println(l->length());
}