#include <iostream>
#include <string>

#define FOR(i, a, b) for (int i = (a); i < (b); i++)
#define REP(i, n) for (int i = 0; i < (n); i++)

// grid-repainting-2.cpp --- Grid Repainting 2
// author: Seong Yong-ju <sei40kr@gmail.com>

using namespace std;

int h, w;
bool data[52][52];

bool check() {
  FOR(i, 1, 51) {
    FOR(j, 1, 51) {
      if (data[i][j] && !data[i - 1][j] && !data[i + 1][j] && !data[i][j - 1] &&
          !data[i][j + 1]) {
        return false;
      }
    }
  }

  return true;
}

int main() {
  ios::sync_with_stdio(false);
  cin.tie(NULL);

  fill((bool *)&data[0], (bool *)&data[51], false);

  cin >> h >> w;
  REP(i, h) {
    string s;
    cin >> s;

    REP(j, w) { data[i + 1][j + 1] = s[j] == '#'; }
  }

  cout << (check() ? "Yes" : "No") << endl;
}
