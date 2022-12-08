#include <cinttypes>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

typedef vector<vector<uint32_t>> matrix_t;

bool visible(const matrix_t& matrix, size_t x, size_t y) {
    if (x == 0 || y == 0 || x >= matrix.size() || y >= matrix[0].size()) return true;

    // check x
    bool before = true;
    bool after = true;
    for (size_t i = 0; i < matrix.size(); i++) {
        if (i == x) {
            if (before) return true;
            continue;
        }
        if (i < x && matrix[i][y] >= matrix[x][y]) {
            before = false;
            i = x + 1;
        }
        if (i > x && matrix[i][y] >= matrix[x][y]) {
            after = false;
            break;
        }
    }

    if (before || after) return true;

    // check y
    before = true;
    after = true;
    for (size_t i = 0; i < matrix[0].size(); i++) {
        if (i == y) {
            if (before) return true;
            continue;
        }
        if (i < y && matrix[x][i] >= matrix[x][y]) {
            before = false;
            i = y + 1;
        }
        if (i > y && matrix[x][i] >= matrix[x][y]) {
            after = false;
            break;
        }
    }
    if (before || after) return true;
    return false;
}

size_t view_count(matrix_t matrix, size_t x, size_t y) {
    size_t vcount = 1;
    for (int dx = -1; dx <= 1; dx++) {
        for (int dy = -1; dy <= 1; dy++) {
            if (dx == 0 && dy == 0) continue;
            if (dx != 0 && dy != 0) continue;
            size_t count = 0;
            for (int i = 1;; i++) {
                int nx = x + i * dx;
                int ny = y + i * dy;
                if (nx < 0 || nx >= (int)matrix.size() || ny < 0 || ny >= (int)matrix[0].size()) break;
                count++;
                if (matrix[nx][ny] >= matrix[x][y]) break;
            }
            vcount *= count;
        }
    }
    return vcount;
}

void solve(string filename) {
    std::fstream input(filename);
    string line;
    matrix_t matrix;
    while (std::getline(input, line)) {
        vector<uint32_t> row;
        for (char c : line) {
            row.push_back(c - '0');
        }
        matrix.push_back(row);
    }

    size_t count = 0;
    for (size_t x = 1; x < matrix.size() - 1; x++) {
        for (size_t y = 1; y < matrix[0].size() - 1; y++) {
            if (visible(matrix, x, y)) count++;
        }
    }
    cout << count + matrix.size() * 2 + (matrix[0].size() - 2) * 2 << endl;

    size_t vcount = 0;
    for (size_t x = 0; x < matrix.size(); x++) {
        for (size_t y = 0; y < matrix[0].size(); y++) {
            vcount = std::max(vcount, view_count(matrix, x, y));
        }
    }
    cout << vcount << endl;
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        solve(argv[1]);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
