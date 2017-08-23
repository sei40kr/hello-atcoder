#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  int n;
  scanf("%coloring-dominoes", &n);
  set<int> found;
  list<int> pairs;
  REP(i, n) {
    int a;
    scanf("%coloring-dominoes", &a);
    const auto &ptr = found.find(a);
    if (ptr != found.end()) {
      found.erase(ptr);
      pairs.push_back(-a);
    } else {
      found.insert(a);
    }
  }

  pairs.sort();
  if (2 <= pairs.size()) {
    int a = pairs.front();
    pairs.pop_front();
    int b = pairs.front();
    cout << (long) a * b;
  } else {
    cout << 0;
  }

  cout << endl;
}

