#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;

char intersection(string first, string second) {
    for (char c : first) {
        if (second.find(c, 0) != string::npos) {
            return c;
        }
    }
    cout << first << " " << second << endl;
    exit(1);
}

char intersection3(string one, string two, string three) {
    for (char  c : one) {
        if (two.find(c, 0) != string::npos) {
            if (three.find(c, 0) != string::npos) {
                return c;
            }
        }
    }
    cout << one << " " << two << " " << three << endl;
    exit(1);
}

void solve(string input) {
    std::fstream infile(input);
    char opponent, me;
    string line;
    int sum = 0;
    while (std::getline(infile, line)) {
        if (line.size() == 0) break;
        size_t half = line.size() / 2;
        string first = line.substr(0, half);
        string second = line.substr(half, half);
        char common = intersection(first, second);
        if (common >= 'a') {
            sum += common - 'a' + 1;
        } else {
            sum += common - 'A' + 27;
        }
    }
    cout << sum << endl;
}

void solve2(string input) {
    std::fstream infile(input);
    char opponent, me;
    string a, b, c;
    int sum = 0;
    while (infile >> a >> b >> c) {
        char common = intersection3(a, b, c);
        if (common >= 'a') {
            sum += common - 'a' + 1;
        } else {
            sum += common - 'A' + 27;
        }
    }
    cout << sum << endl;

}

int main() {
    solve("test.txt");
    solve("input.txt");
    solve2("test.txt");
    solve2("input.txt");
}
