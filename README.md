# Build-my-react-js

For a subdirectory in your project, add this to your `build.rs` to
recurse and build your node project on diverse systems with 
diverse configurations, possibly successfully.

This goes in your build script, for use with react-scripts,
and cra template projects.

It can be helpful for both development and deployment.

So, as an example, with the directory structure like:

```text
.
..
.gitignore
src/
my-frontend/
  src/
    index.js
    CookieNoticeConsent.js
    NotificationsApi.js
    [...]
  package.json
Cargo.toml
Cargo.lock
[...]
```

Add to your project as a build dependency,
```
cargo add --build build-my-react-js
```

Add this to a build.rs file:
```rust
// `build.rs`
use build_my_react_js::*;

fn main() {
    build_react_under!("my-frontend");
}
```

Effectively, will cause cargo, during dirty / new builds,
it will go into the directory and do:
```
test -e my-frontend/build/index.html
npm ping    #only the first time
npm install #only the first time
npm ping
npm run build
```

It uses a cargo facility that tests for updates to 
`my-frontend/src/*` or `my-frontend/package.json`, and so
usually only runs your react-scripts build if you have changed
something.
