#include <bits/stdc++.h>

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(0);
  ios::sync_with_stdio(false);

  int n, m;
  cin >> n >> m;

  bool memo[n][2] = {};

  REP(i, m) {
    int a, b;
    cin >> a >> b;

    if (a == 1) {
      memo[b - 1][0] = true;
    } else if (b == n) {
      memo[a - 1][1] = true;
    }
  }

  bool ans = any_of(&memo[0], &memo[0] + n, [](bool *temp) -> bool {
    return temp[0] && temp[1];
  });
  cout << (ans ? "POSSIBLE" : "IMPOSSIBLE") << endl;
}

