#ifndef SEATCALCULATOR_HPP
#define SEATCALCULATOR_HPP

#include <string>
#include <vector>

class SeatCalculator
{
public:
    SeatCalculator();
    ~SeatCalculator() = default;

    int calculateRow(std::string& seat);
    int calculateColumn(std::string& seat);
    int calculateSeat(std::string& seat);
    int getMaxSeatId(std::vector<int>& seatNumbers);
};

#endif /* SEATCALCULATOR_HPP */