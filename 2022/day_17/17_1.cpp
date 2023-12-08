#include <bits/stdc++.h>

using namespace std;

void moveToHeight(vector<pair<int, int>>& rock, int highest) {
    for (auto& ele : rock) { ele.first += highest + 3; }
}

void moveDown(vector<pair<int, int>>& rock) {
    for (auto& ele : rock) { ele.first--; }
}

bool isIntersecting(vector<vector<char>>& chamber, vector<pair<int, int>>& rock) {
    for (const auto& ele : rock) {
        if (ele.first < chamber.size()) {
            if (chamber[ele.first][ele.second] == '#') { return true; }
        }
    }
    return false;
}

void addRockToChamber(vector<vector<char>>& chamber, vector<pair<int, int>>& rock) {
    for (const auto& ele : rock) {
        while (chamber.size() <= ele.first) {
            vector<char> tmp(7, '.');
            chamber.push_back(tmp);
        }
    }
    for (const auto& ele : rock) { chamber[ele.first][ele.second] = '#'; }
}

void apply_jet(vector<pair<int, int>>& rock, string& jets, int jetIdx,
               vector<vector<char>>& chamber) {
    const auto prev = rock;
    const char j = jets[jetIdx];

    if (j == '>') {
        for (auto& ele : rock) {
            ele.second++;
            if (ele.second > 6) {
                rock = prev;
                return;
            }
        }
    } else if (j == '<') {
        for (auto& ele : rock) {
            ele.second--;
            if (ele.second < 0) {
                rock = prev;
                return;
            }
        }
    }
    if (isIntersecting(chamber, rock)) { rock = prev; }
}

void printChamber(vector<vector<char>>& chamber) {
    cout << chamber.size() << '\n';
    for (int i = chamber.size() - 1; i > 0; i--) {
        cout << "|";
        for (auto& ele : chamber[i]) { cout << ele; }
        cout << "|" << '\n';
    }
    cout << "+";
    for (int i = 0; i < chamber[0].size() + 2; i++) { cout << '-'; }
    cout << "+" << '\n';
}

int main(int argc, char* argv[]) {
    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    string jets;
    fstream file(fileName);
    getline(file, jets);

    vector<vector<pair<int, int>>> rocks = {{{0, 0}, {0, 1}, {0, 2}, {0, 3}},
                                            {{0, 1}, {1, 0}, {1, 1}, {1, 2}, {2, 1}},
                                            {{2, 2}, {1, 2}, {0, 0}, {0, 1}, {0, 2}},
                                            {{0, 0}, {1, 0}, {2, 0}, {3, 0}},
                                            {{0, 0}, {0, 1}, {1, 0}, {1, 1}}};

    for (auto& rock : rocks) {
        for (auto& ele : rock) { ele.second += 2; }
    }

    int highest = 1;
    int rock_count = 0;
    int jet_index = 0;
    int totalRocks = 2022;

    vector<vector<char>> chamber;
    vector<char> temp(7, '#');
    chamber.push_back(temp);

    while (rock_count < totalRocks) {
        auto rock = rocks[rock_count % 5];
        rock_count++;

        moveToHeight(rock, highest);
        auto prev = rock;

        while (!isIntersecting(chamber, rock)) {
            apply_jet(rock, jets, jet_index, chamber);
            jet_index++;
            if (jet_index == jets.size()) jet_index = 0;
            prev = rock;
            moveDown(rock);
        }

        addRockToChamber(chamber, prev);
        // print_chamber(chamber);
        highest = chamber.size();

        cout << "For: " << rock_count << " " << highest - 1 << endl;
    }

    cout << highest - 1 << '\n';
}
