#include "Group.h"
#include "Utils.h"

Group::Group()
    : m_answers()
{

}

void Group::addAnswer(std::string& answer)
{
    m_answers.push_back(answer);
}

void Group::empty()
{
    m_answers.clear();
}

int Group::calculateNumberOfCommonYes()
{

    if(m_answers.size() == 0)
    {
        return 0;
    }

    if (m_answers.size() == 1)
    {
        return countDistinct(m_answers[0]);
    }

    std::string answerTmp = m_answers[0];

    for (int index =0; index < (m_answers.size() -1); index++)
    {
        answerTmp = getCommonCharacters(answerTmp, m_answers[index+1]);
    }

    return countDistinct(answerTmp);
}