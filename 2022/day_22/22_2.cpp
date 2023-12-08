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
            auto l = line;
            maxWidth = max(maxWidth, (int)l.size());
            grid.push_back(l);

        } else {
            getline(file, sequence);
        }
    }

    for (auto& it : grid) {
        string s(" ", maxWidth - it.size());

        for (auto i : s) { it.push_back(i); }
    }

    for (auto it : grid) {
        cout << it << " " << it.size() << endl;
    }

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

    for (auto step : path) {
        // cout << "For step: " << step << endl;

        if (step[0] >= '0' && step[0] <= '9') {
            int walk = stoi(step);

            int nr = row;
            int nc = col;

            while (walk--) {
                // cout << "Current: " << walk << " " << row <<
                // " "
                //      << col << " " << dr << " " << dc <<
                //      endl;

                int cdr = dr;
                int cdc = dc;

                nr = row + dr;
                nc = col + dc;

                // cout << "Current: " << walk << " " << nr << " "
                //      << nc << " " << dr << " " << dc << endl;
                if (nr < 0 && 50 <= nc && nc < 100 &&
                    dr == -1) {
                    dr = 0;
                    dc = 1;
                    nr = nc + 100;
                    nc = 0;
                } else if (nc < 0 && 150 <= nr && nr < 200 &&
                           dc == -1) {
                    dr = 1;
                    dc = 0;
                    nc = nr - 100;
                    nr = 0;
                } else if (nr < 0 && 100 <= nc && nc < 150 &&
                           dr == -1) {
                    nr = 199;
                    nc = nc - 100;
                } else if (nr >= 200 && 0 <= nc && nc < 50 &&
                           dr == 1) {
                    nr = 0;
                    nc = nc + 100;
                } else if (nc >= 150 && 0 <= nr && nr < 50 &&
                           dc == 1) {
                    dc = -1;
                    nr = 149 - nr;
                    nc = 99;
                } else if (nc == 100 && 100 <= nr && nr < 150 &&
                           dc == 1) {
                    dc = -1;
                    nr = 149 - nr;
                    nc = 149;
                } else if (nr == 50 && 100 <= nc && nc < 150 &&
                           dr == 1) {
                    dr = 0;
                    dc = -1;
                    nr = nc - 50;
                    nc = 99;
                } else if (nc == 100 && 50 <= nr && nr < 100 &&
                           dc == 1) {
                    dr = -1;
                    dc = 0;
                    nc = nr + 50;
                    nr = 49;
                } else if (nr == 150 && 50 <= nc && nc < 100 &&
                           dr == 1) {
                    dr = 0;
                    dc = -1;
                    nr = nc + 100;
                    nc = 49;
                } else if (nc == 50 && 150 <= nr && nr < 200 &&
                           dc == 1) {
                    dr = -1;
                    dc = 0;
                    nc = nr - 100;
                    nr = 149;
                } else if (nr == 99 && 0 <= nc && nc < 50 &&
                           dr == -1) {
                    dr = 0;
                    dc = 1;
                    nr = nc + 50;
                    nc = 50;
                } else if (nc == 49 && 50 <= nr && nr < 100 &&
                           dc == -1) {
                    dr = 1;
                    dc = 0;
                    nc = nr - 50;
                    nr = 100;
                } else if (nc == 49 && 0 <= nr && nr < 50 &&
                           dc == -1) {
                    dc = 1;
                    nr = 149 - nr;
                    nc = 0;
                } else if (nc < 0 && 100 <= nr && nr < 150 &&
                           dc == -1) {
                    dc = 1;
                    nr = 149 - nr;
                    nc = 50;
                }

                if (grid[nr][nc] == '#') {
                    // cout << "Found wall at " << nr << " " << nc
                    //      << endl;
                    dr = cdr;
                    dc = cdc;
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
