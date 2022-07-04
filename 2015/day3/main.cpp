#include <iostream>
#include <set>
#include <string>
#include <utility>

std::string read_input_line() {
    std::string stuff;
    std::cin >> stuff;
    return stuff;
}

uint64_t solve1(const std::string& input) {
    std::pair<int, int> pos = {0, 0};
    std::set<std::pair<int, int>> positions;
    positions.insert(pos);
    for (auto c : input) {
        switch (c) {
            case '>':
                pos.first++;
                break;

            case '<':
                pos.first--;
                break;

            case '^':
                pos.second++;
                break;

            case 'v':
                pos.second--;
                break;

            default:
                std::cerr << "Illegal char " << c << std::endl;
                exit(-1);
        }
        positions.insert(pos);
    }

    return positions.size();
}

uint64_t solve2(const std::string& input) {
    std::pair<int, int> santa = {0, 0};
    std::pair<int, int> robot = {0, 0};
    std::set<std::pair<int, int>> positions;
    positions.insert(santa);
    bool santas_turn = true;
    for (auto c : input) {
        switch (c) {
            case '>':
                if (santas_turn) {
                    santa.first++;
                } else {
                    robot.first++;
                }
                break;

            case '<':
                if (santas_turn) {
                    santa.first--;
                } else {
                    robot.first--;
                }
                break;

            case '^':
                if (santas_turn) {
                    santa.second++;
                } else {
                    robot.second++;
                }
                break;

            case 'v':
                if (santas_turn) {
                    santa.second--;
                } else {
                    robot.second--;
                }
                break;

            default:
                std::cerr << "Illegal char " << c << std::endl;
                exit(-1);
        }
        if (santas_turn) {
            positions.insert(santa);
        } else {
            positions.insert(robot);
        }

        santas_turn = !santas_turn;
    }

    return positions.size();
}

int main(int argc, char const* argv[]) {
    std::string input = read_input_line();
    uint64_t solution1 = solve1(input);
    uint64_t solution2 = solve2(input);

    std::cout << "Solution 1: " << solution1 << std::endl;
    std::cout << "Solution 2: " << solution2 << std::endl;
    return 0;
}
