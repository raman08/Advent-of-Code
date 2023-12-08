#include <bits/stdc++.h>

#include <string>
#include <unordered_set>

using namespace std;

struct Point {
    int x;
    int y;

    Point() {
        this->x = 0;
        this->y = 0;
    }

    Point(int x, int y) {
        this->x = x;
        this->y = y;
    }
};

void updatePoints(int f, int s, vector<pair<int, int>>& Points) {
    int xg = Points[f].first - Points[s].first;
    int yg = Points[f].second - Points[s].second;

    if (abs(xg) == 2 && abs(yg) == 0) {
        Points[s].first += xg / 2;
    } else if (abs(xg) == 2 && abs(yg) == 1) {
        Points[s].first += xg / 2;
        Points[s].second += yg;
    } else if (abs(xg) == 0 && abs(yg) == 2) {
        Points[s].second += yg / 2;
    } else if (abs(xg) == 1 && abs(yg) == 2) {
        Points[s].first += xg;
        Points[s].second += yg / 2;
    } else if (abs(xg) == 2 && abs(yg) == 2) {
        Points[s].first += xg / 2;
        Points[s].second += yg / 2;
    }
}

int main(int argc, char* argv[]) {
    ifstream f;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    f.open(fileName);

    string line;

    vector<pair<int, int>> points(10, {0, 0});

    unordered_set<int> st;

    while (getline(f, line)) {
        cout << line << endl;

        int steps = stoi(line.substr(2));
        char dir = line[0];

        for (int i = 0; i < steps; i++) {
            switch (dir) {
                case 'R': {
                    points[0].first++;
                    break;
                };
                case 'U': {
                    points[0].second++;
                    break;
                };
                case 'L': {
                    points[0].first--;
                    break;
                };
                case 'D': {
                    points[0].second--;
                    break;
                };
            }

            for (int i = 0; i < points.size() - 1; i++) {
                cout << "For Point: " << i << " " << i + 1 << endl;

                cout << "H: " << points[i].first << " " << points[i].second << endl;
                cout << "T: " << points[i + 1].first << " " << points[i + 1].second << endl;

                updatePoints(i, i + 1, points);

                cout << "T: " << points[i + 1].first << " " << points[i + 1].second << endl;
                cout << endl;
            }

            st.insert(points.back().first * 100000 + points.back().second);
        }

        for (auto point : points) { cout << point.first << " " << point.second << endl; }
        cout << endl;
    }

    cout << endl;
    cout << st.size();
}
