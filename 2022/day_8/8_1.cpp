#include <bits/stdc++.h>

#include <string>
#include <utility>

using namespace std;

int main() {
    ifstream f;
    // f.open("./input-ex.txt");
    f.open("./input.txt");

    vector<vector<int>> grid;

    string line;

    while (getline(f, line)) {
        cout << line << endl;

        vector<int> tmp;
        for (auto ch : line) { tmp.push_back(ch - '0'); }

        grid.push_back(tmp);
    }

    // for(auto row:grid) {
    // 	for(auto col:row) {
    // 		cout << col << " ";
    // 	}
    // 	cout << endl;
    // }
    // cout << endl;

    cout << "******* TOP **********" << endl;
    vector<vector<pair<int, bool>>> top(grid.size(), vector<pair<int, bool>>(grid[0].size()));
    for (int i = 0; i < grid.size(); i++) {
        for (int j = 0; j < grid[i].size(); j++) {
            if (i == 0) {
                top[i][j].first = grid[i][j];
                top[i][j].second = true;
            } else {
                top[i][j].first = max(grid[i][j], top[i - 1][j].first);
                top[i][j].second = grid[i][j] > top[i - 1][j].first;
            }

            cout << "For ( " << i << ", " << j << " ): " << top[i][j].first << " "
                 << top[i][j].second << endl;
        }
    }
    cout << "******* TOP END **********" << endl;

    cout << "******* BOTTOM **********" << endl;
    vector<vector<pair<int, bool>>> bottom(grid.size(), vector<pair<int, bool>>(grid[0].size()));
    for (int i = grid.size() - 1; i >= 0; i--) {
        for (int j = grid[0].size() - 1; j >= 0; j--) {
            if (i == grid.size() - 1) {
                bottom[i][j].first = grid[i][j];
                bottom[i][j].second = true;
            } else {
                bottom[i][j].first = max(grid[i][j], bottom[i + 1][j].first);
                bottom[i][j].second = grid[i][j] > bottom[i + 1][j].first;
            }

            cout << "For ( " << i << ", " << j << " ): " << bottom[i][j].first << " "
                 << bottom[i][j].second << endl;
        }
    }
    cout << "******* BOTTOM END **********" << endl;

    cout << "******* LEFT **********" << endl;

    vector<vector<pair<int, bool>>> left(grid.size(), vector<pair<int, bool>>(grid[0].size()));
    for (int i = 0; i < grid.size(); i++) {
        for (int j = 0; j < grid[i].size(); j++) {
            if (j == 0) {
                left[i][j].first = grid[i][j];
                left[i][j].second = true;
            } else {
                left[i][j].first = max(grid[i][j], left[i][j - 1].first);
                left[i][j].second = grid[i][j] > left[i][j - 1].first;
            }

            cout << "For ( " << i << ", " << j << " ): " << left[i][j].first << " "
                 << left[i][j].second << endl;
        }
    }
    cout << "******* LEFT END **********" << endl;

    cout << "******* RIGHT **********" << endl;
    vector<vector<pair<int, bool>>> right(grid.size(), vector<pair<int, bool>>(grid[0].size()));
    for (int i = grid.size() - 1; i >= 0; i--) {
        for (int j = grid[0].size() - 1; j >= 0; j--) {
            if (j == grid[0].size() - 1) {
                right[i][j].first = grid[i][j];
                right[i][j].second = true;
            } else {
                right[i][j].first = max(grid[i][j], right[i][j + 1].first);
                right[i][j].second = grid[i][j] > right[i][j + 1].first;
            }

            cout << "For ( " << i << ", " << j << " ): " << right[i][j].first << " "
                 << right[i][j].second << endl;
        }
    }
    cout << "******* RIGHT END **********" << endl;

    int ans = 0;
    for (int i = 0; i < grid.size(); i++) {
        for (int j = 0; j < grid[0].size(); j++) {
            if (top[i][j].second || left[i][j].second || bottom[i][j].second ||
                right[i][j].second) {
                cout << "True for " << i << " " << j << " " << grid[i][j] << endl;
                ans++;
            }
        }
    }

    cout << ans << endl;
}
