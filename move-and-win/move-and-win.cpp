#include <bits/stdc++.h>

// move-and-win.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2018 Seong Yong-ju

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int N, A, B;
  cin >> N >> A >> B;

  cout << ((B - A) % 2 == 0 ? "Alice" : "Borys") << endl;
}
