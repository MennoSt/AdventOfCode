import re
import ast

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

    def readToStringMap(self, inputFile):
        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()
        integerMap = [list(map(str, list(line))) for line in fileString]
        
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

        





            



