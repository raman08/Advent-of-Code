#include <bits/stdc++.h>

#include <algorithm>
#include <string>
#include <utility>

using namespace std;

struct Point {
    int val;
    int x;
    int y;
    int view;
};

bool inBounds(int x, int y, int maxX, int maxY) {
    return (x < maxX && x >= 0 && y < maxY && y >= 0);
}

int main(int argc, char *argv[]) {

    ifstream f;

	string fileName = "./input-ex.txt";
	if(argc > 1)
		fileName = "./input.txt";

    f.open(fileName);
    // f.open("./input.txt");

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

    vector<vector<int>> dirs = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    vector<vector<int>> scores(grid.size(), vector<int>(grid[0].size(), 1));

    for (int i = 0; i < grid.size(); i++) {
        for (int j = 0; j < grid[i].size(); j++) {
            for (auto dir : dirs) {
                int score = 0;

                vector<int> current = {i + dir[0], j + dir[1]};

                while (inBounds(current[0], current[1], grid.size(), grid[0].size()) &&
                       grid[i][j] > grid[current[0]][current[1]]) {
                    score++;
                    current[0] += dir[0];
                    current[1] += dir[1];
                }

                if (inBounds(current[0], current[1], grid.size(), grid[0].size())) score++;

                scores[i][j] *= score;
            }
        }
    }

    int maxi = INT_MIN;

    for (auto row : scores) {
        for (auto col : row) { maxi = max(maxi, col); }
    }

    cout << maxi;

    // int ans = 0;
    // for (int i = 0; i < grid.size(); i++) {
    //     for (int j = 0; j < grid[0].size(); j++) {
    //         if (top[i][j].second || left[i][j].second || bottom[i][j].second ||
    //             right[i][j].second) {
    //             cout << "True for " << i << " " << j << " " << grid[i][j] << endl;
    //             ans++;
    //         }
    //     }
    // }
    //
    // cout << ans << endl;
}
