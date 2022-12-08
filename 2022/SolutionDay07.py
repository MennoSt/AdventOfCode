

# %%
import copy

def calcDirSizes(inputStrIn):
    cwd = ""
    lastDir = []
    sizeArray = []
    size = 0
    uniCnt = 0
    for line in inputStrIn:
        firstEl = line[0]
        if firstEl =="ls":
            pass
        elif firstEl == "cd" and line[1] == "..":
            sizeArray.append({"dir": cwd , "size": size})
            if size != 0:
                size = 0
            lenLastElem = len(lastDir[-1])
            lenCwd = len(cwd)
            cwd = cwd[:lenCwd-lenLastElem]
            lastDir.pop()
        elif firstEl == "cd" and line[1] == "/":
            uniCnt+=1
            uniPath = line[1] + str(uniCnt)
            cwd += (uniPath)
            lastDir.append(uniPath)
        elif firstEl == "cd":
            sizeArray.append({"dir": cwd , "size": size})
            uniCnt+=1
            uniPath = line[1] + str(uniCnt)
            cwd += (uniPath)
            lastDir.append(uniPath)
            size = 0
        elif firstEl.isdigit():
            size+=int(firstEl)

    sizeArray.append({"dir": cwd , "size": size})
    return sizeArray

def calcTotDirSizes(input):
    dirSizesArray = calcDirSizes(input)
    dirSizeArrayCp = copy.deepcopy(dirSizesArray)
    for i in range(0, len(dirSizesArray)):
        for j in range(0, len(dirSizesArray)):
            if i!=j:
                if dirSizesArray[i]["dir"] in dirSizesArray[j]["dir"]:
                    dirSizeArrayCp[i]["size"] += dirSizesArray[j]["size"]
    return dirSizeArrayCp

def sumDirSizes(fileLines):
    dirSizesTmp = calcTotDirSizes(fileLines)
    dirSizesTmp = [dict(t) for t in {tuple(d.items()) for d in dirSizesTmp}]
    sum = 0
    max = 0
    for dir in dirSizesTmp:
        if dir["dir"] == "/1":
            max = dir["size"]
        if dir["size"] <= 100000:
            size = dir["size"]
            sum += int(dir["size"])
    
    minSpaceToFree = 30000000 - (70000000-max)
    value = getClosestSpaceValue(dirSizesTmp, minSpaceToFree)
    return sum, max, value

def getClosestSpaceValue(dirsizes, spaceToFree):
    sizeArray = []
    
    for dir in dirsizes:
        sizeArray.append(dir["size"])
    
    new_list = [item for item in sizeArray if item >= spaceToFree]
    listSort = sorted(new_list)
    return listSort[0]

def parseDir(inputStr):
    inputStr = inputStr.replace("$ ", "")
    inputStr = inputStr.split("\n")
    for i in range(0,len(inputStr)):
        inputStr[i] = inputStr[i].split(" ")
    return inputStr

# %%
# # Example Tests
file = open("input_ut/inpututday7").read()
fileLines = parseDir(file)
assert sumDirSizes(fileLines) == [95437, 48381165]

# %%
# # Example Tests
file = open("input_ut/inpututday7_2").read()
fileLines = parseDir(file)
sizes = sumDirSizes(fileLines)

# %%
# # Solution
file = open("input/inputday7").read()
fileLines = parseDir(file)
sum, max, value = sumDirSizes(fileLines)
assert sum == 1844187
assert value == 4978279
print(value)
