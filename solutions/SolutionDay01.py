from utils.AocUtils import *
from utils.FileReader import *

class CalorieCalculator:
    
    def __init__(self):
        self.calArray = []
        
    def sumNBiggestEaters(self, inputCals, n):
        self.calculateMaxCalories(inputCals)
        self.calArray.sort(reverse=True)
        return sum(self.calArray[:n])

    def calculateMaxCalories(self, inputCals):
        maxCal = 0
        tmpVal = 0
        self.calArray = []
        
        for cal in inputCals:
            tmpVal += cal
            if cal == 0:
                self.calArray.append(tmpVal)
                if tmpVal > maxCal:
                    maxCal = tmpVal
                tmpVal = 0
              
        self.calArray.append(tmpVal)  
        if tmpVal > maxCal:
            maxCal = tmpVal
                    
        return maxCal
                
        
def solutionDay01():
    
    fileReader = FileReader()
    calorieCalculator = CalorieCalculator()
    inputArray = fileReader.readLinesToIntArray("input/inputday1")
    answer1 = calorieCalculator.sumNBiggestEaters(inputArray, 1)
    answer2 = calorieCalculator.sumNBiggestEaters(inputArray, 3)
    printAnswer(1, answer1, answer2)
    