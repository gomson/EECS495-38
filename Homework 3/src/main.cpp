#include <iostream>
#include <cassert>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
#include <deque>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;
int main(int argc, const char** argv) {
	string result("");
	bool isFound = false;
	unordered_map<char, unordered_set<char>> graph;
	unordered_set<char> visitedVertex;
	unordered_map<char, char> childToParent;
	deque<char> dq;
	string line;
	ifstream input("../graph.dat");
	if (input.is_open()) {
		while (getline(input, line)) {
			istringstream subString(line);
			size_t cnt = 0;
			char key = 'a';
			for (string i; subString >> i; ++cnt) {
				if (cnt == 0) {
					key = i[0];
					graph[key].clear();
				}
				else if (cnt >= 1) {
					graph[key].insert(i[0]);
				}
			}
		}
		input.close();
	}
	for (const auto& i : graph) {
		for (const auto& j : i.second) {
			if (j != i.first) {
				graph[j].insert(i.first);
			}
		}
	}
	for (const auto& i : graph) {
		cout << i.first << '\t';
		for (const auto& j : i.second) {
			cout << j << ' ';
		}
		cout << '\n';
	}
	//BEGIN: test 0
	const char start('a');
	const char end('c');
	//END: test 0

	////BEGIN : test 1
	//const char start('a');
	//const char end('b');
	////END: test 1

	////BEGIN : test 2
	//const char start = 'a';
	//const char end = 'c';
	////END: test 2
	assert(graph.count(start));
	assert(graph.count(end));
	dq.push_back(start);
	char it = dq.front();
	while (!dq.empty()) {
		if (graph[it].find(end) != graph[it].end()) {
			childToParent[end] = it;
			isFound = true;
			break;
		}
		else if (graph[it].find(end) == graph[it].end()) {
			if (visitedVertex.find(it) == visitedVertex.end()) {
				dq.insert(dq.end(), graph[it].begin(), graph[it].end());
				visitedVertex.insert(it);
				for (const auto& i : graph[it]) {
					if (visitedVertex.find(i) == visitedVertex.end()) {
						childToParent[i] = it;
					}
				}
				dq.pop_front();
				it = dq.front();
			}
			else if (visitedVertex.find(it) != visitedVertex.end()) {
				dq.pop_front();
				it = dq.front();
			}
		}
	}
	if (isFound) {
		for (char i = end; i != start; i = childToParent[i]) {
			result.push_back(i);
		}
		result.push_back(start);
		reverse(result.begin(), result.end());
	}
	else if (!isFound) {
		result = "No Exists.\n";
	}
	cout << '\n' << result << '\n';
	getchar();
	return 0;
}