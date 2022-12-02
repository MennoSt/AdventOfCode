import pandas as pd

from utils.AocUtils import *


class SubmarineCalculator:
    def GetIncreasedDepthsPart1(self, data):
        increases = 0
        measureResult = self.__IsDepthIncreasedArray(data)

        for data in measureResult:
            if data is True:
                increases += 1

        return increases

    def GetIncreasedDepthsPart2(self, data):

        array = self.__getSummedDepth(data)
        numberOfIncreasedSummedDepths = self.GetIncreasedDepthsPart1(array)

        return numberOfIncreasedSummedDepths

    def __IsDepthIncreased(self, depth, previousDepth):
        if (depth < previousDepth):
            return True
        if (depth > previousDepth):
            return False

    def __IsDepthIncreasedArray(self, data):
        array = []

        for index in range(0, len(data)):
            if index == 0:
                array.append("No previous measurement")
            else:
                array.append(
                    self.__IsDepthIncreased(data[index - 1], data[index]))

        return array

    def __getSummedDepth(self, data):
        array = []
        summedDepth = 0

        for index in range(0, len(data) - 2):
            summedDepth = data[index] + data[index + 1] + data[index + 2]
            array.append(summedDepth)

        return array


def solutionDay01():
    submarineCalculator = SubmarineCalculator()
    data = pd.read_csv("input/inputday1", names=["Depth"])
    data['Depth'] = data['Depth'].astype(int)

    answerPart1 = submarineCalculator.GetIncreasedDepthsPart1(
        data["Depth"].to_numpy())
    answerPart2 = submarineCalculator.GetIncreasedDepthsPart2(
        data['Depth'].to_numpy())

    printAnswer(1, answerPart1, answerPart2)
