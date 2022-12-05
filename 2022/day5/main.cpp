#include <algorithm>
#include <cinttypes>
#include <cstddef>
#include <cstring>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;

// https://stackoverflow.com/a/17244655/7821335
// These are handy bits that go in a header somewhere
template <class e, class t, int N>
std::basic_istream<e, t>& operator>>(std::basic_istream<e, t>& in, const e (&sliteral)[N]) {
    e buffer[N - 1] = {};                   // get buffer
    in >> buffer[0];                        // skips whitespace
    if (N > 2) in.read(buffer + 1, N - 2);  // read the rest
    if (strncmp(buffer, sliteral, N - 1))   // if it failed
        in.setstate(std::ios::failbit);     // set the state
    return in;
}
template <class e, class t>
std::basic_istream<e, t>& operator>>(std::basic_istream<e, t>& in, const e& cliteral) {
    e buffer(0);                         // get buffer
    in >> buffer;                        // read data
    if (buffer != cliteral)              // if it failed
        in.setstate(std::ios::failbit);  // set the state
    return in;
}
// redirect mutable char arrays to their normal function
template <class e, class t, int N>
std::basic_istream<e, t>& operator>>(std::basic_istream<e, t>& in, e (&carray)[N]) {
    return std::operator>>(in, carray);
}

void print(std::vector<std::vector<char>>& ship) {
    for (auto stack : ship) {
        for (auto container : stack) {
            cout << container;
        }
        cout << endl;
    }
}

void solve(string file) {
    std::fstream input(file);
    string line;
    std::getline(input, line);
    size_t stacks = (line.size() + 1) / 4;
    std::vector<std::vector<char>> ship(stacks);
    while (!line.empty()) {
        if (line.find("[") == string::npos) break;
        std::istringstream l(line);

        for (int i = 0; i < stacks; i++) {
            string container;
            container.resize(4);
            l.read(&container[0], 4);

            if (container.find("[") == string::npos) continue;
            ship[i].insert(ship[i].begin(), container[1]);
        }

        std::getline(input, line);
    }
    auto ship2 = ship;
    while (std::getline(input, line)) {
        if (line.empty()) continue;
        std::istringstream l(line);
        size_t from, to, count;
        l >> "move" >> count >> "from" >> from >> "to" >> to;
        to--;
        from--;
        std::vector<char> crane;
        for (int i = 0; i < count; i++) {
            // Ship 1
            ship[to].push_back(ship[from].back());
            ship[from].pop_back();
            // Ship 2
            crane.push_back(ship2[from].back());
            ship2[from].pop_back();
        }
        std::reverse(crane.begin(), crane.end());
        ship2[to].insert(ship2[to].end(), crane.begin(), crane.end());
    }
    for (auto stack : ship) {
        cout << stack.back();
    }
    cout << endl;
    for (auto stack : ship2) {
        cout << stack.back();
    }
    cout << endl;
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        solve(argv[1]);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
