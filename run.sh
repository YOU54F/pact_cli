#!/bin/sh
set -e

BIN=${BIN:-pact_cli}
${BIN} --help
${BIN} pact-broker --help
${BIN} pactflow --help
${BIN} completions --help
${BIN} docker --help
${BIN} plugin --help
${BIN} plugin list --help
${BIN} plugin list installed --help
${BIN} plugin list known --help
${BIN} plugin env --help
${BIN} plugin install --help
${BIN} plugin remove --help
${BIN} plugin enable --help
${BIN} plugin disable --help
${BIN} plugin repository --help
${BIN} plugin repository validate --help
${BIN} plugin repository new --help
${BIN} plugin repository add-plugin-version --help
${BIN} plugin repository add-plugin-version git-hub --help
${BIN} plugin repository add-plugin-version file --help
${BIN} plugin repository add-all-plugin-versions --help
${BIN} plugin repository yank-version --help
${BIN} plugin repository list --help
${BIN} plugin repository list-versions --help
${BIN} stub --help
${BIN} verifier --help
${BIN} mock --help
${BIN} mock start --help
${BIN} mock list --help
${BIN} mock create --help
${BIN} mock verify --help
${BIN} mock shutdown --help
${BIN} mock shutdown-master --help

${BIN} standalone stop || true
${BIN} standalone start -d
${BIN} standalone info
${BIN} pact-broker list-latest-pact-versions
${BIN} pact-broker create-environment --name name_foo1
${BIN} pact-broker create-environment --name name_foo2 --display-name display_name_foo
${BIN} pact-broker create-environment --name name_foo3 --display-name display_name_foo --contact-name contact_name_foo
${BIN} pact-broker create-environment --name name_foo4 --display-name display_name_foo --contact-name contact_name_foo --contact-email-address contact.email.address@foo.bar
export UUID=$(${BIN} pact-broker create-environment --name name_foo5 --output=id)
${BIN} pact-broker describe-environment --uuid $UUID
${BIN} pact-broker update-environment --uuid $UUID --name name_foo6
${BIN} pact-broker update-environment --uuid $UUID --name name_foo7 --display-name display_name_foo6
${BIN} pact-broker update-environment --uuid $UUID --name name_foo8 --contact-name contact_name_foo8
${BIN} pact-broker update-environment --uuid $UUID --name name_foo9 --contact-name contact_name_foo9 --contact-email-address contact_name_foo7
${BIN} pact-broker delete-environment --uuid $UUID
${BIN} pact-broker list-environments | awk -F '│' '{print $2}' | sed -n '3,$p' | sed '$d' | awk '{print $1}' | xargs -I {} ${BIN} pact-broker delete-environment --uuid {} 
${BIN} pact-broker create-environment --name production --production
${BIN} pact-broker publish --dir pacts -r
${BIN} pact-broker publish --dir pacts -a foo --branch bar
${BIN} pact-broker can-i-deploy --pacticipant GettingStartedOrderWeb --version foo --to prod || echo "can-i-deploy fails due to no verification result - expected"
${BIN} pact-broker can-i-deploy --pacticipant GettingStartedOrderWeb --version foo --to prod --dry-run
${BIN} pact-broker record-deployment --version foo --environment production --pacticipant GettingStartedOrderWeb
${BIN} pact-broker record-undeployment --environment production --pacticipant GettingStartedOrderWeb
${BIN} pact-broker record-release --version foo --environment production --pacticipant GettingStartedOrderWeb
${BIN} pact-broker record-support-ended --version foo --environment production --pacticipant GettingStartedOrderWeb
${BIN} pact-broker can-i-merge --pacticipant foo || echo "can-i-merge unimplemented"
${BIN} pact-broker create-or-update-pacticipant --name foo || echo "create-or-update-pacticipant unimplemented"
${BIN} pact-broker describe-pacticipant --name foo || echo "describe-pacticipant unimplemented"
${BIN} pact-broker list-pacticipants || echo "list-pacticipants unimplemented"
${BIN} pact-broker create-webhook http//foo.bar || echo "create-webhook unimplemented"
${BIN} pact-broker create-or-update-webhook http//foo.bar --uuid bar || echo "create-or-update-webhook unimplemented"
${BIN} pact-broker test-webhook --uuid bar || echo "test-webhook unimplemented"
${BIN} pact-broker delete-branch --branch foo --pacticipant foo || echo "delete-branch unimplemented"
${BIN} pact-broker create-version-tag --version foo --pacticipant foo || echo "create-version-tag unimplemented"
${BIN} pact-broker describe-version --pacticipant foo || echo "describe-version unimplemented"
${BIN} pact-broker create-or-update-version --version foo --pacticipant foo || echo "create-or-update-version unimplemented"
${BIN} pact-broker generate-uuid
${BIN} standalone stop

