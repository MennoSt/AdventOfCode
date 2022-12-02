from utils.AocUtils import *
from utils.FileReader import FileReader

class CrabPositioner:
    
    def getLeastAmmountOfFuel(self, positionArray, increasedRate = False):

        fuelSumArray = []
        lowerLimit = min(positionArray)
        highLimit = max(positionArray)
        
        for sugestedPos in range(lowerLimit, highLimit):
            if increasedRate == True:
                fuelSum = self.__calculateFuelSum(positionArray, sugestedPos, True)
            else:
                fuelSum = self.__calculateFuelSum(positionArray, sugestedPos)
            fuelSumArray.append(fuelSum)
        
        minimumFuel = min(fuelSumArray)

        return minimumFuel
    
    def __calculateFuelSum(self, positionArray, suggestedPos, increasedRate = False):
        fuelSum = 0
        for horPostion in positionArray:
            if increasedRate == False:
                fuelSum += self.__calculateFuel(horPostion, suggestedPos)
            else:
                fuelSum += self.__calculateIncreasedFuel(horPostion, suggestedPos)
        
        return fuelSum

    def __calculateFuel(self, horPos, suggestedPos):
        return abs(horPos-suggestedPos)
    
    def __calculateIncreasedFuel(self, horPos, sugestedPos):
        difference = abs(horPos - sugestedPos)
        fuel = self.__recursiveFunction(difference)
        return fuel
    
    def __recursiveFunction(self,x):
        value = int(x*(x+1)/2)
        return value

def solutionDay07():
    
    fileReader = FileReader()
    intArray = fileReader.readToIntArray("input/inputday7")
    crabPositioner = CrabPositioner()

    answerPart1 = crabPositioner.getLeastAmmountOfFuel(intArray)
    answerPart2 = crabPositioner.getLeastAmmountOfFuel(intArray, True)
    
    printAnswer(7, answerPart1, answerPart2)