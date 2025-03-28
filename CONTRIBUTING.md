# Contributing to Stratagem

## Ideas, Experiences and Questions

The easiest way to contribute to Stratagem is to use it and report your experiences, ask questions and contribute ideas. We'd love to hear your thoughts on how to make Stratagem better, or your comments on why you are or are not currently using it.

Issues, ideas, requests and questions should be posted on the issue tracker at:

https://github.com/ezrasingh/stratagem/issues

## A Note on Dependency Updates

Stratagem does not accept pull requests to update dependencies unless specifically
requested by the maintainer(s). Dependencies are updated manually by the maintainer(s) before each
new release.

## Code

Pull requests are welcome, though please raise an issue for discussion first if none exists. We're happy to assist new contributors.

If you're not sure what to work on, try checking the [Good First Issue label](https://github.com/ezrasingh/stratagem/contribute).

To make changes to the code, fork the repo and clone it:

`git clone git@github.com:<your-username>/stratagem.git`

Then make your changes to the code. When you're done, run the tests:

```
cargo test --all
cargo bench
```

It's a good idea to run `clippy` and fix any warnings as well:

```
rustup component add clippy
cargo clippy --workspace --all-targets
```

Finally, run Rustfmt to maintain a common code style:

```
rustup component add rustfmt-preview
cargo fmt --all
```

Don't forget to update the `CHANGELOG.md` file and any appropriate documentation. Once you're finished, push to your fork and submit a pull request. We try to respond to new issues and pull requests quickly, so if there hasn't been any response for more than a few days feel free to ping [@ezrasingh](https://github.com/ezrasingh).

Some things that will increase the chance that your pull request is accepted:

- Write tests
- Clearly document public methods
- Write a good commit message

## Branches

- PRs with breaking changes are made against the unreleased branch. e.g. branch version-0.4
- PRs without breaking changes are made against the `main` branch.

If you're not sure which branch to use just start with `main`, as this can be changed during review.

When it is time to release the unreleased branch, a PR is made from the unreleased branch to `main`. e.g. https://github.com/ezrasingh/stratagem/pull/496

## Github Labels

Stratagem uses a simple set of labels to track issues. Most important are the
difficulty labels:

- **Question** - Further information is requested
- **Good First Issue** - Suitable for people new to Stratagem, or even new to Rust in general
- **Help Wanted** - Extra attention is needed

Additionally, there are a few other noteworthy labels:

- **Bug** - Something isn't working right
- **Documentation** - Improvements or additions to documentation
- **Enhancement** - Request to add a new feature or otherwise improve Stratagem in some way

## Code of Conduct

We follow the [Rust Code of Conduct](http://www.rust-lang.org/conduct.html).
