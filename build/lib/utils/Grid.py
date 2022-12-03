from ast import Str
from utils.AocEnums import *

class Grid:

    def __init__(self):
        self.gridMap = []
        self.mapWidth = 0
        self.mapHeight = 0
    
    def setGrid(self, gridMap):
        self.gridMap = gridMap
        self.mapWidth = len(gridMap[0])
        self.mapHeight = len(gridMap)

    def updateMapSizes(self):
        self.mapWidth = len(self.gridMap[0])
        self.mapHeight = len(self.gridMap)
        
    def countElementsInGrid(self, strElement):
        counts = 0
        for line in self.gridMap:
            count = line.count(strElement)
            counts+=count
        
        return counts
    
    def printGrid(self, name = ""):
        print("\n " + name)
        for line in self.gridMap:
            print(line)
    
    def getTotalSum(self):
        sumtotal = 0
        for line in self.gridMap:
            sumtotal += sum(line)
        return sumtotal
            
    def getValue(self, x, y, direction = Direction.CURRENT):

        value = 0
        if direction == Direction.CURRENT:
            value = self.__getValue(x, y)
        if direction == Direction.UP:
            value = self.__getValue(x, y+1)
        if direction == Direction.DOWN:
            value = self.__getValue(x, y-1)
        if direction == Direction.LEFT:
            value = self.__getValue(x-1, y)
        if direction == Direction.RIGHT:
            value = self.__getValue(x+1, y)

        return value
    
    def __getValue(self, x, y):
        mapWidth = self.mapWidth-1
        mapHeight = self.mapHeight -1
        if x < 0 or x > mapWidth or y < 0 or y > mapHeight:
            value = None
        else:
            value = self.gridMap[y][x]

        return value
    def __setValue(self, x, y, value):
        mapWidth = self.mapWidth-1
        mapHeight = self.mapHeight -1
        
        if x < 0 or x > mapWidth or y < 0 or y > mapHeight:
            return
        else:
            self.gridMap[y][x] = value
        
    def setValue(self, x, y, value, direction = Direction.CURRENT):
        
        if direction == Direction.CURRENT:
            self.__setValue(x,y,value)
        if direction == Direction.UP:
            value = self.__setValue(x, y+1, value)
        if direction == Direction.DOWN:
            value = self.__setValue(x, y-1, value)
        if direction == Direction.LEFT:
            value = self.__setValue(x-1, y, value)
        if direction == Direction.RIGHT:
            value = self.__setValue(x+1, y, value)

    def increaseValue(self,x,y,value):
        self.gridMap[y][x] += value

    def hasValueGreaterThan(self, value):
        boolGreaterThan = False
        for x in range (0, self.mapWidth):
            for y in range(0, self.mapHeight):
                if self.getValue(x,y) > 9:
                    boolGreaterThan = True
        
        return boolGreaterThan
    
    
    def getHeightValue(self, x, y, direction = Direction.CURRENT):
        
        mapx = x
        mapy = y

        if direction == Direction.CURRENT:
            mapx = x
            mapy = y
        if direction == Direction.UP:
            mapx = x
            mapy = y + 1
        if direction == Direction.DOWN:
            mapx = x
            mapy = y -1
        if direction == Direction.LEFT:
            mapx = x-1
            mapy = y
        if direction == Direction.RIGHT:
            mapx = x + 1
            mapy = y
        
        # Return the lowest index 0 if the mapIndex is out of the map range
        mapWidth = int(self.mapWidth) -1
        mapHeight = int(self.mapHeight) -1

        if mapx < 0 or mapx > mapWidth: 
            mapx = 0
            heightValue = 9
        elif mapy < 0 or mapy > mapHeight: 
            mapy = 0
            heightValue = 9
        else:
            heightValue = self.gridMap[mapy][mapx]

        return heightValue
            