# %%
import copy

def executeProgram(intMapin, val1, val2):
    i = 0
    intMap = copy.deepcopy(intMapin)
    intMap[1] = val1
    intMap[2] = val2
    while intMap[i] != 99:
        firstPos = intMap[i+1]
        secPos = intMap[i+2]
        placePos = intMap[i+3]
        if (intMap[i] == 1):
            intMap[placePos] = intMap[firstPos] + intMap[secPos]
        else:
            intMap[placePos] = intMap[firstPos] * intMap[secPos]
        i += 4
    return intMap[0]

def guessAnswer(intMap):
    for i in range(30,60):
        for j in range(30,60):
            if executeProgram(intMap, i, j) == 19690720:
                answer = 100*i+j
                return answer
    
# %%
# Example
assert executeProgram ([1,9,10,3,2,3,11,0,99,30,40,50],9,10) == 3500
assert executeProgram ([1,1,1,4,99,5,6,0,99],1,1) == 30


# %%
# Solution
file = open("input/inputday2").read().split(",")
integerMap = [int(i) for i in file]
assert executeProgram(integerMap, 12, 2) == 4930687
assert guessAnswer(integerMap) == 5335