# %%
def executeProgram(intMap, val1, val2):
    i = 0
    integerMap[1] = val1
    integerMap[2] = val2
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

def calculateNuon(verb):
    nuon = (19690720 -verb)/100
    print(nuon)

# %%
# Example
assert executeProgram ([1,9,10,3,2,3,11,0,99,30,40,50],1,9) == 3500
assert executeProgram ([1,1,1,4,99,5,6,0,99],1,1) == 30


# %%
# Solution
file = open("input/inputday2").read().split(",")
integerMap = [int(i) for i in file]
assert executeProgram(integerMap, 12, 2) == 4930687

