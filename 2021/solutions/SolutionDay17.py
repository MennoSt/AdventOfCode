from utils.AocUtils import *
from utils.FileReader import FileReader
import copy

class ProbeLauncher(object):
    def __init__(self):
        self.velocity = {"vx": 0, "vy": 0}
        self.position = {"x": 0, "y": 0}
        self.targetArea = {"x1" : 0, "x2" :0, "y1": 0, "y2":0}
        self.maxYCoordinate = 0
        self.maxYInitialVelocity = {"vx" : 0 , "vy": 0 }
        self.targetCounter = 0
    
    def determineMaxYCoordinate(self):
        self.targetCounter = 0
        self.maxYCoordinate = 0
        for x in range(0, 300):
            for y in range(self.targetArea["y1"], 300):
                self.reachesTargetArea(x,y)
        
        return self.maxYCoordinate
        
    def __updateMaxYIfPossible(self, yMax, initialVelocity):
        if yMax > self.maxYCoordinate:
            self.maxYCoordinate = yMax
            self.maxYInitialVelocity = initialVelocity
            
    def reachesTargetArea(self, vx, vy):
        self.position["x"] = 0
        self.position["y"] = 0
        self.velocity["vx"] = vx
        self.velocity["vy"] = vy
        targetReached = False
        
        yMax = 0
        initialVelocity = copy.deepcopy(self.velocity)
        while(self.isTargetReachable() == True):
            self.iterateToNextStep()
            if self.position["y"]>yMax:
                yMax = self.position["y"]
            if self.isWithinTargetArea(self.position["x"], self.position["y"]):
                self.__updateMaxYIfPossible(yMax, initialVelocity)
                targetReached = True
                self.targetCounter+=1
                break
        return targetReached

    def isTargetReachable(self):
        canReachTarget = self.position["x"] <= self.targetArea["x2"] and self.position["y"] >= self.targetArea["y1"]
        return canReachTarget
        
    def isWithinTargetArea(self,x,y):
        if x >= self.targetArea["x1"] and x <= self.targetArea["x2"] and y >= self.targetArea["y1"] and y <= self.targetArea["y2"]:
            inTargetArea = True
        else:
            inTargetArea = False
        return inTargetArea

    def iterateToNextStep(self):
        self.position["x"] += self.velocity["vx"]
        self.position["y"] += self.velocity["vy"]
        
        if self.velocity["vx"] < 0:
            self.velocity["vx"] += 1
        elif self.velocity["vx"] > 0:
            self.velocity["vx"] -= 1
        
        self.velocity["vy"] -= 1
            
    def fillTargetArea(self, inputFile):
        filereader = FileReader()
        fileString = filereader.readLinesToStringArray(inputFile)
        string = fileString[0]

        string = string.replace("target area: x=","")
        string = string.replace("y=", "")
        string = string.split(", ")

        array = []
        for part in string:
            array.append(part.split(".."))
    
        self.targetArea["x1"] = int(array[0][0])
        self.targetArea["x2"] = int(array[0][1])
        self.targetArea["y1"] = int(array[1][0])
        self.targetArea["y2"] = int(array[1][1])
        
def solutionDay17():
    probeLauncher = ProbeLauncher()
    probeLauncher.fillTargetArea("input/inputday17")
    
    probeLauncher.determineMaxYCoordinate()
    
    print("Start Velocity coordinate:" + str(probeLauncher.maxYInitialVelocity))
    answerPart1 = probeLauncher.maxYCoordinate
    answerPart2 = probeLauncher.targetCounter

    printAnswer(17, answerPart1, answerPart2)
