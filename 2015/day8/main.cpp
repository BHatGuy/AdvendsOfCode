#include <iostream>
#include <map>
#include <regex>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

vector<string> read_input() {
    vector<string> input;
    string line;
    while (std::getline(std::cin, line)) {
        input.push_back(line);
    }

    return input;
}

int solve1(const vector<string>& input) {
    size_t count_mem = 0;
    size_t count_real = 0;
    for (auto line : input) {
        count_real += line.size();
        bool escaped = false;
        for (size_t i = 0; i < line.size(); i++) {
            char c = line[i];
            if (c == '\\') {
                if (escaped) {
                    count_mem++;
                } else {
                    escaped = true;
                    continue;
                }
            } else if (c == '"') {
                if (escaped) count_mem++;
            } else if (escaped && c == 'x') {
                i += 2;
                count_mem++;
            } else {
                count_mem++;
            }
            escaped = false;
        }
    }
    return count_real - count_mem;
}

int solve2(const vector<string>& input) {
    size_t count_new = 0;
    size_t count_real = 0;
    for (auto line : input) {
        count_real += line.size();
        count_new += 2;
        for (char c : line) {
            if (c == '\"') count_new ++;
            if (c == '\\') count_new ++;
            count_new++;
        }
    }
    return count_new - count_real;
}

int main(int argc, char const* argv[]) {
    auto input = read_input();
    cout << solve1(input) << " " << endl;
    cout << solve2(input) << " " << endl;
    return 0;
}
