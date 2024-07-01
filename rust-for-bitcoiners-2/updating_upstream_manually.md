## updating your repo with upstream if there is any correction made.

Let me try to update mine so I can explain to the others
All right, it's a bit more complicated than cloning again, it will require a rebase. I'll explain in the next message so it can eventually be pinned.

### step1 go to your repo and add a new remote (the repo delcin is updating):
`git remote add upstream https://github.com/delcin-raj/rfb_2_2024_2.git`

### step2 fetch from upstream:
`git pull upstream main --rebase`

in case of conflict (happened to me), edit each conflicting file. It will have markers like these:

` <<<<<<< HEAD
this is some content to mess with
content to append
 =======
totally different content to merge later
>>>>>>> new_branch_to_merge_later `

To select the new text, change it to something like:

`this is some content to mess with
content to append`
There can be potentially many of these, inspect and edit every single one.

### step 3 after fixing all conflicts, stage the conflicting files:
`git add .`

### step4 continue rebasing:
`git rebase --continue`

### step 5 push to your repo:
`git push origin main --force-with-lease`

Github classroom does not update repos automatically when the template repo changes.
When someone accepts the invitation link, it will clone the template in a new repo for the student.
So, if you accepted the invitation before the change, you have to manually rebase like the above.
If you accepted after the change, it will be already right.