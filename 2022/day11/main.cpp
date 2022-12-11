#include <cinttypes>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <regex>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

const std::regex monkey_reg(
    R"(^Monkey (\d+):\s+Starting items:(( \d+,?)+)\s+Operation: new = old ([\w\s\+\*]+)\s+Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)$)");

struct Monkey {
    int index;
    vector<uintmax_t> items;
    string operation;
    int divisor;
    int positive;
    int negative;

    uintmax_t activity = 0;
};

void solve(string filename, bool divide = true, size_t rounds = 20) {
    std::ifstream input(filename);

    vector<Monkey> monkeys;
    while (input) {
        string raw_monkey;
        for (int i = 0; i < 7; i++) {
            string l;
            std::getline(input, l);
            raw_monkey += l;
        }
        std::smatch matches;
        if (std::regex_match(raw_monkey, matches, monkey_reg)) {
            int index = std::stoi(matches[1]);
            string items_s = matches[2];
            string operation = matches[4];
            int divisor = std::stoi(matches[5]);
            int positive = std::stoi(matches[6]);
            int negative = std::stoi(matches[7]);

            vector<uintmax_t> items;
            std::stringstream ss(items_s);
            for (int i; ss >> i;) {
                items.push_back(i);
                if (ss.peek() == ',') ss.ignore();
            }

            monkeys.push_back(Monkey{index, items, operation, divisor, positive, negative});
        } else {
            std::cerr << "No match" << endl;
            exit(1);
        }
    }
    int product = 1;
    for (auto& monkey: monkeys) {
        product *= monkey.divisor;
    }

    for (size_t i = 0; i < rounds; i++) {
        for (auto& monkey : monkeys) {
            // inspect
            for (auto& item : monkey.items) {
                uintmax_t value;
                string _;
                std::stringstream o(monkey.operation);
                o >> _ >> value;
                if (value == 0) value = item;
                if (monkey.operation[0] == '+') item += value;
                if (monkey.operation[0] == '*') item *= value;
                if (divide) item /= 3;
                else item = item % product;
                monkey.activity++;
            }

            // throw
            while (monkey.items.size() > 0) {
                uintmax_t item = monkey.items.front();
                monkey.items.erase(monkey.items.begin());
                int index;
                if (item % monkey.divisor == 0) {
                    index = monkey.positive;
                } else {
                    index = monkey.negative;
                }
                monkeys[index].items.push_back(item);
            }
        }
    }
    std::sort(monkeys.begin(), monkeys.end(), [](Monkey a, Monkey b) { return a.activity > b.activity; });
    cout << monkeys[0].activity * monkeys[1].activity << endl;
}

int main(int argc, char* argv[]) {
    if (argc == 2) {
        solve(argv[1]);
        solve(argv[1], false, 10000);
    } else {
        cout << "Usage: " << argv[0] << " INPUT.txt" << endl;
        return -1;
    }
}
