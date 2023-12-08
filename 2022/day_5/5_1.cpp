#include <bits/stdc++.h>

#include <cstdlib>
#include <string>
#include <vector>
using namespace std;

int main() {
    ifstream f;
    // f.open("./input-ex.txt");
    f.open("./input.txt");

    string line;

    vector<string> raw;
    vector<vector<int>> steps;

    bool gotRaw = false;

    while (getline(f, line)) {
        cout << line << endl;

        if (line == "") gotRaw = true;

        if (!gotRaw) raw.push_back(line);

        else {
            if (line == "") continue;
            string count, from, to;

            long unsigned int cnt = 5;

            while (line[cnt] != ' ') {
                count += line[cnt];
                cnt++;
            }

            cnt += 6;

            while (line[cnt] != ' ') {
                from += line[cnt];
                cnt++;
            }

            cnt += 4;
            while (cnt < line.size()) {
                to += line[cnt];
                cnt++;
            }

            // cout << count << " " << from << " " <<
            // to << endl;
            steps.push_back(
                {stoi(count), stoi(from), stoi(to)});
        }
    }

    int crates =
        raw.back()[raw.back().size() - 2] - '0';

    raw.pop_back();
    vector<stack<char>> stacks(crates);

    for (int i = raw.size() - 1; i >= 0; i--) {
        auto line = raw[i];
        int start = 1;
        int pos = 0;

        while (start < line.size()) {
            if (line[start] != ' ') {
                cout << "Pusing " << line[start]
                     << " at stack " << pos << endl;
                stacks[pos].push(line[start]);
            }

            start += 4;
            pos++;
        }
    }

    for (auto step : steps) {

        int count = step[0];
        int from = step[1] - 1;
        int to = step[2] - 1;

        while (count--) {
            if (stacks[from].empty()) break;

            stacks[to].push(stacks[from].top());
            stacks[from].pop();
        }
    }

    string ans = "";
    for (auto it : stacks) {
        if (it.empty()) break;
        ans.push_back(it.top());
    }
    cout << ans << endl;
}
