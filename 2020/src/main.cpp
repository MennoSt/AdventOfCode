#include "SeatCalculator.h"
#include "SolutionHandler.h"
#include "SpecialOperator.h"
#include "PassportValidator.h"
#include <iostream>

#include "Utils.h"
/*
to do:
- Do debug and release builds in separate folders
- Look into cross compiling
- Syslog Addition?
- Clang include fore code styling
*/

static void executeAllSolutions(SolutionHandler& solutionHandler)
{
    solutionHandler.solutionDayOne();
    solutionHandler.solutionDayTwo();
    solutionHandler.solutionDayThree();
    solutionHandler.solutionDayFour();
    solutionHandler.solutionDayFive();
    solutionHandler.solutionDaySix();
}

int main() {

    SpecialOperator *specialOperator;
    PassportValidator *passportValidator;
    SeatCalculator seatCalculator;
    SolutionHandler solutionHandler(specialOperator, passportValidator, seatCalculator);
    solutionHandler.solutionDaySix();
    return 0;
}
