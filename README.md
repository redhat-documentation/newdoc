# The newdoc tool

[![Crates.io](https://img.shields.io/crates/v/newdoc)](https://crates.io/crates/newdoc)
[![Crates.io](https://img.shields.io/crates/d/newdoc)](https://crates.io/crates/newdoc)
[![Crates.io](https://img.shields.io/crates/l/newdoc)](https://crates.io/crates/newdoc)

[![Travis (.org)](https://img.shields.io/travis/redhat-documentation/newdoc)](https://travis-ci.org/github/redhat-documentation/newdoc)
[![AppVeyor](https://img.shields.io/appveyor/build/mrksu/newdoc)](https://ci.appveyor.com/project/mrksu/newdoc)
[![Copr build](https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/package/newdoc/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/package/newdoc/)

The `newdoc` tool generates pre-populated module and assembly files formatted with AsciiDoc, which are used in Red Hat and Fedora documentation. The generated files follow the Modular Documentation guidelines: <https://redhat-documentation.github.io/modular-docs/>.

## Installing newdoc

* To install `newdoc` on Fedora, RHEL, or CentOS, use the Copr package repository:

    ```
    # dnf copr enable mareksu/newdoc-rs
    # dnf install newdoc
    
    $ newdoc
    ```

    Note that the Copr repository distributes packages only for *supported* releases of Fedora. If you have enabled the repository but the package fails to install, check if your Fedora is still supported.

* To install `newdoc` from source on a Linux distribution, on macOS, or on Microsoft Windows, use the `cargo` package manager:

    ```
    $ cargo install newdoc
    
    $ newdoc
    ```

    For installing `cargo`, see <https://rustup.rs/>.
    
* To install `newdoc` as a Docker image, use the `docker` or `podman` tool. If you use `podman`, replace `docker` with `podman` in the following commands:

    ```
    $ docker pull mrksu/newdoc
    
    $ docker run mrksu/newdoc
    ```
    
    **Warning:** The container currently does not generate files properly. For details and a workaround, see [Issue #17](https://github.com/redhat-documentation/newdoc/issues/17).


## Updating newdoc

* To update `newdoc` that is installed from RPM, use the DNF package manager:

    ```
    # dnf upgrade newdoc
    ```

* To update `newdoc` from source, use the `cargo` package manager:

    ```
    $ cargo install newdoc
    ```

* To update `newdoc` from Docker, use the `docker` or `podman` tool:

    ```
    $ docker pull mrksu/newdoc
    ```


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


## Packaging and distributing newdoc as an RPM

1. Install the  `cargo` package manager. For details, see <https://rustup.rs/>.

2. Install the `cargo-rpm` extension:

    ```
    $ cargo install cargo-rpm
    ```

3. In the `newdoc` project directory, build RPM packages:

    ```
    $ cargo rpm build
    ```

    This command build packages in the `target/release/rpmbuild/` directory.

4. Log into the Copr repository administration: <https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/>.

5. Go to the **Builds** tab.

6. Click **New Build** and select **Upload**.

7. In the **Provide the source** section, upload the most recent SRPM package from the `target/release/rpmbuild/SRPMS/` directory.

8. Click **Build**.


## Packaging and distributing newdoc as a Docker image

Note: The following steps might be sub-optimal. Feel free to suggest improvements.

1. Install the `docker` or `podman` tool.

    If you use `podman`, replace `docker` with `podman` in the following commands.

2. Log into the Docker Hub account:

    ```
    $ docker login --username mrksu docker.io
    ```

3. Build a new image. For example:

    ```
    $ docker build -t mrksu/newdoc:v2.3.3 .
    ```

4. Find the Image ID of the built image:

    ```
    $ docker images
    ```

5. Tag the new version. For example:

    ```
    $ docker tag 390e73cb470d mrksu/newdoc:v2.3.3
    ```

6. Upload the new image:

    ```
    $ docker push mrksu/newdoc:v2.3.3
    ```

## Additional resources

* [Modular Documentation Reference Guide](https://redhat-documentation.github.io/modular-docs/)
* [AsciiDoc Mark-up Quick Reference for Red Hat Documentation](https://redhat-documentation.github.io/asciidoc-markup-conventions/)

