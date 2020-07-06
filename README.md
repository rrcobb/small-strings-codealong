Code, following along with https://fasterthanli.me/articles/small-strings-in-rust

Put on github mostly for comparing notes.

I saw fewer allocations with SmolStr when I ran it, which is because I'm using the post-blogpost updated version of smol_str. https://github.com/rust-analyzer/smol_str/commit/633320cc7cf44b3f216c04a93eeda8e865c247ff looks like it's what explains the screenshot below:

![slightly fewer allocations](smol-output.png)

Sidenote: simply amazing to see how fast things move, this got faster in the 16 hours between fasterthanlime posting on twitter and me trying it out.

Screenshot above should show the output below:

```shell
$ cargo build && ./target/debug/small sample --lib smol 2> events.ldjson && ./target/debug/small report events.ldjson
    Finished dev [unoptimized + debuginfo] target(s) in 0.55s
Read 1000 records
found 36 events
⡁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈⢸⠉⡏ ⠁⠈ ⡁ 73896.0
⠄                                                    ⢸ ⡇    ⠄
⠂                                                    ⢸ ⡇    ⠂
⡁                                                    ⢸ ⡇    ⡁
⠄                                                    ⢸ ⡇    ⠄
⠂                                                    ⢸ ⡇    ⠂
⡁                                                    ⢸ ⣇⣀⣀⣀ ⡁
⠄                                                    ⢸      ⠄
⠂                                                    ⢸      ⠂
⡁                                                    ⢸      ⡁
⠄                                               ⢸⠉⡇  ⢸      ⠄
⠂                                               ⢸ ⡇  ⢸      ⠂
⡁                                               ⢸ ⡇  ⢸      ⡁
⠄                                               ⢸ ⠓⠒⠒⠚      ⠄
⠂                                               ⢸           ⠂
⡁                                            ⡏⢹ ⢸           ⡁
⠄                                            ⡇⢸⣀⣸           ⠄
⠂                                       ⡤⢤   ⡇              ⠂
⡁                                   ⢀⣀⣀ ⡇⠘⠒⠒⠒⠃              ⡁
⠄                        ⣀⣀ ⢀⣀⣀⣰⠒⠲⠤⠤⠼ ⠘⠒⠃                   ⠄
⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠈⠉⠉ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁ 1.0
0.0                                                      36.0
     total events | 36
      peak bytes  | 73.9 KB
     ----------------------------
     alloc events | 20
     alloc bytes  | 98.5 KB
     ----------------------------
     freed events | 16
     freed bytes  | 49.2 KB
```

compare to the 42 events (23 alloc, 19 freed) of the blog post.
