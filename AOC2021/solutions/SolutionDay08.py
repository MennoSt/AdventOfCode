from utils.AocUtils import *
from utils.FileReader import FileReader

class PatternReader:
    
    def __init__(self):
        self.uniquePatternArray = []
        self.fourDigitArray = []
    
    def readPattern(self, inputFile):

        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()

        uniquePatternArray = []
        fourDigitArray = []
        for file in fileString:
            splittedString = file.split("|")
            uniquePattern = splittedString[0].split(" ")
            uniquePattern = list(filter(None, uniquePattern))
            uniquePatternArray.append(uniquePattern)

            fourDigitString = splittedString[1].split(" ")
            fourDigitString = list(filter(None, fourDigitString))
            fourDigitArray.append(fourDigitString)

        self.uniquePatternArray = uniquePatternArray
        self.fourDigitArray = fourDigitArray

class PatternAnalyzer:
    def __init__(self):
        self.stringSeven = []
        self.stringFour = []
        self.stringOne = []

    def getNumberOfDigitInstances(self, digitArray):

        totalDigitSum = 0
        for pattern in digitArray:
            totalDigitSum += self.__calculateDigitSum(pattern)
            
        return totalDigitSum
    
    def calculateSumOfFourDigits(self, digitArrayAll, uniquePatternArrayAll):
        lengthData = len(digitArrayAll)
        totalDigitSum = 0

        for index in range(0,lengthData):
            self.__updateInitalStrings(uniquePatternArrayAll[index])
            totalDigitSum += self.__determineIntNumberTotal(digitArrayAll[index])
            
        return totalDigitSum

    def __calculateDigitSum(self, pattern):

        digitSum = 0
        intArray = [1,4,7,8]
        digitsArray = self.__convertIntToDigitLengthArray(intArray)
        
        for value in pattern:
            length = len(value)
            if length in digitsArray:
                digitSum += 1

        return digitSum 

    def __convertIntToDigitLengthArray(self, IntArray):
        
        digitsArray = []
        for value in IntArray:
            if value == 1 :
                digitsArray.append(2)
            if value == 4:
                digitsArray.append(4)
            if value == 7:
                digitsArray.append(3)
            if value == 8:
                digitsArray.append(7)
        
        return digitsArray
    
    def __determineIntNumberTotal(self, digitArray):
        result = ""

        for pattern in digitArray:
            result += str(self.__determineIntNumber(pattern))
        
        return int(result)
    
    def __updateInitalStrings(self,array):
        returnArray = [1,4,7,8]
        digitsArray = self.__convertIntToDigitLengthArray(returnArray)

        for element in array:
            length = len(element)
            if length in digitsArray:
                if length == 3 :
                    self.stringSeven = list(element)
                elif length == 4:
                    self.stringFour = list(element)
                elif length == 2:
                    self.stringOne = list(element)
                else:
                    Exception("length is not valid")

    def __determineIntNumber(self, string):
        
        returnArray = [1,4,7,8]
        digitsArray = self.__convertIntToDigitLengthArray(returnArray)
        length = len(string)
        number = None

        
        if length in digitsArray:
            if length == 7:
                number = 8
            elif length == 3 :
                number = 7
            elif length == 4:
                number = 4
            elif length == 2:
                number = 1
            else:
                Exception("length is not valid")
        
        # return 0,6 or 9 when the length is equal to six
        if length == 6:
            if all(char in string for char in self.stringFour):
                number = 9
            elif all(char in string for char in self.stringSeven):
                number = 0
            else:
                number = 6

        #return 2,3 or 5 when the length is equal to five
        if length == 5:
            if all(char in string for char in self.stringOne):
                number = 3
            elif len(list(set(string).intersection(self.stringFour))) == 3:
                number = 5
            else:
                number = 2
        
        return number      

def solutionDay08():
    patternReader = PatternReader()
    patternAnalyzer = PatternAnalyzer()
    patternReader.readPattern("input/inputday8")
    
    uniquePatternArray = patternReader.uniquePatternArray
    fourDigitArray = patternReader.fourDigitArray

    answerPart1 = patternAnalyzer.getNumberOfDigitInstances(fourDigitArray)
    answerPart2 = patternAnalyzer.calculateSumOfFourDigits(fourDigitArray, uniquePatternArray)
    printAnswer(8, answerPart1, answerPart2)