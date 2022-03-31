#!/bin/bash


readonly python="reservoir_sampling"
readonly sample_sizes=${1:-1000,5000,10000}
readonly max_stream_size=${2:-500000}
readonly src=${3:-$PORTAGE/corpora/bac-lac.2021/bitextor_2018/201808/permanent/en-fr.deduped.txt.gz}
readonly tmp_src=`mktemp "./benches/reservoir_sampling.src.XXXXX"`


function speedtest {
   echo "Sample size(s): $sample_sizes}"
   echo "Maximum stream size: $max_stream_size"
   echo "Population stream: $src"

   cargo build --release
   python3 -m pip install git+https://github.com/SamuelLarkin/reservoir_sampling.git@07cf5dc77
   hyperfine \
      --shell bash \
      --prepare "zcat --force $src | head -n $max_stream_size > $tmp_src" \
      --cleanup "rm $tmp_src" \
      --export-json hyperfine.text.json \
      --style full \
      --parameter-list sample_size $sample_sizes \
      "$python unweighted --size {sample_size} $tmp_src" \
      "cargo run --release -- --size {sample_size} unweighted < $tmp_src" \
      "$python weighted --size {sample_size} $tmp_src <(cut -f 8 < $tmp_src)" \
      "cargo run --release -- --size {sample_size} weighted $tmp_src <(cut -f 8 < $tmp_src)"
}


speedtest \
| tee \
> hyperfine.text.results
