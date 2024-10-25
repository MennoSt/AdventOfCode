#ifndef PASSPORTVALIDATOR_HPP
#define PASSPORTVALIDATOR_HPP

#include <string>
#include <vector>

class PassportValidator
{
public:
    PassportValidator();
    ~PassportValidator() = default;

    int getNumberOfValidPassportsPart1(const std::string& inputFile);
    int getNumberOfValidPassportsPart2(const std::string& inputFile);
    bool containsAllProperties(const std::string& passportString);
    bool containsCorrectData(std::string& passPortString);

private:
    std::vector<std::string> getManupPassportString(std::vector<std::string>& stringVector);
    bool isCorrectBirthYear(std::string& birthYear);
    bool isCorrectIssueYear(std::string& issueYear);
    bool isExpirationYear(std::string& expirationYear);
    bool isCorrectHeight(std::string& height);
    bool isCorrectHairColor(std::string& hairColor);
    bool isCorrectEyeColor(std::string& eyeColor);
    bool isCorrectPid(std::string& eyeColor);
};

#endif /* PASSPORTVALIDATOR_HPP */