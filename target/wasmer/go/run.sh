echo "load_time,compile_time,exec_first,total,exec_subseq" > results.csv
for i in {1..20}; do
    go run main.go sort-java >> results.csv
done
