from utils.AocUtils import *
from utils.AocEnums import *
from utils.Operations import *
from copy import deepcopy as dc
    
class Scanner:
    def __init__(self, name:str, beaconPositions):
        self.name = name
        self.positions = beaconPositions
            
class ScanManager:
    def __init__(self):
        self.beaconsRelativeToScanner = []
        self.lastOperation = {'x':Operator.PLUSX, 'y':Operator.PLUSY, 'z':Operator.PLUSZ,}
        self.scanners = [Scanner]
        self.scanners.pop(0)
        self.visitedScanners = [0]
        self.scannerCoordinates = [{'x': 0, 'y': 0, 'z': 0}]
        
    def readInputDataIntoScanners(self, input):
        with open (input, "r") as myfile:
            dataread = myfile.read().rstrip()
            chartData = dataread.split('\n\n')

        for data in chartData:
            scanData = data.split("\n")
            self.__readDataIntoScanners(scanData)
            
    def __readDataIntoScanners(self,scanData):
            name = scanData[0]
            scanData.pop(0)
            
            positions = []
            for line in scanData:
                line = list(map(int,line.split(",")))
                position = {"x" : line[0], "y" :line[1], "z": line[2]}
                positions.append(position)
            
            scanner = Scanner(name, positions)
            self.scanners.append(scanner)
        
    def coordinatesBetweenScanners(self, scanner1:Scanner, scanner2:Scanner):
        
        coordinate = {'x':0, 'y':0, 'z':0}
        
        coordinate['x'] = self.__findCommonCoordinate(scanner1, scanner2, "x")
        coordinate['y'] = self.__findCommonCoordinate(scanner1, scanner2, "y")
        coordinate['z'] = self.__findCommonCoordinate(scanner1, scanner2, "z")
        
        if coordinate['x'] == None or coordinate['y'] == None or coordinate['z'] == None:
            coordinate = None
            
        return coordinate
                
    def addBeaconsRelativeToScanner(self, scannerIndex):
        
        self.addAllCoordinatesRelativeToScanner(self.scanners[scannerIndex])
        
        beacons = self.__addBeaconsTogether()
        self.beaconsRelativeToScanner = sorted(beacons, key=lambda x: x['x'])

    def __addBeaconsTogether(self):
        beacons = []
        for scanner in self.scanners:
            for beacon in scanner.positions:
                beacons.append(beacon)
        
        beacons = removeDuplicatesInDictionairy(beacons)
        return beacons
        
    def addAllCoordinatesRelativeToScanner(self, scannerInput:Scanner):
        numberOfScanners = len(self.scanners)
        for index in range(0,numberOfScanners):
            if index not in self.visitedScanners:
                coordinates = self.coordinatesBetweenScanners(scannerInput, self.scanners[index])
                if coordinates != None:
                    if index not in self.visitedScanners:
                        self.scannerCoordinates.append(coordinates)
                        self.visitedScanners.append(index)
                        scannerTmp = []
                        for beacon in self.scanners[index].positions:
                            beaconInit = dc(beacon)
                            self.__updateBeacon(coordinates, beacon, beaconInit)
                            scannerTmp.append(beacon)
                        self.scanners[index].positions = scannerTmp
                        self.addAllCoordinatesRelativeToScanner(self.scanners[index])

    def getLengthBeacons(self):
        return len(self.beaconsRelativeToScanner)
    
    def getLargestManhattanDistance(self):
        maxManhattanSum = 0
        manhattanSum = 0
        for index1 in range(0, len(self.scannerCoordinates)):
            for index2 in range(0, len(self.scannerCoordinates)):
                if index1 != index2:
                    xdist = abs(self.scannerCoordinates[index1]['x'] - self.scannerCoordinates[index2]['x'])
                    ydist = abs(self.scannerCoordinates[index1]['y'] - self.scannerCoordinates[index2]['y'])
                    zdist = abs(self.scannerCoordinates[index1]['z'] - self.scannerCoordinates[index2]['z'])
                    manhattanSum = xdist + ydist + zdist
                if manhattanSum > maxManhattanSum:
                    maxManhattanSum = manhattanSum
        
        return maxManhattanSum
            
    def __updateBeacon(self, coordinates, beacon, beaconInit):
        
        coorStrings = ["x", "y", "z"]
        
        for coor in coorStrings:
            coordinate = coordinates[coor]
            operator = self.lastOperation[coor]
            
            if operator == Operator.PLUSX:
                beacon[coor] = -beaconInit['x'] + coordinate
            elif operator == Operator.MINUSX:
                beacon[coor] = beaconInit['x'] + coordinate
            elif operator == Operator.PLUSY:
                beacon[coor] = -beaconInit['y'] + coordinate
            elif operator == Operator.MINUSY:
                beacon[coor] = beaconInit['y'] + coordinate  
            elif operator== Operator.PLUSZ:
                beacon[coor] = -beaconInit['z'] + coordinate
            elif operator == Operator.MINUSZ:
                beacon[coor] = beaconInit['z'] + coordinate
            else:
                raise Exception("operator is not known")
     
    def __findCommonCoordinate(self, scanner1:Scanner, scanner2:Scanner, forString:str):
        diffList = []
        coordinateOutput = None
        operators = [Operator.MINUSX, Operator.PLUSX, Operator.MINUSY, Operator.PLUSY, Operator.MINUSZ, Operator.PLUSZ]

        for operator in operators:
            for index1 in range (0, len(scanner1.positions)):
                for index2 in range(0,len(scanner2.positions)):
                    if operator == Operator.MINUSX:
                        diff = scanner1.positions[index1][forString] - scanner2.positions[index2]["x"]
                    elif operator == Operator.PLUSX:
                        diff = scanner1.positions[index1][forString] + scanner2.positions[index2]["x"]
                    elif operator == Operator.MINUSY:
                        diff = scanner1.positions[index1][forString] - scanner2.positions[index2]["y"]
                    elif operator == Operator.PLUSY:
                        diff = scanner1.positions[index1][forString] + scanner2.positions[index2]["y"]
                    elif operator == Operator.MINUSZ:
                        diff = scanner1.positions[index1][forString] - scanner2.positions[index2]["z"]
                    elif operator == Operator.PLUSZ:
                        diff = scanner1.positions[index1][forString] + scanner2.positions[index2]["z"]
                    diffList.append(diff)
                
            coordinate = most_frequent(diffList)
            if coordinate["count"] >= 12:
                coordinateOutput = coordinate["number"]
                self.lastOperation[forString] = operator
            
            diffList = []
            
        return coordinateOutput
        
def solutionDay19():
    scanManager = ScanManager()

    scanManager.readInputDataIntoScanners("input/inputday19")
    scanManager.addBeaconsRelativeToScanner(0)
    
    answerPart1 = scanManager.getLengthBeacons()
    answerPart2 = scanManager.getLargestManhattanDistance()
    
    printAnswer(19, answerPart1, answerPart2)