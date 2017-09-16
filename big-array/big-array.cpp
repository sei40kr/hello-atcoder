#include <bits/stdc++.h>

// big-array.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

#define ALL(a) (a).begin(), (a).end()

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  long k;
  cin >> n >> k;
  vector<tuple<int, int>> pairs;
  REP(i, n) {
    int a, b;
    cin >> a >> b;
    pairs.push_back(make_tuple(a, b));
  }
  sort(ALL(pairs));

  long cnt = 0;
  for (auto &pair : pairs) {
    int a, b;
    tie(a, b) = pair;
    cnt += b;
    if (k <= cnt) {
      cout << a << endl;
      break;
    }
  }
}
