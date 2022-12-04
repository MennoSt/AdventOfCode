from utils.AocUtils import *
from utils.FileReader import FileReader

binaryConversionList = [["0","0000"],
                        ["1","0001"],
                        ["2","0010"],
                        ["3","0011"],
                        ["4","0100"],
                        ["5","0101"],
                        ["6","0110"],
                        ["7","0111"],
                        ["8","1000"],
                        ["9","1001"],
                        ["A","1010"],
                        ["B","1011"],
                        ["C","1100"],
                        ["D","1101"],
                        ["E","1110"],
                        ["F","1111"]]

class Packet:
    
    def __init__(self):
        self.version = -1
        self.typeID = -1
        self.literalValue = -1
        self.literalValuePackages = []
        self.literalValuesArray = []
        self.lengthTypeId = -1
        self.numberOfSubPackages = -1
        self.subPackages = []
        self.length = -1
        
    def updateLiteralValue(self):
        litValue=""
        for value in self.literalValuePackages:
            litValue+=value[1:5]
        
        self.literalValue  = int(litValue,2)
    
    def generateLiteralValuesArray(self):
        for packet in self.subPackages:
            self.literalValuesArray.append(packet.literalValue)
            
    def calculateSum(self):
        sumLit = 0
        for packet in self.subPackages:
            sumLit += packet.literalValue
        return sumLit

    def calculateProduct(self):
        multiplication = 0
        
        for index in range(0, len(self.literalValuesArray)):
            if index == 0:
                multiplication = self.literalValuesArray[index]
            else:
                multiplication *= self.literalValuesArray[index]
                
        return multiplication
    
    def calculateMin(self):
        minimum = min(self.literalValuesArray)
        return minimum
    
    def calculateMax(self):
        maximum = max(self.literalValuesArray)
        return maximum
    
    def isGreaterThan(self):
        if self.literalValuesArray[0]>self.literalValuesArray[1]:
            return 1
        else:
            return 0

    def isLessThan(self):
        if self.literalValuesArray[0]<self.literalValuesArray[1]:
            return 1
        else:
            return 0
        
    def isEqualTo(self):
        if self.literalValuesArray[0] == self.literalValuesArray[1]:
            return 1
        else:
            return 0
        
class PacketDecoder:
    
    def __init__(self):
        self.packet = Packet()
        self.versionSum = 0
        self.binItIndex = 0
    
    def calculateResultingValue(self):
        self.packet.generateLiteralValuesArray()
        typeId = self.packet.typeID
        if typeId == 0:
            result = self.packet.calculateSum()
        if typeId == 1:
            result = self.packet.calculateProduct()
        if typeId == 2:
            result = self.packet.calculateMin()
        if typeId == 3:
            result = self.packet.calculateMax()
        if typeId == 5:
            result = self.packet.isGreaterThan()
        if typeId == 6:
            result = self.packet.isLessThan()
        if typeId == 7:
            result = self.packet.isEqualTo()
            
        return result

    def updateLiteralValues(self, packet):
        result = 0
        packet.generateLiteralValuesArray()
        typeId = packet.typeID
        if typeId == 0:
            result = packet.calculateSum()
        if typeId == 1:
            result = packet.calculateProduct()
        if typeId == 2:
            result = packet.calculateMin()
        if typeId == 3:
            result = packet.calculateMax()
        if typeId == 5:
            result = packet.isGreaterThan()
        if typeId == 6:
            result = packet.isLessThan()
        if typeId == 7:
            result = packet.isEqualTo()
            
        packet.literalValue = result
        
        
    def convertToBinaryString(self, hexstring):
        binaryString = ""
        for char in hexstring:
            for conversion in binaryConversionList:
                if char == conversion[0]:
                    binaryString += conversion[1]
        
        return binaryString
        
        
    def decodeStringToPackage(self, hexstring):
        binString = self.convertToBinaryString(hexstring)
        
        self.readBinaryString(binString)

    def readBinaryString(self, binString):
        
        self.versionSum = 0
        self.binItIndex = 0
        
        self.packet.version = int(binString[self.binItIndex:self.binItIndex+3],2)
        self.versionSum += self.packet.version
        self.packet.typeID = int(binString[self.binItIndex+3:self.binItIndex+6],2)
            
            #literal package
        if self.packet.typeID == 4:
            self.packet.literal = True
            self.binItIndex += 6
            self.__readLiteralData(binString, self.packet)
            #operater package
        else:
            self.readOperatorData(binString, self.packet)

    def readOperatorData(self, binString, packet):
        
        packet.lengthTypeId = binString[self.binItIndex+6]
        if packet.lengthTypeId == "0":
            # 15 bit length number
            packet.length = int(binString[self.binItIndex+7:self.binItIndex+22],2)
            lengthSubpackages = 0
            self.binItIndex+=22
            bitStart = self.binItIndex
            while(lengthSubpackages < packet.length):
                subpacket = Packet()
                subpacket.version = int(binString[self.binItIndex:self.binItIndex+3], 2)
                subpacket.typeID = int(binString[self.binItIndex+3:self.binItIndex+6], 2)
                self.versionSum += subpacket.version
                if subpacket.typeID == 4:
                    self.binItIndex += 6
                    self.__readLiteralData(binString, subpacket)
                    packet.subPackages.append(subpacket)
                else:
                    subpacket.operator = True
                    packet.subPackages.append(subpacket)
                    self.readOperatorData(binString, subpacket)
                    self.updateLiteralValues(subpacket)
                bitStop = self.binItIndex
                lengthSubpackages = bitStop-bitStart
                    
        elif packet.lengthTypeId == "1":         
            # 11 bit length number
            packet.length = int(binString[self.binItIndex+7:self.binItIndex+18], 2)
            self.binItIndex += 18
            for _ in range(0, packet.length):
                subpacket = Packet()
                subpacket.version = int(binString[self.binItIndex:(self.binItIndex+3)],2)
                subpacket.typeID = int(binString[(self.binItIndex+3):(self.binItIndex+6)],2)
                self.versionSum += subpacket.version
                if subpacket.typeID == 4:
                    self.binItIndex += 6
                    self.__readLiteralData(binString, subpacket)
                    packet.subPackages.append(subpacket)
                else:
                    packet.subPackages.append(subpacket)
                    self.readOperatorData(binString, subpacket)
                    self.updateLiteralValues(subpacket)
            
    def __readLiteralData(self, binString, packet):

        endReached = False
        
        while(not endReached):
            firstIndex = self.binItIndex
            firstValue = binString[firstIndex]
            if firstValue == '1' and not endReached:
                index1 = self.binItIndex
                index2 = 5 + self.binItIndex
                packet.literalValuePackages.append(binString[index1:index2])
                self.binItIndex += 5
            else:
                index1 = self.binItIndex
                index2 = 5 + self.binItIndex
                packet.literalValuePackages.append(binString[index1:index2])
                self.binItIndex += 5
                endReached = True
    
        packet.updateLiteralValue()
            
            

def solutionDay16():
    fileReader = FileReader()
    packetDecoder = PacketDecoder()
    
    inputString = fileReader.readLinesToStringArray("input/inputday16")
    packetDecoder.decodeStringToPackage(inputString[0])
    
    answerPart1 = packetDecoder.versionSum
    answerPart2 = packetDecoder.calculateResultingValue()
    printAnswer(16, answerPart1, answerPart2)