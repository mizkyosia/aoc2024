for i in {4..24}
do
    cp src/calendar/day1.rs src/calendar/day$i.rs
    touch src/inputs/day$i.txt
    echo "pub mod day$i;" >> src/calendar/mod.rs
done
