library(dplyr)
library(caTools)
library(ggplot2)
library(randomForest)

titanic_data <- "https://goo.gl/At238b" %>% read.csv
sample_dataset <- titanic_data %>% 
mutate(embarked = factor(embarked), sex = factor(sex))

split_dataset <- sample.split(sample_dataset, SplitRatio = 0.7)
train_data <- subset(sample_dataset, split_dataset == TRUE)
test_data <- subset(sample_dataset, split_dataset == FALSE)

feature_extractioin <-
  function(data) {
    labels <- c("pclass",
                "sex",
                "age",
                "sibsp",
                "parch",
                "survived",
                "fare",
                "embarked")
    features <- data[, labels]
    features$age[is.na(features$age)] <- -1
    features$fare[is.na(features$fare)] <- median(features$fare)
    features$embarked[features$embarked == ""] <- "S"
    features$embarked[is.na(features$embarked)] <- "S"
    features$survived[is.na(features$survived)] <- 0
    return(features)
  }

#remove NA values from the feature extraction result
train_data_features <- na.exclude(feature_extractioin(train_data))
test_data_features <- na.exclude(feature_extractioin(test_data))

#Random Forest for training set
r_forest <- randomForest(train_data_features,
                         ntree = 100, importance = TRUE)
varImpPlot(r_forest, main = "Training set")

#Random forest for test set
r_forest2 <- randomForest(test_data_features,
                         ntree = 100, importance = TRUE)
varImpPlot(r_forest2, main = "Test set")
