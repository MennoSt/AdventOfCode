#ifndef SPECIALOPERATOR_HPP
#define SPECIALOPERATOR_HPP

#include <string>
#include <vector>

class SpecialOperator
{
public:
    explicit SpecialOperator();
    ~SpecialOperator() = default;

    bool isCorrectPasswordPart1(std::string& password);
    bool isCorrectPasswordPart2(std::string& password);
    unsigned long getNumberOfTrees(const std::vector<std::string>& stringVector, int rightStep, int downStep);
};

#endif /* SPECIALOPERATOR_HPP */

