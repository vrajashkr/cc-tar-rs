# creates a new tarball given:
# - the path where the archive should be created
# - the base path where the contents are
function create_tarball_from_dir() {
    local archive_path="${1}"
    local contents_base_path="${2}"

    local tar_opts=(
        -cf ${archive_path}
        -C ${contents_base_path}
    )

    tree ${BATS_TEST_TMPDIR} >&3

    tar ${tar_opts[@]} .
}

# creates a new sample tarball with single block file contents (less than 512 bytes) given:
# - the number of files to add
# - the path prefix for files
function create_sample_single_file_block_tarball() {
    local num_files=${1}
    local path_prefix="${2}"
    local data_dir_base_path="${BATS_TEST_TMPDIR}/prepare"

    local dest_dir=""
    if [ ${path_prefix} == "" ]; then
        dest_dir="${data_dir_base_path}"
    else
        dest_dir="${data_dir_base_path}/${path_prefix}"
        mkdir -p "${dest_dir}"
    fi

    for((i=0; i<${num_files}; i++)); do
        echo "file ${i} contents" > "${dest_dir}/file${i}.txt"
    done

    create_tarball_from_dir "${BATS_TEST_TMPDIR}/test-archive.tar" "${data_dir_base_path}"
}
