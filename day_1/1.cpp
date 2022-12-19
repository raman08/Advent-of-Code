#include <bits/stdc++.h>

#include <fstream>
#include <functional>
#include <queue>
#include <string>

using namespace std;

int main() {
    ifstream f;
    f.open("input.txt");

    string line;

    int maxi = 0;
    int sum = 0;


	priority_queue<int> pq;
    while (getline(f, line)) {
        if (line.size() == 0) {
            // maxi = max(maxi, sum);
            pq.push(sum);
			// cout << "SUM: " << sum << endl;
            sum = 0;

        } else {
            // cout << line << endl;
            sum += stoi(line);
        }
    }

	pq.push(sum);

	// cout << pq.size() << endl;

    int n = 3;
    int ans = 0;
    while (n--) {
        // cout << pq.top() << endl;
        ans += pq.top();
        pq.pop();
    }

    cout << ans;
}
