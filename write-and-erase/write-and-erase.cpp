#include <bits/stdc++.h>

// write-and-erase.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;
  set<int> nums;
  REP(i, n) {
    int a;
    cin >> a;
    if (nums.erase(a) == 0) {
      nums.insert(a);
    }
  }

  cout << nums.size() << endl;
}
