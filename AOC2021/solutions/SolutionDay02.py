import pandas as pd

from utils.AocUtils import *

class PositionCalculator:
    def __init__(self):
        self.horizontalP = 0
        self.depthP = 0
        self.aim = 0

    def getMultiplicationPart1(self, directionArray, positionArray):

        self.__updatePosition(directionArray, positionArray)
        multiplication = self.horizontalP * self.depthP

        return multiplication

    def getMultiplicationPart2(self, directionArray, positionArray):

        self.__updatePositionPart2(directionArray, positionArray)
        multiplication = self.horizontalP * self.depthP

        return multiplication

    def __updatePosition(self, directionArray, positionArray):
        self.horizontalP = 0
        self.depthP = 0
        length_dir = len(directionArray)
        length_pos = len(positionArray)

        if (length_dir != length_pos):
            raise Exception("Length of arrays is not equal")

        for index in range(0, length_dir):
            if (directionArray[index] == "up"):
                self.depthP -= positionArray[index]
            elif (directionArray[index] == "down"):
                self.depthP += positionArray[index]
            elif (directionArray[index] == "forward"):
                self.horizontalP += positionArray[index]
            else:
                raise Exception("Invalid Direction")

    def __updatePositionPart2(self, directionArray, positionArray):
        length_dir = len(directionArray)
        length_pos = len(positionArray)
        self.horizontalP = 0
        self.depthP = 0
        self.aim = 0

        if (length_dir != length_pos):
            raise Exception("Length of arrays is not equal")
        
        for index in range(0, length_dir):
            if (directionArray[index] == "up"):
                self.aim -= positionArray[index]
            elif (directionArray[index] == "down"):
                self.aim += positionArray[index]
            elif (directionArray[index] == "forward"):
                self.horizontalP += positionArray[index]
                self.depthP += self.aim * positionArray[index]
            else:
                raise Exception("Invalid Direction")


def solutionDay02():
    positionCalculator = PositionCalculator()
    data = pd.read_csv(
        "input/inputday2", names=["Direction", "Ammount"], delim_whitespace=True)

    answerPart1 = positionCalculator.getMultiplicationPart1(
        data["Direction"].astype(str).values.tolist(),
        data["Ammount"].to_numpy())
    answerPart2 = positionCalculator.getMultiplicationPart2(
        data["Direction"].astype(str).values.tolist(),
        data["Ammount"].to_numpy())
    printAnswer(2, answerPart1, answerPart2)
