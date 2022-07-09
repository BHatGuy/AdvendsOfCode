#include <iostream>
#include <map>
#include <regex>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

enum Groups {
    SINGLE = 2,
    OR = 3,
    AND = 6,
    SHIFT = 9,
    NOT = 13,
    TARGET = 15,
};

bool is_number(const std::string& s) { return !s.empty() && std::all_of(s.begin(), s.end(), ::isdigit); }

vector<string> read_input() {
    vector<string> input;
    string line;
    while (std::getline(std::cin, line)) {
        input.push_back(line);
    }

    return input;
}

int solve(vector<string>& input) {
    string regstring =
        R"--(^((\w+)|((\w+) OR (\w+))|((\w+) AND (\w+))|((\w+) (L|R)SHIFT (\d+))|(NOT (\w+))) -> ([a-z]+)$)--";
    std::regex regex(regstring);

    std::map<string, uint16_t> wires;
    size_t size = input.size();
    while (input.size() > 0) {
        for (auto line = input.begin(); line != input.end();) {
            bool used = true;
            auto matches = std::smatch{};
            std::regex_search(*line, matches, regex);

            string target = matches[TARGET];
            if (matches[SINGLE].matched > 0) {
                if (is_number(matches[SINGLE])) {
                    // literal 1 -> y
                    wires[target] = std::stoi(matches[SINGLE]);
                } else {
                    // assignment x -> y
                    if (wires.count(matches[SINGLE])) {
                        wires[target] = wires[matches[SINGLE]];
                    } else {
                        used = false;
                    }
                }
            } else if (matches[OR].matched > 0) {
                string a = matches[OR + 1];
                string b = matches[OR + 2];
                if (wires.count(a) && wires.count(b)) {
                    wires[target] = wires[a] | wires[b];
                } else if (wires.count(a) && is_number(b)) {
                    wires[target] = wires[a] | std::stoi(b);
                } else if (is_number(a) && wires.count(b)) {
                    wires[target] = std::stoi(a) | wires[b];
                } else {
                    used = false;
                }

            } else if (matches[AND].matched > 0) {
                string a = matches[AND + 1];
                string b = matches[AND + 2];
                if (wires.count(a) && wires.count(b)) {
                    wires[target] = wires[a] & wires[b];
                } else if (wires.count(a) && is_number(b)) {
                    wires[target] = wires[a] & std::stoi(b);
                } else if (is_number(a) && wires.count(b)) {
                    wires[target] = std::stoi(a) & wires[b];
                } else {
                    used = false;
                }
            } else if (matches[SHIFT].matched > 0) {
                string var = matches[SHIFT + 1];
                string dir = matches[SHIFT + 2];
                int dist = std::stoi(matches[SHIFT + 3]);
                if (wires.count(var)) {
                    if (dir == "L") {
                        wires[target] = wires[var] << dist;
                    } else {
                        wires[target] = wires[var] >> dist;
                    }
                } else {
                    used = false;
                }
            } else if (matches[NOT].matched > 0) {
                string var = matches[NOT + 1];
                if (wires.count(var)) {
                    wires[target] = ~wires[var];
                } else {
                    used = false;
                }
            }

            if (!used) {
                line++;
            } else {
                line = input.erase(line);
            }
        }
        if (size == input.size()) {
            break;
        }
        size = input.size();
    }
    for (auto l : input) {
        cout << l << endl;
    }

    // for (auto x : wires) {
    //     cout << x.first << "=" << x.second << endl;
    // }
    return wires["a"];
}

int main(int argc, char const* argv[]) {
    auto input = read_input();
    auto input1(input);
    cout << solve(input1) << " " << endl;
    for (auto line = input.begin(); line != input.end();) {
        if (line->substr(line->size() - 2, 2) == " b"){
            line = input.erase(line);
        } else {
            line++;
        }
    }
    // todo use value from previous solve
    input.push_back("16076 -> b");
    cout << solve(input) << " " << endl;
    return 0;
}
