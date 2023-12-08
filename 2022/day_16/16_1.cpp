#include <bits/stdc++.h>

#include <climits>

using namespace std;

struct Valve {
    string val;
    int flowRate;
    unordered_set<string> tunnels;
};

unordered_map<string, unordered_map<string, int>> floyd_warshell(
    unordered_map<string, Valve *> valves) {
    unordered_map<string, unordered_map<string, int>> graph;

    for (auto [start, valve] : valves) {
        for (auto [end, _] : valves) {
            if (start == end) {
                graph[start][end] = 0;
            } else if (valve->tunnels.find(end) != valve->tunnels.end()) {
                graph[start][end] = 1;
            } else {
                graph[start][end] = (int)valves.size() + 1;
            }
        }
    }

    for (auto [k, _] : valves) {
        for (auto [i, _] : valves) {
            for (auto [j, _] : valves) {
                if (graph[i][j] > graph[i][k] + graph[k][j]) {
                    graph[i][j] = graph[i][k] + graph[k][j];
                }
            }
        }
    }

    return graph;
}

int dfs(string start, int time, unordered_map<string, unordered_map<string, int>> &graph,
        unordered_map<string, Valve *> &valves, int openValves, unordered_map<string, int> &index,
        map<pair<pair<string, int>, int>, int> &dp) {
    int maxVal = 0;

    if (time == 0) { return 0; }

    if (dp.find({{start, time}, openValves}) != dp.end()) {
        return dp[{{start, time}, openValves}];
    }

    for (auto neighbour : graph[start]) {
        // if (openValves.find(neighbour.first) != openValves.end()) { continue; }
        int bit = 1 << index[neighbour.first];

        if (openValves & bit) {
            cout << "Skipping bits " << neighbour.first << " " << index[neighbour.first] << " "
                 << openValves << " " << bit << " " << (openValves & bit) << endl;
            continue;
        } else {
            cout << "Continued " << neighbour.first << " " << index[neighbour.first] << " "
                 << openValves << " " << bit << " " << (openValves & bit) << endl;
        }

        if (valves[neighbour.first]->flowRate == 0) { continue; }

        if (start == neighbour.first) { continue; }

        int remTime = time - graph[start][neighbour.first] - 1;

        if (remTime <= 0) { continue; }

        // openValves.insert(neighbour.first);
        int tmp = openValves | bit;

        maxVal = max(maxVal, dfs(neighbour.first, remTime, graph, valves, tmp, index, dp) +
                                 valves[neighbour.first]->flowRate * remTime);
    }

    return dp[{{start, time}, openValves}] = maxVal;
}

int main(int argc, char *argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;

    unordered_map<string, Valve *> valves;

    while (getline(file, line)) {
        cout << line << endl;

        auto valve = new Valve();

        string val = line.substr(6, 2);

        valve->val = val;

        int flowRate_start = line.find('=') + 1;
        int flowRate_end = line.find(';', flowRate_start);

        int flowRate = stoi(line.substr(flowRate_start, flowRate_end - flowRate_start));

        valve->flowRate = flowRate;

        int tunnel_start = flowRate_end + 24;

        if (line[tunnel_start] == ' ') { tunnel_start++; }

        int comma = line.find(", ", tunnel_start);

        while (comma != string::npos) {
            string t = line.substr(tunnel_start, comma - tunnel_start);

            valve->tunnels.insert(t);
            tunnel_start = comma + 2;
            comma = line.find(", ", tunnel_start);
        }

        valve->tunnels.insert(line.substr(tunnel_start, line.size() - tunnel_start));
        cout << endl;

        valves[val] = valve;
    }

    auto graph = floyd_warshell(valves);

    int idx = 0;
    unordered_map<string, int> index;

    for (auto valve : valves) {
        if (valve.second->flowRate) index[valve.first] = idx++;
    }

    int totalTime = 30;
    string start = "AA";
    int openValves = 63;
    map<pair<pair<string, int>, int>, int> dp;

    int ans = dfs(start, totalTime, graph, valves, openValves, index, dp);

    cout << ans << endl;
}
