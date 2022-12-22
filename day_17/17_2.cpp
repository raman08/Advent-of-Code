#include <bits/stdc++.h>

#include <cstddef>

using namespace std;

struct Memory {
    Memory(vector<vector<char>> topN, int jet_index, int rock_type, size_t highest)
        : topN(move(topN)), jet_index(jet_index), rock_type(rock_type), highest(highest) {}
    vector<vector<char>> topN;
    int jet_index;
    int rock_type;
    size_t highest;
    bool operator==(const Memory& m) const {
        return m.jet_index == jet_index && m.rock_type == rock_type && m.topN == topN;
    }
};

struct MemoryHash {
    size_t operator()(const Memory& m) const {
        size_t ans = 1;
        for (size_t row = 0; row < m.topN.size(); row++) {
            for (size_t i = 0; i < m.topN[row].size(); i++) {
                ans = ans + (i + 1) * m.topN[row][i];
            }
        }
        ans = ans + m.jet_index + m.rock_type;
        return ans;
    }
};

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

tuple<bool, size_t, size_t> check_if_seen_else_add(
    unordered_map<Memory, size_t, MemoryHash>& history, const Memory& memory,
    const size_t& iteration) {
    if (const auto it = history.find(memory); it != history.end()) {
        return {true, it->second, it->first.highest};
    }
    history[memory] = iteration;
    return {false, 0, 0};
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
    // int totalRocks = 2022;
    size_t totalRocks = 1000000000000;

    vector<vector<char>> chamber;
    vector<vector<char>> top_n(5);
    vector<char> temp(7, '#');
    chamber.push_back(temp);

    unordered_map<Memory, size_t, MemoryHash> history;

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

        for (int i = max(chamber.size() - 5, size_t(0)), j = 0; i < chamber.size(); i++, j++) {
            top_n[j] = chamber[i];
        }

        auto memory = Memory(top_n, jet_index, rock_count % 5, highest);
        auto [seen, prev_level, prev_highest] = check_if_seen_else_add(history, memory, rock_count);
        if (seen) {
            cout << "Seen first at level: " << prev_level << '\n';
            cout << "Seen again at level " << rock_count << '\n';
            size_t delta_iter = rock_count - prev_level;
            size_t delta_height = highest - prev_highest;
            size_t n_rep = (size_t(totalRocks) - size_t(prev_level)) / delta_iter;
            size_t n_rem = (size_t(totalRocks) - size_t(prev_level)) % delta_iter;
            // Use STl
            size_t extra_height = 0;
            for (auto& [m, iter] : history) {
                if (size_t(iter) - prev_level == n_rem) {
                    extra_height = m.highest - prev_highest;
                    break;
                }
            }
            size_t total_height = prev_highest + delta_height * (n_rep) + extra_height;
            cout << total_height - 1 << '\n';
            return 0;
        }

        cout << "For: " << rock_count << " " << highest - 1 << endl;
    }

    cout << highest - 1 << '\n';
}
