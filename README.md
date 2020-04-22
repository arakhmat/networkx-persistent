[![Build Status](https://travis-ci.com/arakhmat/networkx-persistent.svg?branch=master)](https://travis-ci.com/arakhmat/networkx-persistent)

# Persistent NetworkX Data Structures

`networkx-persistent` provides [persistent](https://en.wikipedia.org/wiki/Persistent_data_structure) versions of NetworkX Data Structures.

# Installation Instructions

## For Contributors

### Installing Dependencies

Install [Rust](https://www.rust-lang.org/tools/install) and [Conda](https://docs.conda.io/projects/conda/en/latest/user-guide/install/download.html).

### Installing nightly Rust
```bash
rustup install nightly
rustup override set nightly
```

### Installing Conda environment
```bash
conda env create -f environment.yaml
```

### Installing pyrpds
```bash
conda activate pyrpds
maturin build
```


### Testing pyrpds
```bash
conda activate pyrpds
pytest
```
