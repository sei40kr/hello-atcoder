#include <bits/stdc++.h>

// not-so-diverse.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n, k;
  cin >> n >> k;

  int as[n];
  map<int, int> kinds_map;

  REP(i, n) {
    int a;
    cin >> a;

    as[i] = a;
    kinds_map[a] = 0;
  }
  REP(i, n) {
    int a = as[i];

    ++kinds_map[a];
  }

  size_t kind_counts = kinds_map.size();
  vector<int> counts;

  for (auto &pair : kinds_map) {
    counts.push_back(pair.second);
  }

  sort(counts.begin(), counts.end());

  int ans =
      k < kind_counts
          ? accumulate(counts.begin(), counts.begin() + (kind_counts - k), 0,
                       [](int sum, int count) -> int { return sum + count; })
          : 0;
  cout << ans << endl;
}
