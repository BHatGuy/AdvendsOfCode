#include <algorithm>
#include <cinttypes>
#include <climits>
#include <complex>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <regex>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

using std::complex;
using std::cout;
using std::endl;
using std::pair;
using std::string;
using std::vector;

int manhattan(complex<int> c) { return std::abs(c.real()) + std::abs(c.imag()); }

void solve(string filename) {
    std::ifstream input(filename);
    int bx, by, sx, sy;
    string line;
    std::regex reg(R"(Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+))");

    vector<pair<complex<int>, complex<int>>> pairs;
    int min = INT_MAX;
    int max = INT_MIN;
    while (std::getline(input, line)) {
        std::smatch matches;
        if (std::regex_match(line, matches, reg)) {
            sx = std::stoi(matches[1]);
            sy = std::stoi(matches[2]);
            bx = std::stoi(matches[3]);
            by = std::stoi(matches[4]);
        } else {
            std::cerr << "OOPS" << endl;
            exit(1);
        }
        complex<int> sensor(sx, sy);
        complex<int> beacon(bx, by);
        pairs.push_back(pair<complex<int>, complex<int>>(sensor, beacon));
        min = std::min(min, sx - manhattan(sensor - beacon));
        max = std::max(max, sx + manhattan(sensor - beacon));
    }

    int count = 0;
    for (int x = min; x <= max; x++) {
        bool too_close = false;
        complex<int> current(x, 2000000);
        for (auto& pair : pairs) {
            auto& sensor = pair.first;
            auto& beacon = pair.second;
            too_close |= manhattan(sensor - current) <= manhattan(sensor - beacon);
            if (current == beacon) {
                too_close = false;
                break;
            }
        }
        if (too_close) count++;
    }
    cout << count << endl;

    // 2

    for (int x = 0; x <=4000000 ; x++) {
        for (int y = 0; y <= 4000000; y++) {
            bool too_close = false;
            complex<int> current(x, y);
            for (auto& pair : pairs) {
                auto& sensor = pair.first;
                auto& beacon = pair.second;
                too_close |= manhattan(sensor - current) <= manhattan(sensor - beacon);
                if (current == beacon) {
                    too_close = true;
                    break;
                }
            }
            if (!too_close) {
                cout << current.real()*4000000+current.imag() << endl;
                break;
            }
        }
    }
}

    int main(int argc, char* argv[]) {
        if (argc == 2) {
            solve(argv[1]);
        } else {
            cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
            return -1;
        }
    }
