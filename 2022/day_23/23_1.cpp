#include <bits/stdc++.h>

#include <climits>

using namespace std;

using RowCol = std::pair<int, int>;
using Proposals = std::map<RowCol, std::vector<RowCol>>;

enum Dirs {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
};

struct Rule {
    int check, row_dir, col_dir;
};

vector<Rule> rules = {{NE | E | SE, 0, 1},
                      {NW | N | NE, -1, 0},
                      {SW | S | SE, 1, 0},
                      {NW | W | SW, 0, -1}};

auto getNeighbour(set<pair<int, int>>& elves,
                  const RowCol& elf) {
    auto neighbors = 0, bit = 0x1;

    for (auto i = -1; i <= 1; i++)
        for (auto j = -1; j <= 1; j++)
            if (i || j) {
                if (elves.find(std::make_pair(
                        elf.first + i, elf.second + j)) !=
                    elves.end())
                    neighbors |= bit;

                bit <<= 1;
            }

    return neighbors;
}

auto getEmptySpace(set<pair<int, int>>& elves) {}

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);
    int row = 0;

    set<pair<int, int>> elves;

    string line;
    while (getline(file, line)) {
        cout << line << endl;

        for (int col = 0; col < line.size(); col++) {
            if (line[col] == '#') { elves.insert({row, col}); }
        }

        row++;
    }
    cout << endl;

    for (auto round = 0; round < 10; round++) {
        Proposals proposals;

        for (auto& from : elves) {
            auto neighbors = getNeighbour(elves, from);

            if (!neighbors) continue;

            for (auto i = 0;
                 i < sizeof(rules) / sizeof(rules[0]); i++) {
                auto& r = rules[(round + i) % 4];
                if ((neighbors & r.check) == 0) {
                    auto to =
                        std::make_pair(from.first + r.row_dir,
                                       from.second + r.col_dir);
                    proposals[to].push_back(from);
                    break;
                }
            }
        }

        for (auto& p : proposals) {
            if (p.second.size() == 1) {
                elves.insert(p.first);
                elves.erase(elves.find(p.second[0]));
            }
        }
    }

    auto min_row = INT_MAX, max_row = INT_MIN,
         min_col = INT_MAX, max_col = INT_MIN;

    for (auto& elf : elves) {
        min_row = std::min(elf.first, min_row);
        max_row = std::max(elf.first, max_row);
        min_col = std::min(elf.second, min_col);
        max_col = std::max(elf.second, max_col);
    }

    int ans =
        (max_row - min_row + 1) * (max_col - min_col + 1) -
        elves.size();
    cout << elves.size() << " " << ans << endl;
}
