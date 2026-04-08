# Contributing to tantivy-py

Make sure you read this whole document first.

## Culture

We encourage contributions of all sizes, from fixing a single typo to adding a major feature.
We want to make it as easy as possible for you to contribute, and we will do our best to review and merge your contributions quickly.

To set expectations, this project is currently maintained by a small team of volunteers.
This means that it can sometimes take a long time, even months or years in some cases,
for a contribution to be reviewed and merged. Your best bet for a quick review and merge is to submit small, focused PRs.

We can work with you to help get your contributions merged.
If you have less experience writing software, that is ok.
We can work with you to help you with your contributions.
We have an expectation that all maintainers and contributors will provide helpful, respectful, friendly, and constructive feedback.

## What kind of contributions?

Our primary focus with tantivy-py is to provide a wrapper of the underlying rust crate, tantivy.
This means that we try to avoid adding new features to the Python bindings that aren't present in the Rust crate.

If you want to add a new feature in the rust code, please consider contributing it to the Rust crate first.
If you want to add a new feature to the Python bindings that isn't present in the Rust crate, please open an issue to discuss it first.
In general we would prefer such features to go into a higher level package that depends on tantivy-py, but this is not always possible, so it depends.

## When to open issues versus PRs

If you're unsure whether a contribution is desired, please open an issue first to discuss it. This isn't required for clear bug fixes, parity with upstream tantivy features, or very short PRs.

**Keep PRs small and focused.** Shorter PRs can be reviewed *exponentially* faster than longer ones. A PR can be as small as fixing a single typo. Don't be shy. We will be very happy to review and merge small PRs, and we will be much less likely to reject them. What is "small"? Here are basic guidelines:
- Anything under 100 lines can be checked quickly
- Under 500 lines is "medium", and might require some time to review depending on the density and complexity of the changes.
- Under 1000 lines is "large" and will likely require a lot of time to review.
- Over 1000 lines should be preceded by a discussion about whether the change is desired and how to execute it.

These are very rough guidelines only, not hard rules. Sometimes a larger PR is unavoidable. When that happens, please open an issue to discuss the change *before* doing the bulk of the work, not after. Maintainers don't want to feel bad about rejecting a huge PR that was a lot of work for the contributor.

If your PR is just an idea and you're looking for feedback, please say so. If you have high confidence in some parts but less in others, please say so. This information is very helpful for reviewers, and the reviewers want to help you. Let them help you.

In the PR, tell us why you want the change. Maintainers always appreciate understanding the high-level motivation behind a change. It may seem obvious, but it always helps us understand what you're aiming for and gives us more opportunities to help.

## Documentation PRs

We love documentation PRs.

But beware massive AI-generated documentation PRs. Words are precious
and should be used wisely: it takes time for us to review them, and it
will take time in the future for users to read them. AI can generate a
lot of words very quickly, but that doesn't mean they are all valuable.
Your judgement is very important here. Err on the side of brevity
and clarity.

As with code PR's, it is better to submit more smaller documentation PRs. If you have a large documentation change, please open an issue to discuss it before doing the work.

## AI policy

This repo allows the use of AI tools to assist with PRs, but please pay attention to the following:

- **Disclose AI use.** If AI tools beyond simple line-based code completion were used, we really appreciate you indicating *how* they were used. Which models? What was the workflow? This is not used to criticize your work or view it negatively; we want to help you and ourselves understand how to review the PR.

- **Flag what you don't understand.** We will not reject a PR just because you don't understand every line. Some contributors have less experience and AI tools can produce complex code. That's fine. But please indicate which sections you don't understand well and which you do. Let's collaborate on figuring out whether the contribution is valuable and whether it should be added. We would appreciate if you could already have tried to work through
all the code to understand the contribution, with the help of AI for example.

- **Don't submit huge AI-generated PRs.** AI tools can generate a lot of code very quickly. We will likely reject large AI-generated PRs unless you justify why the PR needs to be that large and demonstrate that you have reviewed all the code and understand all changes in detail. We can easily tell whether your notes and justifications were also generated by AI. We want to help you, but maintainers need to understand the changes being made.
