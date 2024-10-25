#ifndef DECLARATIONFORMCALCULATOR_HPP
#define DECLARATIONFORMCALCULATOR_HPP

#include <iostream>
#include <vector>

#include "Group.h"

class DeclarationFormCalculator
{
public:
    DeclarationFormCalculator();
    ~DeclarationFormCalculator() = default;

    int calculateSumYesAnyone(std::vector<std::string>& inputVector);
    int calculateSumYesEveryone(std::vector<std::string>& inputVector);

private:
    void fillGroupsWithAnswers(std::vector<std::string>& inputVector);
    Group m_group;
    std::vector<Group> m_groupVector;
};

#endif /* DECLARATIONFORMCALCULATOR_HPP */
