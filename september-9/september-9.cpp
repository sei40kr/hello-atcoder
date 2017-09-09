#include <bits/stdc++.h>

// september-9.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;

  cout << (n % 10 == 9 || n / 10 == 9 ? "Yes" : "No");
}
