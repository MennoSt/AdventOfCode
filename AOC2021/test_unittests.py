import unittest
from SolutionDay21 import DiceRoller


from solutions.SolutionDay01 import SubmarineCalculator
from solutions.SolutionDay02 import PositionCalculator
from solutions.SolutionDay03 import PowerManager
from solutions.SolutionDay04 import BingoManager
from solutions.SolutionDay05 import VentDetector
from solutions.SolutionDay06 import LaunterFishCalculator
from solutions.SolutionDay07 import CrabPositioner
from solutions.SolutionDay08 import PatternAnalyzer,PatternReader
from solutions.SolutionDay09 import RiskCalculator
from solutions.SolutionDay10 import SyntaxScoreBoard, SyntaxManager
from solutions.SolutionDay11 import OctopusFlashManager
from solutions.SolutionDay12 import CaveNavigator
from solutions.SolutionDay13 import Folder
from solutions.SolutionDay14 import Polymarizator
from solutions.SolutionDay15 import Dijkstra
from solutions.SolutionDay16 import Packet, PacketDecoder
from solutions.SolutionDay17 import *
from solutions.SolutionDay18 import SnailFishUpdater
from solutions.SolutionDay19 import ScanManager
from solutions.SolutionDay20 import Trenchmapper
from SolutionDay21 import DiceRoller


class Test_SumbarineCalculator(unittest.TestCase):

    def setUp(self):
        self.submarineCalculator = SubmarineCalculator()
        self.testdata = [199,200,208,210,200,207,240,269,260,263]
    
    def test_CalculateDepthIncreases_Part1_Example1(self):
        increases = self.submarineCalculator.GetIncreasedDepthsPart1(self.testdata)
        self.assertEqual(increases, 7)
    
    def test_CalculateDepthIncreases_Part1_Example1(self):
        increases = self.submarineCalculator.GetIncreasedDepthsPart2(self.testdata)
        self.assertEqual(increases, 5)

class Test_PositionCalculator(unittest.TestCase):

    def setUp(self):
        self.positionCalculator = PositionCalculator()
        self.testdataAmmount = [5,5,8,3,8,2]
        self.testdataDirection = ["forward", "down", "forward", "up", "down", "forward"]
    
    def test_CalculateMultiplication(self):
        multiplication = self.positionCalculator.getMultiplicationPart1(self.testdataDirection, self.testdataAmmount)
        self.assertEqual(multiplication, 150)

    def test_CalculateMultiplication(self):
        multiplicationPart2 = self.positionCalculator.getMultiplicationPart2(self.testdataDirection, self.testdataAmmount)
        self.assertEqual(multiplicationPart2, 900)

class Test_PowerManager(unittest.TestCase):

    def setUp(self):
        self.powerManager = PowerManager()
        self.testdataPower = ["00100", 
                              "11110", 
                              "10110", 
                              "10111",
                              "10101",
                              "01111",
                              "00111",
                              "11100",
                              "10000",
                              "11001",
                              "00010",
                              "01010"]

    def test_CalculatePowerConsumption(self):
        consumedPower = self.powerManager.calculateRequiredPower(self.testdataPower)
        self.assertEqual(consumedPower, 198)
    
    def test_CalculateLifeSupport(self):
        lifeSupport = self.powerManager.calculateLifeSupport(self.testdataPower)
        self.assertEqual(lifeSupport, 230)


class Test_BingoManager(unittest.TestCase):
    
    def setUp(self):
        self.bingoFileReader = FileReader()
        self.bingoFileReader.readBingoFile("testinput/unittestinputday4")
        self.bingoManager = BingoManager()
        self.bingoNumbers = self.bingoFileReader.bingoNumbers
        self.intChartArray = self.bingoFileReader.intChartArray
        self.bingoManager.createBingoCharts(self.intChartArray)

    def test_CalculateBingoManagerPart1(self):

        bingoScore = self.bingoManager.getFirstBingoScore(self.bingoNumbers)
        self.assertEqual(bingoScore, 4512)
    
    def test_CalculateBingoManagerPart2(self):
        bingoScore = self.bingoManager.getLastBingoScore(self.bingoNumbers)
        self.assertEqual(bingoScore, 1924)

class Test_TestVentDetector(unittest.TestCase):
    
    def setUp(self):
        self.ventDetector = VentDetector()
        self.ventDetector.readVentFile("testinput/unittestinputday5")

    def test_CalculateOverlapsPart1(self):
        numberOfOverlaps = self.ventDetector.getNumberOfOverlapsPart1()
        self.assertEqual(numberOfOverlaps, 5)

    def test_CalculateOverlapsPart2(self):
        numberOfOverlaps = self.ventDetector.getNumberOfOverlapsPart2()
        self.assertEqual(numberOfOverlaps, 12)

class Test_LaunterFishCalculator(unittest.TestCase):
    
    def setUp(self):
        self.launterFishCalculator = LaunterFishCalculator()
        self.initialState = [3,4,3,1,2]

    def test_calculateFishesAfter18Days(self):
        days = 18
        totalFishes = self.launterFishCalculator.getNumberOfFishes(self.initialState, days)
        self.assertEqual(totalFishes, 26)

    def test_calculateFishesAfter80Days(self):
        days = 80
        totalFishes = self.launterFishCalculator.getNumberOfFishes(self.initialState, days)
        self.assertEqual(totalFishes, 5934)

    def test_calculateFishesAfter256Days(self):
        days = 256
        totalFishes = self.launterFishCalculator.getNumberOfFishes(self.initialState, days)
        self.assertEqual(totalFishes, 26984457539)

class Test_CrabPositioner(unittest.TestCase):
    
    def setUp(self):
        self.crabPositioner = CrabPositioner()
        self.testPositions = [16,1,2,0,4,2,7,1,2,14]

    def test_CalculateLeastFuel(self):
        leastFuel = self.crabPositioner.getLeastAmmountOfFuel(self.testPositions)
        self.assertEqual(leastFuel, 37)

    def test_CalculateLeastFuel_IncreasedRate(self):
        leastFuel = self.crabPositioner.getLeastAmmountOfFuel(self.testPositions, True)
        self.assertEqual(leastFuel, 168)


class Test_PatternAnalyzer(unittest.TestCase):
    
    def setUp(self):
        self.patternReader = PatternReader()
        self.patternAnalyser = PatternAnalyzer()
        self.patternReader.readPattern("testinput/unittestinputday8")

    def test_CalculateNumberOfDigitInstances(self):

        digitCounts = self.patternAnalyser.getNumberOfDigitInstances(self.patternReader.fourDigitArray)
        self.assertEqual(digitCounts, 26)

    def test_CalculateSumAdditionPart2(self):

        fourDigitArray = self.patternReader.fourDigitArray
        uniquePatternArray = self.patternReader.uniquePatternArray
        digitCounts = self.patternAnalyser.calculateSumOfFourDigits(fourDigitArray, uniquePatternArray)
        self.assertEqual(digitCounts, 61229)


class Test_RiskCalculator(unittest.TestCase):
    
    def setUp(self):
        self.fileReader = FileReader()
        self.riskCalculator = RiskCalculator()
        heightMap = self.fileReader.readToIntMap("testinput/unittestinputday9")
        self.riskCalculator.setHeightMap(heightMap)

    def test_calculateSum(self):
        riskSum = self.riskCalculator.calculateSumRiskLevels()
        self.assertEqual(riskSum, 15)

    def test_calculateInitalBassin(self):
        bassin = self.riskCalculator.createInitalBassin(0,0)
        self.assertEqual(len(bassin), 3)
        bassin = self.riskCalculator.createInitalBassin(0,1)
        self.assertEqual(len(bassin), 2)
        bassin = self.riskCalculator.createInitalBassin(1,0)
        self.assertEqual(len(bassin), 2)
        bassin = self.riskCalculator.createInitalBassin(8,4)
        self.assertEqual(len(bassin), 6)

    def test_calculateInitalBassinPerIndex(self):
        self.riskCalculator.createInitialBassinsPerIndex()
        multiplicationLargestThree = self.riskCalculator.getMultiplicationLargest3Bassins()
        self.assertEqual(multiplicationLargestThree, 1134)


class Test_SyntaxErrorDetector(unittest.TestCase):
    
    def setUp(self):
        self.syntaxErrorDetector = SyntaxManager()
        self.filereader = FileReader()
        self.navigationLines = self.filereader.readLinesToStringArray("testinput/unittestinputday10")

    def test_checkSyntaxErrorExampleOne(self):
        charSyntaxError = self.syntaxErrorDetector.checkOnSyntaxError("<}")
        self.assertEqual(charSyntaxError,"}")
    
    def test_checkSyntaxErrorExampleTwo(self):
        charSyntaxError = self.syntaxErrorDetector.checkOnSyntaxError("{()()()>")
        self.assertEqual(charSyntaxError,">")
    
    def test_checkSyntaxErrorExampleThree(self):
        charSyntaxError = self.syntaxErrorDetector.checkOnSyntaxError("(((()))}")
        self.assertEqual(charSyntaxError,"}")
    
    def test_checkSyntaxErrorExampleFour(self):
        charSyntaxError = self.syntaxErrorDetector.checkOnSyntaxError("<([]){()}[{}])")
        self.assertEqual(charSyntaxError,")")

    def test_foundErrorClosingBracket(self):
        secondCharSyntaxError = self.syntaxErrorDetector.checkOnSyntaxError("{([(<{}[<>[]}>{[]{[(<()>")
        self.assertEqual(secondCharSyntaxError, "}")
    
    def test_foundErrorRoundBracket(self):
        charSytaxError = self.syntaxErrorDetector.checkOnSyntaxError("[[<[([]))<([[{}[[()]]]")
        self.assertEqual(charSytaxError, ")")

    def test_foundErrorSquareBracket(self):
        charSytaxError = self.syntaxErrorDetector.checkOnSyntaxError("[{[{({}]{}}([{[{{{}}([]")
        self.assertEqual(charSytaxError, "]")
    
    def test_foundErrorRoundBracket_Two(self):
        charSytaxError = self.syntaxErrorDetector.checkOnSyntaxError("[<(<(<(<{}))><([]([]()")
        self.assertEqual(charSytaxError, ")")
    
    def test_foundErrorArrow(self):
        charSytaxError = self.syntaxErrorDetector.checkOnSyntaxError("<{([([[(<>()){}]>(<<{{")
        self.assertEqual(charSytaxError, ">")
    
    def test_foundErrorArrow(self):
        charSytaxError = self.syntaxErrorDetector.checkOnSyntaxError("<{([([[(<>()){}]>(<<{{")
        self.assertEqual(charSytaxError, ">")
    
    def test_autocompleteStringExampleOne(self):
        addedString = self.syntaxErrorDetector.autoCompleteLine("[({(<(())[]>[[{[]{<()<>>")
        self.assertEqual(addedString, "}}]])})]")
    
    def test_autocompleteStringExampleTwo(self):
        addedString = self.syntaxErrorDetector.autoCompleteLine("[(()[<>])]({[<{<<[]>>(")
        self.assertEqual(addedString, ")}>]})")
    
    def test_autocompleteStringExampleThree(self):
        addedString = self.syntaxErrorDetector.autoCompleteLine("(((({<>}<{<{<>}{[]{[]{}")
        self.assertEqual(addedString, "}}>}>))))")
    
    def test_autocompleteStringExampleFour(self):
        addedString = self.syntaxErrorDetector.autoCompleteLine("{<[[]]>}<{[{[{[]{()[[[]")
        self.assertEqual(addedString, "]]}}]}]}>")
    
    def test_autocompleteStringExampleFive(self):
        addedString = self.syntaxErrorDetector.autoCompleteLine("<{([{{}}[<[[[<>{}]]]>[]]")
        self.assertEqual(addedString, "])}>")

    def test_calculateSyntaxErrorScore(self):
        syntaxErrorScore = self.syntaxErrorDetector.calculateSyntaxErrorScore(self.navigationLines)
        self.assertEqual(syntaxErrorScore, 26397)

    def test_calculateMiddleScore(self):
        middleScore = self.syntaxErrorDetector.calculateMiddleScore(self.navigationLines)
        self.assertEqual(middleScore, 288957)

class Test_SyntaxScoreBoard(unittest.TestCase):
    
    def setUp(self):
        self.syntaxScoreBoard = SyntaxScoreBoard()

    def test_calculateScore(self):
        testarray = ["}",">","]",")","}"]
        score = self.syntaxScoreBoard.calculateScore(testarray)
        expectedScore = 1197 + 25137 + 57 + 3 + 1197
        self.assertEqual(score, expectedScore)
    
    def test_calculateScoreOne(self):
        teststring = "])}>"
        score = self.syntaxScoreBoard.calculateAutoCompletionScore(teststring)
        expectedScore = 294
        self.assertEqual(score, expectedScore)

    def test_calculateScoreTwo(self):
        teststring = "}}]])})]"
        score = self.syntaxScoreBoard.calculateAutoCompletionScore(teststring)
        expectedScore = 288957
        self.assertEqual(score, expectedScore)

class Test_OctopusFlashManager(unittest.TestCase):
    
    def setUp(self):
        self.octopusFlashManager = OctopusFlashManager()
        self.filereader = FileReader()
        self.input1 = self.filereader.readToIntMap("testinput/unittestinputday11_1")
        self.input2 = self.filereader.readToIntMap("testinput/unittestinputday11_2")

    def test_calculateFlashesAfter1Day(self):
        self.octopusFlashManager.setOctopusMap(self.input1)
        score = self.octopusFlashManager.getNumberOfFlashes(4)
        self.assertEqual(score, 9)

    def test_calculateFlashesAfter100Days(self):
        self.octopusFlashManager.setOctopusMap(self.input2)
        score = self.octopusFlashManager.getNumberOfFlashes(100)
        self.assertEqual(score, 1656)

    def test_calculateFirstSynchroinicCycle(self):
        self.octopusFlashManager.setOctopusMap(self.input2)
        cycle = self.octopusFlashManager.getFirstSynchronicCycle()
        self.assertEqual(cycle, 195)

class Test_CaveNavigator(unittest.TestCase):
    
    def setUp(self):
        self.caveNavigator = CaveNavigator()
        self.filereader = FileReader()
        self.input1 = self.filereader.readLinesToStringArray("testinput/unittestinputday12_1")
        self.input2 = self.filereader.readLinesToStringArray("testinput/unittestinputday12_2")
        self.input3 = self.filereader.readLinesToStringArray("testinput/unittestinputday12_3")
    
    def test_findNumberOfRoutesInput1(self):
        routes = self.caveNavigator.findNumberOfRoutes(self.input1)
        self.assertEqual(routes, 10)
        
    def test_findNumberOfRoutesInput2(self):
        routes = self.caveNavigator.findNumberOfRoutes(self.input2)
        self.assertEqual(routes, 19)
        
    def test_findNumberOfRoutesInput3(self):
        routes = self.caveNavigator.findNumberOfRoutes(self.input3)
        self.assertEqual(routes, 226)

    def test_findNumberOfRoutesInput1Part2(self):
        routes = self.caveNavigator.findNumberOfRoutes(self.input1, True)
        self.assertEqual(routes, 36)
        
    def test_findNumberOfRoutesInput2Part2(self):
        routes = self.caveNavigator.findNumberOfRoutes(self.input2, True)
        self.assertEqual(routes, 103)
        
    def test_findNumberOfRoutesInput3Part2(self):
        routes = self.caveNavigator.findNumberOfRoutes(self.input3, True)
        self.assertEqual(routes, 3509)
        

class Test_FolderReader(unittest.TestCase):
    
    def setUp(self):
        self.folder = Folder()
        self.folder.importInputFiles("testinput/unittestinputday13_1")

    def test_findNumberOfDots(self):
        dots = self.folder.CalculateNumberOfDots(True)
        self.assertEqual(dots, 17)
    
    def test_findNumberOfDots2(self):
        dots = self.folder.CalculateNumberOfDots()
        self.assertEqual(dots, 16)
    
    def test_findNumberOfDots3(self):
        self.folder.importInputFiles("testinput/unittestinputday13_2")
        dots = self.folder.CalculateNumberOfDots()
        self.assertEqual(dots, 5)

class Test_Polymarizator(unittest.TestCase):
    
    def setUp(self):
        self.fileReader = FileReader()
        self.polymarizator = Polymarizator()
        data = self.fileReader.readPolymerTemplate("testinput/unittestinputday14")
        self.initialString = data[0]
        self.polyPairs = data[1]

    def test_calculateMostCommonMinusLeastCommon10Steps(self):
        steps = 10
        difference = self.polymarizator.calculateDifference(self.initialString, self.polyPairs, steps)
        self.assertEqual(difference, 1588)
        
    def test_calculateMostCommonMinusLeastCommon40Steps(self):
        steps = 40
        string = self.polymarizator.calculateDifference(self.initialString, self.polyPairs, steps)
        self.assertEqual(string, 2188189693529)

class Test_DijkstraTests(unittest.TestCase):
    
    def setUp(self):
        self.fileReader = FileReader()
        self.dijkstra = Dijkstra()
        self.inputMap1 = self.fileReader.readToIntMap("testinput/unittestinputday15_1")
        self.inputMap2 = self.fileReader.readToIntMap("testinput/unittestinputday15_2")
        self.inputMap3 = self.fileReader.readToIntMap("testinput/unittestinputday15_3")
        
    def test_findShortestPathMap1(self):
        self.dijkstra.setGrid(self.inputMap1)
        shortestPath1 = self.dijkstra.findShortestPath()
        self.assertEqual(shortestPath1, 14)
        
    def test_findShortestPathMap2(self):
        self.dijkstra.setGrid(self.inputMap2)
        shortestPath2 = self.dijkstra.findShortestPath()
        self.assertEqual(shortestPath2, 40)
    
    def test_findShortestPathExtendedMap2(self):
        self.dijkstra.setGrid(self.inputMap2, True)
        shortestPath3 = self.dijkstra.findShortestPath()
        self.assertEqual(shortestPath3, 315)
    
    def test_findShortestPathExtendedMap3(self):
        self.dijkstra.setGrid(self.inputMap3)
        shortestPath3 = self.dijkstra.findShortestPath()
        self.assertEqual(shortestPath3, 12)

class Test_PackerDecoder(unittest.TestCase):
    
    def setUp(self):
        self.packetDecoder = PacketDecoder()
        self.examplePackage1 = "D2FE28"
        self.examplePackage2 = "38006F45291200"
        self.examplePackage3 = "EE00D40C823060"
        
    def test_hexToBinaryCoversionTest1(self):
        binString = self.packetDecoder.convertToBinaryString(self.examplePackage1)
        self.assertEqual(binString, "110100101111111000101000")

    def test_hexToBinaryCoversionTest2(self):
        binString = self.packetDecoder.convertToBinaryString(self.examplePackage2)
        self.assertEqual(binString, "00111000000000000110111101000101001010010001001000000000")
 
    def test_hexToBinaryCoversionTest3(self):
        binString = self.packetDecoder.convertToBinaryString(self.examplePackage3)
        self.assertEqual(binString, "11101110000000001101010000001100100000100011000001100000")               
                
    def test_createLiteralPackage(self):
        self.packetDecoder.decodeStringToPackage(self.examplePackage1)
        packet = self.packetDecoder.packet
        self.assertEqual(packet.version, 6)
        self.assertEqual(packet.typeID, 4)
        self.assertEqual(packet.literalValue, 2021)
        
        versionSum = self.packetDecoder.versionSum
        self.assertEqual(versionSum, 6)
        
    def test_createOperatorPackageExampleTwo(self):
        self.packetDecoder.decodeStringToPackage(self.examplePackage2)
        packet = self.packetDecoder.packet
        
        self.assertEqual(packet.version, 1)
        self.assertEqual(packet.typeID, 6)
        self.assertEqual(packet.length, 27)        
        self.assertEqual(packet.lengthTypeId, '0')
        self.assertEqual(packet.subPackages[0].literalValue, 10)
        self.assertEqual(packet.subPackages[1].literalValue, 20)
        
        versionSum = self.packetDecoder.versionSum
        self.assertEqual(versionSum, 9)
        
    def test_createOperatorPackageExampleThree(self):
        packet = Packet()
        self.packetDecoder.decodeStringToPackage(self.examplePackage3)
        packet = self.packetDecoder.packet
        
        self.assertEqual(packet.version, 7)
        self.assertEqual(packet.typeID, 3)     
        self.assertEqual(packet.lengthTypeId, '1')
        self.assertEqual(packet.subPackages[0].literalValue, 1)
        self.assertEqual(packet.subPackages[1].literalValue, 2)
        self.assertEqual(packet.subPackages[2].literalValue, 3) 
        
        versionSum = self.packetDecoder.versionSum
        self.assertEqual(versionSum, 14)
        
    def test_createOperatorPackageExampleFour(self):
        
        self.packetDecoder.decodeStringToPackage("8A004A801A8002F478")
        versionSum = self.packetDecoder.versionSum
        self.assertEqual(versionSum, 16)
        
    def test_createOperatorPackageExampleFive(self):
        self.packetDecoder.decodeStringToPackage("620080001611562C8802118E34")
        versionSum = self.packetDecoder.versionSum
        self.assertEqual(versionSum, 12)
        
    def test_createOperatorPackageExampleSix(self):
        self.packetDecoder.decodeStringToPackage("C0015000016115A2E0802F182340")
        versionSum = self.packetDecoder.versionSum
        self.assertEqual(versionSum, 23)
        
    def test_createOperatorPackageExampleSeven(self):
        self.packetDecoder.decodeStringToPackage("A0016C880162017C3686B18A3D4780")
        versionSum = self.packetDecoder.versionSum
        self.assertEqual(versionSum, 31)        

    def test_resultingValueID0(self):
        self.packetDecoder.decodeStringToPackage("C200B40A82")
        result = self.packetDecoder.calculateResultingValue()
        
        self.assertEqual(result, 3)
        
    def test_resultingValueID1(self):
        self.packetDecoder.decodeStringToPackage("04005AC33890")
        result = self.packetDecoder.calculateResultingValue()
        
        self.assertEqual(result, 54)
        
    def test_resultingValueID2(self):
        self.packetDecoder.decodeStringToPackage("880086C3E88112")
        result = self.packetDecoder.calculateResultingValue()
        
        self.assertEqual(result, 7)
        
    def test_resultingValueID3(self):
        self.packetDecoder.decodeStringToPackage("CE00C43D881120")
        result = self.packetDecoder.calculateResultingValue()
        
        self.assertEqual(result, 9)
        
    def test_resultingValueID5(self):
        self.packetDecoder.decodeStringToPackage("D8005AC2A8F0")
        result = self.packetDecoder.calculateResultingValue()
        
        self.assertEqual(result, 1)

    def test_resultingValueID6(self):
        self.packetDecoder.decodeStringToPackage("F600BC2D8F")
        result = self.packetDecoder.calculateResultingValue()
        
        self.assertEqual(result, 0)
        
    def test_resultingValueID7(self):
        self.packetDecoder.decodeStringToPackage("9C005AC2F8F0")
        result = self.packetDecoder.calculateResultingValue()
        
        self.assertEqual(result, 0)

    def test_resultingValueIDCombined(self):
        self.packetDecoder.decodeStringToPackage("9C0141080250320F1802104A08")
        result = self.packetDecoder.calculateResultingValue()
        
        self.assertEqual(result, 1)  

class Test_ProbeLauncher(unittest.TestCase):
    
    def setUp(self):
        self.probeLauncher = ProbeLauncher()
        self.probeLauncher.fillTargetArea("testinput/unittestinputday17")
        
    def test_testTargetArea(self):
        inTargetArea = self.probeLauncher.isWithinTargetArea(20,-10)
        self.assertEqual(inTargetArea, True)
        inTargetArea = self.probeLauncher.isWithinTargetArea(30,-5)
        self.assertEqual(inTargetArea, True)
        inTargetArea = self.probeLauncher.isWithinTargetArea(25,-8)
        self.assertEqual(inTargetArea, True)
        
        inTargetArea = self.probeLauncher.isWithinTargetArea(31,-10)
        self.assertEqual(inTargetArea, False)
        inTargetArea = self.probeLauncher.isWithinTargetArea(19,-5)
        self.assertEqual(inTargetArea, False)
        inTargetArea = self.probeLauncher.isWithinTargetArea(30,-11)
        self.assertEqual(inTargetArea, False)
        inTargetArea = self.probeLauncher.isWithinTargetArea(20,-4)
        self.assertEqual(inTargetArea, False)
        inTargetArea = self.probeLauncher.isWithinTargetArea(1000,3)
        self.assertEqual(inTargetArea, False)
        inTargetArea = self.probeLauncher.isWithinTargetArea(3,1000)
        self.assertEqual(inTargetArea, False)
        inTargetArea = self.probeLauncher.isWithinTargetArea(3,1000)
        self.assertEqual(inTargetArea, False)
        

    def test_testReachesTarget(self):
        self.probeLauncher.maxYCoordinate = 0
        inTargetArea = self.probeLauncher.reachesTargetArea(7,2)
        self.assertEqual(inTargetArea, True)
        self.assertEqual(self.probeLauncher.position, {"x" : 28 , "y": -7 })
        self.assertEqual(self.probeLauncher.maxYCoordinate, 3)
        
        self.probeLauncher.maxYCoordinate = 0        
        inTargetArea = self.probeLauncher.reachesTargetArea(6,3)
        self.assertEqual(inTargetArea, True)
        self.assertEqual(self.probeLauncher.position, {"x" : 21 , "y": -9 })
        self.assertEqual(self.probeLauncher.maxYCoordinate, 6)
        
        self.probeLauncher.maxYCoordinate = 0
        inTargetArea = self.probeLauncher.reachesTargetArea(9,0)
        self.assertEqual(inTargetArea, True)
        self.assertEqual(self.probeLauncher.position, {"x" : 30 , "y": -6 })
        self.assertEqual(self.probeLauncher.maxYCoordinate, 0)

        self.probeLauncher.maxYCoordinate = 0        
        inTargetArea = self.probeLauncher.reachesTargetArea(17,-4)
        self.assertEqual(inTargetArea, False)
        self.assertEqual(self.probeLauncher.position, {"x" : 33 , "y": -9 })
        self.assertEqual(self.probeLauncher.maxYCoordinate, 0)

    def test_getMaxYPosition(self):
        self.probeLauncher.determineMaxYCoordinate()
        maxYPosition = self.probeLauncher.maxYCoordinate
        InitialVelocity = self.probeLauncher.maxYInitialVelocity
        
        self.assertEqual(maxYPosition, 45)
        self.assertEqual(InitialVelocity, {"vx" : 6 , "vy": 9 })

    def test_getNumberOfTargetVelocities(self):
        self.probeLauncher.determineMaxYCoordinate()
        counter = self.probeLauncher.targetCounter
        self.assertEqual(counter, 112)


class Test_SnailFishUpdater(unittest.TestCase):
    
    def setUp(self):
        self.snailFishUpdater = SnailFishUpdater()
        
    def test_testMutationExampleOnePartOne(self):
        testList = [[[[[9,8],1],2],3],4]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[[[0,9],2],3],4])
        
    def test_testMutationExampleOnePartTwo(self):
        testList = [[[[1,[8,8],1],2],3],4]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[[[9,0,9],2],3],4])

    def test_testMutationExampleTwo(self):
        testList = [7,[6,[5,[4,[3,2]]]]]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [7,[6,[5,[7,0]]]])

    def test_testMutationExampleThreePartOne(self):
        testList = [[6,[5,[4,[3,2]]]],1]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[6,[5,[7,0]]],3])

    def test_testMutationExampleThreePartTwo(self):
        testList = [[6,[5,[4,[3,2]]],1],1]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[6,[5,[7,0]],3],1])
        
    def test_testMutationExampleThreePartThree(self):
        testList = [[6,[5,[4,[3,2]],1],1],1]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[6,[5,[7,0],3],1],1])

    def test_testMutationExampleFour(self):
        testList = [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[3,[2,[8,0]]],[9,[5,[7,0]]]])

    def test_MutationExampleFivePart1(self):
        testList = [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[[[0,7],4],[15,[0,13]]],[1,1]])
        
    def test_MutationExampleFivePart2(self):
        testList = [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[[[0,7],4],[15,[0,13]]],[1,1]])

    def test_MutationSplitExample1Part1(self):
        testList = [11, 12, 13,[2,13]]
        mutatedTestList = self.snailFishUpdater.updateListWithSplits(testList)
        self.assertEqual(mutatedTestList, [[5, 6], 12, 13, [2, 13]])

    def test_MutationSplitExample1Part2(self):
        testList = [[[2,13],2], 11, 12, 13]
        mutatedTestList = self.snailFishUpdater.updateListWithSplits(testList)
        self.assertEqual(mutatedTestList, [[[2, [6, 7]], 2], 11, 12, 13])
        
    def testMutationWithExample2(self):
        testList = [[[[0,7],4],[15,[0,13]]],[1,1]]
        mutatedTestList = self.snailFishUpdater.updateListWithSplits(testList)
        self.assertEqual(mutatedTestList, [[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]])
        
    def testMutationWithExample3(self):
        testList = [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
        mutatedTestList = self.snailFishUpdater.updateListWithExplosions(testList)
        self.assertEqual(mutatedTestList, [[[[0,7],4],[[7,8],[6,0]]],[8,1]])
        
    def testUpdateSnailFishSum1(self):
        testList = [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
        mutatedTestList = self.snailFishUpdater.updateSnailFishNumbers(testList)
        self.assertEqual(mutatedTestList, [[[[0,7],4],[[7,8],[6,0]]],[8,1]])

class Test_SnailFishUpdaterWithInputData(unittest.TestCase):
    
    def setUp(self):
        self.snailFishUpdater = SnailFishUpdater()
        
    def test_testMutationExampleOnePartOne(self):
        self.snailFishUpdater.readDataIntoLists("testinput/unittestinputday18_1")
        finalSum = self.snailFishUpdater.calculateFinalSum()
        self.assertEqual(finalSum, [[[[1,1],[2,2]],[3,3]],[4,4]])
        
    def test_testMutationExampleOnePartTwo(self):
        self.snailFishUpdater.readDataIntoLists("testinput/unittestinputday18_2")
        finalSum = self.snailFishUpdater.calculateFinalSum()
        self.assertEqual(finalSum, [[[[3,0],[5,3]],[4,4]],[5,5]])

    def test_testMutationExampleOnePartThree(self):
        self.snailFishUpdater.readDataIntoLists("testinput/unittestinputday18_3")
        finalSum = self.snailFishUpdater.calculateFinalSum()
        self.assertEqual(finalSum, [[[[5,0],[7,4]],[5,5]],[6,6]])
    
    def test_testMutationExampleOnePartFour(self):
        self.snailFishUpdater.readDataIntoLists("testinput/unittestinputday18_4")
        finalSum = self.snailFishUpdater.calculateFinalSum()
        self.assertEqual(finalSum, [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]])

    def test_testMutationExampleOnePartFive(self):
        self.snailFishUpdater.readDataIntoLists("testinput/unittestinputday18_5")
        finalSum = self.snailFishUpdater.calculateFinalSum()
        self.assertEqual(finalSum, [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]])

    def test_calculateMagnitudeExampleOne(self):
        inputString = [[9,1],[1,9]]
        magnitude = self.snailFishUpdater.calculateMagnitude(inputString)
        self.assertEqual(magnitude, 129)
        
    def test_calculateMagnitudeExampleTwo(self):
        inputString = [[1,2],[[3,4],5]]
        magnitude = self.snailFishUpdater.calculateMagnitude(inputString)
        self.assertEqual(magnitude, 143)

    def test_calculateMagnitudeExampleThree(self):
        inputString = [[[[1,1],[2,2]],[3,3]],[4,4]]
        magnitude = self.snailFishUpdater.calculateMagnitude(inputString)
        self.assertEqual(magnitude, 445)

    def test_calculateMagnitudeExampleFour(self):
        inputString = [[[[3,0],[5,3]],[4,4]],[5,5]]
        magnitude = self.snailFishUpdater.calculateMagnitude(inputString)
        self.assertEqual(magnitude, 791)

    def test_calculateMagnitudeExampleFive(self):
        inputString = [[[[5,0],[7,4]],[5,5]],[6,6]]
        magnitude = self.snailFishUpdater.calculateMagnitude(inputString)
        self.assertEqual(magnitude, 1137)

    def test_calculateMagnitudeExampleSix(self):
        inputString = [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
        magnitude = self.snailFishUpdater.calculateMagnitude(inputString)
        self.assertEqual(magnitude, 3488)

    def test_calculateMagnitudeExampleSeven(self):
        inputString = [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]
        magnitude = self.snailFishUpdater.calculateMagnitude(inputString)
        self.assertEqual(magnitude, 4140)

    def test_calculateMagnitudeExampleFinal(self):
        self.snailFishUpdater.readDataIntoLists("testinput/unittestinputday18_6")
        finalSum = self.snailFishUpdater.calculateFinalSum()
        magnitude = self.snailFishUpdater.calculateMagnitude(finalSum)
        self.assertEqual(finalSum, [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]])
        self.assertEqual(magnitude, 4140)

    def test_calculateHighestMagnitudeSum(self):
        self.snailFishUpdater.readDataIntoLists("testinput/unittestinputday18_6")
        magnitude = self.snailFishUpdater.determineLargestMagnitueOfSums()
        self.assertEqual(magnitude, 3993) 

def readTestAnswerIntoDictionary(answerTestData):
    with open (answerTestData, "r") as myfile:
        dataread = myfile.read().rstrip()
        chartData = dataread.split('\n\n')
        for data in chartData:
            scanData = data.split("\n")
            positions = []
            for line in scanData:
                line = list(map(int,line.split(",")))
                position = {"x" : line[0], "y" :line[1], "z": line[2]}
                positions.append(position)
    
    return positions

class Test_ScanManager(unittest.TestCase):
    
    def setUp(self):
        self.scanManager = ScanManager()
        self.testanswer = readTestAnswerIntoDictionary("testinput/unittestinputday19_answer")
    
    def test_calculateGetCommonBeacons(self):
        self.scanManager.readInputDataIntoScanners("testinput/unittestinputday19")
        overlaps = self.scanManager.coordinatesBetweenScanners(self.scanManager.scanners[0], self.scanManager.scanners[1])
        self.assertEqual(overlaps, {'x': 68, 'y': -1246, 'z': -43})
        
    def test_getLengthBeacons(self):
        self.scanManager.readInputDataIntoScanners("testinput/unittestinputday19")
        self.scanManager.addBeaconsRelativeToScanner(0)
        length = self.scanManager.getLengthBeacons()
        
        self.assertEqual(length, 79)
        self.assertEqual(self.scanManager.beaconsRelativeToScanner, self.testanswer)
        self.assertEqual(self.scanManager.scannerCoordinates, [{'x': 0, 'y': 0, 'z': 0}, 
                                                               {'x': 68, 'y': -1246, 'z': -43}, 
                                                               {'x': -92, 'y': -2380, 'z': -20}, 
                                                               {'x': -20, 'y': -1133, 'z': 1061}, 
                                                               {'x': 1105, 'y': -1205, 'z': 1229}])

    def test_calculateLargestManhattanDistance(self):
        self.scanManager.readInputDataIntoScanners("testinput/unittestinputday19")
        self.scanManager.addBeaconsRelativeToScanner(0)
        largestManhattenDistance = self.scanManager.getLargestManhattanDistance()
        self.assertEqual(largestManhattenDistance, 3621)

class Test_trenchMapper(unittest.TestCase):
    
    def setUp(self):
        self.trenchMapper = Trenchmapper()
        self.trenchMapper.readData("testinput/unittestinputday20")
    
    def test_getUpdatedPixel(self):
        
        binaryNumber = self.trenchMapper.calculateBinaryNumber(2,2)
        self.assertEqual(binaryNumber,"000100010")
        
        updatedPixel = self.trenchMapper.getUpdatedPixel(2,2)
        self.assertEqual(updatedPixel, "#")

    def test_getUpdatedPixel2(self):
        updatedPixel = self.trenchMapper.getUpdatedPixel(0,0)
        self.assertEqual(updatedPixel, ".")

    def test_calculateLitPixelsInit(self):
        self.trenchMapper.enhanceImage(0)
        self.assertEqual(self.trenchMapper.lightPixels, 10)

    def test_calculateLitPixelsThreeTimes(self):
        self.trenchMapper.enhanceImage(2)
        self.assertEqual(self.trenchMapper.lightPixels, 35)

    def test_calculateLitPixelsThreeTimes(self):
        self.trenchMapper.enhanceImage(50)
        self.assertEqual(self.trenchMapper.lightPixels, 3351)


class Test_diceRoller(unittest.TestCase):
    
    def setUp(self):
        self.fileReader = FileReader()
        self.diceRoller = DiceRoller()
        fileString = self.fileReader.readLinesToStringArray("testinput/unittestinputday21")
        self.diceRoller.loadStartPositions(fileString)
    
    def test_rollCycleData(self):
        self.diceRoller.performNextRound()
        info = self.diceRoller.playerInfo
        self.assertEqual(info[0], {'player': 1, 'position': 10, 'score': 10})
        self.assertEqual(info[1], {'player': 2, 'position': 3, 'score': 3})
        
        self.diceRoller.performNextRound()
        info = self.diceRoller.playerInfo
        self.assertEqual(info[0], {'player': 1, 'position': 4, 'score': 14})
        self.assertEqual(info[1], {'player': 2, 'position': 6, 'score': 9})

        self.diceRoller.performNextRound()
        info = self.diceRoller.playerInfo
        self.assertEqual(info[0], {'player': 1, 'position': 6, 'score': 20})
        self.assertEqual(info[1], {'player': 2, 'position': 7, 'score': 16})
        
        self.diceRoller.performNextRound()
        info = self.diceRoller.playerInfo
        self.assertEqual(info[0], {'player': 1, 'position': 6, 'score': 26})
        self.assertEqual(info[1], {'player': 2, 'position': 6, 'score': 22})

    def test_gameScore(self):
        gameScore = self.diceRoller.determineGameScore()
        self.assertEqual(gameScore, 739785)

    def test_gameScore2(self):
        fileString = self.fileReader.readLinesToStringArray("input/inputday21")
        self.diceRoller.loadStartPositions(fileString)
        gameScore = self.diceRoller.determineGameScore()
        self.assertEqual(gameScore, 916083)

    def test_calculateUniverseWinsExample1(self):
        dicePossibilities = [{"throw":3, "count":1},{"throw":4, "count":1}]
        maxPoints = 8
        mostUniverseWins = self.diceRoller.mostUniverseWins(dicePossibilities, maxPoints)
        self.assertEqual(mostUniverseWins, 5)

    def test_calculateUniverseWinsExample2(self):
        dicePossibilities = [{"throw":3, "count":1},{"throw":7, "count":1}]
        maxPoints = 8
        mostUniverseWins = self.diceRoller.mostUniverseWins(dicePossibilities, maxPoints)
        self.assertEqual(mostUniverseWins, 10)

    def test_calculateUniverseWinsExample3(self):
        dicePossibilities = [{"throw":3, "count":1},{"throw":4, "count":1},{"throw":4, "count":1}]
        maxPoints = 8
        mostUniverseWins = self.diceRoller.mostUniverseWins(dicePossibilities, maxPoints)
        self.assertEqual(mostUniverseWins, 11)

    def test_calculateUniverseWinsExample4(self):
        dicePossibilities = [{"throw":3, "count":1},{"throw":4, "count":2}]
        maxPoints = 8
        mostUniverseWins = self.diceRoller.mostUniverseWins(dicePossibilities, maxPoints)
        self.assertEqual(mostUniverseWins, 11)
        
    def test_calculateUniverseWinsExample5(self):
        dicePossibilities = [{"throw":3, "count":2},{"throw":4, "count":1}]
        maxPoints = 8
        mostUniverseWins = self.diceRoller.mostUniverseWins(dicePossibilities, maxPoints)
        self.assertEqual(mostUniverseWins, 19)

    def test_calculateUniverseWinsExample6(self):
        dicePossibilities = [{"throw":3, "count":1}, {"throw":3, "count":1}, {"throw":4, "count":1}]
        maxPoints = 8
        mostUniverseWins = self.diceRoller.mostUniverseWins(dicePossibilities, maxPoints)
        self.assertEqual(mostUniverseWins, 19)
        
    def test_calculateUniverseWinsExample6(self):
        dicePossibilities = [{"throw":3, "count":1},
                             {"throw":4, "count":3},
                             {"throw":5, "count":6},
                             {"throw":6, "count":7},
                             {"throw":7, "count":6},
                             {"throw":8, "count":3},
                             {"throw":9, "count":1}]
        maxPoints = 21
        mostUniverseWins = self.diceRoller.mostUniverseWins(dicePossibilities, maxPoints)
        self.assertEqual(mostUniverseWins, 444356092776315)
        
        
def run_some_tests():
    # Run only the tests in the specified classes

    test_classes_to_run = [Test_diceRoller]

    loader = unittest.TestLoader()

    suites_list = []
    for test_class in test_classes_to_run:
        suite = loader.loadTestsFromTestCase(test_class)
        suites_list.append(suite)
        
    big_suite = unittest.TestSuite(suites_list)

    runner = unittest.TextTestRunner()
    results = runner.run(big_suite)
        
if __name__ == '__main__':
    run_some_tests()
    # unittest.main(defaultTest="Test_diceRollerFailed")
    # unittest.main()
