#include <bits/stdc++.h>

#include <string>

using namespace std;

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;
    int maxWidth = 0;

    vector<string> grid;
    string sequence;

    while (getline(file, line)) {
        // cout << line << endl;

        if (!line.empty()) {
            // auto l = " " + line + " ";
            auto l = line;
            maxWidth = max(maxWidth, (int)l.size());
            grid.push_back(l);

        } else {
            getline(file, sequence);
        }
    }

    for (auto& it : grid) {
        string s(" ", maxWidth - it.size());

        // cout << "Adding: " << maxWidth - it.size() << " to "
        //      << it << endl;

        for (auto i : s) { it.push_back(i); }
        // it += s;
    }

    for (auto it : grid) {
        cout << it << " " << it.size() << endl;
    }

    // grid.insert(grid.begin(), string(grid[0].size(), ' '));
    // grid.push_back(string(grid.back().size(), ' '));

    // for (auto row : grid) { cout << row << endl; }
    // cout << endl;

    cout << sequence << endl;
    cout << endl;

    vector<string> path;

    string tmp = "";
    for (auto it : sequence) {
        if (it >= '0' && it <= '9') {
            tmp.push_back(it);
        } else {
            path.push_back(tmp);
            tmp.clear();

            string tt = "";
            tt.push_back(it);
            path.push_back(tt);
        }
    }
    path.push_back(tmp);

    // for (auto it : path) { cout << it << " "; }
    // cout << endl;

    int row = 0;
    int col = 0;

    for (int i = 0; i < grid.size(); i++) {
        if (grid[row][i] != ' ') {
            col = i;
            break;
        }
    }

    cout << row << " " << col << endl;

    int dr = 0;
    int dc = 1;

    vector<string> pathTrace = grid;

    auto mod = [](int a, int b) { return ((a % b) + b) % b; };

    for (auto step : path) {
        cout << "For step: " << step << endl;

        if (step[0] >= '0' && step[0] <= '9') {
            int walk = stoi(step);

            int nr = row;
            int nc = col;

            while (walk--) {
                cout << "Current: " << walk << " " << row << " "
                     << col << " " << dr << " " << dc << endl;

                while (true) {
                    nr += dr;
                    nr = mod(nr, grid.size());

                    nc += dc;
                    nc = mod(nc, grid[nr].size());

                    if (grid[nr][nc] == '.' ||
                        grid[nr][nc] == '#') {
                        break;
                    }

                    cout << "No tile at " << nr << " " << nc
                         << endl;
                }

                if (grid[nr][nc] == '#') {
                    cout << "Found wall at " << nr << " " << nc
                         << endl;
                    break;
                }

                row = nr;
                col = nc;

                pathTrace[row][col] = '*';
            }
        } else if (step == "R") {
            int tmp = dr;
            dr = dc;
            dc = -1 * tmp;

            cout << "Change: " << step << " " << dr << " " << dc
                 << endl;
        } else {
            int tmp = dr;
            dr = -1 * dc;
            dc = tmp;

            cout << "Change: " << step << " " << dr << " " << dc
                 << endl;
        }
    }

    cout << endl;

    for (auto row : pathTrace) { cout << row << endl; }
    cout << endl;

    cout << row << " " << col << " " << dr << " " << dc << endl;

    int dir;

    if (dr == 0 && dc == 1) {
        dir = 0;
    } else if (dr == 0 && dc == -1) {
        dir = 3;

    } else if (dr == 1 && dc == 0) {
        dir = 1;
    } else {
        dir = 2;
    }

    int ans = (1000 * (row + 1)) + (4 * (col + 1)) + dir;

    cout << ans << endl;
}
