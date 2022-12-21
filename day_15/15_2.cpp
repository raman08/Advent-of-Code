#include <bits/stdc++.h>

#include <climits>
#include <sstream>

using namespace std;

template <typename T>
std::set<T> getUnion(const std::set<T>& a, const std::set<T>& b) {
    std::set<T> result = a;
    result.insert(b.begin(), b.end());
    return result;
}

void parseLine(string& line, map<pair<long long, long long>, pair<long long, long long>>& mapping) {
    long long start = 12;
    long long end = line.find(',', start);

    long long sx = stoll(line.substr(start, end - start));

    start = end + 4;
    end = line.find(':', start);

    long long sy = stoll(line.substr(start, end - start));

    start = end + 25;
    end = line.find(',', start);

    long long bx = stoll(line.substr(start, end - start));

    start = end + 4;
    end = line.size();

    long long by = stoll(line.substr(start, end - start));

    // cout << sx << " " << sy << " " << bx << " " << by << endl;
    mapping[{sx, sy}] = {bx, by};
}

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;

    map<pair<long long, long long>, pair<long long, long long>> mapping;

    while (getline(file, line)) {
        // cout << line << endl;
        parseLine(line, mapping);
    }

    // long long maxC = 20;
    long long maxC = 4000000;

    set<pair<long long, long long>> noBecon;

    for (int targetRow = 0; targetRow <= maxC; targetRow++) {
        cout << "For targetRow: " << targetRow << endl;
        for (auto [s, b] : mapping) {
            long long distance = abs(s.first - b.first) + abs(s.second - b.second);

            long long dy = abs(targetRow - s.second);

            if (dy > distance) { continue; }

            long long left = s.first - distance + dy;
            long long right = left + (2 * (distance - dy) + 1);

            if (left < 0) { left = 0; }

            if (right > maxC) { right = maxC; }

            set<pair<long long, long long>> tmp;

            // cout << "Range of sensor is " << left << " " << right << endl;
            for (int x = left; x < right; x++) {
                // cout << "Inserting " << x << " " << targetRow << " in the set!" << endl;
                tmp.insert({x, targetRow});
            }

            // if (b.second == targetRow && tmp.find({b.first, b.second}) != tmp.end()) {
            //     tmp.erase(tmp.find({b.first, b.second}));
            // }

            cout << tmp.size() << " ";
            noBecon = getUnion(noBecon, tmp);
        }

        cout << endl;
    }
    cout << endl;

    cout << noBecon.size() << endl;

    for (long long i = 0; i < maxC; i++) {
        for (long long j = 0; j < maxC; j++) {
            if (noBecon.find({i, j}) == noBecon.end()) {
                cout << i << " " << j << " " << i * 4000000 + j << endl;
            }
        }
    }
}
