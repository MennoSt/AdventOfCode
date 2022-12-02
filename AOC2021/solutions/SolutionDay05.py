from utils.AocUtils import *

class VentDetector:
    
    def __init__(self ):
        self.ventCoordinates = []
        self.indexArray =[]

    def readVentFile(self, file):
        fileObj = open(file, "r")
        array = fileObj.read().splitlines()
        fileObj.close()

        for item in array:
            item = item.replace(" -> ",",")
            item = item.split(",")
            self.ventCoordinates.append(item)

    def CalculateIndex(self, x, y):
        mapWidth = 1000
        index = x + mapWidth* y
        return index

    def AddVent(self, x1, y1, x2, y2):
        if x1 == x2:
            if (y2 > y1):
                for it in range(y1, y2+1):
                    array = [x1,it]
                    self.indexArray.append(array)
            if (y1 > y2):
                for it in range(y2, y1+1):
                    array = [x1,it]
                    self.indexArray.append(array)
        if y1 == y2:
            if (x2 > x1):
                for it in range(x1, x2+1):
                    array = [it,y1]
                    self.indexArray.append(array)
            if (x1 > x2):
                for it in range(x2, x1+1):
                    array = [it,y1]
                    self.indexArray.append(array)

            
    def getNumberOfOverlapsPart1(self):
        for coordinate in self.ventCoordinates:
            self.AddVent(int(coordinate[0]), int(coordinate[1]), int(coordinate[2]), int(coordinate[3]))
        
        indexes = []
        for index in self.indexArray:
            indexes.append(self.CalculateIndex(index[0], index[1]))

        numberOfOverlaps = 0
        uniqueNumbers = list(set(indexes))

        for number in uniqueNumbers:
            if indexes.count(number) > 1:
                numberOfOverlaps+=1
        
        return numberOfOverlaps
    
    def AddVentPart2(self, x1, y1, x2, y2):
        x1 = int(x1)
        y1 = int(y1)
        x2 = int(x2)
        y2 = int(y2)

        if x1 == x2:
            if (y2 > y1):
                for it in range(y1, y2+1):
                    array = [x1,it]
                    self.indexArray.append(array)
            if (y1 > y2):
                for it in range(y2, y1+1):
                    array = [x1,it]
                    self.indexArray.append(array)

        if y1 == y2:
            if (x2 > x1):
                for it in range(x1, x2+1):
                    array = [it,y1]
                    self.indexArray.append(array)
            if (x1 > x2):
                for it in range(x2, x1+1):
                    array = [it,y1]
                    self.indexArray.append(array)

        if x1 != x2 and y1 != y2:
            if x1 > x2 and y1 > y2:
                ydif = abs(y1-y2)
                for it in range(0, ydif+1):
                    array = [x1-it,y1-it]
                    self.indexArray.append(array) 
            if x1 > x2 and y1 < y2:
                ydif = abs(y1-y2)
                for it in range(0, ydif+1):
                    array = [x1-it,y1+it]
                    self.indexArray.append(array)
            if x1 < x2 and y1 > y2:
                ydif = abs(y1-y2)
                for it in range(0, ydif+1):
                    array = [x1+it,y1-it]
                    self.indexArray.append(array)
            if x1 < x2 and y1 < y2:
                ydif = abs(y1-y2)
                for it in range(0, ydif+1):
                    array = [x1+it,y1+it]
                    self.indexArray.append(array)

    def getNumberOfOverlapsPart2(self):
        for coordinate in self.ventCoordinates:
            self.AddVentPart2(coordinate[0], coordinate[1], coordinate[2], coordinate[3])
        
        indexes = []
        for index in self.indexArray:
            indexes.append(self.CalculateIndex(index[0], index[1]))

        numberOfOverlaps = 0
        uniqueNumbers = list(set(indexes))

        for number in uniqueNumbers:
            if indexes.count(number) > 1:
                numberOfOverlaps+=1
        
        return numberOfOverlaps

def solutionDay05():
    ventDetector = VentDetector()
    ventDetector.readVentFile("input/inputday5")

    answerPart1 = ventDetector.getNumberOfOverlapsPart1()
    answerPart2 = ventDetector.getNumberOfOverlapsPart2()
    
    printAnswer(5, answerPart1, answerPart2)