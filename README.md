# 在`Cargo.toml`中定义包名称
```toml
[package]
name = "rust_py"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
name = "rust_math"
```
需要与`lib.rs`中一致
```rust

// like this
// def add(a:int, b:int) -> str:
//      return a+b
#[pyfunction]
fn add(a: i32, b: i32) -> PyResult<i32> {
    Ok((a + b))
}

#[pymodule]
fn rust_math(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
```




# 使用 maturin 编译

##### 安装

```shell
pip install maturin
```
```
There are three main commands:

maturin publish builds the crate into python packages and publishes them to pypi.
maturin build builds the wheels and stores them in a folder (target/wheels by default), but doesn't upload them. It's possible to upload those with twine.
maturin develop builds the crate and installs it as a python module directly in the current virtualenv. Note that while maturin develop is faster, it doesn't support all the feature that running pip install after maturin build supports.

```
##### 编译


```shell


# 安装 
pip3 install maturin
# 编译
maturin build



```

# 运行

在虚拟环境中安装后运行
```shell
conda create -n rust
conda activate rust
pip install target/wheels/py_test-0.1.0-cp310-none-win_amd64.whl
 

python.exe ./examples/test.py
```

自动安装运行
```shell
$ maturin develop
# lots of progress output as maturin runs the compilation...
$ python
>>> import string_sum
>>> string_sum.sum_as_string(5, 20)
'25'


# or
python.exe ./examples/test.py
```

# PyO3
https://pyo3.rs/v0.14.5/
