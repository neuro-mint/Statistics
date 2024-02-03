library(dplyr)
library(caTools)
library(rpart)
library(rpart.plot)

#Downloads the titanic dataset, converts csv file to dataframe
#and assigns it to a variable
#here we also convert some variables of the data to factors which
#are variables with values in a limited range or types

titanic_data <- "https://goo.gl/At238b" %>% read.csv
sample_dataset <- titanic_data %>%
  mutate(embarked = factor(embarked), sex = factor(sex)) %>%
  select(survived, sex, sibsp, parch, fare)

#splitting the dataset into training and test sets
split_dataset <- sample.split(sample_dataset, SplitRatio = 0.7)

#assigning the separated sets to variables
train_data <- subset(sample_dataset, split_dataset == TRUE)
test_data <- subset(sample_dataset, split_dataset == FALSE)

#Decision tree for training set
rtree <- rpart(survived ~ ., train_data)
rpart.plot(rtree)

#Decision tree for test set
rtree  <- rpart(survived ~ ., test_data)
rpart.plot(rtree)
