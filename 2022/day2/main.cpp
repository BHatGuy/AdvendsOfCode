#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;

int wins(char opponent, char me) {
    if (opponent == 'A') {
        if (me == 'Y') return 6;
        if (me == 'Z') return 0;
    }
    if (opponent == 'B') {
        if (me == 'X') return 0;
        if (me == 'Z') return 6;
    }
    if (opponent == 'C') {
        if (me == 'X') return 6;
        if (me == 'Y') return 0;
    }
    return 3;
}

int calc_shape(char opponent, char me) {
    if (me == 'X') {
        if (opponent == 'A') return 3;
        if (opponent == 'B') return 1;
        if (opponent == 'C') return 2;
    }
    if (me == 'Y') {
        return opponent - 'A' + 1;
    }
    if (me == 'Z') {
        if (opponent == 'A') return 2;
        if (opponent == 'B') return 3;
        if (opponent == 'C') return 1;
    }
    exit(-1);
}

void solve(string input) {
    std::fstream infile(input);
    char opponent, me;
    int sum = 0;
    while (infile >> opponent >> me) {
        int win = wins(opponent, me);
        int shape = me - 'X' + 1;
        sum += shape + win;
    }
    cout << sum << endl;
}

void solve2(string input) {
    std::fstream infile(input);
    char opponent, me;
    int sum = 0;
    while (infile >> opponent >> me) {
        int win = (me - 'X') * 3;
        int shape = calc_shape(opponent, me);
        sum += shape + win;
    }
    cout << sum << endl;
}

int main() {
    solve("test.txt");
    solve("input.txt");
    solve2("test.txt");
    solve2("input.txt");
}
