#include "SeatCalculator.h"
#include <boost/algorithm/string.hpp>
#include "Utils.h"
#include <cmath>

SeatCalculator::SeatCalculator()
{

}

/*
For example, consider just the first seven characters of FBFBBFFRLR:

Start by considering the whole range, rows 0 through 127.
F means to take the lower half, keeping rows 0 through 63.
B means to take the upper half, keeping rows 32 through 63.
F means to take the lower half, keeping rows 32 through 47.
B means to take the upper half, keeping rows 40 through 47.
B keeps rows 44 through 47.
F keeps rows 44 through 45.
The final F keeps the lower of the two, row 44.
*/

int SeatCalculator::calculateRow(std::string& seat)
{
    std::string fString = "B";
    char B = fString[0];

    int maxRow = 7;
    int row = 0;

    if (seat[0] == B)
        row = 64;

    for (int x =1; x < maxRow; x++) {
        if (seat[x] == B)
            row += 64/pow(2,x);
    }

    return row;
}

/*
For example, consider just the last 3 characters of FBFBBFFRLR:

Start by considering the whole range, columns 0 through 7.
R means to take the upper half, keeping columns 4 through 7.
L means to take the lower half, keeping columns 4 through 5.
The final R keeps the upper of the two, column 5.
*/

int SeatCalculator::calculateColumn(std::string& seat)
{
    std::string rString = "R";
    char R = rString[0];

    int maxRow =10;
    int column = 0;
    int powerTemp =0;

    if (seat[7] == R)
        column += 4;
    
    if (seat[8] == R)
        column +=2;
    
    if (seat[9] == R)
        column +=1;

    return column;
}

// Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.

int SeatCalculator::calculateSeat(std::string& seat)
{
    int column = calculateColumn(seat);
    int row = calculateRow(seat);

    return row*8+column;
}

int SeatCalculator::getMaxSeatId(std::vector<int>& seatNumbers)
{
    int max = *std::max_element(seatNumbers.begin(),seatNumbers.end());
    return max;
}