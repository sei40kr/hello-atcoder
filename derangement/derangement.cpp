#include <bits/stdc++.h>

// derangement.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;
  int ps[n];
  bool dp[n];
  REP(i, n) {
    int p;
    cin >> p;
    ps[i] = p;
    dp[i] = p != i + 1;
  }

  int ans = 0;
  REP(i, n) {
    if (dp[i]) {
      continue;
    }

    if (i != n - 1 && !dp[i + 1]) {
      dp[i + 1] = true;
    }

    ++ans;
  }

  cout << ans << endl;
}
