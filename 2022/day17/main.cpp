#include <algorithm>
#include <cinttypes>
#include <climits>
#include <complex>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

struct Rock {
    enum Type { Minus, Plus, L, I, Block };

    int x, y;
    vector<vector<char>>& chamber;
    Type type;
    string& jets;
    bool down = false;

    Rock(vector<vector<char>>& chamber, string& jets) : chamber{chamber}, jets{jets} {
        static Type t = Minus;
        x = 2;
        y = 0;
        type = t;
        t = (Type)((int)t + 1);
        if (t > Block) t = Minus;
    }

    void persist() {
        switch (type) {
            case Minus:
                chamber[y - 0][x + 0] = '#';
                chamber[y - 0][x + 1] = '#';
                chamber[y - 0][x + 2] = '#';
                chamber[y - 0][x + 3] = '#';
                break;
            case Plus:
                chamber[y - 2][x + 1] = '#';
                chamber[y - 1][x + 0] = '#';
                chamber[y - 1][x + 1] = '#';
                chamber[y - 1][x + 2] = '#';
                chamber[y - 0][x + 1] = '#';
                break;
            case L:
                chamber[y - 2][x + 2] = '#';
                chamber[y - 1][x + 2] = '#';
                chamber[y - 0][x + 0] = '#';
                chamber[y - 0][x + 1] = '#';
                chamber[y - 0][x + 2] = '#';
                break;
            case I:
                chamber[y - 3][x + 0] = '#';
                chamber[y - 2][x + 0] = '#';
                chamber[y - 1][x + 0] = '#';
                chamber[y - 0][x + 0] = '#';
                break;
            case Block:
                chamber[y - 1][x + 0] = '#';
                chamber[y - 1][x + 1] = '#';
                chamber[y - 0][x + 0] = '#';
                chamber[y - 0][x + 1] = '#';
                break;
        }
        int free = 0;
        for (auto& row : chamber) {
            if (std::find(row.begin(), row.end(), '#') != row.end()) break;
            free++;
            bool all = true;
            for (char c: row) {
                all &= c == '#';
            }
            if (all) cout << "ALL" << endl;
        }

        for (int i = 0; i < 4 - free; i++) {
            vector<char> row(7, '.');
            chamber.insert(chamber.begin(), row);
        }
    }

    bool fall() {
        static size_t jet_index = 0;

        if (down) {
            bool below = false;
            if (y + 1 >= chamber.size()) {
                persist();
                return false;
            }
            switch (type) {
                case Minus:
                    below |= chamber[y + 1][x + 0] != '.';
                    below |= chamber[y + 1][x + 1] != '.';
                    below |= chamber[y + 1][x + 2] != '.';
                    below |= chamber[y + 1][x + 3] != '.';
                    break;
                case Plus:
                    below |= chamber[y][x] != '.';
                    below |= chamber[y][x + 2] != '.';
                    below |= chamber[y + 1][x + 1] != '.';
                    break;
                case L:
                    below |= chamber[y + 1][x + 0] != '.';
                    below |= chamber[y + 1][x + 1] != '.';
                    below |= chamber[y + 1][x + 2] != '.';
                    break;
                case I:
                    below |= chamber[y + 1][x + 0] != '.';
                    break;
                case Block:
                    below |= chamber[y + 1][x + 0] != '.';
                    below |= chamber[y + 1][x + 1] != '.';
                    break;
            }
            if (below) {
                persist();
                return false;
            } else {
                y++;
            }
        } else {
            int dx = jets[jet_index] == '>' ? 1 : -1;
            bool collide = false;
            switch (type) {
                case Minus:
                    collide |= chamber[y][x + dx] != '.';
                    collide |= chamber[y][x + dx + 3] != '.';
                    break;
                case Plus:
                    collide |= y - 2 >= 0 && chamber[y - 2][x + dx + 1] != '.';
                    collide |= y - 1 >= 0 && chamber[y - 1][x + dx] != '.';
                    collide |= y - 1 >= 0 && chamber[y - 1][x + dx + 2] != '.';
                    collide |= chamber[y][x + dx + 1] != '.';
                    break;
                case L:
                    collide |= chamber[y][x + dx] != '.';
                    collide |= chamber[y][x + dx + 2] != '.';
                    collide |= y - 1 >= 0 && chamber[y - 1][x + dx + 2] != '.';
                    collide |= y - 2 >= 0 && chamber[y - 2][x + dx + 2] != '.';
                    break;
                case I:
                    collide |= chamber[y][x + dx] != '.';
                    collide |= y - 1 >= 0 && chamber[y - 1][x + dx + 0] != '.';
                    collide |= y - 2 >= 0 && chamber[y - 2][x + dx + 0] != '.';
                    collide |= y - 3 >= 0 && chamber[y - 3][x + dx + 0] != '.';
                    break;
                case Block:
                    collide |= chamber[y][x + dx] != '.';
                    collide |= y - 1 >= 0 && chamber[y - 1][x + dx + 0] != '.';
                    collide |= chamber[y][x + dx + 1] != '.';
                    collide |= y - 1 >= 0 && chamber[y - 1][x + dx + 1] != '.';
                    break;
            }
            if (!collide) {
                x += dx;
            }
        }

        if (!down) {
            jet_index++;
            if (jet_index == jets.size()) {
                jet_index = 0;
            }
        }
        down = !down;
        return true;
    }
};

void solve(string filename, uint64_t iterations) {
    std::ifstream input(filename);
    string jets;
    input >> jets;

    vector<vector<char>> chamber(4, vector<char>(7, '.'));
    for (int i = 0; i < iterations; i++) {
        Rock r(chamber, jets);
        while (r.fall())
            ;
        // cout << i << endl;
        // for (auto& row : chamber) {
        //     for (auto&& c : row) {
        //         cout << c;
        //     }
        //     cout << endl;
        // }
        // cout << endl;
    }
    cout << chamber.size() - 4 << endl;
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        solve(argv[1], 2022);
        solve(argv[1], 50455);
        // solve(argv[1], 1000000000000);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
