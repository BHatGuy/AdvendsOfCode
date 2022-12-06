#include <algorithm>
#include <cinttypes>
#include <cstddef>
#include <cstring>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <set>

using std::cout;
using std::endl;
using std::string;

void solve(string input, size_t lenght) {
    string window;
    int count = 0;
    for (char c: input) {
        count++;
        window.insert(window.begin(), c);
        if (window.size() > lenght) {
            window.pop_back();
        }
        if (window.size() < lenght) continue;
        std::set<char> s(window.begin(), window.end());
        if (s.size() == window.size()) {
            break;
        }

    }
    cout << count << endl;
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        std::fstream input(argv[1]);
        string line;
        while(std::getline(input, line)) {
            solve(line, 4);
            solve(line, 14);
        }
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
