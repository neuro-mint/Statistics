library(MASS)
#loading the cats dataset
data(cats)

#useful functions to get a gist of the data
#directly opens the dataframe in the viewer
View(cats)

#gives the structure of the dataframe
str(cats)

#gives the top 5 entries in the dataframe
head(cats, 5)

#gives the summary of the dataframe
summary(cats)

#plotting the data in cats
plot(Hwt ~ Bwt, data = cats)

#building a linear model
mylm <- lm(Hwt ~ Bwt, data = cats)
s1 <- summary(mylm)
s1

#the plot function automatically creates 6 plots
#from the mylm object. These can be accessed by specifying which
plot(mylm, which = 1, main = "mylm")

#plotting the residuals
plot(fitted(mylm),
     residuals(mylm),
     ylab = "Residuals",
     xlab = "Predicted scores",
     main = "mylm")
abline(h = 0)

#checking linear model to remove outliers aka coook's distance
plot(mylm, which = 4, main = "mylm")

#removing the 144th entry
cats_updated <- cats[-144, ]
cats_updated <- cats_updated[-140, ]

#remaking linear model with new data
mylm2 <- lm(Hwt ~ Bwt, data = cats_updated)
s2 <- summary(mylm2)
s2

#replotting with new model
plot(mylm2, which = 1, main = "mylm2")

plot(fitted(mylm2),
     residuals(mylm2),
     ylab = "Residuals",
     xlab = "Predicted scores",
     main = "mylm2")
abline(h = 0)

plot(mylm2, which = 4, main = "mylm2")
