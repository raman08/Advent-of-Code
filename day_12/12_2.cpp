#include <bits/stdc++.h>

#include <climits>
#include <vector>

using namespace std;

struct Node {
    int x, y;
    char val;
    int path;

    Node() { this->path = 0; }
};

bool validDir(pair<int, int> od, pair<int, int> nd, vector<vector<Node*>> grid,
              vector<vector<bool>>& visited) {
    bool vv = (nd.first < 0 || nd.second < 0 || nd.first >= grid.size() ||
               nd.second >= grid[0].size() || visited[nd.first][nd.second]);

    return !(vv || grid[nd.first][nd.second]->val - grid[od.first][od.second]->val > 1);
}

int findPath(pair<int, int> start, pair<int, int> end, vector<vector<Node*>>& grid) {
    vector<vector<bool>> visited(grid.size(), vector<bool>(grid[0].size(), false));
    queue<Node*> qu;

    qu.push(grid[start.first][start.second]);

    visited[start.first][start.second] = true;

    while (!qu.empty()) {
        auto node = qu.front();
        qu.pop();

        cout << "Currently on: " << node->x << " " << node->y << endl;

        if (node->x == end.first && node->y == end.second) {
            cout << "Found End with val " << node->path << endl;
            return node->path;
        }

        vector<pair<int, int>> dirs = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

        for (auto dir : dirs) {
            pair<int, int> nd = {node->x + dir.first, node->y + dir.second};
            if (validDir({node->x, node->y}, nd, grid, visited)) {
                qu.push(grid[nd.first][nd.second]);
                grid[nd.first][nd.second]->path = node->path + 1;
                visited[nd.first][nd.second] = true;
            }
        }
    }

    return INT_MAX;
}

int main(int argc, char* argv[]) {
    ifstream f;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    f.open(fileName);

    string line;

    vector<vector<Node*>> grid;

    pair<int, int> start;
    pair<int, int> end;

    while (getline(f, line)) {
        cout << line << endl;

        vector<Node*> tmp;
        for (auto ch : line) {
            Node* node = new Node();
            node->val = ch;
            tmp.push_back(node);
        }

        grid.push_back(tmp);
    }

    for (int i = 0; i < grid.size(); i++) {
        for (int j = 0; j < grid[i].size(); j++) {
            cout << grid[i][j]->val << " ";
            if (grid[i][j]->val == 'S') start = {i, j};
            if (grid[i][j]->val == 'E') end = {i, j};

            grid[i][j]->x = i;
            grid[i][j]->y = j;
        }

        cout << endl;
    }

    grid[start.first][start.second]->val = 'a';
    grid[end.first][end.second]->val = 'z';

    int ans = INT_MAX;
    for (auto row : grid) {
        for (auto col : row) {
            if (col->val == 'a') {
                for (auto r : grid) {
                    for (auto c : row) { c->path = 0; }
                }
                int a = findPath({col->x, col->y}, end, grid);

                ans = min(a, ans);
            }
        }
    }

    cout << ans << endl;
    // for (auto row : grid) {
    //     for (auto it : row) { cout << it->path << " "; }
    //     cout << endl;
    // }
    //
    // cout << endl;
}
