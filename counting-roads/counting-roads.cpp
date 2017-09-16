#include <bits/stdc++.h>

// counting-roads.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n, m;
  cin >> n >> m;
  int ans[n] = {};
  REP(i, m) {
    int a, b;
    cin >> a >> b;
    ++ans[a - 1];
    ++ans[b - 1];
  }

  REP(i, n) { cout << ans[i] << endl; }
}
