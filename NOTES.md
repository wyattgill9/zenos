## RULES:

[RR, Rule 17.2] Functions can't call themselves recursively (directly or indirectly)
[RR, Directive 4.12] Do not use dynamic memory allocation
[RR, Directive 4.1] Minimize runtime failures
[Rule 8.6] – An identifier declared in an inner scope shall not hide an identifier in an outer scope
[Rule 8.7] – Functions and objects should have internal linkage if not referenced externally
No lossy casting with `as` silently
[Rule 1.1] – Compliance must be documented
NO STD
No unreachable code
ABSOLUTE NO SINGLE POINT OF EXIT/EARLY RETURN
All`if-else if` blocks must be terminated with an else block
All code should be readable by a human ⚠️ Rust allows shadowing and reusing names; best practices help but it's not enforced

### Clippy cmd
cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery
