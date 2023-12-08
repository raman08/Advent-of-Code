#include <bits/stdc++.h>

using namespace std;

struct Resources {
    Resources() : ore(0), clay(0), obsidian(0), geode(0) {}

    int ore;
    int clay;
    int obsidian;
    int geode;
};

struct Blueprint {
    Blueprint()
        : id(),
          oreBotCost(),
          clayBotCost(),
          obsidianBotCost(),
          geodeBotCost(),
          maxOreBots(),
          maxClayBots(),
          maxObsidianBots() {}

    int id;
    Resources oreBotCost;
    Resources clayBotCost;
    Resources obsidianBotCost;
    Resources geodeBotCost;
    int maxOreBots;
    int maxClayBots;
    int maxObsidianBots;
};

struct State {
    State()
        : resources(),
          oreBots(1),
          clayBots(0),
          obsidianBots(0),
          geodeBots(0),
          time(0) {}

    Resources resources;
    int oreBots;
    int clayBots;
    int obsidianBots;
    int geodeBots;
    int time;
};

enum Moves {
    buildOreBot,
    buildClayBot,
    buildObsidianBot,
    buildGeodeBot,
    buildNone
};

bool canBuild(const State& state, const Resources& cost) {
    return state.resources.ore >= cost.ore &&
           state.resources.clay >= cost.clay &&
           state.resources.obsidian >= cost.obsidian;
}

State update(State& state, Moves move,
             const Resources& cost = Resources()) {
    State newState = state;

    newState.resources.ore += (state.oreBots - cost.ore);
    newState.resources.clay += (state.clayBots - cost.clay);
    newState.resources.obsidian +=
        (state.obsidianBots - cost.obsidian);
    newState.resources.geode += (state.geodeBots - cost.geode);

    switch (move) {
        case buildOreBot: {
            newState.oreBots++;
            break;
        }
        case buildClayBot: {
            newState.clayBots++;
            break;
        }
        case buildObsidianBot: {
            newState.obsidianBots++;
            break;
        }
        case buildGeodeBot: {
            newState.geodeBots++;
            break;
        }
        case buildNone: {
            break;
        }
    }

    newState.time++;

    return newState;
}

void dfs__(const Blueprint& blueprint, State& state, Moves move,
           int maxTime, int& maxGeodes) {
    if ((move == buildOreBot &&
         state.oreBots >= blueprint.maxOreBots) ||
        (move == buildClayBot &&
         state.clayBots >= blueprint.maxClayBots) ||
        (move == buildObsidianBot &&
         state.obsidianBots >= blueprint.maxObsidianBots)) {
        // cout << "Return due to maxBots created" << endl;
        return;
    }

    if (move == buildOreBot &&
        state.oreBots >= blueprint.maxOreBots) {
        return;
    } else if (move == buildClayBot &&
               state.clayBots >= blueprint.maxClayBots) {
        return;
    } else if (move == buildObsidianBot &&
               state.obsidianBots >=
                   blueprint.maxObsidianBots) {
        return;
    }

    // int remTime = maxTime - state.time;
    //
    // auto sqSum = [](int a, int b) {
    //     return (b - a) * (b + a) / 2;
    // };
    //
    // if (state.resources.geode +
    //         sqSum(state.geodeBots, state.geodeBots +
    // remTime)
    //         <=
    //     maxGeodes) {
    //     return;
    // }

    while (state.time < maxTime) {
        auto buildNextBot = [&](const Resources& cost) {
            if (canBuild(state, cost)) {
                for (auto nextMove :
                     {buildOreBot, buildClayBot,
                      buildObsidianBot, buildGeodeBot}) {
                    auto newState = update(state, move, cost);
                    dfs__(blueprint, newState, nextMove,
                          maxTime, maxGeodes);
                }
                return true;
            }
            return false;
        };

        // if ((move == buildOreBot &&
        //      buildNextBot(blueprint.oreBotCost)) ||
        //     (move == buildClayBot &&
        //      buildNextBot(blueprint.clayBotCost)) ||
        //     (move == buildObsidianBot &&
        //      buildNextBot(blueprint.obsidianBotCost)) ||
        //     (move == buildGeodeBot &&
        //      buildNextBot(blueprint.geodeBotCost))) {
        //     return;
        // }
        if (move == buildOreBot &&
            buildNextBot(blueprint.oreBotCost)) {
            return;
        } else if (move == buildClayBot &&
                   buildNextBot(blueprint.clayBotCost)) {
            return;
        } else if (move == buildObsidianBot &&
                   buildNextBot(blueprint.obsidianBotCost)) {
            return;
        } else if (move == buildGeodeBot &&
                   buildNextBot(blueprint.geodeBotCost)) {
            return;
        }

        state = update(state, buildNone);

        // cout << "At time: " << state.time << " "
        //      << state.oreBots << " oreBots " <<
        //      state.clayBots
        //      << " clayBots " << state.obsidianBots
        //      << " obsidianBots " << state.geodeBots
        //      << " geodeBots " << state.resources.ore << "
        // ore
        //      "
        //      << state.resources.clay << " clay "
        //      << state.resources.obsidian << " obsidian "
        //      << state.resources.geode << " geode." <<
        // endl;
    }

    maxGeodes = max(state.resources.geode, maxGeodes);

    // cout << "In dfs end: " << blueprint.id << " " << move
    // <<
    // " "
    //      << maxGeodes << endl;
}

void dfs(const Blueprint& blueprint, State state, Moves move,
         int limit, int& max_geodes) {
    if (move == buildOreBot &&
        state.oreBots >= blueprint.maxOreBots) {
        return;
    } else if (move == buildClayBot &&
               state.clayBots >= blueprint.maxClayBots) {
        return;
    } else if (move == buildObsidianBot &&
               state.obsidianBots >=
                   blueprint.maxObsidianBots) {
        return;
    }

    // auto integer_seq_sum = [](int a, int b) {
    //     return (b - a) * (a + b) / 2;
    // };
    // int time_remaining = limit - state.min;
    // if (state.resources.geode +
    //         integer_seq_sum(
    //             state.geode_bots,
    //             state.geode_bots + time_remaining) <=
    //     max_geodes) {
    //     return;
    // }

    while (state.time < limit) {
        auto build_next_bot = [&](const Resources& cost) {
            if (canBuild(state, cost)) {
                for (Moves next_move :
                     {buildOreBot, buildClayBot,
                      buildObsidianBot, buildGeodeBot}) {
                    State new_state = update(state, move, cost);
                    dfs(blueprint, new_state, next_move, limit,
                        max_geodes);
                }
                return true;
            }
            return false;
        };

        if (move == buildOreBot &&
            build_next_bot(blueprint.oreBotCost)) {
            return;
        } else if (move == buildClayBot &&
                   build_next_bot(blueprint.clayBotCost)) {
            return;
        } else if (move == buildObsidianBot &&
                   build_next_bot(blueprint.obsidianBotCost)) {
            return;
        } else if (move == buildGeodeBot &&
                   build_next_bot(blueprint.geodeBotCost)) {
            return;
        }

        state = update(state, buildNone);
    }

    max_geodes = max(state.resources.geode, max_geodes);
}

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;

    vector<Blueprint> blueprints;

    while (getline(file, line)) {
        cout << line << endl;

        Blueprint blue;

        sscanf(line.c_str(),
               "Blueprint %d: Each ore robot costs %d ore. "
               "Each clay robot costs %d ore. Each obsidian "
               "robot costs %d ore and %d clay. Each geode "
               "robot costs %d ore and %d obsidian.",
               &blue.id, &blue.oreBotCost.ore,
               &blue.clayBotCost.ore, &blue.obsidianBotCost.ore,
               &blue.obsidianBotCost.clay,
               &blue.geodeBotCost.ore,
               &blue.geodeBotCost.obsidian);

        blue.maxOreBots = max(
            {blue.oreBotCost.ore, blue.clayBotCost.ore,
             blue.obsidianBotCost.ore, blue.geodeBotCost.ore});
        blue.maxClayBots = blue.obsidianBotCost.clay;
        blue.maxObsidianBots = blue.geodeBotCost.obsidian;

        blueprints.push_back(blue);

        cout << blue.id << " " << blue.oreBotCost.ore << " "
             << blue.clayBotCost.ore << " "
             << blue.obsidianBotCost.ore << " "
             << blue.obsidianBotCost.clay << " "
             << blue.geodeBotCost.ore << " "
             << blue.geodeBotCost.obsidian << " "
             << blue.maxOreBots << " " << blue.maxClayBots
             << " " << blue.maxObsidianBots << endl;
        cout << endl;
    }
    cout << endl;

    int ans = 0;

    int maxTime = 24;

    for (auto& blueprint : blueprints) {
        State state;
        int maxGeodes = 0;

        for (auto move :
             {buildOreBot, buildClayBot, buildObsidianBot,
              buildGeodeBot, buildNone}) {
            dfs(blueprint, state, move, maxTime, maxGeodes);
            cout << "For: " << blueprint.id << " " << move
                 << " " << maxGeodes << endl;
        }

        ans += blueprint.id * maxGeodes;
    }

    cout << ans << endl;
}
