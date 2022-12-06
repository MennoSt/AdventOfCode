# %%
from utils.AocUtils import *
import copy

def parseMovement(movementLines):
    lines = movementLines.split("\n")
    movements = []
    for line in lines:
            res = [int(i) for i in line.split() if i.isdigit()]
            res = {"move": res[0], "from": res[1], "to":res[2]}
            movements.append(res)
    return movements

def parseCrates(crateLines):
    lines = crateLines.split("\n")
    elements = int(((len(lines[0])+1)/4))
    crates = [[] for _ in range(elements)]
    
    for line in lines:
        for i in range(0, elements):
            index = i*4+1
            if line[index].isalpha():
                if crates[i] != []:
                    tmpstr = line[index] + crates[i]
                else:
                    tmpstr = line[index]
                crates[i] = tmpstr
    
    return crates

def lastStringElements(crates):
    tmpStr = ""
    for crate in crates:
        if crate != "":
            tmpStr+=crate[-1]
    return tmpStr


def updateCrates(crates, movements, inverted = True):
    crates = copy.deepcopy(crates)
    for movement in movements:
        ammount = movement["move"]
        fromcrate = movement["from"] - 1
        tocrate = movement["to"] - 1
        
        if ammount > 0:
            tmpStrMove = crates[fromcrate][-ammount:]
            crates[fromcrate] =  crates[fromcrate][:-ammount]
            if inverted == True:
                crates[tocrate] += tmpStrMove[::-1]
            else:
                crates[tocrate] += tmpStrMove
    
    return lastStringElements(crates)
    
    
# Solution
# %%
# Example Tests
crateLines, movementLines = open("input_ut/inpututday5").read().split("\n\n")
crates = parseCrates(crateLines)
movements = parseMovement(movementLines)

assert updateCrates(crates, [{'move': 0, 'from': 2, 'to': 1}]) == "NDP"
assert updateCrates(crates, [{'move': 1, 'from': 2, 'to': 1}]) == "DCP"
assert updateCrates(crates, [{'move': 1, 'from': 2, 'to': 1}\
    ,{'move': 3, 'from': 1, 'to': 3}]) == "CZ"
assert updateCrates(crates, movements) == "CMZ"


# %%
# Solution
crates, movements = open("input/inputday5").read().split("\n\n")
crates = parseCrates(crates)
movements = parseMovement(movements)

assert updateCrates(crates, movements) == "ZRLJGSCTR"
assert updateCrates(crates, movements, False) == "PRTTGRFPB"

