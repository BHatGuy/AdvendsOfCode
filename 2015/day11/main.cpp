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

void inc(string& input) {
    char cin = 1;
    for (int i = input.size() - 1; i >= 0; i--) {
        input[i] = input[i] + cin;
        if (input[i] > 'z') {
            cin = 1;
            input[i] = 'a';
        } else {
            break;
        }
    }
}

void solve(string& input) {
    while (1) {
        inc(input);

        char last = 0;
        size_t straight = 0;
        size_t straight_max = 0;
        bool bad = false;
        size_t pair = false;
        bool last_pair = false;

        for (char c : input) {
            if (last == c - 1) {
                straight++;
            } else {
                straight = 0;
            }
            if (c == 'i' || c == 'o' || c == 'l') {
                bad = true;
                break;
            }
            if (last == c && !last_pair) {
                pair++;
                last_pair = true;
            } else {
                last_pair = false;
            }

            last = c;
            straight_max = std::max(straight, straight_max);
        }

        if (pair >= 2 && !bad && straight_max >= 2) {
            return;
        }
    }
}

int main(int argc, char const* argv[]) {
    auto input = read_input();
    solve(input);
    cout << input << endl;
    solve(input);
    cout << input << endl;
    return 0;
}
