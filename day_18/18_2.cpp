#include <bits/stdc++.h>

#include <climits>
#include <tuple>
#include <vector>

using namespace std;

vector<vector<int>> getNeibours(vector<int> point) {
    return {
        {point[0] + 1, point[1], point[2]},
        {point[0] - 1, point[1], point[2]},
        {point[0], point[1] + 1, point[2]},
        {point[0], point[1] - 1, point[2]},
        {point[0], point[1], point[2] + 1},
        {point[0], point[1], point[2] - 1},
    };
}

vector<int> operator+(const vector<int>& a,
                      const vector<int>& b) {
    return {a[0] + b[0], a[1] + b[1], a[2] + b[2]};
}

bool operator<(const vector<int>& a, const vector<int>& b) {
    return std::tuple<int, int, int>(a[0], a[1], a[2]) <
           std::tuple<int, int, int>(b[0], b[1], b[2]);
}

vector<int> minimum(const vector<int>& a,
                    const vector<int>& b) {
    return {std::min(a[0], b[0]), std::min(a[1], b[1]),
            std::min(a[2], b[2])};
}

vector<int> maximum(const vector<int>& a,
                    const vector<int>& b) {
    return {std::max(a[0], b[0]), std::max(a[1], b[1]),
            std::max(a[2], b[2])};
}

using droplet_t = std::set<vector<int>>;

bool inside_box(const vector<int>& pos,
                const vector<int>& min_pos,
                const vector<int>& max_pos) {
    return pos[0] >= min_pos[0] && pos[0] < max_pos[0] &&
           pos[1] >= min_pos[1] && pos[1] < max_pos[1] &&
           pos[2] >= min_pos[2] && pos[2] < max_pos[2];
}

std::set<vector<int>> flood_fill(
    const std::set<vector<int>>& lava, const vector<int>& pos,
    const vector<int>& min_pos, const vector<int>& max_pos) {
    std::set<vector<int>> water;

    std::queue<vector<int>> q;
    q.push(pos);

    vector<int> outside_min_pos =
        min_pos + vector<int>{-1, -1, -1};
    vector<int> outside_max_pos =
        max_pos + vector<int>{1, 1, 1};

    while (!q.empty()) {
        auto curr = q.front();
        q.pop();

        if (water.count(curr)) { continue; }

        water.insert(curr);

        for (auto& d : std::vector<vector<int>>{{-1, 0, 0},
                                                {1, 0, 0},
                                                {0, -1, 0},
                                                {0, 1, 0},
                                                {0, 0, -1},
                                                {0, 0, 1}}) {
            vector<int> new_pos = curr + d;
            if (!lava.count(new_pos) && !water.count(new_pos) &&
                inside_box(new_pos, outside_min_pos,
                           outside_max_pos)) {
                q.push(new_pos);
            }
        }
    }

    return water;
}

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;

    map<vector<int>, int> points;
    map<vector<int>, int> surroundingPoints;

    set<vector<int>> points_set;

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
        points_set.insert(point);
    }
    cout << endl;

    vector<int> mini(3, INT_MAX);
    vector<int> maxi(3, INT_MIN);

    for (auto [point, _] : points) {
        mini = minimum(mini, point);
        maxi = maximum(maxi, point);
    }

    mini = mini + vector<int>{-1, -1, -1};
    maxi = maxi + vector<int>{1, 1, 1};

    cout << "Mini: " << mini[0] << " " << mini[1] << " "
         << mini[2] << endl;

    cout << "Maxi: " << maxi[0] << " " << maxi[1] << " "
         << maxi[2] << endl;

    std::set<vector<int>> water =
        flood_fill(points_set, mini, mini, maxi);

    int area = 0;
    for (auto& pos : points_set) {
        for (auto& d : std::vector<vector<int>>{{1, 0, 0},
                                                {-1, 0, 0},
                                                {0, 1, 0},
                                                {0, -1, 0},
                                                {0, 0, 1},
                                                {0, 0, -1}}) {
            auto new_pos = pos + d;
            area += !points_set.count(new_pos) &&
                    water.count(new_pos);
        }
    }

    cout << area << endl;
}
