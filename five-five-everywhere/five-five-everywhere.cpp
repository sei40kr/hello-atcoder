#include <cmath>
#include <iostream>
#include <string>

#define MOD 1000000007

#define FOR(i, a, b) for (int i = (a); i < (b); i++)

// five-five-everywhere.cpp ---
// author: Seong Yong-ju <sei40kr@gmail.com>

using namespace std;

int n;
bool isprime[55556] = {false, false};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(NULL);

  fill(&isprime[2], isprime + 55556, true);

  cin >> n;

  FOR(i, 2, floor(sqrt(55556) + 1)) {
    if (!isprime[i]) {
      continue;
    }
    for (int j = i * 2; j < 55556; j += i) {
      isprime[j] = false;
    }
  }

  int cnt = 0;
  FOR(i, 2, 55556) {
    if (isprime[i] && i % 5 == 1) {
      cout << i;

      if (n <= ++cnt) {
        cout << endl;
        break;
      } else {
        cout << ' ';
      }
    }
  }
}
