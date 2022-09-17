FILE=`git status --porcelain | sed s/^...//`
git add $FILE
git commit -m "Create $FILE"
git push
rm $FILE
