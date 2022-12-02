from utils.AocUtils import *
from utils.FileReader import FileReader

class LaunterFishCalculator:
    
    def __init__(self):
        self.fishArray = []
        self.initialState = []

    def getNumberOfFishes(self, initialstate, days):
        self.fishArray = self.__convertArrayToCounterArray(initialstate)
        self.__updateToNextGeneration()

        for _ in range(days-1):
            self.__updateToNextGeneration()
    
        sumFishes = sum(self.fishArray)

        return sumFishes
    
    def __convertArrayToCounterArray(self, initialState):
        countArray = []
        for i in range(0,8):
            countArray.append(initialState.count(i))
        
        for _ in range(2):
            countArray.append(0)

        return countArray

    def __updateToNextGeneration(self):
        
        updatedArray = []
        additionalEights = self.fishArray[0]
        additionalSixes = self.fishArray[0] + self.fishArray[7]

        for i in range(1,7):
            updatedArray.append(self.fishArray[i])
            
        updatedArray.append(additionalSixes)
        updatedArray.append(self.fishArray[8])
        updatedArray.append(additionalEights)

        self.fishArray = updatedArray

def solutionDay06():
    fileReader = FileReader()
    initalState = fileReader.readToIntArray("input/inputday6")
    launterFishCalulator = LaunterFishCalculator()

    answerPart1 = launterFishCalulator.getNumberOfFishes(initalState, 80)
    answerPart2 = launterFishCalulator.getNumberOfFishes(initalState, 256)
    
    printAnswer(6, answerPart1, answerPart2)