FILE=`git status --porcelain | sed s/^...//`
git add $FILE
git commit -m "Upsert $FILE"
https_proxy=localhost:1088 git push
