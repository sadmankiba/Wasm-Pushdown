echo "exec_first,exec_subseq" > results.csv

SUDOKU=300600000500007000870090400080050000064000790000020030001040078000300002000005004

cd sort/src/main/java/com/example/sort
rm Sort.class
javac Sort.java
cd ../../../
for i in {1..20}; do
    java com.example.sort.Sort 3 10 >> ../../../../results.csv
done