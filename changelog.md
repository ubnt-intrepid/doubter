<a name="0.1.0"></a>
## 0.1.0 (2018-10-15)

The initial release of current iteration

<a name="0.0.6"></a>
### 0.0.6 (2018-10-14)

* remove feature flag `external-doc`
  - use the field `use_external_doc` in `doubter!()` instead
* add initial support for build script
* add support for files outside of `CARGO_MANIFEST_DIR`
* add mode to extract code blocks in Markdown

<a name="0.0.5"></a>
### 0.0.5 (2018-10-13)

* add glob pattern support ([#4](https://github.com/ubnt-intrepid/doubter/pull/4))
* save the original directory tree as module structure ([#6](https://github.com/ubnt-intrepid/doubter/pull/6))

Compatibility Notes:
* rename the field key `file` to `include`

<a name="0.0.4"></a>
### 0.0.4 (2018-10-12)

* fix sanitization scheme of file path

<a name="0.0.3"></a>
### 0.0.3 (2018-10-11)

* revert: extract code blocks from Markdown files
* add feature flag whether to use the unstable `#[doc(include = "...")]`

<a name="0.0.2"></a>
### 0.0.2 (2018-10-11)

* extract code blocks from Markdown files

<a name="0.0.1"></a>
### 0.0.1 (2018-10-11)
First release