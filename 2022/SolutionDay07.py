

# %%
import copy

def calcDirSizes(inputStrIn):
    cwd = ""
    lastDir = []
    sizeArray = []
    size = 0
    for line in inputStrIn:
        firstEl = line[0]
        if firstEl =="ls":
            pass
        elif firstEl == "cd" and line[1] == "..":
            if size != 0:
                sizeArray.append({"dir": cwd , "size": size})
                size = 0
            lenLastElem = len(lastDir[-1])
            cwd = cwd[:lenLastElem-3]
            lastDir.pop()
            
        elif firstEl == "cd" and line[1] == "/":
            cwd += (line[1])
            lastDir.append(line[1])
        elif firstEl == "cd":
            if size != 0:
                sizeArray.append({"dir": cwd , "size": size})
            cwd += (line[1])
            lastDir.append(line[1])
            size = 0
        elif firstEl.isdigit():
            size+=int(firstEl)

    sizeArray.append({"dir": cwd , "size": size})
    return sizeArray

def calcTotDirSizes(input):
    dirSizesArray = calcDirSizes(input)
    for i in range(0, len(dirSizesArray)):
        for j in range(0, len(dirSizesArray)):
            if i!=j:
                if dirSizesArray[i]["dir"] in dirSizesArray[j]["dir"]:
                    dirSizesArray[i]["size"] += dirSizesArray[j]["size"]

    return dirSizesArray

def sumDirSizes(fileLines):
    dirSizesTmp = calcTotDirSizes(fileLines)
    sum =0
    for dir in dirSizesTmp:
        if dir["size"] <= 100000:
            size = dir["size"]
            sum += int(dir["size"])
    return sum

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
assert sumDirSizes(fileLines) == 95437

# %%
# # Example Tests
file = open("input_ut/inpututday7_2").read()
fileLines = parseDir(file)
sizes = calcTotDirSizes(fileLines)
print(sizes)
# %%
# # Solution
file = open("input/inputday7").read()
fileLines = parseDir(file)
print(sumDirSizes(fileLines))
#wrong 1678450
