# Build-my-react-js

For a subdirectory in your project, add this to your `build.rs` to
recurse and build your node project on diverse systems with 
diverse configurations, possibly successfully.

This goes in your build script, for use with react-scripts,
and cra template projects.

It can be helpful for both development and deployment.

So, as an example, with the directory structure like:

```toml
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

Add this to a build.rs file:
```rust
// `build.rs`
use build_my_react_js::*;

fn main() {
    build_react_under!("my-frontend");
}
```