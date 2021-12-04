#include <bits/stdc++.h>

using namespace std;

#define all(x) (x).begin(), (x).end()
#define mp make_pair
#define fi first
#define se second
#define pb push_back
#define eb emplace_back
#define forn(i, n) for (int i = 0; i < (int)n; ++i)
#define ford(i, n) for (int i = (int)n - 1; i >= 0; --i)

typedef long long ll;
typedef vector<int> vi;
typedef vector<vi> vvi;
typedef pair<int, int> ii;
typedef vector<ii> vii;
typedef vector<vii> vvii;

const ll mod = 1000000007;
int gcd(int a, int b) { return b ? gcd(b, a % b) : a; }
ll lcm(ll a, ll b) { return a * (b / gcd(a, b)); }

int binaryStringToInt(string num) {
    int res = 0;
    for (int i = 0; i < num.length(); ++i) {
        if (num[i] == '1') res = res | (1 << num.length() - 1 - i);
    }
    return res;
}

int main() {
    ifstream in("in.txt");
    vector<string> nums;
    string tmp;
    while (!in.eof()) {
        in >> tmp;
        nums.eb(tmp);
    }

    pair<bool, bool> running = {true, true};
    int counter_most = nums.size(), counter_least = nums.size();
    vector<pair<bool, bool>> ok(nums.size(), {true, true});
    vii keepers(12, {0, 0});
    vii counters_most(12, {0, 0});
    vii counters_least(12, {0, 0});

    for (int j = 0; j <= 12; ++j) {
        if (!running.fi && !running.se) break;
        for (int i = 0; i < nums.size(); ++i) {
            if (j != 0) {
                if (running.fi && ok[i].fi) {
                    if (keepers[j - 1].fi == 0 && nums[i][j - 1] != '0') {
                        counter_least--;
                        ok[i].fi = false;
                    } else if (keepers[j - 1].fi == 1 && nums[i][j - 1] != '1') {
                        counter_least--;
                        ok[i].fi = false;
                    }
                }
                if (running.se && ok[i].se) {
                    if (keepers[j - 1].se == 0 && nums[i][j - 1] != '0') {
                        counter_most--;
                        ok[i].se = false;
                    } else if (keepers[j - 1].se == 1 && nums[i][j - 1] != '1') {
                        counter_most--;
                        ok[i].se = false;
                    }
                }
            }

            if (running.fi && counter_least == 1) {
                running.fi = false;
            }

            if (running.se && counter_most == 1) {
                running.se = false;
            }

            if (j < 12) {
                // Counters
                if (nums[i][j] == '0') {
                    if (running.fi && ok[i].fi) counters_least[j].fi++;
                    if (running.se && ok[i].se) counters_most[j].fi++;
                } else {
                    if (running.fi && ok[i].fi) counters_least[j].se++;
                    if (running.se && ok[i].se) counters_most[j].se++;
                }
            }
        }
        if (j < 12) {
            if (running.fi) keepers[j].fi = (counters_least[j].fi <= counters_least[j].se ? 0 : 1);
            if (running.se) keepers[j].se = (counters_most[j].fi > counters_most[j].se ? 0 : 1);
        }
    }

    int ogr, csr;
    for (int i = 0; i < nums.size(); ++i) {
        if (ok[i].fi) cout << "Oxygen generator rating: " << (ogr = binaryStringToInt(nums[i])) << endl;
        if (ok[i].se) cout << "CO2 scrubber rating: " << (csr = binaryStringToInt(nums[i])) << endl;
    }

    cout << "Life support rating: " << ogr * csr << endl;

    return 0;
}