

# %%
from utils.Grid import *
from utils.FileReader import *
from utils.AocEnums import *
fileReader = FileReader()

def isVisible(grid, x, y):
    gridValue = grid.getValue(x, y) 
    
    for i in range(0, grid.mapWidth):
        nextVal = grid.getValue(x+i, y, Direction.RIGHT)
        if nextVal == None:
            return True
        if nextVal >= gridValue:
            break
    for i in range(0, grid.mapWidth):
        nextVal = grid.getValue(x-i, y, Direction.LEFT)
        if nextVal == None:
            return True
        if nextVal >= gridValue:
            break
    for j in range(0, grid.mapHeight):
        nextVal = grid.getValue(x, y-j, Direction.DOWN)
        if nextVal == None:
            return True
        if nextVal >= gridValue:
            break
    for j in range(0, grid.mapHeight):
        nextVal = grid.getValue(x, y+j, Direction.UP)
        if nextVal == None:
            return True
        if nextVal >= gridValue:
            break
        
    return False

def sceneticScore(grid, x, y):
    gridValue = grid.getValue(x, y) 
    scoreArray = [0,0,0,0]

    for i in range(0, grid.mapHeight):
        nextVal = grid.getValue(x+i, y, Direction.RIGHT)
        if nextVal == None:
            break
        if nextVal >= gridValue:
            scoreArray[0] += 1
            break
        scoreArray[0] += 1
    for i in range(0, grid.mapHeight):
        nextVal = grid.getValue(x-i, y, Direction.LEFT)
        if nextVal == None:
            break
        if nextVal >= gridValue:
            scoreArray[1] += 1
            break
        scoreArray[1] += 1
    for i in range(0, grid.mapHeight):
        nextVal = grid.getValue(x, y-i, Direction.DOWN)
        if nextVal == None:
            break
        if nextVal >= gridValue:
            scoreArray[2] += 1
            break
        scoreArray[2] += 1
    for i in range(0, grid.mapHeight):
        nextVal = grid.getValue(x, y+i, Direction.UP)
        if nextVal == None:
            break
        if nextVal >= gridValue:
            scoreArray[3] += 1
            break
        scoreArray[3] += 1
    
    return scoreArray[0]*scoreArray[1]*scoreArray[2]*scoreArray[3]

def calcVisibleTrees(input):
    grid = Grid()
    grid.setGrid(input)
    count = 0 
    for i in range(0, grid.mapWidth):
        for j in range(0, grid.mapHeight):
            if isVisible(grid,i,j):
                count+=1
    return count

def calcSceneticScore(input):
    grid = Grid()
    grid.setGrid(input)
    maxScore = 0
    for i in range(0, grid.mapWidth):
        for j in range(0, grid.mapHeight):
            maxScore = max(maxScore, sceneticScore(grid, i, j))
    return maxScore

# %%
# Example Tests
inputStr = fileReader.readToIntMap("input_ut/inpututday8")
assert calcVisibleTrees(inputStr) == 21
assert calcSceneticScore(inputStr) == 8

# %%
# Solution
inputStr = fileReader.readToIntMap("input/inputday8")
assert calcVisibleTrees(inputStr) == 1776
assert calcSceneticScore(inputStr) == 234416