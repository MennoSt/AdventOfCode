from utils.AocUtils import *
from utils.FileReader import FileReader

class Node():
    def __init__(self, character):
        self.character = character
        self.neighbours = []
        self.numberOfVisits = 0
        self.array = []
        
    def addNeighour(self, neighbour):
        self.neighbours.append(neighbour)
    
    def getCharacter(self):
        return self.character

    def increaseNumberOfVisits(self, string):
        self.numberOfVisits += 1
    
    def decreaseNumberOfVisits(self):
        self.numberOfVisits -= 1

class CaveNavigator:
    def __init__(self):
        self.Nodes = [Node("")]
        self.numberOfPaths = 0
        self.nextnode = Node("")
        self.visitedNodes = ["start","start"]
        
    def findNumberOfRoutes(self, fileinput, smallCavesTwice = False):
        
        self.__updateNodeInfo(fileinput)
        routes = self.iterateThroughPath(smallCavesTwice)
        return routes


    def iterateThroughPath(self, smallCavesTwice):
        
        startNode = self.__getNode("start")
        endNode = self.__getNode("end")
        
        self.visitedNodes = ["start", "start"]
        self.pathCount = 0
        
        if not smallCavesTwice:
            self.__compareNodePart1(startNode, endNode)
        else:
            self.__compareNodePart2(startNode, endNode)

        return self.pathCount
    
    def __compareNodePart1(self, currentNode, endNode):
        for str in currentNode.neighbours:
            nextnode = self.__getNode(str)
            if str == endNode.character:
                self.pathCount += 1
            elif str not in self.visitedNodes:
                if not str.isupper():
                    self.visitedNodes.append(str)
                self.__compareNodePart1(nextnode, endNode)
                if str in self.visitedNodes: 
                    self.visitedNodes.remove(str)
    
    def __compareNodePart2(self, currentNode, endNode):
        for str in currentNode.neighbours:
            nextnode = self.__getNode(str)
            if str == endNode.character:
                self.pathCount += 1
            elif self.__isNextVisitAllowed(str) == True:
                nextnode.increaseNumberOfVisits(str)
                if not str.isupper():
                    self.visitedNodes.append(str)
                self.__compareNodePart2(nextnode, endNode)
                self.__removeVisitedNode(str)
                nextnode.decreaseNumberOfVisits()
    
    def __removeVisitedNode(self, str):
        if str in self.visitedNodes: 
            self.visitedNodes.remove(str)
            
    def __isNextVisitAllowed(self, str):
        allowed = False
        if str not in self.visitedNodes:
            allowed = True
        elif str in self.visitedNodes and not self.__isSmallNodeVisitedMoreThanTwice() and str!="start" and not str.isupper():
            allowed = True
        else:
            allowed = False
            
        return allowed
                        
    def __isSmallNodeVisitedMoreThanTwice(self):
        for node in self.Nodes:
            if not node.character.isupper() and node.character != "start":
                if node.numberOfVisits > 1:
                    return True

        return False
                         
    def __getNode(self, char):
        for node in self.Nodes:
            if node.getCharacter() == char:
                return node       
            
    def __getUniqeNodeArray(self, fileinput):
        array = []
        for file in fileinput:
            connection = file.split("-")
            array.append(connection[0])
            array.append(connection[1])
        
        uniqueNodeList = list(set(array))
        return uniqueNodeList
    
    def __updateNodeInfo(self, fileinput):
        
        NodeList = self.__getUniqeNodeArray(fileinput)
        
        self.Nodes.pop()
        
        for char in NodeList:
            newnode = Node(char)
            self.Nodes.append(newnode)
        
        for file in fileinput:
            nodePair = file.split("-")

            for node in self.Nodes:
                if node.getCharacter() == nodePair[0]:
                    node.addNeighour(nodePair[1])
                if node.getCharacter() == nodePair[1]:
                    node.addNeighour(nodePair[0])

def solutionDay12():
    fileReader = FileReader()
    initialMap = fileReader.readLinesToStringArray("input/inputday12")
    caveNavigator = CaveNavigator()

    answerPart1 = caveNavigator.findNumberOfRoutes(initialMap)
    answerPart2 = caveNavigator.findNumberOfRoutes(initialMap, True)
    printAnswer(12, answerPart1, answerPart2)