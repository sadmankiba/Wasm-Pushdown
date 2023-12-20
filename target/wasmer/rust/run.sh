echo "load_time,compile_time,exec_first,total,exec_subseq" > results.csv
for i in {1..20}; do
    cargo run --release -- sudoku-java >> results.csv
done
