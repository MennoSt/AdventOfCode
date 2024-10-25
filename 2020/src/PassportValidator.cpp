#include "PassportValidator.h"
#include <boost/algorithm/string.hpp>
#include "Utils.h"
#include <string>

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

const std::string c_byr = "byr:";
const std::string c_iyr = "iyr:";
const std::string c_eyr = "eyr:";
const std::string c_hgt = "hgt:";
const std::string c_hcl = "hcl:";
const std::string c_ecl = "ecl:";
const std::string c_pid = "pid:";
const std::string c_cid = "cid:";

PassportValidator::PassportValidator()
{

}

std::vector<std::string> PassportValidator::getManupPassportString(std::vector<std::string>& stringVector)
{
    std::vector<std::string> manupVector;
    std::string tempStr = "";

    for (std::string passportStr:stringVector) {
        if (passportStr != "") {
            tempStr.append(" " + passportStr);
        } else {
            manupVector.push_back(tempStr);
            tempStr ="";
        }
    }

    // Last remaining part also needs to be pushed back since there is no space at the end.
    manupVector.push_back(tempStr);

    return manupVector;
}

int PassportValidator::getNumberOfValidPassportsPart1(const std::string& inputFile)
{
    std::vector<std::string> stringVector = getStringVector(inputFile);
    std::vector<std::string> manupVector = getManupPassportString(stringVector);

    std::cout << "total number of passports:" + std::to_string(manupVector.size()) <<std::endl;

    int counter = 0;
    for (std::string vect:manupVector)
        if (containsAllProperties(vect))
            counter++;
    
    return counter;
}

int PassportValidator::getNumberOfValidPassportsPart2(const std::string& inputFile)
{
    std::vector<std::string> stringVector = getStringVector(inputFile);
    std::vector<std::string> manupVector = getManupPassportString(stringVector);

    std::cout << "total number of passports:" + std::to_string(manupVector.size()) <<std::endl;

    int counter = 0;
    for (std::string vect:manupVector)
        if (containsAllProperties(vect) && containsCorrectData(vect))
            counter++;
    
    return counter;
}

bool PassportValidator::containsAllProperties(const std::string& passPortString) 
{
    std::vector<std::string> passCheckVector = {c_byr,c_iyr,c_eyr,c_hgt,c_hcl,c_ecl,c_pid};

    for(std::string passCheck: passCheckVector) {
        if(!boost::algorithm::contains(passPortString, passCheck))
            return false;
    }

    return true;
}

bool PassportValidator::isCorrectBirthYear(std::string& birthYear)
{
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    boost::algorithm::replace_all(birthYear, c_byr, "");

    int birthYearInt = std::stoi(birthYear);

    if (birthYearInt >= 1920 && birthYearInt <= 2002)
        return true;

    return false;
}

bool PassportValidator::isCorrectIssueYear(std::string& issueYear)
{
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    boost::algorithm::replace_all(issueYear, c_iyr, "");

    int issueYearInt = std::stoi(issueYear);

    if (issueYearInt >= 2010 && issueYearInt <= 2020)
        return true;

    return false;
}

bool PassportValidator::isExpirationYear(std::string& expirationYear)
{
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.

    boost::algorithm::replace_all(expirationYear, c_eyr, "");
    
    int expirationYearInt = std::stoi(expirationYear);

    int x = numDigits(expirationYearInt);

    if(x!=4)
        return false;

    if (expirationYearInt < 2020 || expirationYearInt > 2030)
        return false;

    return true;
}

bool PassportValidator::isCorrectHeight(std::string& height)
{
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.

    boost::algorithm::replace_all(height, c_hgt, "");

    if(boost::algorithm::contains(height, "cm")) {
        boost::algorithm::replace_all(height, "cm", "");
        int heightInt = std::stoi(height);
        if (heightInt >= 150 && heightInt <= 193) {
            return true;
        }
    } else if(boost::algorithm::contains(height, "in")) {
        boost::algorithm::replace_all(height, "in", "");
        int heightInt = std::stoi(height);
        if (heightInt >= 59 && heightInt <= 76) {
            return true;
        }
    }

    return false;
}

bool PassportValidator::isCorrectHairColor(std::string& hairColor)
{
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    boost::algorithm::replace_all(hairColor, c_hcl, "");
    
    if(hairColor.at(0) != '#')
      return false;     

    hairColor.erase(0, 1);

    if (hairColor.length() != 6)
        return false;
    
    if (!contains_characters(hairColor))
        return false;
    
    return true;
}

bool PassportValidator::isCorrectEyeColor(std::string& eyeColor)
{
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    boost::algorithm::replace_all(eyeColor, c_ecl, "");  
    
    std::vector<std::string> validEyeColors = {"amb","blu","brn","gry","grn","hzl","oth"};
    for (std::string validColor:validEyeColors) {
        if (validColor == eyeColor) {
            return true;
        }
    }
    
    return false;
}

bool PassportValidator::isCorrectPid(std::string& pid)
{
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    boost::algorithm::replace_all(pid, c_pid, "");  
    if (!contains_characters(pid))
        return false;

    if (pid.length() != 9)
        return false;

    return true;
}

bool PassportValidator::containsCorrectData(std::string& passportString) 
{
    std::vector<std::string> vectorStr = split(passportString, ' ');

    for(std::string vec:vectorStr) {
        if(boost::algorithm::contains(vec, c_byr)) {
            if (!isCorrectBirthYear(vec)) {
                std::cout << "Invalid BirthYear" << std::endl;
                return false;
            }
        } else if(boost::algorithm::contains(vec, c_iyr)) {
            if (!isCorrectIssueYear(vec)) {
                std::cout << "Invalid Issuer year" << std::endl;
                return false;
            }
        } else if(boost::algorithm::contains(vec, c_eyr)) {
            if (!isExpirationYear(vec)) {
                std::cout << "Invalid ExpirationYear" << std::endl;
                return false;
            }
        } else if(boost::algorithm::contains(vec, c_hgt)) {
            if (!isCorrectHeight(vec)) {
                std::cout << "Invalid Height" << std::endl;
                return false;
            }
        } else if(boost::algorithm::contains(vec, c_hcl)) {
            if (!isCorrectHairColor(vec)) {
                std::cout << "Invalid Hair Color" << std::endl;
                return false;
            }
        } else if(boost::algorithm::contains(vec, c_ecl)) {
            if (!isCorrectEyeColor(vec)) {
                std::cout << "Invalid Eye Color" << std::endl;
                return false;
            }
        } else if(boost::algorithm::contains(vec, c_pid)) {
            if (!isCorrectPid(vec)) {
                std::cout << "Invalid Pid" << std::endl;
                return false;
            }
        }
    }

    return true;
}