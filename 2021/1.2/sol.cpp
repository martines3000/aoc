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

int main() {
    ifstream in("in.txt");
    vector<int> input;
    bool start = true;
    int counter = 0, tmp;

    while (!in.eof()) {
        in >> tmp;
        input.eb(tmp);
    }

    int l = 0, r = 0;
    for (int i = 0; i < input.size(); ++i) {
        if (i <= 2) {
            l += input[i];
            r += input[i + 1];
        } else {
            if (r > l) counter++;
            if (i == input.size() - 1) break;
            l = l - input[i - 3] + input[i];
            r = r - input[i - 2] + input[i + 1];
        }
    }

    cout << counter << endl;

    return 0;
}