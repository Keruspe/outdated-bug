```
keruspe@Lou ~/Sources/outdated-bug/lib (git)-[master] % cargo outdated     
All dependencies are up to date, yay!
keruspe@Lou ~/Sources/outdated-bug/lib (git)-[master] % cd ../bin          
keruspe@Lou ~/Sources/outdated-bug/bin (git)-[master] % cargo outdated     
error: Failed to run 'cargo update' with error 'did not exit successfully'
```

When commenting the `path =` line in bin/Cargo.toml:

```
keruspe@Lou ~/Sources/outdated-bug/bin (git)-[master] % cargo outdated -d 1
[130][13:59:50 - 20/09/2017]
Name  Project Ver  SemVer Compat  Latest Ver
libc     0.2.31         RM            RM  
```

libc is a dependency from lib, not bin, but present in the common Cargo.lock from the workspace.
