#include <iostream>
#include <numeric>
#include <regex>
#include <set>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

string read_input() {
    string line;
    std::getline(std::cin, line);

    return line;
}

void solve(const string& input, size_t rounds) {
    string curr = input;
    string next;
    for (size_t i = 0; i < rounds; i++) {
        string acc;
        for (auto c : curr) {
            if (acc.size() > 0 && acc[acc.size() - 1] != c) {
                next += std::to_string(acc.size());
                next += acc[0];
                acc = string();
            }
            acc += c;
        }
        next += std::to_string(acc.size());
        next += acc[0];

        curr = next;
        next = string();
    }
    cout << curr.size() << endl;
}

int main(int argc, char const* argv[]) {
    auto input = read_input();
    solve(input, 40);
    solve(input, 50);
    return 0;
}
