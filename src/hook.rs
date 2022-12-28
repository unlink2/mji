// outputs the hook to a string
// the hook expects one command line input (the commit message)
// and will modify it by adding the mjis
pub fn hook() -> String {
    "
#!/bin/sh 

# =========================
# mji commit hook
# expects commit 
# message file to be in $1 
# the source in $2
# and the sha1 in $3
# this is intended to be 
# located in 
# .git/hooks/prepare-commit-msg
# or 
# .git/hooks/commit-msg
# make sure the hooks
# are executable!
# =========================
COMMIT_MSG_FILE=$1

cat $COMMIT_MSG_FILE | mji -o $COMMIT_MSG_FILE -q --no-mji-find-error
exit $?
# =========================
        "
    .into()
}
