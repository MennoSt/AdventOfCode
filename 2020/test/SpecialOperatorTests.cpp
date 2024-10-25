#include <gtest/gtest.h>

#include "PassportValidator.h"
#include "SeatCalculator.h"
#include "SolutionHandler.h"
#include "SpecialOperator.h"
#include "mocks/MockPassportValidator.h"


class SpecialOperatorTests : public ::testing::Test {
protected:
    SpecialOperator* m_specialOperator;
    SolutionHandler m_solutionHandler;
    MockPassportValidator* m_mockPassportValidator;
    SeatCalculator m_seatCalculator;

SpecialOperatorTests()
    : m_mockPassportValidator()
    , m_specialOperator()
    , m_seatCalculator()
    , m_solutionHandler(m_specialOperator, m_mockPassportValidator, m_seatCalculator)
    {

    }

std::string firstString = "1-3 a: abcde";
std::string secondString = "1-3 b: cdefg";
std::string thirdString = "2-9 c: ccccccccc";

};

TEST_F(SpecialOperatorTests, firstCorrectPassword) {
    EXPECT_TRUE(m_specialOperator->isCorrectPasswordPart1(firstString));
}

TEST_F(SpecialOperatorTests, firstWrongPassword) {
    EXPECT_FALSE(m_specialOperator->isCorrectPasswordPart1(secondString));
}

TEST_F(SpecialOperatorTests, secondCorrectPassword) {
    EXPECT_TRUE(m_specialOperator->isCorrectPasswordPart1(thirdString));
}

TEST_F(SpecialOperatorTests, firstCorrectPasswordPart2) {
    EXPECT_TRUE(m_specialOperator->isCorrectPasswordPart2(firstString));
}

TEST_F(SpecialOperatorTests, firstWrongPasswordPart2) {
    EXPECT_FALSE(m_specialOperator->isCorrectPasswordPart2(secondString));
}

TEST_F(SpecialOperatorTests, secondWrongPasswordPasswordPart2) {
    EXPECT_FALSE(m_specialOperator->isCorrectPasswordPart2(thirdString));
}

TEST_F(SpecialOperatorTests, getNumberOfTrees) {
    std::vector<std::string> testForest = 
                            {"..##.......",
                            "#...#...#..",
                            ".#....#..#.",
                            "..#.#...#.#",
                            ".#...##..#.",
                            "..#.##.....",
                            ".#.#.#....#",
                            ".#........#",
                            "#.##...#...",
                            "#...##....#",
                            ".#..#...#.#"};

    EXPECT_EQ(m_specialOperator->getNumberOfTrees(testForest,3,1), 7);
}