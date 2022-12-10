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
using std::string;

void solve(string filename) {
    std::ifstream input(filename);
    string line;

    int reg = 1;
    size_t cycles = 1;
    size_t next_key = 20;
    int sum = 0;

    while (std::getline(input, line)) {
        if (line[0] == 'a' && cycles + 1 == next_key) {
            next_key += 40;
            sum += reg * (cycles + 1);
        }
        if (cycles == next_key) {
            next_key += 40;
            sum += reg * cycles;
        }
        // draw

        int pixels = 1;
        if (line[0] == 'a') {
            pixels = 2;
        }
        for (int i = 0; i < pixels; i++) {
            int pos = (cycles + i - 1) % 40;
            if (reg - 1 <= pos && pos <= reg + 1) {
                cout << '#';
            } else {
                cout << '.';
            }
            if (pos == 39) cout << endl;
        }

        // execute
        if (line[0] == 'n') {
            cycles += 1;
        } else if (line[0] == 'a') {
            cycles += 2;
            std::stringstream l(line);
            string _;
            int operand;
            l >> _ >> operand;
            reg += operand;
        } else {
            cout << "Illigeal Instruction" << endl;
            exit(-1);
        }
    }
    cout << sum << endl;
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        solve(argv[1]);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
