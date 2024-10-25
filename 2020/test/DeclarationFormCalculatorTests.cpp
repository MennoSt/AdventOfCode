#include <gtest/gtest.h>

#include "DeclarationFormCalculator.h"
#include "Utils.h"


class DeclarationFormCalculatorTests : public ::testing::Test {
protected:
    DeclarationFormCalculator declarationFormCalculator;

DeclarationFormCalculatorTests()
    {

    }
};

TEST_F(DeclarationFormCalculatorTests, testDeclarationFormCommonYes) {
    std::string testString = "aasdaaadas";
    int x = countDistinct(testString);
    EXPECT_EQ(x,3);
}

TEST_F(DeclarationFormCalculatorTests, testDeclarationFormAnyone) {
    std::vector<std::string> testVector = {"abc","","a","b","c","","ab","ac","","a","a","a","a","","b"};
    int x = declarationFormCalculator.calculateSumYesAnyone(testVector);
    EXPECT_EQ(x,11);
}

TEST_F(DeclarationFormCalculatorTests, testDeclarationFormEveryone) {
    std::vector<std::string> testVector = {"abc","","a","b","c","","ab","ac","","a","a","a","a","","b"};
    int x = declarationFormCalculator.calculateSumYesEveryone(testVector);
    EXPECT_EQ(x,6);
}

TEST_F(DeclarationFormCalculatorTests, testDeclarationFormEveryone2) {
    std::vector<std::string> testVector = {"abc","","a","b","c","","ab","ac","","a","a","a","a","","ba","basxda","bbaxb","lllbxallll","xxd"};
    int x = declarationFormCalculator.calculateSumYesEveryone(testVector);
    EXPECT_EQ(x,5);
}

TEST_F(DeclarationFormCalculatorTests, testDeclarationFormEveryone3) {
    std::vector<std::string> testVector = {"ab","abc","acb","","a","b","c","","ab","ac","","a","a","a","a","","b"};
    int x = declarationFormCalculator.calculateSumYesEveryone(testVector);
    EXPECT_EQ(x,5);
}

TEST_F(DeclarationFormCalculatorTests, testDeclarationFormEveryone4) {
    std::vector<std::string> testVector = {"abcaaaz","","a","b","c","","abx","acx","adx","","a","abcdseras","a","a","","b"};
    int x = declarationFormCalculator.calculateSumYesEveryone(testVector);
    EXPECT_EQ(x,8);
}

TEST_F(DeclarationFormCalculatorTests, testDeclarationFormEveryone5) {
    std::vector<std::string> testVector = {"abc","","a","","ab","","a","","b"};
    int x = declarationFormCalculator.calculateSumYesEveryone(testVector);
    EXPECT_EQ(x,8);
}