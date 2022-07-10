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

vector<string> read_input() {
    vector<string> input;
    string line;
    while (std::getline(std::cin, line)) {
        input.push_back(line);
    }

    return input;
}

void solve(const vector<string>& input) {
    string regstring = R"--(^(\w+) to (\w+) = (\d+)$)--";
    std::regex regex(regstring);
    std::set<string> names;
    for (auto line : input) {
        auto matches = std::smatch{};
        std::regex_search(line, matches, regex);
        names.insert(matches[1]);
        names.insert(matches[2]);
    }
    vector<vector<size_t>> matrix(names.size(), vector<size_t>(names.size()));

    for (auto line : input) {
        auto matches = std::smatch{};
        std::regex_search(line, matches, regex);
        size_t start = std::distance(names.begin(), std::find(names.begin(), names.end(), matches[1]));
        size_t end = std::distance(names.begin(), std::find(names.begin(), names.end(), matches[2]));
        size_t dist = std::stoi(matches[3]);
        matrix[start][end] = dist;
        matrix[end][start] = dist;
    }

    vector<size_t> indexes(names.size());
    std::iota(indexes.begin(), indexes.end(), 0);
    size_t min = __SIZE_MAX__;
    size_t max = 0;
    do {
        size_t d = 0;
        for (size_t i = 1; i < indexes.size(); i++) {
            d += matrix[indexes[i-1]][indexes[i]];
        }
        min = std::min(min, d);
        max = std::max(max, d);
    } while (std::next_permutation(indexes.begin(), indexes.end()));

    cout << min << " " << max << endl;
}

int main(int argc, char const* argv[]) {
    auto input = read_input();
    solve(input);
    return 0;
}
