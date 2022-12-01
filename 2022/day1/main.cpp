#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

void solve() {
    std::fstream infile("input.txt");
    std::string line;

    int sum = 0;
    std::vector<int> sums;
    while (std::getline(infile, line)) {
        if (line.empty()) {
            sums.push_back(sum);
            sum = 0;
        } else {
            std::istringstream l(line);
            int a;
            l >> a;
            sum += a;
        }
    }

    int max = 0;
    for (auto x : sums) {
        max = std::max(max, x);
    }
    std::sort(sums.begin(), sums.end());
    std::cout << sums[sums.size() - 1] << std::endl;
    std::cout << sums[sums.size() - 1] + sums[sums.size() - 2] + sums[sums.size() - 3] << std::endl;
}

int main() { solve(); }
