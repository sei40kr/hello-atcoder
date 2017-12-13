#include <bits/stdc++.h>

// placing-marbles.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int x;
  cin >> x;

  cout << (x % 10 + (x % 100) / 10 + x / 100) << endl;
}
