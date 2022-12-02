from unittest.result import TestResult
from utils.FileReader import FileReader
from utils.AocUtils import *


class BingoChart:
    def __init__(self, bingoChart):
        self.bingoChart = bingoChart
        self.horizontalSums = []
        self.verticalSums = []

    def getBingoScore(self, lastNumber):
        array = []
        for number in self.bingoChart:
            if number != -1:
                array.append(number)

        sumChart = sum(array)
        score = sumChart * lastNumber

        return score

    def strikeThroughNumber(self, number):
        for index in range(0, len(self.bingoChart)):
            if self.bingoChart[index] == number:
                self.bingoChart[index] = -1

        self.__updateSums()

    def hasBingo(self):
        for index in range(0, len(self.horizontalSums)):
            if(self.horizontalSums[index] == -5):
                return True

        for index in range(0, len(self.verticalSums)):
            if(self.verticalSums[index] == -5):
                return True

        return False

    def __updateSums(self):
        self.horizontalSums = []
        self.verticalSums = []
        bingochart = self.bingoChart

        for index in range(0, 5):
            addition = index * 5
            sumH = 0
            for i in range(0, 5):
                sumH += bingochart[addition + i]
            self.horizontalSums.append(sumH)

        for index in range(0, 5):
            sumV = 0
            for i in range(0, 25, 5):
                sumV += bingochart[index + i]
            self.verticalSums.append(sumV)


class BingoManager:
    def __init__(self):
        self.bingoChartArray = [BingoChart]
        self.sumChart = 0
        self.lastNumber = 0

    def createBingoCharts(self, chartData):
        self.bingoChartArray = []
        for chart in chartData:
            bingochart = BingoChart(chart)
            self.bingoChartArray.append(bingochart)

    def getFirstBingoScore(self, numbers):
        bingo = False
        for number in numbers:
            for index in range(0, len(self.bingoChartArray)):
                if not bingo:
                    self.bingoChartArray[index].strikeThroughNumber(number)
                if self.bingoChartArray[index].hasBingo() and not bingo:
                    bingoScore = self.bingoChartArray[index].getBingoScore(
                        number)
                    self.lastNumber = number
                    bingo = True

        return bingoScore

    def getLastBingoScore(self, numbers):
        lastBingoScore = 0
        indexesToPop = []

        for number in numbers:
            leftBingoCharts = self.bingoChartArray
            for index in range(0, len(leftBingoCharts)):
                length = len(leftBingoCharts)
                if length == 1:
                    index = 0
                leftBingoCharts[index].strikeThroughNumber(number)
                if leftBingoCharts[index].hasBingo() is True:
                    lastBingoScore = leftBingoCharts[index].getBingoScore(
                        number)
                    indexesToPop.append(index)
            for index in sorted(indexesToPop, reverse=True):
                del self.bingoChartArray[index]
                indexesToPop = []

        return lastBingoScore


def solutionDay04():
    bingoFileReader = FileReader()
    bingoFileReader.readBingoFile("input/inputday4")
    bingoManager = BingoManager()
    bingoNumbers = bingoFileReader.bingoNumbers
    intChartArray = bingoFileReader.intChartArray
    bingoManager.createBingoCharts(intChartArray)

    answerPart1 = bingoManager.getFirstBingoScore(bingoNumbers)
    answerPart2 = bingoManager.getLastBingoScore(bingoNumbers)

    printAnswer(4, answerPart1, answerPart2)
