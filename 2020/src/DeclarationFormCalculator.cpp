#include "DeclarationFormCalculator.h"

#include "Utils.h"


DeclarationFormCalculator::DeclarationFormCalculator()
    : m_group(), 
      m_groupVector()
{

}

int DeclarationFormCalculator::calculateSumYesAnyone(std::vector<std::string>& inputVector)
{
    std::vector<std::string> vectorTmp = appendAndRemoveEmptyLines(inputVector);
    int sumCommonYes = 0;
    for (std::string answer:vectorTmp)
    {
        sumCommonYes += countDistinct(answer);
    }

    return sumCommonYes;
}

void DeclarationFormCalculator::fillGroupsWithAnswers(std::vector<std::string>& inputVector)
{
    for (int index = 0; index < inputVector.size(); index++)
    {
        if (inputVector[index] == "")
        {
            m_groupVector.push_back(m_group);
            m_group.empty();
        } else {
            m_group.addAnswer(inputVector[index]);
        }

        if (index == (inputVector.size()-1))
        {
            m_groupVector.push_back(m_group);
            m_group.empty();
        }
    }
}

int DeclarationFormCalculator::calculateSumYesEveryone(std::vector<std::string>& inputVector)
{
    fillGroupsWithAnswers(inputVector);
    
    int sumCommonYes = 0;
    for (Group group:m_groupVector)
    {
        int commonYestmp = group.calculateNumberOfCommonYes();
        sumCommonYes += commonYestmp;
    }
    
    return sumCommonYes;
}