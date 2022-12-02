from utils.AocUtils import *
from utils.FileReader import FileReader

import numpy as np
import copy
import math

class Polymers:
    
    def __init__(self):
        self.Polymer = []
        self.Connection = []
        self.nextGen = []
        self.Count = []
    
    def initPolymer(self, polyPairs):
        self.Polymer = []
        self.Connection = []
        self.nextGen = []
        self.Count = []
        
        for pair in polyPairs:
            self.Polymer.append(pair[0])
            self.Connection.append(pair[1])
            self.Count.append([pair[0],0])
            
            element1 = pair[0][0] + pair[1]
            element2 = pair[1] + pair[0][1]
            self.nextGen.append([element1, element2])

class Polymarizator:
    
    def __init__(self):
        self.polyString = ""
        self.polyPairs = []
        self.polymers = Polymers()
        self.length = 0
    
    def calculateDifference(self, initialString, polyPairs, steps):
        
        self.polymers.initPolymer(polyPairs)
        self.__initializeCounterList(initialString)
        
        for _ in range(steps):
            self.__updateToNextGeneration()
        
        singleCount = self.__getSingleCountArray()
        
        countArray = []
        for count in singleCount:
            countArray.append(count[1])
            
        maxValue = max(countArray)
        minValue = min(countArray)
        
        difference = maxValue - minValue
        return difference

    def __getSingleCountArray(self):
        array = np.array(self.polymers.Connection)
        insertChars = set(array)

        CountArray = []
        for character in insertChars:
            sumChar = 0
            for count in self.polymers.Count:
                number = count[0].count(character)
                if number > 0:
                    sumChar += number*count[1]
            
            sumChar = int((math.ceil(sumChar/2)))
            CountArray.append([character, sumChar])
            sumChar = 0     
        
        return CountArray
        
    def __updateToNextGeneration(self):
        polymerCurrent = copy.deepcopy(self.polymers)
        
        for index in range(0,len(polymerCurrent.Count)):
            if polymerCurrent.Count[index][1] > 0:
                nextgen1 = polymerCurrent.nextGen[index][0]
                nextgen2 = polymerCurrent.nextGen[index][1]
                polymer = polymerCurrent.Polymer[index]
                count = polymerCurrent.Count[index][1]
                self.__decreaseCounterArray(polymer, count)
                self.__increaseCounterArray(nextgen1 , polymerCurrent.Count[index][1])
                self.__increaseCounterArray(nextgen2 , polymerCurrent.Count[index][1])
    
    def __initializeCounterList(self, initialString):
        for count in self.polymers.Count:
            counts = initialString.count(count[0])
            if counts > 0:
                count[1] += counts
    
    def __increaseCounterArray(self, str, number = 1):
        for index in range(0, len(self.polymers.Count)):
            if self.polymers.Polymer[index] == str:
                self.polymers.Count[index][1] += number
    
    def __decreaseCounterArray(self, str, number = 1):
        for index in range(0, len(self.polymers.Count)):
            if self.polymers.Polymer[index] == str:
                self.polymers.Count[index][1] -= number

def solutionDay14():
    fileReader = FileReader()
    polymarizator = Polymarizator()
    
    data = fileReader.readPolymerTemplate("input/inputday14")
    initialString = data[0]
    polyPairs = data[1]
    stepsPart1 = 10
    stepsPart2 = 40
    
    answerPart1 = polymarizator.calculateDifference(initialString, polyPairs, stepsPart1)
    answerPart2 = polymarizator.calculateDifference(initialString, polyPairs, stepsPart2)
    printAnswer(14, answerPart1, answerPart2)