import re
import ast
import copy

def split(word):
    return [char for char in word]

class FileReader:
    
    def __init__(self):
        self.bingoNumbers = ""
        self.intChartArray = []
    
    def readLinesToStringArray(self, inputFile):
        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()

        return fileString
    
    def readLinesToIntArray(self, inputFile):
        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()
        for i in range(0, len(fileString)):
            if fileString[i] == "":
                fileString[i] = 0
            fileString[i] = int(fileString[i])

        return fileString

    def readToIntArray(self, inputFile):
        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()
        intArray = list(map(int,fileString[0].split(",")))
        
        return intArray
    
    def readLinesToListArray(self, inputFile):
        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()

        listArray = []
        for line in fileString:
            listArray.append(ast.literal_eval(line))
        
        return listArray
    
    def readToIntMap(self, inputFile):
        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()
        integerMap = [list(map(int, list(line))) for line in fileString]
        
        return integerMap

    def readToStringMap(self, inputFile, removeEmpty =False):
        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()
        integerMap = [list(map(str, list(line))) for line in fileString]


        if removeEmpty == True:
            newIntegerMap = copy.deepcopy(integerMap)
            for i in range(0,len(integerMap)):
                for j in range(0, len(integerMap[i])):
                    if integerMap[i][j] == "" or integerMap[i][j] == " ":
                        del newIntegerMap[i][j]
            return newIntegerMap
        
        return integerMap
    
    def readPolymerTemplate(self, inputFile):
        fileString = self.readLinesToStringArray(inputFile)
        
        initalString = fileString[0]
        polymerPairs = []
        for line in fileString:
            if " -> " in line:
                string = line.split(" -> ")
                polymerPairs.append(string)
        
        data = [initalString, polymerPairs]
        return data
        
    def readBingoFile(self, inputFile):
        self.intChartArray = []

        with open (inputFile, "r") as myfile:
            dataread = myfile.read().rstrip()
        
        chartData = dataread.split('\n\n')
        chartbingoNumbers = chartData[0]
        self.bingoNumbers = list(map(int, chartbingoNumbers.split(",")))
        chartData.pop(0)

        for data in chartData:
            data = data.replace("\n"," ")
            data = data.replace("  "," ")
            array = re.split(' ',data)
            array = filter(None, array)
            self.intChartArray.append(list(map(int, array)))

        





            



