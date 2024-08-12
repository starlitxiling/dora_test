# Here are some minimal test tools helping reproduce issues.

## Folder C_Python_test 
It is a minimal example the one for reproduce [#515](https://github.com/dora-rs/dora/issues/515) and [#516](https://github.com/dora-rs/dora/issues/516). For instructions please go to folder and you will see the steps.
## Folder C_node_test
It is a minimal example the one for reproduce [#502](https://github.com/dora-rs/dora/issues/502). For instructions please go to folder and you will see the steps.


If you encounter the above problems, please check whether the API versions of various languages ​​​​you are using dora are consistent. Please make sure they are consistent. If you are using C or C++, please check your dora repository. If it is the main branch, please make sure that the Python and Rust APIs are also the latest.For Python,you can use 'maturin develop -m apis/python/node/Cargo.toml --release'.For Rust,you can write your toml like this https://github.com/starlitxiling/dora_test/blob/704773c256d449c92a35a55c58b0505b7fbe2a07/C_Rust_test/sink/Cargo.toml#L10