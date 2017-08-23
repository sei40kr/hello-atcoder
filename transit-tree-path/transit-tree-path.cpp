#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

struct edge {
  int to, cost;
};

void dfs(list<edge> *p_tree, long *p_dp, int i, long cost) {
  if (0 <= p_dp[i]) {
    return;
  }

  p_dp[i] = cost;
  for (auto &edge : p_tree[i]) {
    dfs(p_tree, p_dp, edge.to, cost + edge.cost);
  }
}

int main() {
  cin.tie(nullptr);

  int n;
  cin >> n;
  list<edge> tree[n];
  REP(i, n - 1) {
    int a, b, c;
    cin >> a >> b >> c;
    --a;
    --b;

    tree[a].push_back(edge{b, c});
    tree[b].push_back(edge{a, c});
  }

  int q, k;
  cin >> q >> k;
  --k;

  long dp[n];
  fill(dp, dp + n, -1);
  dfs(tree, dp, k, 0);

  REP(i, q) {
    int x, y;
    cin >> x >> y;
    --x;
    --y;
    cout << (dp[x] + dp[y]) << endl;
  }
}

