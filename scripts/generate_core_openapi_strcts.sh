#!/bin/bash
set -e
# set -x
cd "$(dirname "$0")"
cd ..

# NOTE, it is only ok to use bash for simple scripts like this.
# Bash scripts inside this repository should not contain any logical keywords like IF or FOR.
# More complex scripts should be written using Python or similar language.
# This notice should be copied to every bash script that is used in this project.

echo 'Remember to install docker before running this script!'
echo 'Please install "api-spec-converter" before running this script.'
echo 'API spec converter can be installed using:'
echo 'sudo npm install -g api-spec-converter '

echo 'Convert OpeanAPI 3 schema to OpeanAPI 2 schema, so that Rust generator can be used.'
api-spec-converter -f openapi_3 -t swagger_2 docs/openapi/openapi_core.yaml > openapi_core.yaml

echo 'Generate Rust server using OpenAPI generator'
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i /local/openapi_core.yaml \
    -g rust-server \
    -o /local/src/core \
    --generate-alias-as-model \
    -p packageName=potku_console_core \
    -p packageVersion=0.1.0

echo 'Remove temporary OpenAPI 2 conversion file'
rm openapi_core.yaml

echo "Success!"
echo "Please modify the API implementation if necessary!"