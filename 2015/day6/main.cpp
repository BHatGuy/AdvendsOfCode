#include <iostream>
#include <regex>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

enum Action { TURN_ON, TURN_OFF, TOGGLE };

struct Command {
    Action action;
    int from[2];
    int to[2];
};

vector<Command> read_input() {
    vector<Command> input;
    string line;
    auto const regex = std::regex(R"((turn off|turn on|toggle) (\d+),(\d+) through (\d+),(\d+))");

    while (std::getline(std::cin, line)) {
        auto matches = std::smatch{};
        std::regex_search(line, matches, regex);
        Command com{
            TURN_ON, {std::stoi(matches[2]), std::stoi(matches[3])}, {std::stoi(matches[4]), std::stoi(matches[5])}};
        if (matches[1] == "turn off") com.action = TURN_OFF;
        if (matches[1] == "turn on") com.action = TURN_ON;
        if (matches[1] == "toggle") com.action = TOGGLE;
        input.push_back(com);
    }

    return input;
}

int solve1(const vector<Command>& input) {
    bool grid[1000][1000] = {0};
    for (auto command : input) {
        for (int x = command.from[0]; x <= command.to[0]; x++) {
            for (int y = command.from[1]; y <= command.to[1]; y++) {
                switch (command.action) {
                    case TURN_ON:
                        grid[x][y] = true;
                        break;
                    case TURN_OFF:
                        grid[x][y] = false;
                        break;
                    case TOGGLE:
                        grid[x][y] = !grid[x][y];
                        break;
                }
            }
        }
    }
    int count = 0;
    for (int x = 0; x <= 999; x++) {
        for (int y = 0; y <= 999; y++) {
            if (grid[x][y]) count++;
        }
    }
    return count;
}

uint64_t solve2(const vector<Command>& input) {
    uint64_t grid[1000][1000] = {0};
    for (auto command : input) {
        for (int x = command.from[0]; x <= command.to[0]; x++) {
            for (int y = command.from[1]; y <= command.to[1]; y++) {
                switch (command.action) {
                    case TURN_ON:
                        grid[x][y] += 1;
                        break;
                    case TURN_OFF:
                        if (grid[x][y] <= 1) grid[x][y] = 0;
                        else grid[x][y] -= 1;
                        
                        break;
                    case TOGGLE:
                        grid[x][y] += 2;
                        break;
                }
            }
        }
    }
    uint64_t sum = 0;
    for (int x = 0; x <= 999; x++) {
        for (int y = 0; y <= 999; y++) {
            sum += grid[x][y];
        }
    }
    return sum;
}

int main(int argc, char const* argv[]) {
    auto input = read_input();
    cout << solve1(input) << endl;
    cout << solve2(input) << endl;
    return 0;
}
