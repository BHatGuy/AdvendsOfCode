#include <iostream>
#include <string>
#include <utility>
#include <vector>

using std::cout;
using std::endl;

std::vector<std::string> read_input() {
    std::vector<std::string> input;
    std::string line;
    while (std::getline(std::cin, line)) {
        input.push_back(line);
    }

    return input;
}

int solve1(const std::vector<std::string>& input) {
    int nice = 0;
    for (auto l : input) {
        int vowls = 0;
        bool pair = false;
        char last = 0;
        bool bad = false;
        for (auto c : l) {
            if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') vowls++;
            if (last == c) pair = true;
            if ((last == 'a' && c == 'b') || (last == 'c' && c == 'd') || (last == 'p' && c == 'q') ||
                (last == 'x' && c == 'y'))
                bad = true;
            last = c;
        }
        if (vowls >= 3 && pair && !bad) nice++;
    }
    return nice;
}

int solve2(const std::vector<std::string>& input) {
    int nice = 0;
    for (auto l : input) {
        std::vector<std::pair<std::string, size_t>> pairs;
        char last[2] = {0, 0};
        bool contains_pair = false;
        bool repeat = false;
        size_t i = 0;
        for (auto c : l) {
            std::string pair;
            pair += last[1];
            pair += c;

            std::pair<std::string, size_t>* match = nullptr;
            for (auto p : pairs) {
                if (p.first == pair) {
                    match = &p;
                    break;
                }
            }

            if (match) {
                if (match->second != i - 1) contains_pair = true;
            } else {
                pairs.push_back(std::pair<std::string, size_t>(pair, i));
            }

            if (last[0] == c) {
                repeat = true;
            }

            last[0] = last[1];
            last[1] = c;
            i++;
        }
        if (contains_pair && repeat) nice++;
    }
    return nice;
}

int main(int argc, char const* argv[]) {
    auto input = read_input();
    cout << solve1(input) << endl;
    cout << solve2(input) << endl;
    return 0;
}
