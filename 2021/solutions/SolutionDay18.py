from utils.AocUtils import *
from utils.AocEnums import *
from utils.FileReader import FileReader
import copy
import math

class SnailFishUpdater:
    def __init__(self):
         self.fishLists = []
         
    def readDataIntoLists(self, input):
        fileReader = FileReader()
        self.fishLists = fileReader.readLinesToListArray(input)

    def determineLargestMagnitueOfSums(self):
        magnitude = 0
        for index in range(0, len(self.fishLists)-1):
            for fishList in self.fishLists:
                if fishList != self.fishLists[index+1]:
                    updatedList = self.__addTwoLists(fishList, self.fishLists[index])
                    copyUpdatedList = copy.deepcopy(updatedList)
                    copyUpdatedList = self.updateSnailFishNumbers(copyUpdatedList)
                    newMagnitude = self.calculateMagnitude(copyUpdatedList)
                
                magnitude = max(magnitude,newMagnitude)

        return magnitude
                        
    def calculateMagnitude(self, snailFishSumini):
        
        leftScore = 3
        rightScore = 2
        
        snailFishSum = copy.deepcopy(snailFishSumini)
        
        if not isinstance(snailFishSum,int):
            for index in range(0, len(snailFishSum)+1):
                if isinstance(snailFishSum, int):
                    break
                elif len(snailFishSum) == 2:
                    if isinstance(snailFishSum[0], int) and isinstance(snailFishSum[1],int):
                        snailFishSum = snailFishSum[0] *leftScore + snailFishSum[1]*rightScore
                    else:
                        snailFishSum[index] = self.calculateMagnitude(snailFishSum[index])
    
        return snailFishSum
    
    
    def calculateFinalSum(self):
        
        updatedList = self.fishLists[0]
        
        for index in range(0, len(self.fishLists)-1):
            updatedList = self.__addTwoLists(updatedList, self.fishLists[index+1])
            updatedList = self.updateSnailFishNumbers(updatedList)
        
        return updatedList
            
    def __addTwoLists(self, list1, list2):
        listSum = [list1,list2]
        return listSum
    
    def updateSnailFishNumbers(self, initialList):
        
        listChanged = True
        startList = copy.deepcopy(initialList)
        updatedList = self.updateListWithExplosions(initialList)
        updatedList = self.updateListWithSplits(updatedList)
        
        if startList == updatedList:
            listChanged = False
        else:
            listChanged = True
        
        if not listChanged:
            return updatedList
        else:
            return self.updateSnailFishNumbers(updatedList)
        
    def updateListWithSplits(self,initialList):
        
        for index1 in range(0, len(initialList)):
            splitted = self.__splitListIfHigherThanTen(initialList, index1)
            if splitted:
                return initialList
            if isinstance(initialList[index1], list):
                for index2 in range(0, len(initialList[index1])):
                    splitted = self.__splitListIfHigherThanTen(initialList[index1], index2)
                    if splitted:
                        return initialList
                    if isinstance(initialList[index1][index2], list):
                        for index3 in range(0, len(initialList[index1][index2])):
                            splitted = self.__splitListIfHigherThanTen(initialList[index1][index2], index3)
                            if splitted:
                                return initialList
                            if isinstance(initialList[index1][index2][index3], list):
                                for index4 in range(0, len(initialList[index1][index2][index3])):
                                    splitted = self.__splitListIfHigherThanTen(initialList[index1][index2][index3], index4)
                                    if splitted:
                                        return initialList
        
        return initialList

    def __splitListIfHigherThanTen(self, initialList, index1):
        splitted = False
        if type(initialList[index1]) != list:
            if initialList[index1] > 9:
                initialList[index1] =[math.floor(initialList[index1]/2), math.ceil(initialList[index1]/2)]
                splitted = True
        return splitted
        
    def updateListWithExplosions(self, initialList):
        
        for index1 in range(0, len(initialList)):
            if isinstance(initialList[index1], list):
                for index2 in range(0, len(initialList[index1])):
                    if isinstance(initialList[index1][index2], list):
                        for index3 in range(0, len(initialList[index1][index2])):
                            if isinstance(initialList[index1][index2][index3], list):
                                for index4 in range(0, len(initialList[index1][index2][index3])):
                                    if isinstance(initialList[index1][index2][index3][index4], list):
                                        
                                        self.depth0 = initialList
                                        self.depth1 = initialList[index1]
                                        self.depth2 = initialList[index1][index2]
                                        self.depth3 = initialList[index1][index2][index3]
                                        self.depth4 = initialList[index1][index2][index3][index4]         
                                        if index4-1 >= 0:
                                            self.__updateFourthNestedList(index4, Direction.LEFT)
                                        elif index3-1 >= 0:
                                            self.__addExplodedValue(index3, self.depth2, Direction.LEFT)
                                        elif index2-1 >= 0:
                                            self.__addExplodedValue(index2, self.depth1, Direction.LEFT)
                                        elif index1-1 >= 0:
                                            self.__addExplodedValue(index1, self.depth0, Direction.LEFT)
                                            
                                        if index4+1 < len(self.depth3):
                                            self.__updateFourthNestedList(index4, Direction.RIGHT)
                                        elif index3+1 < len(self.depth2):
                                            self.__addExplodedValue(index3, self.depth2, Direction.RIGHT)
                                        elif index2+1 < len(self.depth1):
                                            self.__addExplodedValue(index2, self.depth1, Direction.RIGHT)
                                        elif index1+1 < len(self.depth0):
                                            self.__addExplodedValue(index1, self.depth0, Direction.RIGHT)

                                        initialList[index1][index2][index3][index4] = 0
                                        
                                        self.updateListWithExplosions(initialList)
    
        return initialList

    def __updateFourthNestedList(self, index4, direction=Direction.RIGHT):
        
        if direction == Direction.RIGHT:
            i = 1
            dir = 1
            iadd = 0
        else:
            i = -1
            dir = 0
            iadd = 1
        
        self.__checkOnNestedList(index4, self.depth3, i, dir, iadd)

    def __addExplodedValue(self, index, depth, direction=Direction.RIGHT):
        if direction == Direction.RIGHT:
            i = 1
            dir = 1
            iadd = 0
        else:
            i = -1
            dir = 0
            iadd = 1
     
        self.__checkOnNestedList(index, depth, i, dir, iadd)

    def __checkOnNestedList(self, index, depth, i, dir, iadd):
        if isinstance(depth[index+i], list):
            if isinstance(depth[index+i][iadd], list):
                if isinstance(depth[index+i][iadd][iadd], list):
                    if isinstance(depth[index+i][iadd][iadd][iadd], list):
                        depth[index+i][iadd][iadd][iadd][iadd] += self.depth4[dir]
                    else:
                        depth[index+i][iadd][iadd][iadd] += self.depth4[dir]        
                else:
                    depth[index+i][iadd][iadd] += self.depth4[dir]   
            else:
                depth[index+i][iadd] += self.depth4[dir]
        else:
            depth[index+i] += self.depth4[dir]
    
def solutionDay18():
    snailFishUpdater = SnailFishUpdater()
    
    snailFishUpdater.readDataIntoLists("input/inputday18")
    finalSum = snailFishUpdater.calculateFinalSum()
    answerPart1 = snailFishUpdater.calculateMagnitude(finalSum)
    
    snailFishUpdater.readDataIntoLists("input/inputday18")
    answerPart2 = snailFishUpdater.determineLargestMagnitueOfSums()
    
    printAnswer(18, answerPart1, answerPart2)
    
    