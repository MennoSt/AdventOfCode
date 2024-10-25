#include <gtest/gtest.h>

#include "PassportValidator.h"
#include "SolutionHandler.h"
#include "SpecialOperator.h"


class PassportValidatorTests : public ::testing::Test {
protected:
    PassportValidator* m_passportValidator;

PassportValidatorTests()
    : m_passportValidator()
    {

    }

};

TEST_F(PassportValidatorTests, testValidPassportString) {
    std::string passPortString = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    EXPECT_TRUE(m_passportValidator->containsAllProperties(passPortString));
}

TEST_F(PassportValidatorTests, testInValidPassportString2) {
    std::string passPortString = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884hcl:#cfa07d byr:1929";
    EXPECT_FALSE(m_passportValidator->containsAllProperties(passPortString));
}

TEST_F(PassportValidatorTests, testInValidPassportString3) {
    std::string passPortString = "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm";
    EXPECT_TRUE(m_passportValidator->containsAllProperties(passPortString));
}

TEST_F(PassportValidatorTests, testInValidPassportString4) {
    std::string passPortString = "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in";
    EXPECT_FALSE(m_passportValidator->containsAllProperties(passPortString));
}

TEST_F(PassportValidatorTests, testInValidPassportString5) {
    std::string passPortString = "";
    EXPECT_FALSE(m_passportValidator->containsAllProperties(passPortString));
}

TEST_F(PassportValidatorTests, testNumberOfValidPassports) {

    EXPECT_EQ(m_passportValidator->getNumberOfValidPassportsPart1("testInput1Part1.txt"),2);
}

TEST_F(PassportValidatorTests, testNumberOfValidPassports2) {

    EXPECT_EQ(m_passportValidator->getNumberOfValidPassportsPart1("testInput2Part1.txt"),2);
}

TEST_F(PassportValidatorTests, testNumberOfInValidPassports3) {

    EXPECT_EQ(m_passportValidator->getNumberOfValidPassportsPart2("testInput3WrongPassportsPart2.txt"),0);
}

TEST_F(PassportValidatorTests, testNumberOfValidPassports4) {

    EXPECT_EQ(m_passportValidator->getNumberOfValidPassportsPart2("testInput4CorrectPassportsPart2.txt"),4);
}