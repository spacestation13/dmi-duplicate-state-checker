# dmi-duplicate-state-checker
Simple CI tool to check that all DMIs in a directory do not have duplicate icon states

Example:

![](https://i.imgur.com/ZZt6Enq.png)

## Action

There is a bundled GitHub Action with this repository, found via the button or [this link](https://github.com/marketplace/actions/check-duplicate-dmi-icon-states).
Here is the simplest example usage within a workflow:
```yml
steps:
  - uses: actions/checkout@v2

  - name: Check Duplicate DMI Icon States
    uses: spacestation13/dmi-duplicate-state-checker@v1.0.3
    with:
      GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```
Possible arguments:
* `GITHUB_TOKEN` [REQUIRED] - This should always just be passed the `secrets.GITHUB_TOKEN`, used for fetching and installing the tool.
* `folders_to_search` - Folders to search, comma delineated (default: `./`)
* `flags` - Put other flags for the program here (default: `--warn_read  --actions-fmt` (for old/corrupt DMI detection & gh-actions))
* `version` - A release/tag of the binary tool you want to use, in case you don't want to use the corresponding version.