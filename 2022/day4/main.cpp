#include <cinttypes>
#include <fstream>
#include <iostream>
#include <string>

using std::cout;
using std::endl;
using std::string;

void solve(string file) {
    std::fstream input(file);
    char _;
    uint32_t a, b, c, d;
    uint32_t contains = 0;
    uint32_t overlaps = 0;

    while (input >> a >> _ >> b >> _ >> c >> _ >> d) {
        if (a >= c && b <= d || c >= a && d <= b) contains++;
        if (a >= c && a <= d || b >= c && b <= d || c >= a && d <= b) overlaps++;
    }
    cout << contains << endl;
    cout << overlaps << endl;
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        solve(argv[1]);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
