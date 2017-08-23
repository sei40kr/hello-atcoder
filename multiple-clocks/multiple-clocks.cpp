#include <bits/stdc++.h>

#define REP(i, n) for (int i = 0; i < (n); ++i)

// author: Seong Yong-ju ( @sei40kr )

using namespace std;

template<typename T>
T gcd(T m, T n) {
  while (m % n != 0) {
    return gcd(n, m % n);
  }

  return n;
}

template<typename T>
T lcm(T m, T n) {
  return m / gcd(m, n) * n;
}

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;
  long ts[n];
  REP(i, n) {
    cin >> ts[i];
  }

  long ans = accumulate(ts, ts + n, (long) 1, [](long x, long t) {
    return lcm(x, t);
  });
  cout << ans << endl;
}
