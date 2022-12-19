#include <bits/stdc++.h>

#include <queue>
#include <string>

using namespace std;

struct Monkey {
    int id;
    queue<int> items;
    string operation;
    string operant;
    int test;
    int mt;
    int mf;

    int total_inspection;

    Monkey() { this->total_inspection = 0; }

    Monkey(int id, queue<int> items, char operation, string opVal, int test, int mt, int mf,
           int total_inspection) {
        this->id = id;
        this->items = items;
        this->operation = operation;
        this->operant = opVal;
        this->test = test;
        this->mt = mt;
        this->mf = mf;
        this->total_inspection = total_inspection;
    }
};

queue<int> parseItems(string& line) {
    queue<int> qu;

    int start = 18;
    int end = line.find(", ", start);

    while (end != string::npos) {
        qu.push(stoi(line.substr(start, end - start)));
        start = end + 2;
        end = line.find(", ", start);
    }

    qu.push(stoi(line.substr(start, line.size() - start)));

    return qu;
}

int do_worry_operation(int item, string operation, string operant) {
    int op;
    if (operant == "old") {
        op = item;

    } else {
        op = stoi(operant);
    }

    if (operation == "+") { return item + op; }

    if (operation == "*") { return item * op; }

    return -1;
}

bool passTest(int item, int test) { return item % test == 0; }

int main(int argc, char* argv[]) {
    ifstream f;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    f.open(fileName);

    vector<Monkey*> monkeys;

    string line;
    while (getline(f, line)) {
        cout << line << endl;

        Monkey* mon = new Monkey();
        int id = stoi(line.substr(7, line.size() - 8));
        mon->id = id;

        getline(f, line);
        auto items = parseItems(line);
        mon->items = items;

        getline(f, line);
        string operation = line.substr(23, 1);
        string opVal = line.substr(25);
        mon->operation = operation;
        mon->operant = opVal;

        getline(f, line);
        int test = stoi(line.substr(21));
        mon->test = test;

        getline(f, line);
        int mt = stoi(line.substr(29));
        mon->mt = mt;

        getline(f, line);
        int mf = stoi(line.substr(30));
        mon->mf = mf;

        // cout << "Id: " << id << endl;

        getline(f, line);

        monkeys.push_back(mon);
    }

    int maxRounds = 20;

    for (int i = 0; i < maxRounds; i++) {
        for (auto monkey : monkeys) {
            while (!monkey->items.empty()) {
                auto item = monkey->items.front();
                monkey->items.pop();

                cout << "Testing item: " << item << " from monkey " << monkey->id << endl;

                monkey->total_inspection++;

                int worry_item = do_worry_operation(item, monkey->operation, monkey->operant);

                worry_item = worry_item / 3;

                bool test_result = passTest(worry_item, monkey->test);

                if (test_result) {
                    cout << "WorryItem " << worry_item << " is pass to monkey " << monkey->mt
                         << endl;
                    monkeys[monkey->mt]->items.push(worry_item);
                } else {
                    cout << "WorryItem " << worry_item << " is pass to monkey " << monkey->mf
                         << endl;
                    monkeys[monkey->mf]->items.push(worry_item);
                }
            }

            cout << endl;
        }

        cout << "After Round " << i + 1 << ": " << endl;

        for (auto monkey : monkeys) {
            int sz = monkey->items.size();

            cout << "Monkey " << monkey->id << ": ";
            while (sz--) {
                auto it = monkey->items.front();
                monkey->items.pop();
                cout << it << " ";
                monkey->items.push(it);
            }

            cout << endl;
        }

        cout << endl;
    }

    cout << "After " << maxRounds << " rounds: " << endl;

    priority_queue<int> inspection;
    for (auto monkey : monkeys) {
        cout << "Monkey " << monkey->id << ": " << monkey->total_inspection << endl;
        inspection.push(monkey->total_inspection);
    }

    cout << endl;

    int fi = inspection.top();
    inspection.pop();
    int sec = inspection.top();

    cout << "Top two inspection are: " << fi << " " << sec << " with score " << fi * sec << endl;
}
