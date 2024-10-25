#include <gtest/gtest.h>

#include "SeatCalculator.h"


class SeatCalculatorTest : public ::testing::Test {
protected:
    SeatCalculator* m_seatCalculator;

SeatCalculatorTest()
    : m_seatCalculator()
    {

    }

// Here are some other boarding passes:

// BFFFBBFRRR: row 70, column 7, seat ID 567.
// FFFBBBFRRR: row 14, column 7, seat ID 119.
// BBFFBBFRLL: row 102, column 4, seat ID 820.
// FBFBBFFRLR row44 column5;

std::string firstSeatString = "BFFFBBFRRR";
std::string secondSeatString = "FFFBBBFRRR";
std::string thirdSeatString = "BBFFBBFRLL";
std::string fourthSeatString = "BBBBBBBRRR";
std::string fifthSeatString = "FBFBBFFRLR";
std::string wrongString = "BBBFBBBRRL";

std::vector<int> intVector = {1002,1231,939,223,214,932,234,123};
};

TEST_F(SeatCalculatorTest, wrongStringRow) {

    EXPECT_EQ(m_seatCalculator->calculateRow(wrongString),119);
}

TEST_F(SeatCalculatorTest, wrongStringColumn) {

    EXPECT_EQ(m_seatCalculator->calculateColumn(wrongString),6);
}

TEST_F(SeatCalculatorTest, wrongStringSeatId) {

    EXPECT_EQ(m_seatCalculator->calculateSeat(wrongString),958);
}

TEST_F(SeatCalculatorTest, testSeatRowString) {

    EXPECT_EQ(m_seatCalculator->calculateRow(firstSeatString),70);
}

TEST_F(SeatCalculatorTest, testSeatRowStringTwo) {
    
    EXPECT_EQ(m_seatCalculator->calculateRow(secondSeatString),14);

}

TEST_F(SeatCalculatorTest, testSeatRowStringThree) {

    EXPECT_EQ(m_seatCalculator->calculateRow(thirdSeatString),102);

}

TEST_F(SeatCalculatorTest, testSeatRowStringFour) {

    EXPECT_EQ(m_seatCalculator->calculateRow(fourthSeatString),127);

}

TEST_F(SeatCalculatorTest, testSeatRowStringFive) {

    EXPECT_EQ(m_seatCalculator->calculateRow(fifthSeatString),44);

}

TEST_F(SeatCalculatorTest, testSeatColumnString) {

    EXPECT_EQ(m_seatCalculator->calculateColumn(firstSeatString),7);

}

TEST_F(SeatCalculatorTest, testSeatColumn2String) {

    EXPECT_EQ(m_seatCalculator->calculateColumn(secondSeatString),7);

}

TEST_F(SeatCalculatorTest, testSeatColumn3String) {

    EXPECT_EQ(m_seatCalculator->calculateColumn(thirdSeatString),4);

}

TEST_F(SeatCalculatorTest, testSeatColumn5String) {

    EXPECT_EQ(m_seatCalculator->calculateColumn(fifthSeatString),5);
}

TEST_F(SeatCalculatorTest, testSeatString) {

    EXPECT_EQ(m_seatCalculator->calculateSeat(firstSeatString),567);

}

TEST_F(SeatCalculatorTest, testSeat2String) {

    EXPECT_EQ(m_seatCalculator->calculateSeat(secondSeatString),119);

}

TEST_F(SeatCalculatorTest, testSeat3String) {

    EXPECT_EQ(m_seatCalculator->calculateSeat(thirdSeatString),820);

}

TEST_F(SeatCalculatorTest, testSeat4String) {

    EXPECT_EQ(m_seatCalculator->calculateSeat(fourthSeatString),1023);
}

TEST_F(SeatCalculatorTest, testCalcMax) {

    EXPECT_EQ(m_seatCalculator->getMaxSeatId(intVector),1231);
}