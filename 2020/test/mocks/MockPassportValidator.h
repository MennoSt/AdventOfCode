#include "../../src/PassportValidator.h"

#include <gmock/gmock.h>

using ::testing::NiceMock;

class MockPassportValidator : public PassportValidator {
public:
    MOCK_METHOD(int, getNumberOfValidPassportsPart1, (const std::string& inputFile));
    MOCK_METHOD(int, getNumberOfValidPassportsPart2, (const std::string& inputFile));
    MOCK_METHOD(bool, containsAllProperties, (const std::string& passportString));
    MOCK_METHOD(bool, containsCorrectData, (std::string& passPortString));
};

class NiceMockPassportValidator : public NiceMock<MockPassportValidator>
{};
