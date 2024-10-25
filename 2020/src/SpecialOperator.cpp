#include "SpecialOperator.h"
#include "Utils.h"

#include <boost/algorithm/string.hpp>

SpecialOperator::SpecialOperator()
{

}

unsigned long SpecialOperator::getNumberOfTrees(const std::vector<std::string>& stringVector, int rightStep, int downStep)
{
    size_t counter = 0;
    size_t  x = 0;
    std::string treeStr = "#";

    for (int i=downStep; i< stringVector.size(); i+=downStep) {
            x+=rightStep;
            std::string treeStr = "#";
            std::string forestLine = stringVector[i];
            if (x >= forestLine.size())
                x -= forestLine.size();
            if (forestLine.at(x)  == treeStr[0])
                ++counter;
    }
    return counter;
}

bool SpecialOperator::isCorrectPasswordPart1(std::string& password) 
{
    std::string chars = ":";

    for (char c: chars) {
        password.erase(std::remove(password.begin(), password.end(), c), password.end());
    }

    std::vector<std::string> vectorStr = split(password, ' ');
    std::vector<std::string> numberVec = split(vectorStr[0], '-');

    std::string number1 = numberVec[0];
    std::string number2 = numberVec[1];
    std::string letterPwd = vectorStr[1];
    std::string pwdString = vectorStr[2];

    int LowestNumber = std::stoi(number1);
    int HighestNumber = std::stoi(number2);

    const char* a = letterPwd.c_str();
    int numberOfOccurences = std::count(pwdString.begin(), pwdString.end(), *a);

    if (numberOfOccurences < LowestNumber || numberOfOccurences > HighestNumber ) {
        return false;
    } else {
        return true;
    }
}

bool SpecialOperator::isCorrectPasswordPart2(std::string& password) 
{
    std::string chars = ":";

    for (char c: chars) {
        password.erase(std::remove(password.begin(), password.end(), c), password.end());
    }

    std::vector<std::string> vectorStr = split(password, ' ');
    std::vector<std::string> numberVec = split(vectorStr[0], '-');

    std::string number1 = numberVec[0];
    std::string number2 = numberVec[1];
    std::string letterPwd = vectorStr[1];
    std::string pwdString = vectorStr[2];

    int LowestNumber = std::stoi(number1);
    int HighestNumber = std::stoi(number2);

    char lowCharacter = pwdString[LowestNumber-1];
    char highCharacter = pwdString[HighestNumber-1];


    if (lowCharacter == letterPwd[0] ^ highCharacter == letterPwd[0])
        return true;
    else 
        return false;
    
}

