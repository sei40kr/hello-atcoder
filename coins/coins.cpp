#include <bits/stdc++.h>

// coins.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2018 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int a, b, c, x;
  cin >> a >> b >> c >> x;

  int ans = 0;
  REP(i, a + 1) {
    REP(j, b + 1) {
      REP(k, c + 1) { ans += 500 * i + 100 * j + 50 * k == x ? 1 : 0; }
    }
  }

  cout << ans << endl;
}
