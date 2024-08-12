# Here are some minimal test tools helping reproduce issues.

## Folder C_Python_test 
It is a minimal example the one for reproduce [#515](https://github.com/dora-rs/dora/issues/515) and [#516](https://github.com/dora-rs/dora/issues/516). For instructions please go to folder and you will see the steps.
## Folder C_node_test
It is a minimal example the one for reproduce [#502](https://github.com/dora-rs/dora/issues/502). For instructions please go to folder and you will see the steps.


If you encounter the above problems, please check whether the API versions of various languages ​​​​you are using dora are consistent. Please make sure they are consistent. If you are using C or C++, please check your dora repository. If it is the main branch, please make sure that the Python and Rust APIs are also the latest.For Python,you can use 'maturin develop -m apis/python/node/Cargo.toml --release'.For Rust,you can write your toml like this https://github.com/starlitxiling/dora_test/blob/af17d8a7bd0b9a41f94fd6ff6b67ae4152124996/C_Rust_test/sink/Cargo.toml#L10