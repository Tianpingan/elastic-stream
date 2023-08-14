#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ES_NUM_CONTAINERS=${ES_NUM_CONTAINERS:-5}
TC_PATHS=${TC_PATHS:-./estest/}
REBUILD=${REBUILD:f}

die() {
    echo $@
    exit 1
}

if [ "$REBUILD" == "t" ]; then
    ./gradlew clean systemTestLibs
fi

if ${SCRIPT_DIR}/ducker-es ssh | grep -q '(none)'; then
    ${SCRIPT_DIR}/ducker-es up -n "${ES_NUM_CONTAINERS}" || die "ducker-es up failed"
fi

[[ -n ${_DUCKTAPE_OPTIONS} ]] && _DUCKTAPE_OPTIONS="-- ${_DUCKTAPE_OPTIONS}"

${SCRIPT_DIR}/ducker-es test ${TC_PATHS} ${_DUCKTAPE_OPTIONS} || die "ducker-es test failed"
