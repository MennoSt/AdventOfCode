#include <string>
#include <sstream>
#include <vector>
#include <iterator>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <unordered_map>

template <typename Out>
void split(std::string &s, char delim, Out result) {
    std::istringstream iss(s);
    std::string item;
    while (std::getline(iss, item, delim)) {
        *result++ = item;
    }
}

// generic solution
template <class T>
int numDigits(T number)
{
    int digits = 0;
    if (number < 0) digits = 1; // remove this line if '-' counts as a digit
    while (number) {
        number /= 10;
        digits++;
    }
    return digits;
}

static std::vector<std::string> split(std::string &s, char delim) {
    std::vector<std::string> elems;
    split(s, delim, std::back_inserter(elems));
    return elems;
}

static int countDistinct(std::string& s) 
{ 

    std::unordered_map<char, int> m; 
  
    for (int i = 0; i < s.length(); i++) { 
        m[s[i]]++; 
    } 
  
    return m.size(); 
} 

static std::vector<std::string> getStringVector(const std::string& inputFile)
{
    std::vector<std::string> stringVector;
 
    std::string filename(inputFile);
    std::string line;

    std::ifstream input_file(filename);
    if (!input_file.is_open()) {
        std::cerr << "Could not open the file - '"
             << filename << "'" << std::endl;
        return stringVector;
    }

    while (std::getline(input_file, line)) {
        stringVector.push_back(line);
    }

    return stringVector;
}

static bool contains_characters(const std::string &str)
{
    bool containtsDigits = str.find_first_not_of("0123456789abcdef") == std::string::npos;
    return containtsDigits;
}

static std::vector<std::string> appendAndRemoveEmptyLines(std::vector<std::string> inputVector)
{
    std::vector<std::string> tmpVector;
    std::string tmpString;
    
    for (std::string answer:inputVector)
    {
        if (answer!="")
        {
            tmpString.append(answer);
        } else 
        {
            tmpVector.push_back(tmpString);
            tmpString = "";
        }
    }

    tmpVector.push_back(tmpString); // push back last one since there is no empty line at the end
    
    return tmpVector;
}

static std::string getCommonCharacters(std::string& string1, std::string& string2)
{
    std::string tmpString;
    // to store the count of
    // letters in the first string
    int a1[26] = {0};
     
    // to store the count of
    // letters in the second string
    int a2[26] = {0};
    int i , j;
    char ch;
    char ch1 = 'a';
    int k = (int)ch1, m;
     
    // for each letter present, increment the count
    for(i = 0 ; i < string1.length() ; i++)
    {
        a1[(int)string1[i] - k]++;
    }
    for(i = 0 ; i < string2.length() ; i++)
    {
        a2[(int)string2[i] - k]++;
    }
 
    for(i = 0 ; i < 26 ; i++)
    {
        // the if condition guarantees that
        // the element is common, that is,
        // a1[i] and a2[i] are both non zero
        // means that the letter has occurred
        // at least once in both the strings
        if (a1[i] != 0 and a2[i] != 0)
        {
            // print the letter for a number
            // of times that is the minimum
            // of its count in s1 and s2
            for(j = 0 ; j < std::min(a1[i] , a2[i]) ; j++)
            {
                m = k + i;
                ch = (char)(k + i);
                tmpString.push_back(ch);
            }
        }
    }
    return tmpString;
}