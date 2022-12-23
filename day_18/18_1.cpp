#include <bits/stdc++.h>

#include <cstdlib>

using namespace std;

vector<vector<int>> getNeibours(vector<int> point) {
    return {
        {point[0] + 1, point[1], point[2]}, {point[0] - 1, point[1], point[2]},
        {point[0], point[1] + 1, point[2]}, {point[0], point[1] - 1, point[2]},
        {point[0], point[1], point[2] + 1}, {point[0], point[1], point[2] - 1},
    };
}

int main(int argc, char *argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;
    map<vector<int>, int> points;

    vector<int> point;
    while (getline(file, line)) {
        cout << line << endl;

        point.clear();
        int start = 0;
        int end = line.find(',', start);

        while (end != string::npos) {
            auto pp = line.substr(start, end - start);
            point.push_back(stoi(pp) + 1);
            start = end + 1;
            end = line.find(',', start);
        }

        auto pp = line.substr(start, end - start);
        point.push_back(stoi(pp) + 1);

        points[point] = 0;
    }
    cout << endl;

    for (auto [point, _] : points) {
        for (auto neigbour : getNeibours(point)) {
            if (points.find(neigbour) != points.end()) { points[point]++; }
        }
    }

    int ans = points.size() * 6;

    for (auto [_, face] : points) { ans -= face; }

    cout << ans << endl;
}
