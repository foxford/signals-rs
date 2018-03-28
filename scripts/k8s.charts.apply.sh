#!/bin/bash -e
if [[ ! ${PATH_TO_CHARTS} ]]; then >&2 echo "path/to/chart[s] should be specified"; exit 1; fi
if [[ ! ${APP} ]]; then >&2 echo "APP is not specified"; exit 1; fi
if [[ ! ${NAMESPACE} ]]; then >&2 echo "NAMESPACE is not specified"; exit 1; fi

kubectl --namespace=${NAMESPACE} apply -f ${PATH_TO_CHARTS}
kubectl --namespace=${NAMESPACE} patch deployment ${APP} \
    -p "{\"metadata\":{\"annotations\":{\"date\":\"`date +'%s'`\"}}}"