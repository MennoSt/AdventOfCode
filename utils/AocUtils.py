def printAnswer(day, answer1, answer2):
    print("The answer of Day " + str(day) + " part 1 is equal to " + str(answer1))
    print("The answer of Day " + str(day) + " part 2 is equal to " + str(answer2))

def most_frequent(List):
    counter = 0
    num = List[0]
     
    for i in List:
        curr_frequency = List.count(i)
        if(curr_frequency> counter):
            counter = curr_frequency
            num = i
    
    numberCount = {"number": num, "count":counter}
    
    return numberCount