#include <bits/stdc++.h>

// non-decreasing.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define FOR(i, a, b) for (int i = (a); i < (b); ++i)

#define REP(i, n) for (int i = 0; i < (n); ++i)
#define RREP(i, n) for (int i = (n)-1; i >= 0; --i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;

  int xs[n];

  REP(i, n) {
    int x;
    cin >> x;

    xs[i] = x;
  }

  size_t M = max_element(xs, xs + n) - xs;
  size_t m = min_element(xs, xs + n) - xs;

  size_t base = abs(xs[m]) <= abs(xs[M]) ? M : m;

  cout << n * 2 - 1 << endl;

  REP(i, n) { cout << base + 1 << ' ' << (i + 1) << endl; }

  if (0 <= xs[base]) {
    FOR(i, 1, n) { cout << i << ' ' << i + 1 << endl; }
  } else {
    RREP(i, n - 1) { cout << i + 2 << ' ' << i + 1 << endl; }
  }
}
