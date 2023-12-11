# Tik

Tik is an attempt at the next iteration of code-related version control. It would be weird for a new app to launch without some sort of self-managing save or check point system. Even the Google Suite has had this feature in some form for over a decade. 

It is important that we maintain control over what, when, and how code is committed, but what if we had a tool that attempted to handle some of this burden for us?

## How it works

### Start

Tik runs in the background as a daemon process. It should largely live on your machine sight unseen. The goal is to run it with `tik start` and forget about it. Might be worth considering some sort of startup command toggle in the future.

### Writing code

As you write code, Tik will intelligently create `tiks` that store your changes with contextual `tik messages`. You can either wait for `tik` to group `tiks` and create a commit for you, or when you're ready, run `tik -c` and let `tik` create a commit and commit message for you. 

### The future

Ideally, `tik` should be its own solution rather than an abstraction on top of `git`, but it will take some time to play around and find the proper implementation.