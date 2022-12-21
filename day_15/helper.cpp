#include <algorithm>
#include <array>
#include <boost/tokenizer.hpp>
#include <cassert>
#include <cstddef>
#include <deque>
#include <fstream>
#include <functional>
#include <iostream>
#include <map>
#include <memory>
#include <numeric>
#include <optional>
#include <regex>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string_view>
#include <tuple>

using namespace std;

long long calcDistance(pair<long long, long long> &p1,
                       pair<long long, long long> &p2) {
  return (abs(p1.first - p2.first) + abs(p1.second - p2.second));
}

class Sensor {
public:
  Sensor(pair<long long, long long> &sc, pair<long long, long long> &bc)

  {
    this->sensor_cord = sc;
    this->becon_cord = bc;
    this->dist = calcDistance(sc, bc);
  }

  bool isBeconAt(pair<long long, long long> &cord) {
    return (cord == becon_cord);
  }

  bool CoverageTest(pair<long long, long long> &tCord) {
    return (calcDistance(sensor_cord, tCord) <= this->dist);
  }

  pair<long long, long long>
  StepAroundPerimeter(optional<pair<long long, long long>> testCord) {
    auto &[sx, sy] = this->sensor_cord;

    if (!testCord) {
      return {sx, (sy - this->dist - 1)};
    }

    auto &[tx, ty] = testCord.value();

    if ((ty < sy) && (tx >= sx)) {
      return {tx + 1, ty + 1}; // upper right, downwards
    } else if ((ty >= sy) && (tx > sx)) {
      return {tx - 1, ty + 1}; // lower right, downwards
    } else if ((ty > sy) && (tx <= sx)) {
      return {tx - 1, ty - 1}; // lower left, upwards
    } else if ((ty <= sy) && (tx < sx)) {
      if ((tx + 1) == sx) {
        return {}; // circumference completed
      } else {
        return {tx + 1, ty - 1}; // upper left, upwards
      }
    } else {
      assert(false);
      return {};
    }
  }

private:
  pair<long long, long long> sensor_cord;
  pair<long long, long long> becon_cord;
  uint64_t dist;
};

class Coverage {
public:
  void addSensor(string &line) {
    auto rg = regex{"Sensor at x=(-?[[:digit:]]+), "
                    "y=(-?[[:digit:]]+): closest beacon is at "
                    "x=(-?[[:digit:]]+), y=(-?[[:digit:]]+)$"};

    auto match = smatch{};
    if (regex_search(line, match, rg) && (match.size() > 1)) {
      sensors.emplace_back(pair<long long, long long>{stoi(string{match[1]}),
                                                      stoi(string{match[2]})},
                           pair<long long, long long>{stoi(string{match[3]}),
                                                      stoi(string{match[4]})});
    } else {
      assert(false);
    }
  }

  bool NoBeaconPossible(pair<long long, long long> &testCord, bool space) {

    for (auto &sensor : sensors) {
      if (sensor.isBeconAt(testCord)) {
        return space; // riddle A: false (do not count but stop), riddle
                      // B: true (treat as covered -> no space)
      } else if (sensor.CoverageTest(testCord)) {
        return true; // covered, no space
      } else {
        // Not covered by this sensor, undecided
      } // else
    }   // for

    return false; // not covered by any sensor
  }

  pair<long long, long long> searchSpace(uint64_t width) {

    for (auto &sensor : sensors) {
      pair<long long, long long> tmp = {};
      auto pos = sensor.StepAroundPerimeter(tmp);

      while (pos.has_value()) {
        auto &[X, Y] = pos.value();
        if ((X >= 0) && (X <= width) && (Y >= 0) && (Y <= width)) {
          if (!NoBeaconPossible({X, Y}, true)) {
            return {X, Y};
          }
        }

        pos = sensor.StepAroundPerimeter(pos);
      }
    }

    return {};
  }

private:
  vector<Sensor> sensors;
};

void Play(string fileName, int64_t testRow, uint64_t maxC) {
  // Read sensor coverage information
  cout << endl
       << "Reading sensor coverage from file \"" << fileName << "\"" << endl;

  ifstream file{string{fileName}};
  string line;

  auto coverage = Coverage{};

  while (getline(file, line)) {
    coverage.addSensor(line);
  }

  long long noBecons = 0;

  for (auto x = (testRow * (-5)); x < (testRow * 5); ++x) {
    pair<long long, long long> cord = {x, testRow};

    if (coverage.NoBeaconPossible(cord, false)) {
      ++noBecons;
    }
  }

  cout << "A: number of positions where the beacon cannot be = " << noBecons
       << endl;

  auto [x, y] = coverage.searchSpace(maxC);

  cout << "B: found beacon spot at " << x << ":" << y
       << " with tuning frequency " << ((4000000 * x) + y) << endl;
}

int main(int argc, char **argv) {
  Play("input-ex.txt", 10, 20);
  Play("input.txt", 2000000, 4000000);
}
