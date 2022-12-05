# %%
from utils.AocUtils import *
from utils.FileReader import *
import copy

def parseMovement(file):
    movements = []
    start = False
    for line in file:
        if start ==True:
            res = [int(i) for i in line.split() if i.isdigit()]
            res = {"move": res[0], "from": res[1], "to":res[2]}
            movements.append(res)
        if line == "":
            start = True
    return movements

def parseCrates(file):
    arrayPos = []
    movement = []
    j = 0
    for line in file:
        if line == "":
            break
        
        length = len(line)
        i=1
        while(i < length):
            if line[i].isalpha():
                arrayPos.append({'pos':i, 'char':line[i]})
            i += 4

    arrayOut = []
    i =1
    while(i < length):
        tmp = ""
        for char in arrayPos:
            if char["pos"] == i:
                tmp+=char["char"]
        i+=4
        tmp = tmp[::-1]
        arrayOut.append(tmp)
    
    return arrayOut

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
file = open("input_ut/inpututday5").read().split("\n")
crates = parseCrates(file)
movements = parseMovement(file)

assert updateCrates(crates, [{'move': 0, 'from': 2, 'to': 1}]) == "NDP"
assert updateCrates(crates, [{'move': 1, 'from': 2, 'to': 1}]) == "DCP"
assert updateCrates(crates, [{'move': 1, 'from': 2, 'to': 1}\
    ,{'move': 3, 'from': 1, 'to': 3}]) == "CZ"
assert updateCrates(crates, movements) == "CMZ"


# %%
file = open("input/inputday5").read().split("\n")
crates = parseCrates(file)
movements = parseMovement(file)
answer1 = updateCrates(crates, movements, False)
answer2 = updateCrates(crates, movements)
printAnswer(5, answer1, answer2)
assert answer1 == "PRTTGRFPB"
assert answer2 == "ZRLJGSCTR"

