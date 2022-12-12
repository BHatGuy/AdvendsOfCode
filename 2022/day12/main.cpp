#include <algorithm>
#include <cinttypes>
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

struct Node {
    int x;
    int y;

    char elevation;
    int distance = -1;
    int estimate = -1;
    Node* predecessor = nullptr;
};

void solve(string filename) {
    std::ifstream input(filename);
    vector<vector<Node*>> map;

    string line;

    Node* start;
    Node* end;
    int x = 0, y = 0;
    while (std::getline(input, line)) {
        vector<Node*> row;
        for (char& c : line) {
            row.push_back(new Node{x, y, c});
            Node* node = row.back();

            if (c == 'S') {
                start = node;
                start->elevation = 'a';
                start->distance = 0;
                start->estimate = 0;
            }
            if (c == 'E') {
                end = node;
                end->elevation = 'z';
            }
            x++;
        }
        map.push_back(row);
        x = 0;
        y++;
    }

    vector<Node*> visited;
    vector<Node*> pending;
    pending.push_back(start);

    while (!pending.empty()) {
        Node* current = pending.back();
        pending.pop_back();
        if (current == end) {
            cout << current->distance << endl;
            break;
        }
        visited.push_back(current);

        // expand current node
        for (int dx = -1; dx <= 1; dx++) {
            for (int dy = -1; dy <= 1; dy++) {
                if (!((dx == 0) != (dy == 0))) continue;
                size_t x = dx + current->x;
                size_t y = dy + current->y;
                if (x >= map[0].size() || y >= map.size()) continue;

                Node* successor = map[y][x];

                if (successor->elevation - current->elevation > 1) continue;

                if (std::find(visited.begin(), visited.end(), successor) != visited.end()) continue;

                int distance = current->distance + 1;

                if (std::find(pending.begin(), pending.end(), successor) != pending.end() &&
                    distance >= successor->distance)
                    continue;
                successor->predecessor = current;
                successor->distance = distance;
                int estimate = distance + std::abs((int)x - end->x) + std::abs((int)y - end->y);
                if (std::find(pending.begin(), pending.end(), successor) == pending.end()) {
                    pending.push_back(successor);
                }
            }
        }
        std::sort(pending.begin(), pending.end(), [](Node* a, Node* b) { return a->estimate > b->estimate; });
    }
    Node* n = end;
    while (n) {
        n->elevation += 'A' - 'a';
        n = n->predecessor;
    }

    for (auto row : map) {
        for (auto p : row) {
            if (p->elevation < 'a') cout << "\x1b[41m";
            cout << p->elevation;
            if (p->elevation < 'a') cout << "\x1b[0m";
        }
        cout << endl;
    }

    for (auto row : map) {
        for (auto p : row) {
            delete p;
        }
    }
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        solve(argv[1]);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
