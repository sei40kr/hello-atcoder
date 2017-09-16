#include <bits/stdc++.h>

// between-two-integers.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int a, b, c;
  cin >> a >> b >> c;
  cout << (a <= c && c <= b ? "Yes" : "No") << endl;
}
