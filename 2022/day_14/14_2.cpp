#include <bits/stdc++.h>

#include <climits>
#include <sstream>

using namespace std;

void processPoints(pair<int, int> p1, pair<int, int> p2, map<pair<int, int>, char>& mapping) {
    if (p1.first == p2.first) {
        for (int y = min(p1.second, p2.second); y <= max(p1.second, p2.second); y++) {
            mapping[{p1.first, y}] = '#';
        }
    } else {
        for (int x = min(p1.first, p2.first); x <= max(p1.first, p2.first); x++) {
            mapping[{x, p1.second}] = '#';
        }
    }
}

void parseLine(string& line, map<pair<int, int>, char>& mapping) {
    cout << "Parsing:::::::" << endl;
    stringstream ss(line);

    char comma;
    string arrow;
    int x1, y1;
    int x2, y2;

    ss >> x1 >> comma >> y1 >> arrow >> x2 >> comma >> y2;

    do {
        cout << "Processing: " << x1 << " " << y1 << " " << x2 << " " << y2 << endl;

        processPoints({x1, y1}, {x2, y2}, mapping);
        x1 = x2;
        y1 = y2;
    } while (ss >> arrow >> x2 >> comma >> y2);

    cout << "END Parsing:::::::" << endl;
}

int dfs(map<pair<int, int>, char>& mapping, int bottom) {
    vector<pair<int, int>> dirs = {{0, 1}, {-1, 1}, {1, 1}};

    pair<int, int> start = {500, 0};

	if(mapping.find(start) != mapping.end()) {
		return false;
	}

    while (true) {
        if (start.second+1 == bottom) {
            mapping[start] = '#';
            return true;
        }
        bool moving = false;

        for (auto dir : dirs) {
            pair<int, int> np = {start.first + dir.first, start.second + dir.second};

            cout << "New Point: " << np.first << " " << np.second << endl;
            if (mapping.find(np) == mapping.end()) {
                moving = true;
                start = np;
                break;
            }
        }

        if (moving) { continue; }

        mapping[start] = 'o';
        cout << endl;
        return true;
    }
}

int main(int argc, char* argv[]) {
    ifstream file;

    map<pair<int, int>, char> mapping;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;

    int lowest = INT_MIN;
    while (getline(file, line)) {
        cout << line << endl;
        parseLine(line, mapping);
    }

    for (auto it : mapping) {
        cout << it.first.first << " " << it.first.second << " " << it.second << " " << endl;

        if (it.first.second > lowest) { lowest = it.first.second; }
    }

    lowest = lowest + 2;
    cout << "Lowest: " << lowest << endl;

    int ans = 0;

    while (dfs(mapping, lowest)) { ans++; }

    cout << ans << endl;
}
