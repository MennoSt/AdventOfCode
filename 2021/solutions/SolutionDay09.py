from utils.AocUtils import *
from utils.FileReader import FileReader

from utils.AocEnums import *
from utils.Operations import *
from utils.Grid import Grid

class RiskCalculator():
    
    def __init__(self):
        self.grid = Grid()

    def setHeightMap(self, heightMap):
        self.grid.setGrid(heightMap)
    
    def __calcualteRiskLevel(self, height):
        return height+1

    def calculateSumRiskLevels(self):

        riskSum = 0
        for x in range (0, self.grid.mapWidth):
            for y in range(0, self.grid.mapHeight):
                if self.__isRiskSpot(x,y):
                    height = self.grid.getHeightValue(x,y)
                    riskLevel = self.__calcualteRiskLevel(height)
                    riskSum += riskLevel
        
        return riskSum

    def __isHeightGreater(self, x,y, direction):
        heightDirection = self.grid.getHeightValue(x,y, direction)
        heightCurrent = self.grid.getHeightValue(x,y, Direction.CURRENT)
        
        if heightDirection > heightCurrent:
            return True
        else:
            return False

    def __isRiskSpot(self, x, y):
        riskSpot = False
        isUpGreater = self.__isHeightGreater(x, y, Direction.UP)
        isDownGreater = self.__isHeightGreater(x, y, Direction.DOWN)
        isLeftGreater = self.__isHeightGreater(x, y, Direction.LEFT)
        isRightGreater = self.__isHeightGreater(x, y, Direction.RIGHT)

        if isUpGreater and isDownGreater and isLeftGreater and isRightGreater:
            riskSpot = True

        return riskSpot

    def __toListIndex(self,x,y):
        listIndex = x+y*self.grid.mapWidth
        return listIndex

    def createInitalBassin(self,x,y):
        bassin = []
        x_init = x
        y_init = y
        
        if self.grid.getHeightValue(x,y) == 9:
            return bassin
        else:
            bassin.append(self.__toListIndex(x,y))

        x = x_init
        for _ in range(self.grid.mapWidth):
            x+=1
            height = self.grid.getHeightValue(x,y)
            if height != 9:
                bassin.append(self.__toListIndex(x,y))
            else:
                break
        
        x = x_init
        for _ in range(self.grid.mapWidth):
            x-=1
            height = self.grid.getHeightValue(x,y)
            if height != 9:
                bassin.append(self.__toListIndex(x,y))
            else:
                break
        
        x = x_init
        y = y_init
        for _ in range(self.grid.mapHeight):
            y+=1
            height = self.grid.getHeightValue(x,y)
            if height != 9:
                bassin.append(self.__toListIndex(x,y))
            else:
                break
        
        y = y_init
        for _ in range(self.grid.mapHeight):
            y-=1
            height = self.grid.getHeightValue(x,y)
            if height != 9:
                bassin.append(self.__toListIndex(x,y))
            else:
                break
        
        return bassin

    def createInitialBassinsPerIndex(self):
        bassinsPerIndex = []
        for y in range(0, self.grid.mapHeight):
            for x in range (0, self.grid.mapWidth):
                bassinsPerIndex.append(self.createInitalBassin(x,y))
        
        bassinsPerIndex = [x for x in bassinsPerIndex if x != []]
        
        return bassinsPerIndex

    def __getLengthArray(self, bassinArray):
        bassinSizeArray =[]
        for array in bassinArray:
            bassinSizeArray.append(len(array))
        
        return bassinSizeArray

    def getMultiplicationLargest3Bassins(self):
        bassins = self.createInitialBassinsPerIndex()
        
        mergedbassins = list(merge_common(bassins))
        sizeArray = self.__getLengthArray(mergedbassins)
        threeLargest = Nmaxelements(sizeArray,3)

        multiplication = threeLargest[0] * threeLargest[1] *threeLargest[2]

        return multiplication

def solutionDay09():
    fileReader = FileReader()
    riskCalculator = RiskCalculator()
    heightMap = fileReader.readToIntMap("input/inputday9")

    riskCalculator.setHeightMap(heightMap)
    answerPart1 =riskCalculator.calculateSumRiskLevels()
    answerPart2 =riskCalculator.getMultiplicationLargest3Bassins()

    printAnswer(9, answerPart1, answerPart2)