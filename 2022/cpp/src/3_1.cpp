
#include <bits/stdc++.h>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <queue>
#include <math.h>
#include <map>
#include <set>
#include <unordered_set>
#include <stack>
#include <list>
#include <bitset>
#include <utility>
#include <algorithm>
#include <iomanip>
#include <cstdio>
#include <cstring>
#include <numeric>

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
ll gcd(ll a, ll b) { return b ? gcd(b, a % b) : a; }
ll lcm(ll a, ll b) { return a * (b / gcd(a, b)); }

auto inside = [](int row, int col, int r, int c)
{
    return 0 <= row && 0 <= col && row < r && col < c;
};

int main()
{
    // Read file
    ifstream in("../data/in/3.in");

    // HashMap of a-z and A-Z
    map<char, int> priority_map;

    for (int i = 0; i < 26; i++)
    {
        priority_map['a' + i] = i + 1;
        priority_map['A' + i] = i + 27;
    }

    int res = 0;

    // Count map
    map<char, int> count_map;

    for (string line; getline(in, line);)
    {
        // Split line in half
        string first = line.substr(0, line.size() / 2), second = line.substr(line.size() / 2);

        // Count characters
        for (char c : first)
        {
            count_map[c]++;
        }

        for (char c : second)
        {
            if (count_map[c] > 0)
            {
                res += priority_map[c];
                break;
            }
        }

        count_map.clear();
    }

    cout << res << endl;
    return 0;
}