from utils.AocUtils import *

from utils.Grid import Grid
import numpy as np

class FoldReader:
    
    def __init__(self):
        self.coordinatesArray = []
        self.foldArray = []

    def readToFoldArray(self, inputFile):
        fileObj = open(inputFile, "r")
        fileString = fileObj.read().splitlines()
        fileObj.close()
        
        for line in fileString:
            if "fold along" in line:
                string = line.replace("fold along ", "")
                string = string.split("=")
                string[1] = int(string[1])
                self.foldArray.append(string)
            elif line != "":
                intValue = list(map(int,line.split(",")))
                self.coordinatesArray.append(intValue)
                
class Folder:
    def __init__(self):
        self.grid = Grid()
        self.coordinates = []
        self.foldings = []

    def importInputFiles(self, inputFile):

        foldreader = FoldReader()
        foldreader.readToFoldArray(inputFile)
        
        self.coordinates = foldreader.coordinatesArray
        self.foldings = foldreader.foldArray
        
        array = np.array(self.coordinates)
        max = np.max(array,axis=0)
        w = max[0]
        h = max[1]
        FoldMatrix = [["." for x in range(w+1)] for y in range (h+1)]
        self.grid.setGrid(FoldMatrix)
        
        self.__setDotsOnGrid()
    
    def CalculateNumberOfDots(self, onlyFirstFold = False):
    
        if onlyFirstFold == True:
            self.__foldPaper(self.foldings[0])
        else:
            for fold in self.foldings:
                self.__foldPaper(fold)
        
        
        dots = self.__GetNumberOfVisibleDots()
        
        # Letters can be printed in the grid for the answer
        # if not onlyFirstFold:
        #     self.printLetters()
        
        
        return dots
        
    def printLetters(self):
        
        array = np.array(self.grid.gridMap)
            
        for i in range(0,7):
            print(array[i][:10])
        
        for i in range(0,7):
            print(array[i][10:20])
        
        for i in range(0,7):
            print(array[i][20:30])
        
        for i in range(0,7):
            print(array[i][30:40])
            
    def __foldPaper(self,fold):
        self.__drawFoldLine(fold)
        foldAxis = fold[0]
        axisValue = fold[1]
        
        if foldAxis == "x":
            for y in range(0, self.grid.mapHeight):
                for x in range(axisValue, self.grid.mapWidth):
                    xnew = 2*axisValue-x
                    if self.grid.getValue(x, y) == "#":
                        self.grid.setValue(xnew, y, "#")
                        self.grid.setValue(x, y, 0)
            self.grid.mapWidth = axisValue
        elif foldAxis == "y":
            for y in range(axisValue, self.grid.mapHeight):
                for x in range(0, self.grid.mapWidth):
                    ynew = 2*axisValue-y
                    if self.grid.getValue(x, y) == "#":
                        self.grid.setValue(x, ynew, "#")
                        self.grid.setValue(x, y, 0)
            self.grid.mapHeight = axisValue
        else:
            Exception("wrong fold order")
    
    def __drawFoldLine(self, fold):
        if fold[0] == "x":
            for x in range(0, self.grid.mapHeight):
                self.grid.setValue(fold[1], x, "-")
        elif fold[0] == "y":
            for y in range(0, self.grid.mapWidth):
                self.grid.setValue(y, fold[1], "-")
                
    def __setDotsOnGrid(self):
        for coordinate in self.coordinates:
            self.grid.setValue(coordinate[0],coordinate[1],"#")
    
    def __GetNumberOfVisibleDots(self):
        visibleDots = 0
        for line in self.grid.gridMap:
            count = line.count('#')
            visibleDots+=count
        
        return visibleDots

def solutionDay13():
    folder = Folder()
    
    folder.importInputFiles("input/inputday13")
    answerPart1 = folder.CalculateNumberOfDots(True)
    printAnswer(13, answerPart1, "PFKLKCFP")
    folder.CalculateNumberOfDots()