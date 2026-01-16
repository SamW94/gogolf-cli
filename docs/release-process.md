# 🏌🏼‍♂️ gogolf-cli release process 

1. Pull the latest changes from the `dev` branch. 
		```
		git switch dev
		git pull
		```

2. Checkout the *release branch*. Use (https://semver.org/)[semantic versioning] based on the latest tag to decide what the new version should be. Name the branch `r-<major-version>-<minor-version>-<patch-version>` like this.

		git checkout -b r-0-0-1

3. Create or update the `CHANGELOG.md` file according to the (Keep a Changelog Style)[https://keepachangelog.com/en/1.1.0/]

		git add CHANGELOG.md


4. Update the version in the `Cargo.toml` file to remove the `rc-*` suffix and leave only the SemVar numbers (i.e. 0.0.1-rc-alpha would be changed to 1.0.0)
	
5. Push your changes to the remote repository on GitHub.

		git commit -m "chore: release v0.0.1"
		git push --set-upstream origin r-0-0-1

6. Open a PR and review the changes. Paste the relevant section of the `CHANGELOG.md` file into the PR body instead of the PR template that's already there.

7. Merge the PR to **main**. **Do not squash the commits. Do not delete the release branch yet.**

8. Switch to the `main` branch and create a tag that matches the release version you decided to use above. 

		git switch main
		git tag v0.0.1
		git push origin --tag

9. Create a second PR to merge any changes you made to the release branch/`CHANGELOG.md` file back to the `dev` branch. Paste the relevant section of the `CHANGELOG.md` file into the PR body instead of the PR template that's already there.

10. Merge the PR. You can squash the commits. You can now delete the release branch. 