#!/bin/bash
#
# Run some benchmarks on our app.
#

# Errors are fatal
set -e

BINARY="target/debug/monte_carlo"

OUTPUT="benchmarks-raw.md"

COUNT=1000000
BATCH_SIZE=1000
GRID_SIZE=1000000
MAX_NUM_THREADS=4
OUTPUT_FILE=""


#
# Wrapper script to run Hyperfine with various arguments.
#
function benchmark() {

    # Any args that are passed in
    ARGS=$@

    hyperfine \
        --parameter-scan num_threads 1 ${MAX_NUM_THREADS} \
        --export-markdown "benchmarks/${OUTPUT_FILE}" \
        "${BINARY} -c ${COUNT} -b ${BATCH_SIZE} -n {num_threads} -g ${GRID_SIZE} ${ARGS}" 

}


#
# Append our most recent benchmark to the output file.
#
function update_output {

    echo "" >> ${OUTPUT}
    echo $OUTPUT_FILE >> ${OUTPUT}
    cat benchmarks/$OUTPUT_FILE >> ${OUTPUT}

}

echo "# Cleaning up any old benchmark outputs..."
pushd benchmarks > /dev/null
rm -fv *.md
popd > /dev/null
rm -fv ${OUTPUT}

echo "# Testing dev build..."
OUTPUT_FILE="dev.md"
benchmark
update_output

echo "# Testing release build..."
BINARY="target/release/monte_carlo"
OUTPUT_FILE="release.md"
benchmark
update_output

echo "# Now testing release build with 10 million points..."
COUNT=10000000
OUTPUT_FILE="10-million-points.md"
benchmark
update_output

BATCH_SIZE=10000
echo "# Now testing release build with batch size of ${BATCH_SIZE}"
OUTPUT_FILE="batch-size-10000.md"
benchmark
update_output

BATCH_SIZE=100000
echo "# Now testing release build with batch size of ${BATCH_SIZE}"
OUTPUT_FILE="batch-size-100000.md"
benchmark
update_output

BATCH_SIZE=10000
echo "# Now testing release build with batch size of ${BATCH_SIZE} and turbo mode"
OUTPUT_FILE="turbo.md"
benchmark --turbo
update_output

echo "# Now testing release build with batch size of ${BATCH_SIZE} and cache mode"
OUTPUT_FILE="cache.md"
benchmark --cache
update_output

echo "# Now testing release build with batch size of ${BATCH_SIZE} and cache mode with pre-compute"
OUTPUT_FILE="cache-precompute.md"
benchmark --cache-precompute
update_output

GRID_SIZE=100000000
echo "# Now testing release build with grid size of ${GRID_SIZE}, and cache mode with pre-compute"
OUTPUT_FILE="grid-size-${GRID_SIZE}.md"
benchmark 
update_output

echo "# Now testing release build with grid size of ${GRID_SIZE}, and cache mode with pre-compute"
OUTPUT_FILE="grid-size-${GRID_SIZE}-cache.md"
benchmark --cache
update_output

echo "# Now testing release build with grid size of ${GRID_SIZE}, and cache mode with pre-compute"
OUTPUT_FILE="grid-size-${GRID_SIZE}-cache-precompute.md"
benchmark  --cache-precompute
update_output

GRID_SIZE=1000000000
echo "# Now testing release build with grid size of ${GRID_SIZE}"
OUTPUT_FILE="grid-size-${GRID_SIZE}.md"
benchmark 
update_output

COUNT=100000000
echo "# Now testing release build with grid size of ${GRID_SIZE}, and count of ${COUNT}"
OUTPUT_FILE="grid-size-${GRID_SIZE}-count-${COUNT}.md"
benchmark 
update_output

echo "# Done! (Benchmarks all in ${OUTPOUT_FILE})"


