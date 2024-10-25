#ifndef SOLUTIONHANDLER_HPP
#define SOLUTIONHANDLER_HPP

#include <string>
#include <vector>

class SpecialOperator;
class PassportValidator;
class SeatCalculator;

class SolutionHandler
{
public:
    SolutionHandler(SpecialOperator* specialOperator, PassportValidator* passportValidator, SeatCalculator& seatCalculator);
    ~SolutionHandler() = default;

    void solutionDayOne();
    void solutionDayTwo();
    void solutionDayThree();
    void solutionDayFour();
    void solutionDayFive();
    void solutionDaySix();


private:
    SpecialOperator* m_specialOperator;
    PassportValidator* m_passportValidator;
    SeatCalculator& m_seatCalculator;
};

#endif /* SOLUTIONHANDLER_HPP */

