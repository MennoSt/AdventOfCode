from utils.AocUtils import *
from utils.FileReader import *

class CalorieCalculator:
    
    def sumNBiggestEaters(self, inputCals, n):
        elfCalories = self.__calcElfCalorieArray(inputCals)
        return sum(elfCalories[:n])

    def __calcElfCalorieArray(self, inputCals):
        tmpVal = 0
        calorieArray = []
        
        for cal in inputCals:
            tmpVal += cal
            if cal == 0:
                calorieArray.append(tmpVal)
                tmpVal = 0
              
        calorieArray.append(tmpVal)
        calorieArray.sort(reverse=True)
        return calorieArray
                
        
def solutionDay01():
    
    fileReader = FileReader()
    calorieCalculator = CalorieCalculator()
    inputArray = fileReader.readLinesToIntArray("input/inputday1")
    answer1 = calorieCalculator.sumNBiggestEaters(inputArray, 1)
    answer2 = calorieCalculator.sumNBiggestEaters(inputArray, 3)
    printAnswer(1, answer1, answer2)
    