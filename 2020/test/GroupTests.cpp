#include <gtest/gtest.h>

#include "Group.h"


class GroupTests : public ::testing::Test {
protected:
    Group m_group;

GroupTests()
    {

    }

};

TEST_F(GroupTests, zeroAnswerInputsResultCorrect) {

    EXPECT_EQ(m_group.calculateNumberOfCommonYes(),0);

    m_group.empty();
}

TEST_F(GroupTests, oneAnswerInputsResultCorrect) {
    std::string testString1 = "asd";
    m_group.addAnswer(testString1);

    EXPECT_EQ(m_group.calculateNumberOfCommonYes(),3);

    m_group.empty();
}

TEST_F(GroupTests, threeAnswerInputsResultCorrect) {
    std::string testString1 = "asd";
    std::string testString2 = "ad";
    std::string testString3 = "afd";
    m_group.addAnswer(testString1);
    m_group.addAnswer(testString2);
    m_group.addAnswer(testString3);

    EXPECT_EQ(m_group.calculateNumberOfCommonYes(),2);

    m_group.empty();
}

TEST_F(GroupTests, fiveAnswerInputsResultCorrect) {
    std::string testString1 = "ba";
    std::string testString2 = "basxda";
    std::string testString3 = "bbaxb";
    std::string testString4 = "lllbxallll";
    std::string testString5 = "xxd";

    m_group.addAnswer(testString1);
    m_group.addAnswer(testString2);
    m_group.addAnswer(testString3);
    m_group.addAnswer(testString4);
    m_group.addAnswer(testString5);

    EXPECT_EQ(m_group.calculateNumberOfCommonYes(),0);

    m_group.empty();
}