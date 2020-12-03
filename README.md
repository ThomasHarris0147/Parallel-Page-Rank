# Parallel-Page-Rank
I did this or a parallel class project\
![equation](https://latex.codecogs.com/gif.latex?j%20%3D%20i&plus;1)\
![equation](https://latex.codecogs.com/gif.latex?Pagerank%28A%29_j%20%3D%20%5Csum_%7BB%3Dnodes%20%5Cmapsto%20A%20%7D%20%5Cleft%20%28%5Cfrac%7BPagerank%28B%29_i%7D%7BNumberOfNodesBPointsTo%7D%20%5Cright%29)

non parallel page rank tests
|----|
|1329|1|  
|1279|2|
|1369|3|
|1337|4|
|1200|5|
|1495|6|
|1333|7|
|1293|8|
|1187|9|
|1418|10|
Average 1324
----
parallel page rank tests
|----|
|337|1|
|342|2|
|346|3|
|323|4|
|328|5|
|372|6|
|351|7|
|334|8|
|328|9|
|334|10|
Average 339.5
----
the code could run faster using more primative ways.\ 
my code used to use a separate vector in the main function to hold the page ranks compared to having the nodes hold the page rank\
in that case it would run much much faster\
However, the method I used was this way\
Also I couldnt figure out how to parallel iterate through all nodes and update the pagerank vector using ^ method.\
hence I stuck with each node holding the pagerank\
