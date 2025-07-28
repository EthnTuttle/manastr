# Dependency Management Rules

## Universal Rules for Mana Strategy Repository

### Git Dependencies
- **ALWAYS** pin git dependencies to specific commits using `rev = "commit_hash"`
- **NEVER** use branch names like `main` or `master` in production
- Update git dependencies by changing the commit hash, not by using floating references

```toml
# ✅ CORRECT
cdk = { git = "https://github.com/cashubtc/cdk", rev = "e346ab63", features = ["mint", "wallet"] }

# ❌ WRONG - causes drift
cdk = { git = "https://github.com/cashubtc/cdk", features = ["mint", "wallet"] }
cdk = { git = "https://github.com/cashubtc/cdk", branch = "main", features = ["mint", "wallet"] }
```

### Crate Dependencies
- Use exact versions for critical dependencies (those affecting security, cryptography, or protocol implementation)
- Use caret (`^`) versions for utility crates that don't affect core functionality
- Document version choices in comments

```toml
# ✅ CORRECT - critical crypto dependency
secp256k1 = "0.28"  # Exact version for crypto stability

# ✅ CORRECT - utility dependency  
serde = "^1.0"  # Can accept compatible updates

# Document important version decisions
sqlx = { version = "0.8", features = ["sqlite"] }  # v0.8 required for CDK compatibility
```

### Dependency Resolution Conflicts
- When native library conflicts occur (like `libsqlite3-sys`), align all dependent crates to use the same version
- Check `cargo tree` regularly to identify version conflicts
- Prefer the version required by the most critical dependency (e.g., CDK over utility crates)

### Testing Dependencies
- Use exact versions for test-only dependencies to ensure reproducible test results
- Pin development tools and build dependencies

```toml
[dev-dependencies]
tempfile = "3.8.1"  # Exact version for test stability
```

### Documentation
- Document major version choices in `Cargo.toml` comments
- Maintain this file with rationale for major dependency decisions
- Update commit hashes with release notes explaining what changed

### Verification
- Run `cargo check` and `cargo test` after any dependency changes
- Use `cargo audit` to check for security vulnerabilities
- Run `cargo tree --duplicates` to identify version conflicts

## Rationale

This prevents:
- **Dependency drift**: Code that works on one machine but breaks on another
- **Build breakage**: Upstream changes breaking our builds unexpectedly  
- **Security issues**: Automatic updates pulling in vulnerable versions
- **Integration problems**: Version mismatches between interdependent crates

## Maintenance Schedule

- Review and update pinned git dependencies monthly
- Check for security updates weekly using `cargo audit`
- Document all dependency changes in commit messages
- Test dependency updates in isolation before merging

## Emergency Exception Process

If a critical security update requires immediate dependency changes:
1. Create separate PR with only the security fix
2. Test thoroughly in isolation
3. Document the emergency nature in commit message
4. Update this file with the new baseline

---

**Last Updated**: 2025-01-27  
**Next Review**: 2025-02-27