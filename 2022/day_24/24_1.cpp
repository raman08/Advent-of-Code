#include <bits/stdc++.h>

#include <algorithm>

using namespace std;

template <typename T>
struct Coord {
    constexpr Coord(T X = 0, T Y = 0) : first(X), second(Y) {}
    constexpr bool operator<(const Coord& p) const {
        if (first < p.first) return true;
        else if (p.first < first)
            return false;
        else
            return second < p.second;
    }
    constexpr Coord operator+(const Coord& p) const {
        return Coord(first + p.first, second + p.second);
    }
    constexpr bool operator==(const Coord& p) const {
        return first == p.first && second == p.second;
    }
    T first, second;
};

using Point = Coord<int>;
// struct Point {
//     Point(int first = 0, int second = 0)
//         : first(first), second(second) {}
//
//     int first;
//     int second;
//
//     constexpr bool operator<(Point& p) {
//         if (first < p.first) return true;
//         else if (p.first < first)
//             return false;
//         else
//             return second < p.second;
//     }
//
//     bool operator==(Point& p) {
//         return first == p.first && second == p.second;
//     }
//
//     Point operator+(Point& p) {
//         return {first + p.first, second + p.second};
//     }
// };

struct Boundries {
    Boundries(int minx, int miny, int maxx, int maxy)
        : minx(minx), miny(miny), maxx(maxx), maxy(maxy) {}

    int minx;
    int miny;
    int maxx;
    int maxy;

    bool isInside(Point& point) {
        return (minx <= point.first && point.first <= maxx &&
                miny <= point.second && point.second <= maxy);
    }
};

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;

    int maxX = 0;
    int maxY = 0;

    Point start;
    Point end;

    vector<Point> blizzards;
    vector<Point> upBliz;
    vector<Point> downBliz;
    vector<Point> leftBliz;
    vector<Point> rightBliz;

    while (getline(file, line)) {
        cout << line << endl;

        if (!maxX++) {
            start = {0, static_cast<int>(line.find('.'))};
            maxY = line.size() - 1;
        } else {
            for (int i = 1; i < line.size() - 1; i++) {
                switch (line[i]) {
                    case '>':
                        rightBliz.push_back({maxX - 1, i});
                        break;
                    case '<':
                        leftBliz.push_back({maxX - 1, i});
                        break;
                    case '^':
                        upBliz.push_back({maxX - 1, i});
                        break;
                    case 'v':
                        downBliz.push_back({maxX - 1, i});
                        break;
                    case '.': end = {maxX - 1, i};
                }
            }
        }
    }
    cout << endl;

    Boundries field = {1, 1, --maxX - 1, maxY - 1};

    vector<Point> moves = {
        {1, 0}, {0, 1}, {-1, 0}, {0, -1}, {0, 0}};

    int time = 0;
    vector<Point> curr = {start};
    vector<Point> next;

    while (!binary_search(curr.rbegin(), curr.rend(), end)) {
        blizzards.clear();

        for (auto p : upBliz) {
            if (!--p.first) { p.first = maxX - 1; }
            blizzards.push_back(p);
        }

        for (auto p : leftBliz) {
            if (!--p.second) { p.second = maxY - 1; }
            blizzards.push_back(p);
        }

        for (auto p : downBliz) {
            if (++p.first == maxX) { p.first = 1; }
            blizzards.push_back(p);
        }

        for (auto p : rightBliz) {
            if (++p.second == maxY) { p.second = 1; }
            blizzards.push_back(p);
        }

        sort(blizzards.begin(), blizzards.end());

        blizzards.erase(unique(blizzards.begin(),
                               blizzards.end(),
                               blizzards.end()));

        for (auto point : curr) {
            for (auto move : moves) {
                Point np = {point.first + move.first,
                            point.second + move.second};

                if ((field.isInside(np) || np == start ||
                     np == end) &&
                    !binary_search(blizzards.cbegin(),
                                   blizzards.cend(), np)) {
                    next.push_back(np);

                    sort(next.begin(), next.end());
                    next.erase(unique(next.begin(), next.end(),
                                      next.end()));

                    swap(curr, next);
                    next.clear();
                    time++;
                }
            }
        }
    }

    cout << time << endl;
}
