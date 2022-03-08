#!/bin/bash


readonly python="reservoir_sampling"
readonly sample_sizes=1000,5000,10000
readonly max_stream_size=500000
readonly src=${1:-$PORTAGE/corpora/bac-lac.2021/bitextor_2018/201808/permanent/en-fr.deduped.txt.gz}

cargo build --release
hyperfine \
   --shell bash \
   --prepare "zcat --force $src | head -n $max_stream_size > benches/src.txt" \
   --cleanup "rm benches/src.txt" \
   --export-json hyperfine.text.json \
   --style full \
   --parameter-list sample_size $sample_sizes \
   "$python unweighted --size {sample_size} benches/src.txt" \
   "cargo run --release unweighted --size {sample_size} < benches/src.txt" \
   "$python weighted --size {sample_size} benches/src.txt <(cut -f 8 < benches/src.txt)" \
   "cargo run --release weighted --size {sample_size} benches/src.txt <(cut -f 8 < benches/src.txt)" \
   | tee \
   > hyperfine.text.json.results