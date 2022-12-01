
from utils.AocUtils import *
from utils.FileReader import *


    
class CalorieCalculator:
    def calculateMaxCalories(self, inputCals):
        maxCal = 0
        tmpVal = 0
        
        for cal in inputCals:
            tmpVal += cal
            if cal == 0:
                if tmpVal > maxCal:
                    maxCal = tmpVal
                tmpVal = 0
                
        return maxCal
                
        
def solutionDay01():
    
    fileReader = FileReader()
    calorieCalculator = CalorieCalculator()
    inputArray = fileReader.readLinesToIntArray("input/inputday1")
    answer = calorieCalculator.calculateMaxCalories(inputArray)
    print(answer)
    
    
    # CalorieCalculator.calculateCaloriesElf(inputArray)
    
    # printAnswer(1, answerPart1, answerPart2)
    
solutionDay01()