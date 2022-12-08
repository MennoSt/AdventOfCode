

# %%
from utils.Grid import *
from utils.FileReader import *
from utils.AocEnums import *
fileReader = FileReader()

def isVisible(grid, x, y):
    value = 0
    gridValue = grid.getValue(x, y) 
    i = 1
    visible = False
    
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

def calcVisibleTrees(input):
    grid = Grid()
    grid.setGrid(input)
    grid.printGrid()
    count = 0 
    for i in range(0, grid.mapWidth):
        for j in range(0, grid.mapHeight):
            if isVisible(grid,i,j):
                count+=1
        
    return count

# %%
# Example Tests
inputStr = fileReader.readToIntMap("input_ut/inpututday8")
grid = Grid()
grid.setGrid(inputStr)
grid.printGrid()
# print(grid.getValue(0, 0))

assert isVisible(grid,0,0,) == True
assert isVisible(grid,4,0) == True
assert isVisible(grid,0,4) == True
assert isVisible(grid,4,4) == True
assert isVisible(grid,2,2) == False
assert isVisible(grid,3,3) == False
assert isVisible(grid,2,1) == True
assert calcVisibleTrees(inputStr) == 21

# calcVisibleTrees(inputStr)

# %%
# Solution
