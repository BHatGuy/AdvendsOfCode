#include <cinttypes>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

using std::cout;
using std::endl;
using std::set;
using std::string;
using std::vector;

struct Point2D {
    int x;
    int y;

    Point2D operator+(const Point2D& other) const { return Point2D{this->x + other.x, this->y + other.y}; }
    Point2D operator-(const Point2D& other) const { return Point2D{this->x - other.x, this->y - other.y}; }
    Point2D operator/(int a) { return Point2D{this->x / a, this->y / a}; }
    Point2D& operator/=(int a) {
        this->x /= a;
        this->y /= a;
        return *this;
    }
    Point2D& operator+=(const Point2D& other) {
        this->x += other.x;
        this->y += other.y;
        return *this;
    }

    unsigned abs() const { return std::max(std::abs(x), std::abs(y)); }

    void print() { cout << "(" << x << ", " << y << ")" << endl; }

    void follow(const Point2D& other) {
        // follow with tail
        Point2D d = other - *this;
        if (d.abs() == 2) {
            if (d.x == 0 || d.y == 0 || (std::abs(d.x) == 2 && std::abs(d.y) == 2)) {
                *this += d / 2;
            } else {
                if (std::abs(d.x) > std::abs(d.y)) {
                    d.x /= 2;
                } else {
                    d.y /= 2;
                }
                *this += d;
            }
        } else if (d.abs() > 2) {
            cout << "Too much distance" << endl;
            exit(1);
        }
    }
};

void solve(string filename, size_t len) {
    std::fstream input(filename);
    string line;
    vector<Point2D> rope(len, Point2D{0, 0});
    set<std::pair<int, int>> visited;

    while (std::getline(input, line)) {
        string direction;
        size_t steps;
        std::istringstream l(line);
        l >> direction >> steps;

        for (size_t i = 0; i < steps; i++) {
            // move head
            if (direction == "L")
                rope[0].x--;
            else if (direction == "R")
                rope[0].x++;
            else if (direction == "U")
                rope[0].y++;
            else if (direction == "D")
                rope[0].y--;

            // pull rope
            for (size_t i = 1; i < rope.size(); i++) {
                rope[i].follow(rope[i - 1]);
            }
            visited.insert(std::pair<int, int>(rope.back().x, rope.back().y));
        }
    }
    cout << visited.size() << endl;
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        solve(argv[1], 2);
        solve(argv[1], 10);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
