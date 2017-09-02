#include <bits/stdc++.h>

// together.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

#define ALL(a) (a).begin(), (a).end()

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;
  int as[n];
  REP(i, n) { cin >> as[i]; }

  vector<int> cnt(100002, 0);
  REP(i, n) {
    int a = as[i];
    ++cnt[a];
    ++cnt[a + 1];
    ++cnt[a + 2];
  }

  cout << *max_element(ALL(cnt)) << endl;
}
