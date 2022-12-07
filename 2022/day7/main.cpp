#include <algorithm>
#include <cinttypes>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;

std::regex re_cd("^\\$ cd (\\w+|/|\\.+)$");
std::regex re_ls("^\\$ ls");
std::regex re_file("^(\\d+) (.+)$");
std::regex re_dir("^dir (.+)$");

struct Node {
    string name;
    size_t size;
    std::vector<Node> children;
    Node *parent;

    void print() { this->print(""); }
    void print(string prefix) {
        cout << prefix << name << " " << size << endl;
        for (auto c : children) {
            c.print(prefix + "    ");
        }
    }

    size_t total_size() {
        size_t s = size;
        for (auto &c : children) {
            s += c.total_size();
        }
        return s;
    }

    size_t acc_size() {
        if (children.size() == 0) return 0;
        size_t s = total_size();
        if (s > 100000) s = 0;
        for (auto &c : children) {
            s += c.acc_size();
        }
        return s;
    }

    size_t find(size_t needed) {
        size_t min = total_size();
        for (auto &c : children) {
            if (c.children.size() == 0) continue;
            min = std::min(min, c.find(needed));
        }
        if (min > needed)
            return min;
        else
            return SIZE_MAX;
    }
};

void solve(string filename) {
    std::fstream input(filename);
    string line;

    Node root{"/", 0, std::vector<Node>(), nullptr};
    Node *cwd = &root;

    while (std::getline(input, line)) {
        std::smatch matches;
        if (std::regex_search(line, matches, re_cd)) {
            string name = matches[1];
            if (name == "..") {
                cwd = cwd->parent;
                continue;
            } else {
                for (auto &c : cwd->children) {
                    if (c.name == name) {
                        cwd = &c;
                        break;
                    }
                }
            }
        }
        if (std::regex_search(line, matches, re_file)) {
            string name = matches[2];
            size_t size = std::stoi(matches[1]);
            cwd->children.push_back({name, size, std::vector<Node>(), cwd});
        }
        if (std::regex_search(line, matches, re_dir)) {
            string name = matches[1];
            cwd->children.push_back({name, 0, std::vector<Node>(), cwd});
        }
    }

    cout << root.acc_size() << endl;

    size_t needed = 30000000 - (70000000 - root.total_size());
    cout << root.find(needed) << endl;
}

int main(int argc, char *argv[]) {
    if (argc == 2) {
        solve(argv[1]);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
