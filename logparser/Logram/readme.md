# Logram

## Algorihm

### Step1

In this step, we extract a list of tokens(i.e., separated words) from each log message. First of all, we extract the content of a log message by using a pre-defined regular expression.

### Step2

generate n-gram dictionary
if our highest n in the n-gram is 3, we would check two more tokens at the end of the prior log message and the beginning of the following log message.

### Step3

parse log messages using an n-gram dictionary

1. For each n-gram from the log message, we check its number of appearances in the dictionary. If the number of occurrence of a n-gram is smaller than a automatically determined threshold(see Section 4.3.3), we consider that the n-gram may contain a token that is generated from dynamic variables.
2. after collecting all low-appearing n-grams, we transform each of these n-grams into n − 1-grams, and check the number of appearance of each n − 1-gram. We recursively apply this step until we have a list of low-appearing 2-grams, where each of them may contain one or two tokens generated from dynamic variables

### Step4

we generate additional n-grams by considering the ending tokens from the prior log message; 

and for the ending tokens of each log message, we generate additional n-grams by considering the beginning tokens from the next log message.the token from the dynamic variable must reside in two low-appearing 2-grams(i.e., one ends with the dynamic variable and one starts with the dynamic variable).

### Step5
In particular, first, we measure the occurrences of each n-gram. Then, for each occurrence value, we calculate the number of n-grams that have the exact occurrence value. We use a two-dimensional coordinate to represent the oc-currence values(i.e., the X values) and the number of n-grams that have the exact occurrence values(i.e., the Y values). Then we use the loess function to smooth the Y values and calculate the derivative of the Y values against the X values. After getting the derivatives, we use Ckmeans.1d.dp, a one-dimensional clustering method, to find a break point to separate the derivatives into two groups, i.e., a group for static n-grams and a group for dy-namic n-grams. The breaking point would be automatically determined as the threshold.