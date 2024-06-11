PROJECT_ROOT="$( git rev-parse --show-toplevel )"
BINARY="${PROJECT_ROOT}/target/release/cc-tar-rs"
TEST_RESULTS_DIR="${BATS_TEST_TMPDIR}/results"
TEST_EXPECTATIONS_DIR="${BATS_TEST_TMPDIR}/expect"

load tar_test_helper

setup() {
    mkdir -p "${TEST_RESULTS_DIR}"
    mkdir -p "${TEST_EXPECTATIONS_DIR}"
}

@test "is able to list the files in a tar archive from stdin with prefixed files" {
    create_sample_single_file_block_tarball 4 "example"

    # build expectation
    echo "./" >> "${TEST_EXPECTATIONS_DIR}/expect.txt"
    echo "./example/" >> "${TEST_EXPECTATIONS_DIR}/expect.txt"
    for ((i=0; i < 4; i++)); do
        echo "./example/file${i}.txt" >> "${TEST_EXPECTATIONS_DIR}/expect.txt"
    done

    cat "${BATS_TEST_TMPDIR}/test-archive.tar" | ${BINARY} -t | sort | tee "${TEST_RESULTS_DIR}/results.txt" >&3

    diff "${TEST_RESULTS_DIR}/results.txt" "${TEST_EXPECTATIONS_DIR}/expect.txt" >&3
}

@test "is able to list the files in a tar archive from stdin having files without a prefix" {
    create_sample_single_file_block_tarball 4

    # build expectation
    echo "./" >> "${TEST_EXPECTATIONS_DIR}/expect.txt"
    for ((i=0; i < 4; i++)); do
        echo "./file${i}.txt" >> "${TEST_EXPECTATIONS_DIR}/expect.txt"
    done

    cat "${BATS_TEST_TMPDIR}/test-archive.tar" | ${BINARY} -t | sort | tee "${TEST_RESULTS_DIR}/results.txt" >&3

    diff "${TEST_RESULTS_DIR}/results.txt" "${TEST_EXPECTATIONS_DIR}/expect.txt" >&3
}

@test "is able to list the files in the specified tar archive with prefixed files" {
    create_sample_single_file_block_tarball 4 "example"

    # build expectation
    echo "./" >> "${TEST_EXPECTATIONS_DIR}/expect.txt"
    echo "./example/" >> "${TEST_EXPECTATIONS_DIR}/expect.txt"
    for ((i=0; i < 4; i++)); do
        echo "./example/file${i}.txt" >> "${TEST_EXPECTATIONS_DIR}/expect.txt"
    done

    ${BINARY} -t -f "${BATS_TEST_TMPDIR}/test-archive.tar" | sort | tee "${TEST_RESULTS_DIR}/results.txt" >&3

    diff "${TEST_RESULTS_DIR}/results.txt" "${TEST_EXPECTATIONS_DIR}/expect.txt" >&3
}
