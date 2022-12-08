

# %%
from utils.Grid import *
from utils.FileReader import *
from utils.AocEnums import *
fileReader = FileReader()

def isVisible(grid, x, y):
    value = 0
    gridValue = grid.getValue(x, y) 
    visible = False
    
    if visible == False:
        i = 1
        while value != None:
            value = grid.getValue(x, y-i)
            if value == None:
                visible = True
                break
            if value >= gridValue:
                visible = False
                break
            i += 1
    
    if visible == False:
        i = -1
        while value != None:
            value = grid.getValue(x, y-i)
            if value == None:
                visible = True
                break
            if value >= gridValue:
                visible = False
                break
            i -= 1
    
    if visible == False:
        j = 1
        while value != None:
            value = grid.getValue(x-j, y)
            if value == None:
                visible = True
                break
            if value >= gridValue:
                visible = False
                break
            j += 1
    
    if visible == False:
        j = -1
        while value != None:
            value = grid.getValue(x-j, y)
            if value == None:
                visible = True
                break
            if value >= gridValue:
                visible = False
                break
            j -= 1
            
    return visible

def sceneticScore(grid, x, y):
    value = 0
    gridValue = grid.getValue(x, y) 
    visible = False
    sceneticScore1 = 0
    sceneticScore2 = 0
    sceneticScore3 = 0
    sceneticScore4 = 0

    i = 1
    value = grid.getValue(x, y-i)
    while value != None:
        value = grid.getValue(x, y-i)
        if value == None:
            break
        if value >= gridValue:
            sceneticScore1 += 1
            break
        i += 1
        sceneticScore1 += 1
    
    i = -1
    value = grid.getValue(x, y-i)    
    while value != None:
        value = grid.getValue(x, y-i)
        if value == None:
            visible = True
            break
        if value >= gridValue:
            sceneticScore2 += 1
            visible = False
            break
        i -= 1
        sceneticScore2 += 1
            

    j = 1
    value = grid.getValue(x-j, y)
    while value != None:
        value = grid.getValue(x-j, y)
        if value == None:
            visible = True
            break
        if value >= gridValue:
            sceneticScore3 += 1
            visible = False
            break
        j += 1
        sceneticScore3 += 1
        
    j = -1
    value = grid.getValue(x-j, y)
    while value != None:
        value = grid.getValue(x-j, y)
        if value == None:
            visible = True
            break
        if value >= gridValue:
            sceneticScore4 += 1
            visible = False
            break
        j -= 1
        sceneticScore4 += 1
    
    score = sceneticScore1*sceneticScore2*sceneticScore3*sceneticScore4
    return score

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
    sceneticScoreCount = 0
    for i in range(0, grid.mapWidth):
        for j in range(0, grid.mapHeight):
            sceneticScoreCount = sceneticScore(grid, i, j)
            if sceneticScoreCount > maxScore:
                maxScore = sceneticScoreCount
    return maxScore

# %%
# Example Tests
inputStr = fileReader.readToIntMap("input_ut/inpututday8")
grid = Grid()
grid.setGrid(inputStr)

# assert isVisible(grid,0,0,) == True
# assert isVisible(grid,4,0) == True
# assert isVisible(grid,0,4) == True
# assert isVisible(grid,4,4) == True
# assert isVisible(grid,2,2) == False
# assert isVisible(grid,3,3) == False
# assert isVisible(grid,2,1) == True
# assert calcVisibleTrees(inputStr) == 21
assert sceneticScore(grid,4,0) == 0
assert sceneticScore(grid,0,0) == 0
assert sceneticScore(grid,2,3) == 8
# calcVisibleTrees(inputStr)
calcSceneticScore(inputStr) == 8

# %%
# Solution
inputStr = fileReader.readToIntMap("input/inputday8")
grid = Grid()
grid.setGrid(inputStr)
calcVisibleTrees(inputStr)
calcSceneticScore(inputStr)