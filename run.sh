#!/bin/sh
set -e

target/release/pact_cli --help
target/release/pact_cli pact-broker --help
target/release/pact_cli pactflow --help
target/release/pact_cli completions --help
target/release/pact_cli docker --help
target/release/pact_cli plugin --help
target/release/pact_cli plugin list --help
target/release/pact_cli plugin list installed --help
target/release/pact_cli plugin list known --help
target/release/pact_cli plugin env --help
target/release/pact_cli plugin install --help
target/release/pact_cli plugin remove --help
target/release/pact_cli plugin enable --help
target/release/pact_cli plugin disable --help
target/release/pact_cli plugin repository --help
target/release/pact_cli plugin repository validate --help
target/release/pact_cli plugin repository new --help
target/release/pact_cli plugin repository add-plugin-version --help
target/release/pact_cli plugin repository add-plugin-version git-hub --help
target/release/pact_cli plugin repository add-plugin-version file --help
target/release/pact_cli plugin repository add-all-plugin-versions --help
target/release/pact_cli plugin repository yank-version --help
target/release/pact_cli plugin repository list --help
target/release/pact_cli plugin repository list-versions --help
target/release/pact_cli stub --help
target/release/pact_cli verifier --help
target/release/pact_cli mock --help
target/release/pact_cli mock start --help
target/release/pact_cli mock list --help
target/release/pact_cli mock create --help
target/release/pact_cli mock verify --help
target/release/pact_cli mock shutdown --help
target/release/pact_cli mock shutdown-master --help

target/release/pact_cli standalone stop || true
target/release/pact_cli standalone start -d
target/release/pact_cli standalone info
target/release/pact_cli pact-broker list-latest-pact-versions
target/release/pact_cli pact-broker create-environment --name name_foo1
target/release/pact_cli pact-broker create-environment --name name_foo2 --display-name display_name_foo
target/release/pact_cli pact-broker create-environment --name name_foo3 --display-name display_name_foo --contact-name contact_name_foo
target/release/pact_cli pact-broker create-environment --name name_foo4 --display-name display_name_foo --contact-name contact_name_foo --contact-email-address contact.email.address@foo.bar
export UUID=$(target/release/pact_cli pact-broker create-environment --name name_foo5 --output=id)
target/release/pact_cli pact-broker describe-environment --uuid $UUID
target/release/pact_cli pact-broker update-environment --uuid $UUID --name name_foo6
target/release/pact_cli pact-broker update-environment --uuid $UUID --name name_foo7 --display-name display_name_foo6
target/release/pact_cli pact-broker update-environment --uuid $UUID --name name_foo8 --contact-name contact_name_foo8
target/release/pact_cli pact-broker update-environment --uuid $UUID --name name_foo9 --contact-name contact_name_foo9 --contact-email-address contact_name_foo7
target/release/pact_cli pact-broker delete-environment --uuid $UUID
target/release/pact_cli pact-broker list-environments | awk -F '│' '{print $2}' | sed -n '3,$p' | sed '$d' | awk '{print $1}' | xargs -I {} target/release/pact_cli pact-broker delete-environment --uuid {} 
target/release/pact_cli pact-broker create-environment --name production --production
target/release/pact_cli pact-broker publish --dir pacts -r
target/release/pact_cli pact-broker publish --dir pacts -a foo --branch bar
target/release/pact_cli pact-broker can-i-deploy --pacticipant GettingStartedOrderWeb --version foo --to prod || echo "can-i-deploy fails due to no verification result - expected"
target/release/pact_cli pact-broker can-i-deploy --pacticipant GettingStartedOrderWeb --version foo --to prod --dry-run
target/release/pact_cli pact-broker record-deployment --version foo --environment production --pacticipant GettingStartedOrderWeb
target/release/pact_cli pact-broker record-undeployment --environment production --pacticipant GettingStartedOrderWeb
target/release/pact_cli pact-broker record-release --version foo --environment production --pacticipant GettingStartedOrderWeb
target/release/pact_cli pact-broker record-support-ended --version foo --environment production --pacticipant GettingStartedOrderWeb
target/release/pact_cli pact-broker can-i-merge --pacticipant foo || echo "can-i-merge unimplemented"
target/release/pact_cli pact-broker create-or-update-pacticipant --name foo || echo "create-or-update-pacticipant unimplemented"
target/release/pact_cli pact-broker describe-pacticipant --name foo || echo "describe-pacticipant unimplemented"
target/release/pact_cli pact-broker list-pacticipants || echo "list-pacticipants unimplemented"
target/release/pact_cli pact-broker create-webhook http//foo.bar || echo "create-webhook unimplemented"
target/release/pact_cli pact-broker create-or-update-webhook http//foo.bar --uuid bar || echo "create-or-update-webhook unimplemented"
target/release/pact_cli pact-broker test-webhook --uuid bar || echo "test-webhook unimplemented"
target/release/pact_cli pact-broker delete-branch --branch foo --pacticipant foo || echo "delete-branch unimplemented"
target/release/pact_cli pact-broker create-version-tag --version foo --pacticipant foo || echo "create-version-tag unimplemented"
target/release/pact_cli pact-broker describe-version --pacticipant foo || echo "describe-version unimplemented"
target/release/pact_cli pact-broker create-or-update-version --version foo --pacticipant foo || echo "create-or-update-version unimplemented"
target/release/pact_cli pact-broker generate-uuid
target/release/pact_cli standalone stop

