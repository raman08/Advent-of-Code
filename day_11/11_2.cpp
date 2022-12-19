#include <bits/stdc++.h>

#include <queue>
#include <string>

using namespace std;

struct Monkey {
    int id;
    queue<long long> items;
    string operation;
    string operant;
    long long test;
    long long mt;
    long long mf;

    long long total_inspection;

    Monkey() { this->total_inspection = 0; }

    Monkey(int id, queue<long long> items, char operation, string opVal, long long test,
           long long mt, long long mf, long long total_inspection) {
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

queue<long long> parseItems(string& line) {
    queue<long long> qu;

    long long start = 18;
    long long end = line.find(", ", start);

    while (end != string::npos) {
        qu.push(stoi(line.substr(start, end - start)));
        start = end + 2;
        end = line.find(", ", start);
    }

    qu.push(stoi(line.substr(start, line.size() - start)));

    return qu;
}

long long do_worry_operation(long long item, string operation, string operant) {
    long long op;
    if (operant == "old") {
        op = item;

    } else {
        op = stoi(operant);
    }

    if (operation == "+") { return item + op; }

    if (operation == "*") { return item * op; }

    return -1;
}

bool passTest(long long item, long long test) { return item % test == 0; }

int main(int argc, char* argv[]) {
    ifstream f;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    f.open(fileName);

    vector<Monkey*> monkeys;

    int diviser = 1;
    string line;
    while (getline(f, line)) {
        cout << line << endl;

        Monkey* mon = new Monkey();
        long long id = stoi(line.substr(7, line.size() - 8));
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
        long long test = stoi(line.substr(21));
        mon->test = test;
        diviser *= mon->test;

        getline(f, line);
        long long mt = stoi(line.substr(29));
        mon->mt = mt;

        getline(f, line);
        long long mf = stoi(line.substr(30));
        mon->mf = mf;

        // cout << "Id: " << id << endl;

        getline(f, line);

        monkeys.push_back(mon);
    }

    long long maxRounds = 10000;

    for (long long i = 0; i < maxRounds; i++) {
        for (auto monkey : monkeys) {
            while (!monkey->items.empty()) {
                auto item = monkey->items.front();
                monkey->items.pop();

                cout << "Testing item: " << item << " from monkey " << monkey->id << endl;

                monkey->total_inspection++;

                long long worry_item = do_worry_operation(item, monkey->operation, monkey->operant);

                // worry_item = worry_item / 3;
                worry_item = worry_item % diviser;

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
            long long sz = monkey->items.size();

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

    priority_queue<long long> inspection;
    for (auto monkey : monkeys) {
        cout << "Monkey " << monkey->id << ": " << monkey->total_inspection << endl;
        inspection.push(monkey->total_inspection);
    }

    cout << endl;

    long long fi = inspection.top();
    inspection.pop();
    long long sec = inspection.top();

    cout << "Top two inspection are: " << fi << " " << sec << " with score " << fi * sec << endl;
}
