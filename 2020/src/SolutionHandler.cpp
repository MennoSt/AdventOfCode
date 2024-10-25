#include "SolutionHandler.h"

#include "DeclarationFormCalculator.h"
#include "PassportValidator.h"
#include "SeatCalculator.h"
#include "SpecialOperator.h"
#include "Utils.h"

const std::string c_filePathDay1 = "input/input1.txt";


SolutionHandler::SolutionHandler(SpecialOperator* specialOperator, PassportValidator* passportValidator, SeatCalculator& seatCalculator)
    : m_specialOperator(specialOperator)
    , m_passportValidator(passportValidator)
    , m_seatCalculator(seatCalculator)
{

}

void SolutionHandler::solutionDayOne ()
{
    std::vector<int> intVector;

    std::string filename(c_filePathDay1);
    int number;

    std::ifstream input_file(filename);
    if (!input_file.is_open()) {
        std::cerr << "Could not open the file - '"
             << filename << "'" << std::endl;
    }

    while (input_file >> number) {
        intVector.push_back(number);
    }

    input_file.close();

    std::cout << "Solution Day1:" << std::endl;
    int year = 2020;
    for (int i = 0; i < intVector.size(); i++) {
        for (int j = 0; j < intVector.size(); j++) {
            int sum = intVector[i] + intVector[j];
            if (sum == year) {
                int multipliedValue = intVector[i] * intVector[j];
                std::cout << "first value is: " + std::to_string(intVector[i]) << std::endl;
                std::cout << "second value is: " + std::to_string(intVector[j]) << std::endl;
                std::cout << "sum is equal to:" + std::to_string(sum) << std::endl;
                std::cout << "multiplied value is: " << std::to_string(multipliedValue) << std::endl;
                std::cout << std::endl;
                return;
            }
        }
    }
}

void SolutionHandler::solutionDayTwo()
{
    std::cout << "Solution Day2:" << std::endl; 
    std::vector<std::string> stringVector = getStringVector("input/input2.txt");

    int correctCounterPart1 = 0;
    int wrongCounterPart1 = 0;
    for (std::string password:stringVector) {
        if (m_specialOperator->isCorrectPasswordPart1(password))
            correctCounterPart1++;
        else
            wrongCounterPart1++;
    }
    std::cout << "Part1:" << std::endl; 
    std::cout << "number of correct passwords is:" + std::to_string(correctCounterPart1) << std::endl;
    std::cout << "number of wrong passwords is:" + std::to_string(wrongCounterPart1) << std::endl;
    std::cout << "total ammount of passwords is:" + std::to_string(stringVector.size()) << std::endl;
    std::cout << std::endl;

    int correctCounterPart2 = 0;
    int wrongCounterPart2 = 0;
    for (std::string password:stringVector) {
        if (m_specialOperator->isCorrectPasswordPart2(password))
            correctCounterPart2++;
        else
            wrongCounterPart2++;
    }
    std::cout << "Part2:" << std::endl;
    std::cout << "number of correct passwords is:" + std::to_string(correctCounterPart2) << std::endl;
    std::cout << "number of wrong passwords is:" + std::to_string(wrongCounterPart2) << std::endl;
    std::cout << "total ammount of passwords is:" + std::to_string(stringVector.size()) << std::endl;
    std::cout << std::endl;
}

void SolutionHandler::solutionDayThree()
{
    std::cout << "Solution Day3: " << std::endl;
    std::vector<std::string> stringVector = getStringVector("input/input3.txt");
    int counter = m_specialOperator->getNumberOfTrees(stringVector,3,1);

    std::cout << "Part1: " + std::to_string(stringVector.size())<< std::endl;
    std::cout << "number of trees is equal to " + std::to_string(counter)<< std::endl;


    unsigned long multipliedCounter = m_specialOperator->getNumberOfTrees(stringVector,1,1)*
                                      m_specialOperator->getNumberOfTrees(stringVector,3,1)*
                                      m_specialOperator->getNumberOfTrees(stringVector,5,1)*
                                      m_specialOperator->getNumberOfTrees(stringVector,7,1)*
                                      m_specialOperator->getNumberOfTrees(stringVector,1,2);

    std::cout << "multiplied of trees is equal to " + std::to_string(multipliedCounter)<< std::endl;
    std::cout << std::endl;
}

void SolutionHandler::solutionDayFour()
{
    std::cout << "Solution Day4: " << std::endl;
    std::cout << "Part1: " << std::endl;
    std::cout << "number of valid passports:" + std::to_string(m_passportValidator->getNumberOfValidPassportsPart1("input/input4.txt")) <<std::endl;

    std::cout << "Part2: " << std::endl;
    std::cout << "number of valid passports:" + std::to_string(m_passportValidator->getNumberOfValidPassportsPart2("input/input4.txt")) <<std::endl;
}

void SolutionHandler::solutionDayFive()
{
    std::vector<std::string> seatVector = getStringVector("input/input5.txt");
    std::vector<int> seatNumbers;

    for (std::string seat:seatVector) {
        seatNumbers.push_back(m_seatCalculator.calculateSeat(seat));
    }


    std::cout << "Solution Day5: " << std::endl;

    std::cout << "Part1: " << std::endl;
    std::cout << "Max SeatNumber is:" << m_seatCalculator.getMaxSeatId(seatNumbers) << std::endl;

    std::sort(seatNumbers.begin(), seatNumbers.end());


    for (int i =27; i < 963; i++){
        if (std::count(seatNumbers.begin(), seatNumbers.end(), i)) {
        }
        else {
            std::cout << "Empty seat is equal to: " << i << std::endl;
        }
    }
}

void SolutionHandler::solutionDaySix()
{
    std::vector<std::string> declarationVector = getStringVector("input/input6.txt");
    DeclarationFormCalculator declarationFormCalculator;
    int sumAnyone = declarationFormCalculator.calculateSumYesAnyone(declarationVector);

    std::cout << "Solution Day6: " << std::endl;
    std::cout << "Part 1: " << std::endl;
    std::cout << "Sum is equal to: " << sumAnyone << std::endl;

    int sumEveyone = declarationFormCalculator.calculateSumYesEveryone(declarationVector);

    std::cout << "Part 2: " << std::endl;
    std::cout << "Sum is equal to: " << sumEveyone << std::endl;


} 