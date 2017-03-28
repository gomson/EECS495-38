#include <iostream>
#include <sstream>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <string>
#include <algorithm>
#include <cstdlib>
#include <utility>
using namespace std;
bool is1stDeletion(const string& src, const string& des) {
	bool result = false;
	if (des.size() >= 1 && src.size() >= 2 && src != des && des.size() == (src.size() - 1)) {
		size_t i = 0;
		for (i = 0; i < des.size(); ++i) {
			if (src[i] != des[i]) {
				break;
			}
		}
		string desCopy(des);
		desCopy.insert(desCopy.begin() + i, src[i]);
		result = (src == desCopy);
	}
	return result;
}
bool is1stTransposition(const string& src, const string& des) {
	bool result = false;
	if (des.size() >= 2 && src.size() >= 2 && src != des && des.size() == src.size()) {
		size_t i = 0;
		for (i = 0; i < des.size() - 1; ++i) {
			if (src[i] != des[i]) {
				break;
			}
		}
		if(i < des.size() - 1) {
			string desCopy(des);
			swap(desCopy[i], desCopy[i + 1]);
			result = (src == desCopy);
		}
	}
	return result;
}
bool is1stReplacement(const string& src, const string& des) {
	bool result = false;
	if (des.size() >= 1 && src.size() >= 1 && src != des && des.size() == src.size()) {
		size_t cnt = 0;
		for(size_t i = 0; i < des.size(); ++i){
			if(des[i] != src[i]){
				++cnt;
			}
		}
		result = (cnt == 1);
	}
	return result;
}
bool is1stInsertion(const string& src, const string& des) {
	bool result = false;
	if (des.size() >= 2 && src.size() >= 1 && src != des && des.size() == (src.size() + 1)) {
		size_t i = 0;
		for (i = 0; i < src.size(); ++i) {
			if (src[i] != des[i]) {
				break;
			}
		}
		string desCopy(des);
		desCopy.erase(desCopy.begin() + i);
		result = (src == desCopy);
	}
	return result;
}
bool is1stEdit(const string& src, const string& des) {
	bool result = is1stDeletion(src, des);
	result = result || is1stTransposition(src, des);
	result = result || is1stReplacement(src, des);
	result = result || is1stInsertion(src, des);
	return result;
}
bool is2ndDeletionAndDeletion(const string& src, const string& des) {
	bool result = false;
	if (src.size() >= 3 && des.size() >= 1 && src != des && !is1stEdit(src, des) && src.size() == (des.size() + 2)) {
		size_t i = 0;
		for (i = 0; i < des.size(); ++i) {
			if (src[i] != des[i]) {
				break;
			}
		}
		string desCopy(des);
		desCopy.insert(desCopy.begin() + i, src[i]);
		for (i = 0; i < desCopy.size(); ++i) {
			if (src[i] != desCopy[i]) {
				break;
			}
		}
		desCopy.insert(desCopy.begin() + i, src[i]);
		result = (desCopy == src);
	}
	return result;
}
bool is2ndDeletionAndTransposition(const string& src, const string& des) {
	bool result = false;
	if (src.size() >= 3 && des.size() >= 2 && src != des && !is1stEdit(src, des) && src.size() == (des.size() + 1)) {
		string desCopy1(des);
		string desCopy2(des);
		size_t i = 0;
		for(i = 0; i < des.size() - 1; ++i){
			if(src[i] != des[i]){
				break;
			}
		}
		if(i < des.size() - 1){
			swap(desCopy1[i], desCopy1[i + 1]);
			result = is1stDeletion(src, desCopy1);
			desCopy2.insert(desCopy2.begin() + i, src[i]);
			result = result || is1stTransposition(src, desCopy2);
		}
	}
	return result;
}
bool is2ndDeletionAndReplacement(const string& src, const string& des) {
	bool result = false;
	if (src.size() >= 2 && des.size() >= 1 && src != des && !is1stEdit(src, des) && src.size() == (des.size() + 1)) {
		size_t i = 0;
		for (i = 0; i < des.size(); ++i) {
			if (src[i] != des[i]) {
				break;
			}
		}
		if(i < des.size()){
			string desCopy1(des);
			desCopy1.insert(desCopy1.begin() + i, src[i]);
			result = is1stReplacement(src, desCopy1);
			string desCopy2(des);
			desCopy2[i] = src[i];
			result = result || is1stDeletion(src, desCopy2);
		}
	}
	return result;
}
bool is2ndDeletionAndInsertion(const string& src, const string& des) {
	bool result = false;
	if (src.size() >= 2 && des.size() >= 2 && src != des && !is1stEdit(src, des) && src.size() == des.size()) {
		size_t i = 0;
		for (i = 0; i < src.size(); ++i) {
			if (src[i] != des[i]) {
				break;
			}
		}
		if(i < src.size() - 1){
			string desCopy1(des);
			string desCopy2(des);
			desCopy1.erase(desCopy1.begin() + i);
			result = is1stDeletion(src, desCopy1);
			desCopy2.insert(desCopy2.begin() + i, src[i]);
			result = result || is1stInsertion(src, desCopy2);
		}
	}
	return result;
}
bool is2ndTranspositionAndTransposition(const string& src, const string& des) {
	bool result = false;
	if (src.size() >= 3 && des.size() >= 3 && src != des && !is1stEdit(src, des) && src.size() == des.size()) {
		size_t i = 0;
		for (i = 0; i < src.size(); ++i) {
			if (src[i] != des[i]) {
				break;
			}
		}
		if(i < des.size() - 2){
			string desCopy1(des);
			string desCopy2(des);
			string desCopy3(des);
			swap(desCopy1[i], desCopy1[i + 2]);
			swap(desCopy1[i + 1], desCopy1[i + 2]);
			result = (desCopy1 == src);
			swap(desCopy2[i], desCopy2[i + 1]);
			for (i = 0; i < desCopy2.size(); ++i) {
				if (src[i] != desCopy2[i]) {
					break;
				}
			}
			if(i < desCopy2.size() - 1){
				swap(desCopy2[i], desCopy2[i + 1]);
				result = result || (desCopy2 == src);	
			}		
		}
	}
	return result;
}
bool is2ndTranspositionAndReplacement(const string& src, const string& des) {
	bool result = false;
	if (src.size() >= 2 && des.size() >= 2 && src != des && !is1stEdit(src, des) && src.size() == des.size()) {
		string desCopy1(des);
		string desCopy2(des);
		string desCopy3(des);
		size_t i = 0;
		for(i = 0; i < src.size(); ++i){
			if(des[i] != src[i]){
				break;
			}
		}
		if(i < des.size() - 1){
			swap(desCopy1[i], desCopy1[i + 1]);
			string desCopy1a(desCopy1);
			desCopy1a[i] = src[i];
			result = result || (desCopy1a == src);
			string desCopy1b(desCopy1);
			desCopy1b[i + 1] = src[i + 1];
			result = result || (desCopy1b == src);
		}
		if(i < des.size() - 2){
			desCopy2[i] = src[i];
			result = result || is1stTransposition(src, desCopy2);
			swap(desCopy3[i], desCopy3[i + 1]);
			result = result || is1stReplacement(src, desCopy3);
		}

	}
	return result;
}
bool is2ndTranspositionAndInsertion(const string& src, const string& des) {
	bool result = false;
	if (src.size() >= 2 && des.size() >= 3 && src != des && !is1stEdit(src, des) && des.size() == src.size() + 1) {
		size_t i = 0;
		for(i = 0; i < src.size(); ++i){
			if(des[i] != src[i]){
				break;
			}
		}
		if(i < des.size() - 2){
			string desCopy1(des);
			desCopy1.erase(desCopy1.begin() + i);
			result = is1stTransposition(src, desCopy1);
			string desCopy2(des);
			swap(desCopy2[i], desCopy2[i + 1]);
			result = result || is1stInsertion(src, desCopy2);
			string desCopy3(des);
			swap(desCopy3[i], desCopy3[i + 2]);
			desCopy3.erase(desCopy3.begin() + i + 1);
			result = result || (src == desCopy3);
		}
	}
	return result;
}
bool is2ndReplacementAndReplacement(const string& src, const string& des){
	bool result = false;
	if (des.size() >= 2 && src.size() >= 2 && src != des && des.size() == src.size()) {
		size_t cnt = 0;
		for(size_t i = 0; i < src.size(); ++i){
			if(des[i] != src[i]){
				++cnt;
			}
		}
		result = (cnt == 2);
	}
	return result;	
}
bool is2ndReplacementAndInsertion(const string& src, const string& des){
	bool result = false;
	if (des.size() >= 2 && src.size() >= 1 && src != des && des.size() == (src.size() + 1)) {
		size_t i = 0;
		for(i = 0; i < src.size(); ++i){
			if(des[i] != src[i]){
				break;
			}
		}
		if(i < des.size() - 1){
			string desCopy1(des);
			desCopy1.erase(desCopy1.begin() + i);
			result = is1stReplacement(src, desCopy1);
			string desCopy2(des);
			desCopy2[i] = src[i];
			result = result || is1stInsertion(src, desCopy2);
		}
	}
	return result;	
}
bool is2ndInsertionAndInsertion(const string& src, const string& des){
	bool result = false;
	if (des.size() >= 3 && src.size() >= 1 && src != des && des.size() == (src.size() + 2)) {
		size_t i = 0;
		for(i = 0; i < src.size(); ++i){
			if(des[i] != src[i]){
				break;
			}
		}
		string desCopy(des);
		desCopy.erase(desCopy.begin() + i);
		for(i = 0; i < src.size(); ++i){
			if(desCopy[i] != src[i]){
				break;
			}
		}
		desCopy.erase(desCopy.begin() + i);
		result = (desCopy == src);		
	}
	return result;
}
bool is2ndEdit(const string& src, const string& des) {
	bool result = is2ndDeletionAndDeletion(src, des);
	result = result || is2ndDeletionAndTransposition(src, des);
	result = result || is2ndDeletionAndReplacement(src, des);
	result = result || is2ndDeletionAndInsertion(src, des);
	result = result || is2ndTranspositionAndTransposition(src, des);
	result = result || is2ndTranspositionAndReplacement(src, des);
	result = result || is2ndTranspositionAndInsertion(src, des);
	result = result || is2ndReplacementAndReplacement(src, des);
	result = result || is2ndReplacementAndInsertion(src, des);
	result = result || is2ndInsertionAndInsertion(src, des);
	return result;
}
int main(const int argc, const char** argv) {
	string corpus = "hello world hello word hello world";
	string input = "hello\n hell\n word\n wordl\n wor\n wo\n w";
	unordered_set<string> corpusInUnorderedSet;
	vector<string> inputInVector;
	vector<pair<string, string>> ouput;
	istringstream cor(corpus);
	for (string i; cor >> i; ) {
		corpusInUnorderedSet.insert(i);
	}
	istringstream in(input);
	for (string i; in >> i; ) {
		inputInVector.push_back(i);
	}
	for (const auto& i : inputInVector) {
		for (const auto& j : corpusInUnorderedSet) {
			if (j == i) {
				ouput.push_back(pair<string, string>(i, ""));
			}
			else if (j != i && is1stEdit(j, i)) {
				ouput.push_back(pair<string, string>(i, ", " + j));
			}
			else if (j != i && !is1stEdit(j, i) && is2ndEdit(j, i)){
				ouput.push_back(pair<string, string>(i, ", " + j));
			}
			else if(j != i && !is1stEdit(j, i) && !is2ndEdit(j, i)){
				ouput.push_back(pair<string, string>(i, ", -"));				
			}
		}
	}
	for (const auto& i : ouput) {
		cout << i.first << i.second << "\n";
	}
	cout << "\nPlease enter any key to exit.\n";
	getchar();
	return 0;
}