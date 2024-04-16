# pyo3为python编写原生模块
版本约定：
- python 3.7+
- rust 1.56+

# 创建python env环境并安装maturin工具
maturin主要作用：使用pyo3, rust-cpython或cffi绑定以及rust二进制文件作为python包构建和发布crate

```shell
cd string_sum
python3 -m venv .env
source .env/bin/activate
# .env/bin/python3 -m pip install --upgrade pip
pip3 install maturin
```

# 通过maturin创建library库
```shell
maturin init --bindings pyo3
```
- 运行上面的命令后，就会在string_sum初始化一个rust lib项目，这个命令相当于 cargo init
- 如果你是通过cargo new --lib string_sum方式创建的lib，就不需要执行上面的init操作。

在Cargo.toml文件中添加如下内容：
```toml
[lib]
# The name of the native library. This is the name which will be used in Python to import the
# library (i.e. `import string_sum`). If you change this, you must also change the name of the
# `#[pymodule]` in `src/lib.rs`.
name = "string_sum"

# "cdylib" is necessary to produce a shared library for Python to import from.
#
# Downstream Rust code (including code in `bin/`, `examples/`, and `tests/`) will not be able
# to `use string_sum;` unless the "rlib" or "lib" crate type is also included, e.g.:
# crate-type = ["cdylib", "rlib"]
crate-type = ["cdylib"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
pyo3 ={ version = "0.21.2",features = ["extension-module"]}
```

# lib.rs代码
```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// 注册的python模块
/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    println!("a = {},b={}", a, b);
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

# 下载相关依赖并构建模块string_sum
```shell
# 当lib.rs代码发生改变，就需要重新运行这个命令构建和变异
marturin develop
```

# 进入python3终端运行
```shell
(.env) ➜  string_sum python3
Python 3.9.6 (default, Nov 10 2023, 13:38:27)
[Clang 15.0.0 (clang-1500.1.0.2.5)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import string_sum
>>> string_sum.sum_as_string(12,1)
'13'
>>>
```

到这里，恭喜你，pyo3编写python拓展完毕！
