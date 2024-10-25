#ifndef GROUP_HPP
#define GROUP_HPP

#include <iostream>
#include <vector>

class Group
{
public:
    Group();
    ~Group() = default;

    void addAnswer(std::string& answer);
    void empty();
    int calculateNumberOfCommonYes();

private:
    std::vector<std::string> m_answers;

};

#endif /* GROUP_HPP */