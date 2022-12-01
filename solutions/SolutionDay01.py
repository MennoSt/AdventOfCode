from utils.AocUtils import *
from utils.FileReader import *

class CalorieCalculator:
    
    def __init__(self):
        self.calArray = []
        
    def sumNBiggestEaters(self, inputCals, n):
        calArray = self.__calculateElfCalories(inputCals)
        return sum(calArray[:n])

    def __calculateElfCalories(self, inputCals):
        tmpVal = 0
        calArray = []
        
        for cal in inputCals:
            tmpVal += cal
            if cal == 0:
                calArray.append(tmpVal)
                tmpVal = 0
              
        calArray.append(tmpVal)
        calArray.sort(reverse=True)
        return calArray
                
        
def solutionDay01():
    
    fileReader = FileReader()
    calorieCalculator = CalorieCalculator()
    inputArray = fileReader.readLinesToIntArray("input/inputday1")
    answer1 = calorieCalculator.sumNBiggestEaters(inputArray, 1)
    answer2 = calorieCalculator.sumNBiggestEaters(inputArray, 3)
    printAnswer(1, answer1, answer2)
    