# Release Process

To make a `flip-link` release:

1. Get the current version number, which we call `$oldver`

1. Decide if the new version requires a major, minor or patch version bump

1. Compute the new version number, which we call `$newver`

1. Create the branch `release/v$newver`

    ```bash
    git checkout -b release/v$newver
    ```

1. Update `Cargo.toml` to include `version = "$newver"`

1. Update `CHANGELOG.md`:

    1. Rename `## [Unreleased]` to `## [v$newver] - YYYY-MM-DD` (note the 'v')

    1. Add `[v$newver]` as a link to a diff to the previous version:

        ```markdown
        [v$newver]: https://github.com/knurling-rs/flip-link/compare/v$oldver...v$newver
        ```

    1. Add a new `## [Unreleased]` section above the new version

    1. Ensure all updates have PR numbers with working links

1. Commit all changes, push the branch, and open a PR

    ```bash
    git add CHANGELOG.mg
    git add Cargo.toml
    git commit -m "Preparing release v$newver"
    git push -u origin release/v$newver
    ```

1. Get a review on the PR from another team member

1. Merge the PR into `main`

1. Checkout the `main` branch locally (ensure it's the version from the PR you just merged, and no later changes have arrived)

1. Check that `cargo` is happy:

    ```bash
    cargo publish --dry-run
    ```

1. Check that `cargo-dist` is happy:

    ```bash
    cargo dist plan
    ```

1. Tag the release and push the tags:

    ```bash
    git tag -a v$newver -m "Release v$newver"
    git push --tags
    ```

1. Wait. CI will run and generate a github release containing binary artefacts. A second workflow will run and publish to crates.io.

1. If the release fails, repeat the entire process with a new version, starting at point 1.
