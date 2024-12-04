#!/bin/bash
#
# Run some benchmarks on our app.
#

# Errors are fatal
set -e

BINARY="target/debug/monte_carlo"

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


echo "# Cleaning up any old benchmark outputs..."
pushd benchmarks > /dev/null
rm -fv *.md
popd > /dev/null

echo "# Testing dev build..."
OUTPUT_FILE="dev.md"
benchmark

echo "# Testing prod build..."
BINARY="target/release/monte_carlo"
OUTPUT_FILE="prod.md"
benchmark

echo "# Now testing prod with 10 million points..."
COUNT=10000000
OUTPUT_FILE="10-million-points.md"
benchmark

BATCH_SIZE=10000
echo "# Now testing prod with batch size of ${BATCH_SIZE}"
OUTPUT_FILE="batch-size-10000.md"
benchmark

BATCH_SIZE=100000
echo "# Now testing prod with batch size of ${BATCH_SIZE}"
OUTPUT_FILE="batch-size-100000.md"
benchmark

BATCH_SIZE=10000
echo "# Now testing prod with batch size of ${BATCH_SIZE} and turbo mode"
OUTPUT_FILE="turbo.md"
benchmark --turbo

echo "# Now testing prod with batch size of ${BATCH_SIZE} and cache mode"
OUTPUT_FILE="cache.md"
benchmark --cache

echo "# Now testing prod with batch size of ${BATCH_SIZE} and cache mode with pre-compute"
OUTPUT_FILE="cache-precompute.md"
benchmark --cache-precompute

GRID_SIZE=1000000
echo "# Now testing prod with batch size of ${BATCH_SIZE}, grid size of ${GRID_SIZE}, and cache mode with pre-compute"
OUTPUT_FILE="cache-precompute-grid-size-${GRID_SIZE}.md"
benchmark 

RAW="benchmarks-raw.md"
echo "# Writing all benchmarks to ${RAW}..."
touch ${RAW}
for FILE in benchmarks/*
do
    echo ${FILE} >> ${RAW}
    cat ${FILE} >> ${RAW}
    echo >> ${RAW}
done

echo "# Done!"


