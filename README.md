# The newdoc tool

[![Crates.io](https://img.shields.io/crates/v/newdoc)](https://crates.io/crates/newdoc)
[![Crates.io](https://img.shields.io/crates/d/newdoc)](https://crates.io/crates/newdoc)
[![Crates.io](https://img.shields.io/crates/l/newdoc)](https://crates.io/crates/newdoc)

[![Travis (.org)](https://img.shields.io/travis/redhat-documentation/newdoc)](https://travis-ci.org/github/redhat-documentation/newdoc)
[![AppVeyor](https://img.shields.io/appveyor/build/mrksu/newdoc)](https://ci.appveyor.com/project/mrksu/newdoc)
[![Copr build](https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/package/newdoc/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/package/newdoc/)

The `newdoc` tool generates pre-populated module and assembly files formatted with AsciiDoc, which are used in Red Hat and Fedora documentation. The generated files follow the Modular Documentation guidelines: <https://redhat-documentation.github.io/modular-docs/>.

## Installing newdoc

* To install `newdoc` on current Fedora, RHEL 8 or later, or CentOS 8 or later, enable the Copr package repository:

    1. Enable the repository:

        ```
        # dnf copr enable mareksu/newdoc-rs
        ```

    2. Install `newdoc`:

        ```
        # dnf install newdoc
        ```

        The Copr repository distributes packages only for *supported* releases of Fedora. If you have enabled the repository but the package fails to install, check if your Fedora is still supported.

* To install `newdoc` on openSUSE:

    1. Enable the Copr package repository:

        * On openSUSE Leap 15.3:

            ```
            # zypper addrepo \
              'https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/repo/opensuse-leap-15.3/mareksu-newdoc-rs-opensuse-leap-15.3.repo'
            ```

        * On openSUSE Tumbleweed:

            ```
            # zypper addrepo \
              'https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/repo/opensuse-tumbleweed/mareksu-newdoc-rs-opensuse-tumbleweed.repo'
            ```

    2. Install `newdoc`:

        ```
        # zypper refresh
        # zypper install --allow-vendor-change newdoc
        ```

* To install `newdoc` from source on a Linux distribution, on macOS, or on Microsoft Windows, use the `cargo` package manager:

    1. Install the Rust toolchain: see <https://rustup.rs/>.

    2. Install `newdoc`:

        ```
        $ cargo install newdoc
        ```

Test that `newdoc` works:

```
$ newdoc
```

<!--
Note: The configuration files for a container image are still usable in the repo, but Docker Hub no longer provides free builds, so I'm disabling this part of instructions.

* To install `newdoc` as a Docker image, use the `docker` or `podman` tool. If you use `podman`, replace `docker` with `podman` in the following commands:

    ```
    $ docker pull mrksu/newdoc

    $ docker run mrksu/newdoc
    ```

    **Warning:** The container currently does not generate files properly. For details and a workaround, see [Issue #17](https://github.com/redhat-documentation/newdoc/issues/17).
-->

## Updating newdoc

* To update `newdoc` that is installed from RPM on Fedora, RHEL, or CentOS, use the DNF package manager:

    1. Make sure that you are using a supported release of your Linux distribution. The Copr repository does not publish `newdoc` packages for unsupported distribution releases.

    2. Refresh repository metadata and update the package:

        ```
        # dnf --refresh upgrade newdoc
        ```

* To update `newdoc` installed on openSUSE:

    1. Make sure that you are using a supported release of your Linux distribution. The Copr repository does not publish `newdoc` packages for unsupported distribution releases.

    2. Refresh repository metadata:

        ```
        # zypper refresh
        ```

    3. Update the package:

        ```
        # zypper update newdoc
        ```

* To update `newdoc` from source, use the `cargo` package manager:

    1. Update the Rust toolchain:

        ```
        $ rustup update
        ```

    2. Update `newdoc`:

        ```
        $ cargo install newdoc
        ```

<!--
Note: The configuration files for a container image are still usable in the repo, but Docker Hub no longer provides free builds, so I'm disabling this part of instructions.

* To update `newdoc` from Docker, use the `docker` or `podman` tool:

    ```
    $ docker pull mrksu/newdoc
    ```
-->

## Creating a new module

1. In the directory where modules are located, use `newdoc` to create a new file:

    ```
    modules-dir]$ newdoc --procedure "Setting up thing"
    ```

    The script also accepts the `--concept` and `--reference` options. You can use these short forms instead: `-p`, `-c`, and `-r`.

2. Rewrite the placeholders in the generated file with your docs.


## Creating a new assembly

1. In the directory where assemblies are located, use `newdoc` to create a new file:

    ```
    assemblies-dir]$ newdoc --assembly "Achieving thing"
    ```

    You can use the short form of the `--assembly` option instead: `newdoc -a "Achieving thing"`.

2. Rewrite the placeholders in the generated file with your docs.

    Add AsciiDoc include statements to include modules. See [Include Files](https://asciidoctor.org/docs/asciidoc-syntax-quick-reference/#include-files) in the AsciiDoc Syntax Quick Reference.

    Alternatively, you can use the `--include-in` option when creating the assembly to generate modules and include them automatically, in a single step. See the description in the *Options* section.


## Validating a file for Red Hat requirements

You can use the `--validate` (`-l`) option to check an existing file for Red Hat publishing requirements. For example:

```
$ newdoc --validate modules/empty-file.adoc

💾 File: empty-file.adoc
    🔴 Error: The file has no title or headings.
    🔴 Error: The file is missing an ID.
    🔶 Warning: The file is missing the _abstract flag. The flag is recommended but not required.
    🔴 Error: Cannot determine the module type.
```

```
$ newdoc --validate modules/con_proper-module.adoc

💾 File: modules/con_proper-module.adoc
    🔷 Information: No issues found in this file.
```


## Options

* To generate the file without the explanatory comments, add the `--no-comments` or `-C` option when creating documents.

* To generate the file without the example, placeholder content, add the `--no-examples` or `-E` option when creating documents.

* To create the file without the module type prefix in the ID and the file name, add the `--no-prefixes` or `-P` option.

* To specify the directory where `newdoc` saves the generated file, add the `--target-dir=<directory>` or `-T <directory>` option.

* To generate an assembly with include statements for other generated modules, use the `--include-in` or `-i` option:

    ```
    $ newdoc --include-in "An assembly for two modules" --concept "First module" --procedure "Second module"
    ```

    This creates the two modules and an assembly that features the include statements for the modules.

For more options, see the output of the following command:

```
$ newdoc --help
```

## Release notes

You can find a brief change log on the [Releases](https://github.com/redhat-documentation/newdoc/releases) page.


## Additional resources

* The `newdoc --help` command
* [Modular Documentation Reference Guide](https://redhat-documentation.github.io/modular-docs/)
* [AsciiDoc Mark-up Quick Reference for Red Hat Documentation](https://redhat-documentation.github.io/asciidoc-markup-conventions/)
