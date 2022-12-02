import copy

from utils.AocUtils import *
from utils.FileReader import FileReader
from utils.AocEnums import Direction
from utils.Grid import Grid

class Dijkstra:
    def __init__(self):
        self.initialGrid = Grid()
        self.dijkstraGrid = Grid()
    
    def setGrid(self, gridMap, extendMap = False):
        copyGridMap = copy.deepcopy(gridMap)
        
        if extendMap == True:
            self.extendGridMap(copyGridMap)
            self.initialGrid.setGrid(copyGridMap)
        else:
            self.initialGrid.setGrid(copyGridMap)
        
        array = [[999999 for col in range(self.initialGrid.mapHeight)] for row in range(self.initialGrid.mapWidth)]
        array[0][0] = 0
        self.dijkstraGrid.setGrid(array)
    
    def extendGridMap(self,gridmap):
        for line in gridmap:
            lineCopy = copy.deepcopy(line)
            for i in range(1,5):
                line+=self.__increaseLineByValue(lineCopy,i)
        
        copyGridmap = copy.deepcopy(gridmap)
        for i in range(1,5):
            for line in copyGridmap:
                lineCopy = copy.deepcopy(line)
                gridmap.append(self.__increaseLineByValue(lineCopy,i))

        for line in gridmap:
            for index in range(0,len(line)):
                if line[index] > 9:
                    line[index] -= 9
                    
    def __increaseLineByValue(self, line, value):
            
        incrementedList = [x+value for x in line]
                   
        return incrementedList
        
    def findShortestPath(self):
        
        self.iterateThroughGrid()
        shortestPath = self.dijkstraGrid.getValue(self.dijkstraGrid.mapWidth-1, self.dijkstraGrid.mapHeight-1)
        return shortestPath
    
    def iterateThroughGrid(self):
        directions = [Direction.LEFT, Direction.UP, Direction.DOWN, Direction.RIGHT]
        totalSumAfterIt = 0
        totalSumbeforeIt = self.dijkstraGrid.getTotalSum()
        
        it =0
        while(totalSumbeforeIt != totalSumAfterIt):
            totalSumbeforeIt = self.dijkstraGrid.getTotalSum()
            self.UpdateDijkstraValues(directions)
            totalSumAfterIt = self.dijkstraGrid.getTotalSum() 
            it+=1
            # print("Dijkstra iteration:" + str(it))

    def UpdateDijkstraValues(self, directions):
        for y in range(0,self.dijkstraGrid.mapHeight):
            for x in range(0,self.dijkstraGrid.mapWidth):
                for direction in directions:
                    self.updateDijkstra(y, x, direction)

    def updateDijkstra(self, y, x, direction):

        value = self.initialGrid.getValue(x,y,direction)
        if value != None:
            valueCurrent = self.dijkstraGrid.getValue(x,y)
            updatedValueDijkstra = value + valueCurrent
            currentValueDijkstra = self.dijkstraGrid.getValue(x,y,direction)
                
            if updatedValueDijkstra < currentValueDijkstra:
                self.dijkstraGrid.setValue(x,y,updatedValueDijkstra, direction)
        else:
            return

def solutionDay15():
    fileReader = FileReader()
    dijkstra = Dijkstra()
    
    input = fileReader.readToIntMap("input/inputday15")
    dijkstra.setGrid(input)
    answerPart1 = dijkstra.findShortestPath()
    
    dijkstra.setGrid(input,True)
    answerPart2 = dijkstra.findShortestPath()
    
    printAnswer(15, answerPart1, answerPart2)
    